/**
 * API Services Index
 *
 * Re-exports all API clients and utilities for convenient imports.
 *
 * @example
 * ```ts
 * import { restClient, rpcCall, ApiError } from '@/services/api'
 * import { login, logoff } from '@/services/api/rest'
 * import { getUserInfoByUserId } from '@/services/api/rpc'
 * ```
 */

// Base API client and types
export {
  ApiError,
  createApiClient,
  getBizErrorMessage,
  type ApiErrorResponse,
  type ApiSuccessResponse,
  // REST response types
  type RestResponse,
  type RestListResponse,
  type RestPagedResponse,
  type PageInfo,
  // Backwards compatibility alias
  type ApiPagedResponse,
} from './api-client'

// REST client
export { restClient, restClientSilent } from './rest-client'

// RPC client
export {
  rpcClient,
  rpcClientSilent,
  rpcCall,
  rpcCallSilent,
  type RpcRequest,
  type RpcResponse,
} from './rpc-client'

// REST API modules
export * from './rest/index'

// RPC API modules
export * from './rpc/index'
