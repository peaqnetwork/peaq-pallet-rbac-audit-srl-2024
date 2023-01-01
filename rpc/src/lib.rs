use std::sync::Arc;
use std::convert::From;

use codec::Codec;
use sp_std::{vec, vec::Vec};
use sp_api::{ProvideRuntimeApi, ApiError};
use sp_blockchain::HeaderBackend;
use peaq_pallet_rbac::structs::{Entity, Role2User, Role2Group, User2Group, Permission2Role};
pub use peaq_pallet_rbac_runtime_api::PeaqRBACRuntimeApi;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};




/// Trait defines RBAC-RPC interface
#[rpc(client, server)]
pub trait PeaqRBACApi<BlockHash, AccountId, EntityId> {
	/// RPC method for extrinsic call fetchRole
	#[method(name = "peaqrbac_fetchRole")]
	fn fetch_role(
		&self, account: AccountId, entity: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Entity<EntityId>>>;

	/// RPC method for extrinsic call fetchRoles
	#[method(name = "peaqrbac_fetchRoles")]
	fn fetch_roles(
		&self, owner: AccountId, at: Option<BlockHash>
	) -> RpcResult<Vec<Entity<EntityId>>>;

	/// RPC method for extrinsic call fetchUserRoles
	#[method(name = "peaqrbac_fetchUserRoles")]
	fn fetch_user_roles(
		&self, owner: AccountId, user_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Vec<Role2User<EntityId>>>>;

	/// RPC method for extrinsic call fetchPermission
	#[method(name = "peaqrbac_fetchPermission")]
	fn fetch_permission(
		&self, owner: AccountId, permission_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Entity<EntityId>>>;

	/// RPC method for extrinsic call fetchPermissions
	#[method(name = "peaqrbac_fetchPermissions")]
	fn fetch_permissions(
		&self, owner: AccountId, at: Option<BlockHash>
	) -> RpcResult<Vec<Entity<EntityId>>>;

	/// RPC method for extrinsic call fetchRolePermissions
	#[method(name = "peaqrbac_fetchRolePermissions")]
	fn fetch_role_permissions(
		&self, owner: AccountId, role_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Vec<Permission2Role<EntityId>>>>;

	/// RPC method for extrinsic call fetchGroup
	#[method(name = "peaqrbac_fetchGroup")]
	fn fetch_group(
		&self, owner: AccountId, group_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Entity<EntityId>>>;

	/// RPC method for extrinsic call fetchGroups
	#[method(name = "peaqrbac_fetchGroups")]
	fn fetch_groups(
		&self, owner: AccountId, at: Option<BlockHash>
	) -> RpcResult<Vec<Entity<EntityId>>>;

	/// RPC method for extrinsic call fetchGroupRoles
	#[method(name = "peaqrbac_fetchGroupRoles")]
	fn fetch_group_roles(
		&self, owner: AccountId, group_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Vec<Role2Group<EntityId>>>>;
	
	/// RPC method for extrinsic call fetchUserGroups
	#[method(name = "peaqrbac_fetchUserGroups")]
	fn fetch_user_groups(
		&self, owner: AccountId, user_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Vec<User2Group<EntityId>>>>;

	/// RPC method for extrinsic call fetchUserPermissions
	#[method(name = "peaqrbac_fetchUserPermissions")]
	fn fetch_user_permissions(
		&self, owner: AccountId, user_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Vec<Entity<EntityId>>>>;
	
	/// RPC method for extrinsic call fetchGroupPermissions
	#[method(name = "peaqrbac_fetchGroupPermissions")]
	fn fetch_group_permissions(
		&self, owner: AccountId, group_id: EntityId, at: Option<BlockHash>
	) -> RpcResult<Option<Vec<Entity<EntityId>>>>;
}

/// A struct that implements the [`PeaqRBACApi`].
pub struct PeaqRBAC<Client, Block> {
	client: Arc<Client>,
	_marker: std::marker::PhantomData<Block>,
}

impl<Client, Block> PeaqRBAC<Client, Block> {
	/// Create new `PeaqRBAC` with the given reference to the client.
	pub fn new(client: Arc<Client>) -> Self {
		PeaqRBAC {
			client,
			_marker: Default::default(),
		}
	}
}

pub enum Error {
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
		}
	}
}


/// This macro simplifies copy&paste-work in every rpc-method
macro_rules! dry_api_at {
    ( $self:expr, $at:expr ) => {
        ($self.client.runtime_api(),
		BlockId::hash($at.unwrap_or(
			// If the block hash is not supplied assume the best block.
			$self.client.info().best_hash,
		)))
    };
}

/// Default error mapping in rpc methods
#[inline]
fn map_api_err(err: ApiError) -> JsonRpseeError {
	JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
		Error::RuntimeError.into(),
		"Unable to get value.",
		Some(format!("{:?}", err)),
	)))
}


#[async_trait]
impl<Client, Block, AccountId, EntityId> PeaqRBACApiServer<<Block as BlockT>::Hash, AccountId, EntityId> for PeaqRBAC<Client, Block>
where
    Block: BlockT,
    Client: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    Client::Api: PeaqRBACRuntimeApi<Block, AccountId, EntityId>,
	AccountId: Codec,
	EntityId: Codec
{
	fn fetch_role(&self,
            account: AccountId, 
            entity: EntityId, 
            at: Option<<Block as BlockT>::Hash>) -> 
		RpcResult<Option<Entity<EntityId>>>
    {
		let (api, at) = dry_api_at!(self, at);
		api.fetch_role(&at, account, entity).map_err(|e| map_api_err(e))
    }

	fn fetch_roles(&self,
			owner: AccountId,
			at: Option<<Block as BlockT>::Hash>) -> 
		RpcResult<Vec<Entity<EntityId>>>
	{
		let (api, at) = dry_api_at!(self, at);
		api.fetch_roles(&at, owner).map_err(|e| map_api_err(e))
	}

	fn fetch_user_roles(&self,
			owner: AccountId,
			user_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) -> 
		RpcResult<Option<Vec<Role2User<EntityId>>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_user_roles(&at, owner, user_id).map_err(|e| map_api_err(e))
	}

	fn fetch_permission(&self,
			owner: AccountId,
			permission_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Entity<EntityId>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_permission(&at, owner, permission_id).map_err(|e| map_api_err(e))
	}

	fn fetch_permissions(&self,
			owner: AccountId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Vec<Entity<EntityId>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_permissions(&at, owner).map_err(|e| map_api_err(e))
	}

	fn fetch_role_permissions(&self,
			owner: AccountId,
			role_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Vec<Permission2Role<EntityId>>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_role_permissions(&at, owner, role_id).map_err(|e| map_api_err(e))
	}

	fn fetch_group(&self,
			owner: AccountId,
			group_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Entity<EntityId>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_group(&at, owner, group_id).map_err(|e| map_api_err(e))
	}

	fn fetch_groups(&self,
			owner: AccountId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Vec<Entity<EntityId>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_groups(&at, owner).map_err(|e| map_api_err(e))
	}

	fn fetch_group_roles(&self,
			owner: AccountId,
			group_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Vec<Role2Group<EntityId>>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_group_roles(&at, owner, group_id).map_err(|e| map_api_err(e))
	}
	 
	fn fetch_user_groups(&self,
			owner: AccountId,
			user_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Vec<User2Group<EntityId>>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_user_groups(&at, owner, user_id).map_err(|e| map_api_err(e))
	}

	fn fetch_user_permissions(&self,
			owner: AccountId,
			user_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Vec<Entity<EntityId>>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_user_permissions(&at, owner, user_id).map_err(|e| map_api_err(e))
	}
	
	fn fetch_group_permissions(&self,
			owner: AccountId,
			group_id: EntityId,
			at: Option<<Block as BlockT>::Hash>) ->
		RpcResult<Option<Vec<Entity<EntityId>>>>
	{
		let (api, at) = dry_api_at!(self, at);
        api.fetch_group_permissions(&at, owner, group_id).map_err(|e| map_api_err(e))
	}
}
