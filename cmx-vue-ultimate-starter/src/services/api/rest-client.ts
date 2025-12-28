/**
 * REST API Client
 *
 * Use this client for RESTful API calls.
 * Error handling and toast notifications are automatically applied.
 *
 * @example
 * ```ts
 * import { restClient } from '@/services/api/rest-client'
 *
 * // GET request
 * const response = await restClient.get<User[]>('/api/users')
 *
 * // POST request
 * const response = await restClient.post<User>('/api/users', { name: 'John' })
 * ```
 */

import { createApiClient, type ApiSuccessResponse, type ApiPagedResponse } from './api-client'

// region:    --- REST Client Instance

/**
 * REST API client with automatic error handling and toast notifications
 */
export const restClient = createApiClient({
  showToast: true,
})

/**
 * REST API client without toast notifications
 * Use this when you want to handle errors manually
 */
export const restClientSilent = createApiClient({
  showToast: false,
})

// endregion: --- REST Client Instance

// region:    --- Type Re-exports

export type { ApiSuccessResponse, ApiPagedResponse }
export { ApiError } from './api-client'

// endregion: --- Type Re-exports
