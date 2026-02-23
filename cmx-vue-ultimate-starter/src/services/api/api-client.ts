import axios, { AxiosError, type AxiosInstance, type AxiosResponse } from 'axios'
import { toast } from 'vue-sonner'

import { i18n } from '@/plugins/i18n'
import env from '@/utils/env'

// region:    --- Types

/**
 * Backend API error response structure
 */
export interface ApiErrorResponse {
  success: false
  error: {
    code: number
    biz_code: string
    message: string
    detail?: unknown
    resource?: string
    action?: string
  }
  meta?: {
    req_uuid: string
  }
}

/**
 * Backend API success response structure (for RPC)
 */
export interface ApiSuccessResponse<T = unknown> {
  success: true
  result: T
  meta?: {
    req_uuid: string
  }
}

// region:    --- REST Response Types

/**
 * REST single entity response (GET /resource/:id, POST, PUT, DELETE)
 * @example { "success": true, "data": { "id": 1, "name": "test" } }
 */
export interface RestResponse<T = unknown> {
  success: true
  data: T
}

/**
 * REST list response without pagination
 * @example { "success": true, "data": [{ "id": 1 }, { "id": 2 }] }
 */
export interface RestListResponse<T = unknown> {
  success: true
  data: T[]
}

/**
 * Page info structure for paginated responses
 */
export interface PageInfo {
  total: number
  page_size: number
  page_number: number
  total_pages: number
  has_more: boolean
}

/**
 * REST paged response with pagination info
 * @example { "success": true, "data": [...], "page_info": { "total": 100, ... } }
 */
export interface RestPagedResponse<T = unknown> {
  success: true
  data: T[]
  page_info: PageInfo
}

// Keep alias for backwards compatibility
export type ApiPagedResponse<T = unknown> = RestPagedResponse<T>

// endregion: --- REST Response Types

// endregion: --- Types

// region:    --- Error Handling Utils

/**
 * Get localized error message from business error code
 * This is a standalone function that can be used outside of Vue components
 */
function getBizErrorMessage(bizCode: string | undefined | null): string {
  const t = i18n.global.t as (key: string) => string

  if (!bizCode) {
    return t('bizErrors.UNKNOWN')
  }

  // Check if translation exists for this biz_code
  const key = `bizErrors.${bizCode}`
  const te = i18n.global.te as (key: string) => boolean
  if (te(key)) {
    return t(key)
  }

  // Fallback to UNKNOWN
  return t('bizErrors.UNKNOWN')
}

/**
 * Get network error message based on error type
 */
function getNetworkErrorMessage(error: AxiosError): string {
  const t = i18n.global.t as (key: string) => string

  if (error.code === 'ECONNABORTED' || error.message.includes('timeout')) {
    return t('errorMessages.serverError') // Connection timeout
  }

  if (!error.response) {
    return t('errorMessages.serverError') // Network error
  }

  return t('errorMessages.serverError')
}

/**
 * Check if response is an API error response
 */
function isApiErrorResponse(data: unknown): data is ApiErrorResponse {
  return (
    typeof data === 'object' &&
    data !== null &&
    'success' in data &&
    data.success === false &&
    'error' in data
  )
}

// endregion: --- Error Handling Utils

// region:    --- 401 AUTH#002 callback (avoid circular dependency with router/pinia)

/** Called when any request returns 401 with biz_code AUTH#002. Register in app bootstrap to clear login and redirect. */
let on401Auth002Callback: (() => void) | null = null

export function setOn401Auth002Callback(cb: () => void) {
  on401Auth002Callback = cb
}

// endregion: --- 401 AUTH#002 callback

// region:    --- Base Client Configuration

/**
 * Base axios configuration
 */
const baseConfig = {
  baseURL: env.VITE_SERVER_API_URL,
  timeout: env.VITE_SERVER_API_TIMEOUT,
  withCredentials: true, // Important: send cookies for authentication
}

/**
 * Create response interceptor for error handling
 * @param showToast - Whether to show toast notifications for errors
 */
