/**
 * User Info RPC API
 *
 * RPC methods for user profile information management.
 */

import type { UserInfo, UserInfoFilter } from '@/services/types/user/index'
import { rpcCall, rpcCallSilent, type RpcListResult } from '../../rpc-client'

// region:    --- Types

export interface GetUserInfoParams {
  id: number
}

export interface GetUserInfoByUserIdParams {
  user_id: number
}

export interface ListUserInfosParams {
  filters?: UserInfoFilter[]
  list_options?: ListOptions
}

/** List options for pagination and sorting */
export interface ListOptions {
  limit?: number
  offset?: number
  order_bys?: string | OrderBy[]
}

export interface OrderBy {
  field: string
  dir?: 'asc' | 'desc' | 'ASC' | 'DESC'
}

export type ListUserInfosResult = RpcListResult<UserInfo>

export interface CreateUserInfoParams {
  data: UserInfoForCreate
}

export interface UserInfoForCreate {
  user_id: number
  nickname?: string | null
  avatar?: string | null
  bio?: string | null
  email?: string | null
  phone?: string | null
  gender?: 'Unknown' | 'Male' | 'Female' | null
  birthday?: string | null
  country?: string | null
  province?: string | null
  city?: string | null
  address?: string | null
  postal_code?: string | null
  timezone?: string | null
  locale?: string | null
  theme?: string | null
}

export interface UpdateUserInfoParams {
  id: number
  data: UserInfoForUpdate
}

export interface UserInfoForUpdate {
  nickname?: string | null
  avatar?: string | null
  bio?: string | null
  email?: string | null
  email_verified?: boolean | null
  phone?: string | null
  phone_verified?: boolean | null
  gender?: 'Unknown' | 'Male' | 'Female' | null
  birthday?: string | null
  country?: string | null
  province?: string | null
  city?: string | null
  address?: string | null
  postal_code?: string | null
  timezone?: string | null
  locale?: string | null
  theme?: string | null
  status?: 'Active' | 'Inactive' | 'Suspended' | 'Deleted' | null
}

export interface DeleteUserInfoParams {
  id: number
}

// endregion: --- Types

// region:    --- RPC Methods

/**
 * Get user info by ID
 */
export async function getUserInfo(params: GetUserInfoParams): Promise<UserInfo> {
  return rpcCall<UserInfo, GetUserInfoParams>('get_user_info', params)
}

/**
 * Get user info by ID (silent, no toast on error)
 */
export async function getUserInfoSilent(params: GetUserInfoParams): Promise<UserInfo> {
  return rpcCallSilent<UserInfo, GetUserInfoParams>('get_user_info', params)
}

/**
 * Get user info by user_id (more commonly used)
 */
export async function getUserInfoByUserId(
  params: GetUserInfoByUserIdParams
): Promise<UserInfo> {
  return rpcCall<UserInfo, GetUserInfoByUserIdParams>('get_user_info_by_user_id', params)
}

/**
 * Get user info by user_id (silent, no toast on error)
 */
export async function getUserInfoByUserIdSilent(
  params: GetUserInfoByUserIdParams
): Promise<UserInfo> {
  return rpcCallSilent<UserInfo, GetUserInfoByUserIdParams>('get_user_info_by_user_id', params)
}

/**
 * Get current logged-in user's info
 */
export async function getCurrentUserInfo(): Promise<UserInfo> {
  return rpcCall<UserInfo>('get_current_user_info')
}

/**
 * Get current logged-in user's info (silent, no toast on error)
 */
export async function getCurrentUserInfoSilent(): Promise<UserInfo> {
  return rpcCallSilent<UserInfo>('get_current_user_info')
}

/**
 * List user infos with optional filters and pagination
 */
export async function listUserInfos(
  params?: ListUserInfosParams
): Promise<UserInfo[]> {
  const result = await rpcCall<ListUserInfosResult, ListUserInfosParams | undefined>(
    'list_user_infos',
    params
  )
  return result.data
}

/**
 * List user infos (silent, no toast on error)
 */
export async function listUserInfosSilent(
  params?: ListUserInfosParams
): Promise<UserInfo[]> {
  const result = await rpcCallSilent<ListUserInfosResult, ListUserInfosParams | undefined>(
    'list_user_infos',
    params
  )
  return result.data
}

/**
 * Create a new user info
 */
export async function createUserInfo(params: CreateUserInfoParams): Promise<UserInfo> {
  return rpcCall<UserInfo, CreateUserInfoParams>('create_user_info', params)
}

/**
 * Update user info
 */
export async function updateUserInfo(params: UpdateUserInfoParams): Promise<UserInfo> {
  return rpcCall<UserInfo, UpdateUserInfoParams>('update_user_info', params)
}

/**
 * Delete user info
 */
export async function deleteUserInfo(params: DeleteUserInfoParams): Promise<UserInfo> {
  return rpcCall<UserInfo, DeleteUserInfoParams>('delete_user_info', params)
}

// endregion: --- RPC Methods
