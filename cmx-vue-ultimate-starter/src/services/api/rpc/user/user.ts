/**
 * User RPC API
 *
 * RPC methods for user account management.
 */

import type { UserWithInfo } from '@/services/types/user/index'
import { rpcCall, rpcCallSilent } from '../../rpc-client'

// region:    --- Types

export interface ListUsersParams {
  filters?: unknown[]
  page_size?: number
  page_number?: number
}

export interface PageInfo {
  total: number
  page_size: number
  page_number: number
  total_pages: number
  has_more: boolean
}

export interface PagedResult<T> {
  data: T[]
  page_info: PageInfo
}

export type ListUsersResult = PagedResult<UserWithInfo>

export interface CreateUserParams {
  username: string
  password: string
}

export interface ResetUserPwdParams {
  user_id: number
  new_password: string
}

// endregion: --- Types

// region:    --- RPC Methods

/**
 * List users with their profile info (LEFT JOIN user_info)
 */
export async function listUsers(params?: ListUsersParams): Promise<ListUsersResult> {
  const result = await rpcCall<{ data: ListUsersResult }, ListUsersParams | undefined>(
    'list_users',
    params
  )
  return result.data
}

/**
 * List users (silent, no toast on error)
 */
export async function listUsersSilent(params?: ListUsersParams): Promise<ListUsersResult> {
  const result = await rpcCallSilent<{ data: ListUsersResult }, ListUsersParams | undefined>(
    'list_users',
    params
  )
  return result.data
}

/**
 * Create a new user account
 */
export async function createUser(params: CreateUserParams): Promise<number> {
  const result = await rpcCall<{ data: number }, CreateUserParams>('create_user', params)
  return result.data
}

/**
 * Reset user password
 */
export async function resetUserPwd(params: ResetUserPwdParams): Promise<void> {
  await rpcCall<{ data: null }, ResetUserPwdParams>('reset_user_pwd', params)
}

// endregion: --- RPC Methods
