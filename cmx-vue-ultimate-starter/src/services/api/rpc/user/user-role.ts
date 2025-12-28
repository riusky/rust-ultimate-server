/**
 * User Role RPC API
 *
 * RPC methods for managing user role assignments.
 */

import type { Role } from '@/services/types/acs/Role'
import { rpcCall } from '../../rpc-client'

// region:    --- Types

export interface ListRolesForUserParams {
  user_id: bigint
}

export interface SetRolesForUserParams {
  user_id: bigint
  role_ids: bigint[]
}

// endregion: --- Types

// region:    --- RPC Methods

/**
 * List all roles assigned to a user
 */
export async function listRolesForUser(params: ListRolesForUserParams): Promise<Role[]> {
  const result = await rpcCall<{ data: Role[] }, ListRolesForUserParams>(
    'list_roles_for_user',
    params
  )
  return result.data
}

/**
 * Set roles for a user (replaces all existing roles)
 */
export async function setRolesForUser(params: SetRolesForUserParams): Promise<void> {
  await rpcCall<{ data: null }, SetRolesForUserParams>('set_roles_for_user', params)
}

// endregion: --- RPC Methods
