use async_trait::async_trait;
use log::info;
use pingora::prelude::*;
use pingora::proxy::{http_proxy_service, ProxyHttp, Session};
use std::env;

pub struct Gateway {
	api_addr: String,
	api_port: u16,
	frontend_addr: String,
	frontend_port: u16,
}

impl Gateway {
	fn from_env() -> Self {
		Self {
			api_addr: env::var("UPSTREAM_API_ADDR").unwrap_or_else(|_| "web-server".into()),
			api_port: env::var("UPSTREAM_API_PORT")
				.unwrap_or_else(|_| "8080".into())
				.parse()
				.unwrap_or(8080),
			frontend_addr: env::var("UPSTREAM_FRONTEND_ADDR")
				.unwrap_or_else(|_| "frontend".into()),
			frontend_port: env::var("UPSTREAM_FRONTEND_PORT")
				.unwrap_or_else(|_| "80".into())
				.parse()
				.unwrap_or(80),
		}
	}
}

#[async_trait]
impl ProxyHttp for Gateway {
	type CTX = ();

	fn new_ctx(&self) -> Self::CTX {}

	async fn upstream_peer(
		&self,
		session: &mut Session,
		_ctx: &mut Self::CTX,
	) -> Result<Box<HttpPeer>> {
		let path = session.req_header().uri.path();

		let (addr, port) = if path.starts_with("/api") {
			(&self.api_addr, self.api_port)
		} else {
			(&self.frontend_addr, self.frontend_port)
		};

		info!("routing {} -> {}:{}", path, addr, port);

		let peer = Box::new(HttpPeer::new(
			(addr.as_str(), port),
			false,
			String::new(),
		));
		Ok(peer)
	}
}

fn main() {
	env_logger::init();

	let mut server = Server::new(None).unwrap();
	server.bootstrap();

	let gateway = Gateway::from_env();

	let listen_port = env::var("GATEWAY_LISTEN_PORT").unwrap_or_else(|_| "80".into());
	let listen_addr = format!("0.0.0.0:{listen_port}");

	info!("Pingora gateway listening on {}", listen_addr);

	let mut proxy = http_proxy_service(&server.configuration, gateway);
	proxy.add_tcp(&listen_addr);

	server.add_service(proxy);
	server.run_forever();
}
