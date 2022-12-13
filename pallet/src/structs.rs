use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug, Serialize, Deserialize
)]
pub struct Entity<EntityId> {
    pub id: EntityId,
    pub name: Vec<u8>,
    pub enabled: bool,
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct Role2User<EntityId> {
    pub role: EntityId,
    pub user: EntityId,
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct Role2Group<EntityId> {
    pub role: EntityId,
    pub group: EntityId,
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct User2Group<EntityId> {
    pub user: EntityId,
    pub group: EntityId,
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Default, TypeInfo, Decode, Encode, RuntimeDebug,
)]
pub struct Permission2Role<EntityId> {
    pub permission: EntityId,
    pub role: EntityId,
}
