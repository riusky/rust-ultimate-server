/**
 * User Authentication REST API
 */

import { restClient, restClientSilent } from '../../rest-client'

// region:    --- Types

export interface LoginPayload {
  username: string
  pwd: string
}

export interface LoginResponse {
  result: {
    success: boolean
  }
}

export interface LogoffPayload {
  logoff: boolean
}

export interface LogoffResponse {
  result: {
    logged_off: boolean
  }
}

/** Response from GET /api/me (session check). */
export interface MeResponse {
  result: {
    user_id: number
    username: string
  }
}

// endregion: --- Types

// region:    --- API Functions

/**
 * Login with username and password
 */
export async function login(payload: LoginPayload): Promise<LoginResponse> {
  const response = await restClient.post<LoginResponse>('/api/login', payload)
  return response.data
}

/**
 * Logoff the current user
 */
export async function logoff(): Promise<LogoffResponse> {
  const response = await restClient.post<LogoffResponse>('/api/logoff', { logoff: true })
  return response.data
}

/**
 * Get current user (session check). Uses silent client so 401 does not show toast.
 * Caller should clear login state and redirect to login on rejection.
 */
export async function getCurrentUser(): Promise<MeResponse> {
  const response = await restClientSilent.get<MeResponse>('/api/me')
  return response.data
}

// endregion: --- API Functions
