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

    // room default implementation
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
                landlord: [0u8; 32].into(),
                current_tenant: [0u8; 32].into(),
            }
        }
    }

    // room agreement
    #[derive(scale::Decode, Default, scale::Encode, Debug, Clone, Eq, PartialEq)]
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
                tenant_address: [0u8; 32].into(),
                land_lord_address: [0u8; 32].into(),
            }
        }
    }

    #[ink(storage)]
    pub struct Hotel {
        tenant: AccountId,
        land_lord: AccountId,
        no_of_rooms: i32,
        no_of_agreement: i32,
        no_of_rent: i32,

        room: Mapping<RoomId, Room>,
        agreement: Mapping<AgreementId, RoomAgreement>,
        rent: Mapping<RentId, Rent>,
    }

    impl Hotel {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tenant: [0u8; 32].into(),
                land_lord: [0u8; 32].into(),
                no_of_rooms: Default::default(),
                no_of_agreement: Default::default(),
                no_of_rent: Default::default(),

                room: Mapping::default(),
                agreement: Mapping::default(),
                rent: Mapping::default(),
            }
        }

        // only owner can create
        #[ink(message)]
        pub fn add_room(
            &mut self,
            room_name: String,
            room_address: String,
            rent_per_month: u128,
            security_deposit: u128,
            time_stamp: Timestamp,
        ) {
            let caller = self.env().caller();

            let new_room = Room {
                room_id: 1,
                agreement_id: 1,
                room_name,
                room_address,
                rent_per_month,
                security_deposit,
                time_stamp,
                vacant: false,
                landlord: caller,
                current_tenant: [0u8; 32].into(),
            };

            // insert data to the room with respect to id
            self.room.insert(1, &new_room);
        }

        // user shouldn't be owner/landlord to sign_aggrement
        // enough agreement fee
        // agreement happens when particular room is empty

        #[ink(message, payable)]
        pub fn sign_aggrement(&mut self, room_id: RoomId) {
            // contract caller
            let caller = self.env().caller();

            // get the room of specific `room_id`
            let mut room = self.room.get(room_id).unwrap_or_default();

            // room landlord of the hotel
            let landlord = room.landlord;

            // `total_fee` to sign room agreement
            let total_fee = room.rent_per_month + room.security_deposit;

            // transfer `total_fee` to landlord
            self.env().transfer(landlord, total_fee).unwrap_or_default();

            // increas `no_of_agreements` of the room
            self.no_of_agreement += 1;

            // update room data for success agreement
            room.current_tenant = caller;
            room.vacant = false;
            room.time_stamp = self.env().block_timestamp();
            room.agreement_id = self.no_of_agreement;

            // insert agreement data with respect to agreement_id
            let agreement = RoomAgreement {
                room_id,
                agreement_id: self.no_of_agreement,
                room_name: room.room_name.clone(),
                room_address: room.room_address.clone(),
                rent_per_month: room.rent_per_month,
                security_deposit: room.security_deposit,
                lock_in_period: 1,
                time_stamp: room.time_stamp,
            };

            self.agreement.insert(self.no_of_agreement, &agreement);
            self.no_of_rent += 1;
            let rent = Rent {
                rent_id: self.no_of_rent,
                room_id,
                agreement_id: self.no_of_agreement,
                room_name: room.room_name,
                room_address: room.room_address,
                rent_per_month: room.rent_per_month,
                time_stamp: room.time_stamp,
                tenant_address: caller,
                land_lord_address: landlord,
            };
            self.rent.insert(self.no_of_rent, &rent);
        }

        // must be same tenant address
        // renttimestamp should be true
        // enough rent
        #[ink(message, payable)]
        pub fn pay_rent(&mut self, room_id: RoomId) {
            let caller = self.env().caller();
            let mut room = self.room.get(room_id).unwrap_or_default();
            let landlord = room.landlord;
            let rent = room.rent_per_month;
            self.env().transfer(landlord, rent).unwrap_or_default();
            room.current_tenant = caller;
            room.vacant = false;

            self.no_of_rent += 1;
            let rent = Rent {
                rent_id: self.no_of_rent,
                room_id,
                agreement_id: self.no_of_agreement,
                room_name: room.room_name,
                room_address: room.room_address,
                rent_per_month: room.rent_per_month,
                time_stamp: room.time_stamp,
                tenant_address: caller,
                land_lord_address: landlord,
            };
            self.rent.insert(self.no_of_rent, &rent);
        }

        // only landlord
        // agreement timesup
        // room vaccant must be false
        #[ink(message)]
        pub fn agreement_completed(&self, room_id: RoomId) {
            let room = self.room.get(room_id).unwrap_or_default();
            let tenant = room.current_tenant;
            let security_deposit = room.security_deposit;

            self.env().transfer(tenant, security_deposit).unwrap_or_default();
        }

        // only landlord
        // agreement time left
        #[ink(message)]
        pub fn agreement_terminated(&self, room_id: RoomId) {
            let mut room = self.room.get(room_id).unwrap_or_default();
            room.vacant = true;
        }
    }
}
