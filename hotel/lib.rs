#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod hotel {
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use logics::{
        impls::room_book::{room_book::HotelRoomBookingEvents, types::RoomId, *},
        traits::room_book::*,
    };
    use openbrush::{contracts::ownable::*, traits::Storage};

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct Hotel {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        hotel_data: types::Data,
    }

    impl RoomBook for Hotel {}

    #[ink(event)]
    pub struct AddRoomEvent {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct SignAgreementEvent {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        agreement_signer: AccountId,
    }

    #[ink(event)]
    pub struct RentPaymentEvent {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        rent_payment_signer: AccountId,
    }

    #[ink(event)]
    pub struct AgreementCompletedEvent {
        #[ink(topic)]
        room_id: RoomId,
    }

    #[ink(event)]
    pub struct AgreementTerminatedEvent {
        #[ink(topic)]
        room_id: RoomId,
    }

    impl Hotel {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance.hotel_data.land_lord = Self::env().caller();
            instance
        }
    }

    impl HotelRoomBookingEvents for Hotel {
        fn emit_add_room_event(&self, room_id: RoomId, owner: AccountId) {
            self.env().emit_event(AddRoomEvent { room_id, owner });
        }
        fn emit_sign_agreement_event(&self, room_id: RoomId, agreement_signer: AccountId) {
            self.env().emit_event(SignAgreementEvent {
                room_id,
                agreement_signer,
            });
        }
        fn emit_rent_payment_event(&self, room_id: RoomId, rent_payment_signer: AccountId) {
            self.env().emit_event(RentPaymentEvent {
                room_id,
                rent_payment_signer,
            });
        }
        fn emit_agreement_complete_event(&self, room_id: RoomId) {
            self.env().emit_event(AgreementCompletedEvent { room_id });
        }
        fn emit_agreement_terminated_event(&self, room_id: RoomId) {
            self.env().emit_event(AgreementTerminatedEvent { room_id });
        }
    }
}
