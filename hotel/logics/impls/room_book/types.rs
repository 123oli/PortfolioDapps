use ink::prelude::string::String;
use openbrush::traits::Timestamp;
use openbrush::{
    contracts::ownable::OwnableError,
    storage::Mapping,
    traits::{AccountId, ZERO_ADDRESS},
};

// type defination for `room_id`, `agreement_id`, `rent_id`
pub type RoomId = i32;
pub type AgreementId = i32;
pub type RentId = i32;

// pub const ROOM_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Room);

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
// #[openbrush::upgradeable_storage(ROOM_STORAGE_KEY)]
pub struct Room {
    pub room_id: RoomId,
    pub agreement_id: AgreementId,
    pub room_name: String,
    pub room_address: String,
    pub rent_per_month: u128,
    pub security_deposit: u128,
    pub time_stamp: Timestamp,
    pub vacant: bool,
    pub landlord: AccountId,
    pub current_tenant: AccountId,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            room_id: Default::default(),
            agreement_id: Default::default(),
            room_name: Default::default(),
            room_address: Default::default(),
            rent_per_month: Default::default(),
            security_deposit: Default::default(),
            time_stamp: Default::default(),
            vacant: Default::default(),
            landlord: ZERO_ADDRESS.into(),
            current_tenant: ZERO_ADDRESS.into(),
        }
    }
}

// pub const ROOM_AGREEMENT_STORAGE_KEY: u32 = openbrush::storage_unique_key!(RoomAgreement);

#[derive(scale::Decode, Default, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
// #[openbrush::upgradeable_storage(ROOM_AGREEMENT_STORAGE_KEY)]
pub struct RoomAgreement {
    pub room_id: RoomId,
    pub agreement_id: AgreementId,
    pub room_name: String,
    pub room_address: String,
    pub rent_per_month: u128,
    pub security_deposit: u128,
    pub lock_in_period: i32,
    pub time_stamp: Timestamp,
}

// pub const ROOM_RENT_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Rent);

#[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
// #[openbrush::upgradeable_storage(ROOM_RENT_STORAGE_KEY)]
pub struct Rent {
    pub rent_id: RentId,
    pub room_id: RoomId,
    pub agreement_id: AgreementId,
    pub room_name: String,
    pub room_address: String,
    pub rent_per_month: u128,
    pub time_stamp: Timestamp,
    pub tenant_address: AccountId,
    pub land_lord_address: AccountId,
}

// rent default implementation
impl Default for Rent {
    fn default() -> Self {
        Rent {
            rent_id: Default::default(),
            room_id: Default::default(),
            agreement_id: Default::default(),
            room_name: Default::default(),
            room_address: Default::default(),
            rent_per_month: Default::default(),
            time_stamp: Default::default(),
            tenant_address: ZERO_ADDRESS.into(),
            land_lord_address: ZERO_ADDRESS.into(),
        }
    }
}

pub const HOTEL_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(HOTEL_STORAGE_KEY)]

pub struct Data {
    pub tenant: AccountId,
    pub land_lord: AccountId,
    pub room_id: i32,
    pub agreement_id: i32,
    pub rent_id: i32,

    pub room: Mapping<RoomId, Room>,
    pub agreement: Mapping<AgreementId, RoomAgreement>,
    pub rent: Mapping<RentId, Rent>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            tenant: ZERO_ADDRESS.into(),
            land_lord: ZERO_ADDRESS.into(),
            room_id: Default::default(),
            agreement_id: Default::default(),
            rent_id: Default::default(),

            room: Mapping::default(),
            agreement: Mapping::default(),
            rent: Mapping::default(),
        }
    }
}

#[derive(scale::Decode, scale::Encode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum HotelError {
    // Caller is not a marketplace owner.
    OwnableError(OwnableError),
    CallerIsNotOwner,
    CallerIsOwner,
    NotEnoughAgreementFee,
    RoomIsNotVacant,
    NotATenantAddress,
    NotEnoughRentFee,
    RoomNotFound,
    RoomIsVacant,
}

impl From<OwnableError> for HotelError {
    fn from(error: OwnableError) -> Self {
        HotelError::OwnableError(error)
    }
}
