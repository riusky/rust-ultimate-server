/**
 * Role Permission RPC API
 *
 * RPC methods for managing role permission assignments.
 */

import type { Permission } from '@/services/types/acs/Permission'
import { rpcCall, rpcCallSilent } from '../../rpc-client'

// region:    --- Types

export interface ListPermissionsForRoleParams {
  role_id: bigint
}

export interface SetPermissionsForRoleParams {
  role_id: bigint
  permission_ids: bigint[]
}

// endregion: --- Types

// region:    --- RPC Methods

/**
 * List all permissions assigned to a role
 */
export async function listPermissionsForRole(params: ListPermissionsForRoleParams): Promise<Permission[]> {
  const result = await rpcCall<{ data: Permission[] }, ListPermissionsForRoleParams>(
    'list_permissions_for_role',
    params
  )
  return result.data
}

/**
 * Set permissions for a role (replaces all existing permissions)
 */
export async function setPermissionsForRole(params: SetPermissionsForRoleParams): Promise<void> {
  await rpcCall<{ data: null }, SetPermissionsForRoleParams>('set_permissions_for_role', params)
}

/**
 * List all available permissions
 */
export async function listAllPermissions(): Promise<Permission[]> {
  const result = await rpcCall<{ data: Permission[] }, undefined>(
    'list_all_permissions',
    undefined
  )
  return result.data
}

/**
 * List all available permissions (silent, no toast on error)
 */
export async function listAllPermissionsSilent(): Promise<Permission[]> {
  const result = await rpcCallSilent<{ data: Permission[] }, undefined>(
    'list_all_permissions',
    undefined
  )
  return result.data
}

// endregion: --- RPC Methods
