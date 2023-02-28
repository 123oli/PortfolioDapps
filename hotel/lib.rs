#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod hotel {

    use ink::{prelude::string::String, storage::Mapping};

    pub type RoomId = i32;
    pub type AgreementId = i32;
    pub type RentId = i32;

    // Room struct
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Room {
        room_id: RoomId,
        agreement_id: AgreementId,
        room_name: String,
        room_address: String,
        rent_per_month: u128,
        security_deposit: u128,
        time_stamp: Timestamp,
        vacant: bool,
        landlord: AccountId,
        current_tenant: AccountId,
    }

    // room agreement
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct RoomAgreement {
        room_id: RoomId,
        agreement_id: AgreementId,
        room_name: String,
        room_address: String,
        rent_per_month: u128,
        security_deposit: u128,
        lock_in_period: i32,
        time_stamp: Timestamp,
    }

    // room rent
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Rent {
        rent_id: RentId,
        room_id: RoomId,
        agreement_id: AgreementId,
        room_name: String,
        room_address: String,
        rent_per_month: u128,
        time_stamp: Timestamp,
        tenant_address: AccountId,
        land_lord_address: AccountId,
    }

    #[ink(storage)]
    pub struct Hotel {
        tenant: AccountId,
        land_lord: AccountId,
        no_of_rooms: i32,
        no_of_agreement: i32,
        no_of_rent: i32,

        room_by_id: Mapping<RoomId, Room>,
        room_agreement_by_id: Mapping<AgreementId, RoomAgreement>,
        rent_by_id: Mapping<RentId, Rent>,
    }

    impl Hotel {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn add_room(&self) {}
        pub fn sign_aggrement(&self, room_id: RoomId) {}
        pub fn pay_rent(&self, room_id: RoomId) {}
        pub fn agreement_completed(&self, room_id: RoomId) {}
        pub fn agreement_terminated(&self, room_id: RoomId) {}
    }
}
