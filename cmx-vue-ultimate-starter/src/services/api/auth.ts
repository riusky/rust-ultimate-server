import axios from 'axios'

import env from '@/utils/env'

const apiClient = axios.create({
  baseURL: env.VITE_SERVER_API_URL,
  timeout: env.VITE_SERVER_API_TIMEOUT,
  withCredentials: true, // 重要：发送 cookies
})

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

export async function login(payload: LoginPayload): Promise<LoginResponse> {
  const response = await apiClient.post<LoginResponse>('/api/login', payload)
  return response.data
}

export async function logoff(): Promise<LogoffResponse> {
  const response = await apiClient.post<LogoffResponse>('/api/logoff', { logoff: true })
  return response.data
}

// endregion: --- API Functions