function createErrorInterceptor(showToast = true) {
  return {
    onFulfilled: (response: AxiosResponse) => {
      // Check if response body indicates an error (for cases where HTTP status is 2xx but business logic failed)
      if (isApiErrorResponse(response.data)) {
        const errorData = response.data
        const message = getBizErrorMessage(errorData.error.biz_code)

        console.debug('[API] Business error in 2xx response:', errorData)

        if (showToast) {
          toast.error(message)
        }

        // Reject with error data for further handling
        return Promise.reject(new ApiError(errorData))
      }

      return response
    },
    onRejected: (error: AxiosError<ApiErrorResponse>) => {
      console.log('[API] Request failed:', {
        status: error.response?.status,
        data: error.response?.data,
        isApiError: error.response?.data ? isApiErrorResponse(error.response.data) : false,
        showToast,
      })

      let message: string

      if (error.response?.data && isApiErrorResponse(error.response.data)) {
        // Business error with biz_code
        const errorData = error.response.data
        const is401NoAuth =
          error.response.status === 401 && errorData.error.biz_code === 'AUTH#002'

        if (is401NoAuth && on401Auth002Callback) {
          on401Auth002Callback()
          return Promise.reject(new ApiError(errorData))
        }

        message = getBizErrorMessage(errorData.error.biz_code)

        console.log('[API] Business error:', { bizCode: errorData.error.biz_code, message, showToast })

        if (showToast) {
          console.log('[API] Calling toast.error with message:', message)
          toast.error(message)
        }

        return Promise.reject(new ApiError(errorData))
      } else {
        // Network error or unknown error
        message = getNetworkErrorMessage(error)

        console.log('[API] Network/unknown error:', message)

        if (showToast) {
          toast.error(message)
        }

        return Promise.reject(error)
      }
    },
  }
}

// endregion: --- Base Client Configuration

// region:    --- Custom Error Class

/**
 * Custom API Error class for typed error handling
 */
export class ApiError extends Error {
  public readonly code: number
  public readonly bizCode: string
  public readonly detail?: unknown
  public readonly resource?: string
  public readonly action?: string
  public readonly reqUuid?: string

  constructor(errorResponse: ApiErrorResponse) {
    super(errorResponse.error.message)
    this.name = 'ApiError'
    this.code = errorResponse.error.code
    this.bizCode = errorResponse.error.biz_code
    this.detail = errorResponse.error.detail
    this.resource = errorResponse.error.resource
    this.action = errorResponse.error.action
    this.reqUuid = errorResponse.meta?.req_uuid
  }

  /**
   * Check if this is a specific business error
   */
  isBizError(bizCode: string): boolean {
    return this.bizCode === bizCode
  }

  /**
   * Check if this is an authentication error
   */
  isAuthError(): boolean {
    return this.bizCode.startsWith('AUTH#')
  }

  /**
   * Check if this is a permission error
   */
  isPermissionError(): boolean {
    return this.bizCode.startsWith('PERM#')
  }
}

// endregion: --- Custom Error Class

// region:    --- Client Factory

/**
 * Create an axios instance with error handling
 * @param options - Configuration options
 */
export function createApiClient(options: {
  baseURL?: string
  showToast?: boolean
}): AxiosInstance {
  const { baseURL = baseConfig.baseURL, showToast = true } = options

  const client = axios.create({
    ...baseConfig,
    baseURL,
  })

  // Add request interceptor for common headers
  client.interceptors.request.use(
    (config) => {
      // Add any common request headers here
      // e.g., config.headers['X-Request-ID'] = generateRequestId()
      return config
    },
    (error) => Promise.reject(error)
  )

  // Add response interceptor for error handling
  const errorInterceptor = createErrorInterceptor(showToast)
  client.interceptors.response.use(
    errorInterceptor.onFulfilled,
    errorInterceptor.onRejected
  )

  return client
}

// endregion: --- Client Factory

// region:    --- Exports

export { getBizErrorMessage }

// endregion: --- Exports
