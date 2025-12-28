import { restClient } from './rest-client'

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

// endregion: --- API Functions
