/**
 * Role RPC API
 *
 * RPC methods for role management and access control.
 */

import type { Role } from '@/services/types/acs/Role'
import { rpcCall, rpcCallSilent } from '../../rpc-client'

// region:    --- Types

export interface ListRolesParams {
  filters?: unknown[]
  list_options?: {
    limit?: number
    offset?: number
    order_bys?: string
  }
}

export interface ListRolesResult {
  data: Role[]
}

export interface CreateRoleParams {
  name: string
  display_name?: string | null
  description?: string | null
}

export interface UpdateRoleParams {
  id: bigint
  data: {
    name?: string
    display_name?: string | null
    description?: string | null
  }
}

export interface DeleteRoleParams {
  id: bigint
}

export interface GetRoleParams {
  id: bigint
}

// endregion: --- Types

// region:    --- RPC Methods

/**
 * List all roles
 */
export async function listRoles(params?: ListRolesParams): Promise<Role[]> {
  const result = await rpcCall<ListRolesResult, ListRolesParams | undefined>(
    'list_roles',
    params
  )
  return result.data
}

/**
 * List roles (silent, no toast on error)
 */
export async function listRolesSilent(params?: ListRolesParams): Promise<Role[]> {
  const result = await rpcCallSilent<ListRolesResult, ListRolesParams | undefined>(
    'list_roles',
    params
  )
  return result.data
}

/**
 * Get a role by ID
 */
export async function getRole(params: GetRoleParams): Promise<Role> {
  return rpcCall<Role, GetRoleParams>('get_role', params)
}

/**
 * Create a new role
 */
export async function createRole(params: CreateRoleParams): Promise<bigint> {
  return rpcCall<bigint, { data: CreateRoleParams }>('create_role', { data: params })
}

/**
 * Update a role
 */
export async function updateRole(params: UpdateRoleParams): Promise<void> {
  return rpcCall<void, UpdateRoleParams>('update_role', params)
}

/**
 * Delete a role
 */
export async function deleteRole(params: DeleteRoleParams): Promise<void> {
  return rpcCall<void, DeleteRoleParams>('delete_role', params)
}

// endregion: --- RPC Methods
