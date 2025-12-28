/**
 * RPC API Client
 *
 * Use this client for JSON-RPC style API calls.
 * Error handling and toast notifications are automatically applied.
 *
 * @example
 * ```ts
 * import { rpcClient, rpcCall } from '@/services/api/rpc-client'
 *
 * // Using rpcCall helper
 * const result = await rpcCall<User>('user.get', { id: 123 })
 *
 * // Direct client usage
 * const response = await rpcClient.post('/api/rpc', {
 *   method: 'user.get',
 *   params: { id: 123 }
 * })
 * ```
 */

import { createApiClient, type ApiSuccessResponse } from './api-client'

// region:    --- RPC Types

/**
 * RPC request payload structure (JSON-RPC 2.0)
 */
export interface RpcRequest<P = unknown> {
  jsonrpc: '2.0'
  id: number
  method: string
  params?: P
}

/**
 * RPC response structure (same as REST but with result)
 */
export type RpcResponse<T = unknown> = ApiSuccessResponse<T>

// endregion: --- RPC Types

// region:    --- RPC Client Instance

/**
 * RPC API client with automatic error handling and toast notifications
 */
export const rpcClient = createApiClient({
  showToast: true,
})

/**
 * RPC API client without toast notifications
 * Use this when you want to handle errors manually
 */
export const rpcClientSilent = createApiClient({
  showToast: false,
})

// endregion: --- RPC Client Instance

// region:    --- RPC Helper Functions

const RPC_ENDPOINT = '/api/rpc'
let rpcRequestId = 0

/**
 * Generate a unique request ID for JSON-RPC
 */
function getNextRequestId(): number {
  return ++rpcRequestId
}

/**
 * Make an RPC call with automatic type inference
 * @param method - RPC method name (e.g., 'user.get', 'permission.list')
 * @param params - Optional parameters for the RPC call
 * @param options - Additional options
 */
export async function rpcCall<TResult = unknown, TParams = unknown>(
  method: string,
  params?: TParams,
  options?: {
    silent?: boolean
  }
): Promise<TResult> {
  const client = options?.silent ? rpcClientSilent : rpcClient

  const response = await client.post<RpcResponse<TResult>>(RPC_ENDPOINT, {
    jsonrpc: '2.0',
    id: getNextRequestId(),
    method,
    params,
  })

  return response.data.result
}

/**
 * Make a silent RPC call (no toast notifications)
 * Shorthand for rpcCall with silent: true
 */
export async function rpcCallSilent<TResult = unknown, TParams = unknown>(
  method: string,
  params?: TParams
): Promise<TResult> {
  return rpcCall<TResult, TParams>(method, params, { silent: true })
}

// endregion: --- RPC Helper Functions

// region:    --- Type Re-exports

export type { ApiSuccessResponse }
export { ApiError } from './api-client'

// endregion: --- Type Re-exports
