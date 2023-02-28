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
                landlord: zero_address(),
                current_tenant: zero_address(),
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
                tenant_address: zero_address(),
                land_lord_address: zero_address(),
            }
        }
    }

    #[ink(storage)]
    pub struct Hotel {
        tenant: AccountId,
        land_lord: AccountId,
        room_id: i32,
        agreement_id: i32,
        rent_id: i32,

        room: Mapping<RoomId, Room>,
        agreement: Mapping<AgreementId, RoomAgreement>,
        rent: Mapping<RentId, Rent>,
    }

    #[derive(scale::Decode, scale::Encode, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum HotelError {}

    impl Hotel {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tenant: zero_address(),
                land_lord: zero_address(),
                room_id: Default::default(),
                agreement_id: Default::default(),
                rent_id: Default::default(),

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
            // get the caller of contract
            let caller = self.env().caller();

            // get the `next_room_id`
            let room_id = self.next_room_id();
            // get the `next_agreement_id`
            let agreement_id = self.next_agreement_id();

            // create a new `Room` object with the given fields
            let new_room = Room {
                room_id,
                agreement_id,
                room_name,
                room_address,
                rent_per_month,
                security_deposit,
                time_stamp,
                vacant: false,
                landlord: caller,
                current_tenant: zero_address(),
            };

            // insert data to the room with respect to id
            self.room.insert(1, &new_room);
        }

        // user shouldn't be owner/landlord to sign_aggrement
        // enough agreement fee
        // agreement happens when particular room is empty
        #[ink(message, payable)]
        pub fn sign_aggrement(&mut self, room_id: RoomId) {
            // get the caller of contract
            let caller = self.env().caller();

            // get the room of specific `room_id`
            let mut room = self.room.get(room_id).unwrap_or_default();

            // room landlord of the hotel
            let landlord = room.landlord;

            // `total_fee` to sign room agreement
            let total_fee = room.rent_per_month + room.security_deposit;

            // transfer `total_fee` to landlord
            self.env().transfer(landlord, total_fee).unwrap_or_default();

            // get `the next_agreement_id`
            let agreement_id = self.next_agreement_id();

            // update room data for success agreement
            room.current_tenant = caller;
            room.vacant = false;
            room.time_stamp = self.env().block_timestamp();
            room.agreement_id = agreement_id;

            // create new `RoomAgreement` of the room
            let agreement = RoomAgreement {
                room_id,
                agreement_id,
                room_name: room.room_name.clone(),
                room_address: room.room_address.clone(),
                rent_per_month: room.rent_per_month,
                security_deposit: room.security_deposit,
                lock_in_period: 1,
                time_stamp: room.time_stamp,
            };

            // insert `sign_agreement` to the agreement mapping
            self.agreement.insert(agreement_id, &agreement);


            // get the `next_rent_id`
            let rent_id = self.next_rent_id();

            // create new `Rent` object with the given filds
            let rent = Rent {
                rent_id,
                room_id,
                agreement_id,
                room_name: room.room_name,
                room_address: room.room_address,
                rent_per_month: room.rent_per_month,
                time_stamp: room.time_stamp,
                tenant_address: caller,
                land_lord_address: landlord,
            };

            // insert `rent` in the mapping
            self.rent.insert(rent_id, &rent);
        }

        // must be same tenant address
        // renttimestamp should be true
        // enough rent
        #[ink(message, payable)]
        pub fn pay_rent(&mut self, room_id: RoomId) {
            // get the caller of the contract
            let caller = self.env().caller();

            // get the room of specific `room_id`
            let mut room = self.room.get(room_id).unwrap_or_default();

            // get he `landlord` of the room
            let landlord = room.landlord;

            // get the `rent_per_month` of the room
            let rent = room.rent_per_month;

            // transfer `rent` to the `landlord`
            self.env().transfer(landlord, rent).unwrap_or_default();

            // update `current_tenant` of the room
            room.current_tenant = caller;

            // update room `vacant`
            room.vacant = false;

            // get the `next_rent_id & next_agreement_id` of the room
            let rent_id = self.next_rent_id();
            let agreement_id = self.next_agreement_id();

            // create new `Rent` object with the given fields
            let rent = Rent {
                rent_id,
                room_id,
                agreement_id,
                room_name: room.room_name,
                room_address: room.room_address,
                rent_per_month: room.rent_per_month,
                time_stamp: room.time_stamp,
                tenant_address: caller,
                land_lord_address: landlord,
            };

            // insert updated rent after the `rent_payment`
            self.rent.insert(rent_id, &rent);
        }

        // only landlord
        // agreement timesup
        // room vaccant must be false
        #[ink(message)]
        pub fn agreement_completed(&self, room_id: RoomId) {
            // get the `room` of specific `room_id`
            let room = self.room.get(room_id).unwrap_or_default();

            // get the `tenant` of the room
            let tenant = room.current_tenant;

            // get the `security_deposit` of the room
            let security_deposit = room.security_deposit;

            // transfer back `security_deposit` to the tenant
            self.env()
                .transfer(tenant, security_deposit)
                .unwrap_or_default();
        }

        // only landlord
        // agreement time left
        #[ink(message)]
        pub fn agreement_terminated(&self, room_id: RoomId) {
            // get the `room` of specific `room_id`
            let mut room = self.room.get(room_id).unwrap_or_default();

            // update `room` vacant and make it `false`
            // so that it is available for rent again
            room.vacant = true;
        }

        // get next_id for `room`, `agreement` & `rent`
        pub fn next_room_id(&mut self) -> RoomId {
            let room_id = self.room_id;
            self.room_id += 1;
            room_id
        }

        pub fn next_agreement_id(&mut self) -> RoomId {
            let agreement_id = self.agreement_id;
            self.agreement_id += 1;
            agreement_id
        }

        pub fn next_rent_id(&mut self) -> RoomId {
            let rent_id = self.rent_id;
            self.rent_id += 1;
            rent_id
        }
    }

    // zero address defination for `AccountId`
    pub fn zero_address() -> AccountId {
        [0u8; 32].into()
    }
}
