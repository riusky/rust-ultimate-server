//! Publish/Subscribe commands: PUBLISH, SUBSCRIBE, PSUBSCRIBE, UNSUBSCRIBE

use crate::Result;

/// Publish a message to a channel
pub async fn publish<C, Ch, M>(conn: &mut C, channel: Ch, message: M) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	Ch: redis::ToRedisArgs + Send + Sync,
	M: redis::ToRedisArgs + Send + Sync,
{
	let subscribers: u64 = redis::cmd("PUBLISH")
		.arg(channel)
		.arg(message)
		.query_async(conn)
		.await?;
	Ok(subscribers)
}

/// Publish a JSON-serializable message to a channel
pub async fn publish_json<C, Ch, M>(conn: &mut C, channel: Ch, message: &M) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	Ch: redis::ToRedisArgs + Send + Sync,
	M: serde::Serialize,
{
	let json = serde_json::to_string(message)?;
	publish(conn, channel, json).await
}

/// Get list of active channels matching pattern
pub async fn pubsub_channels<C>(conn: &mut C, pattern: Option<&str>) -> Result<Vec<String>>
where
	C: redis::aio::ConnectionLike + Send,
{
	let mut cmd = redis::cmd("PUBSUB");
	cmd.arg("CHANNELS");
	if let Some(p) = pattern {
		cmd.arg(p);
	}
	let channels: Vec<String> = cmd.query_async(conn).await?;
	Ok(channels)
}

/// Get the number of subscribers for channels
pub async fn pubsub_numsub<C>(conn: &mut C, channels: &[&str]) -> Result<Vec<(String, u64)>>
where
	C: redis::aio::ConnectionLike + Send,
{
	let mut cmd = redis::cmd("PUBSUB");
	cmd.arg("NUMSUB");
	for ch in channels {
		cmd.arg(*ch);
	}
	let result: Vec<(String, u64)> = cmd.query_async(conn).await?;
	Ok(result)
}

/// Get the number of pattern subscriptions
pub async fn pubsub_numpat<C>(conn: &mut C) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
{
	let count: u64 = redis::cmd("PUBSUB")
		.arg("NUMPAT")
		.query_async(conn)
		.await?;
	Ok(count)
}

// Note: For actual subscription handling, use redis::aio::PubSub
// This module provides the basic publish and monitoring commands.
// Full subscription support requires dedicated connection handling.

use std::sync::Arc;
use tokio::sync::mpsc;

/// Message received from a subscription
#[derive(Debug, Clone)]
pub struct PubSubMessage {
	pub channel: String,
	pub payload: String,
}

/// A simple PubSub subscriber that runs in the background
pub struct Subscriber {
	rx: mpsc::Receiver<PubSubMessage>,
	_handle: tokio::task::JoinHandle<()>,
}

impl Subscriber {
	/// Create a new subscriber for the given channels
	pub async fn new(url: &str, channels: Vec<String>) -> Result<Self> {
		let client = redis::Client::open(url)?;
		let mut pubsub = client.get_async_pubsub().await?;

		for channel in &channels {
			pubsub.subscribe(channel).await?;
		}

		let (tx, rx) = mpsc::channel(100);

		let handle = tokio::spawn(async move {
			let mut stream = pubsub.on_message();
			while let Some(msg) = stream.next().await {
				let channel: String = msg.get_channel_name().to_string();
				if let Ok(payload) = msg.get_payload::<String>() {
					let _ = tx
						.send(PubSubMessage { channel, payload })
						.await;
				}
			}
		});

		Ok(Self { rx, _handle: handle })
	}

	/// Create a subscriber with pattern matching
	pub async fn with_patterns(url: &str, patterns: Vec<String>) -> Result<Self> {
		let client = redis::Client::open(url)?;
		let mut pubsub = client.get_async_pubsub().await?;

		for pattern in &patterns {
			pubsub.psubscribe(pattern).await?;
		}

		let (tx, rx) = mpsc::channel(100);

		let handle = tokio::spawn(async move {
			let mut stream = pubsub.on_message();
			while let Some(msg) = stream.next().await {
				let channel: String = msg.get_channel_name().to_string();
				if let Ok(payload) = msg.get_payload::<String>() {
					let _ = tx
						.send(PubSubMessage { channel, payload })
						.await;
				}
			}
		});

		Ok(Self { rx, _handle: handle })
	}

	/// Receive the next message
	pub async fn recv(&mut self) -> Option<PubSubMessage> {
		self.rx.recv().await
	}

	/// Try to receive a message without blocking
	pub fn try_recv(&mut self) -> Option<PubSubMessage> {
		self.rx.try_recv().ok()
	}
}

/// A shared subscriber that can be cloned
pub struct SharedSubscriber {
	inner: Arc<tokio::sync::Mutex<Subscriber>>,
}

impl SharedSubscriber {
	pub async fn new(url: &str, channels: Vec<String>) -> Result<Self> {
		let subscriber = Subscriber::new(url, channels).await?;
		Ok(Self {
			inner: Arc::new(tokio::sync::Mutex::new(subscriber)),
		})
	}

	pub async fn recv(&self) -> Option<PubSubMessage> {
		self.inner.lock().await.recv().await
	}
}

impl Clone for SharedSubscriber {
	fn clone(&self) -> Self {
		Self {
			inner: Arc::clone(&self.inner),
		}
	}
}

use futures_util::StreamExt;

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils::{init_test, test_key_prefix};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_publish_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_pubsub");
		let channel = format!("{}:channel", prefix);

		// -- Exec (publish without subscribers returns 0)
		let subs = publish(&mut *conn, &channel, "hello").await?;

		// -- Check
		assert_eq!(subs, 0); // No subscribers

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_pubsub_channels_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;

		// -- Exec
		let channels = pubsub_channels(&mut *conn, Some("nonexistent:*")).await?;

		// -- Check
		assert!(channels.is_empty());

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_pubsub_numpat_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;

		// -- Exec
		let count = pubsub_numpat(&mut *conn).await?;

		// -- Check (should succeed without error, actual value depends on server state)
		// count is u64, always >= 0, so we just verify the call succeeded
		let _ = count;

		Ok(())
	}
}

// endregion: --- Tests
