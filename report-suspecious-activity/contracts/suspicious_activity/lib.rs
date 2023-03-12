#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod suspicious_activity {
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use logics::{
        impls::report::{report::SuspeciousActivityEvent, types::ActivityId, *},
        traits::report::*,
    };
    use openbrush::{
        contracts::ownable::*,
        traits::{Storage, String},
    };

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct SuspiciousActivitiesContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        activity_data: types::Data,
    }

    #[ink(event)]
    pub struct RegisterUserEvent {
        #[ink(topic)]
        user_address: AccountId,
        #[ink(topic)]
        user_name: String,
    }

    #[ink(event)]
    pub struct ActivityEvent {
        #[ink(topic)]
        activity_reporter: AccountId,
        #[ink(topic)]
        time_stamp: Timestamp,
    }

    #[ink(event)]
    pub struct SuspeciousActivityUpdateEvent {
        #[ink(topic)]
        activity_id: ActivityId,
    }

    #[ink(event)]
    pub struct RewardUserEvent {
        #[ink(topic)]
        user_address: AccountId,
        value: u128,
    }

    impl SuspeciousActivitiesTrait for SuspiciousActivitiesContract {}

    impl SuspiciousActivitiesContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance.activity_data.contract_owner = Self::env().caller();
            instance
        }
    }

    impl SuspeciousActivityEvent for SuspiciousActivitiesContract {
        fn emit_register_user_event(&self, user_address: AccountId, user_name: String) {
            self.env().emit_event(RegisterUserEvent {
                user_address,
                user_name,
            });
        }
        fn emit_suspecious_activity_event(
            &self,
            activity_reporter: AccountId,
            time_stamp: Timestamp,
        ) {
            self.env().emit_event(ActivityEvent {
                activity_reporter,
                time_stamp,
            });
        }
        fn emit_update_activity_event(&self, activity_id: ActivityId) {
            self.env()
                .emit_event(SuspeciousActivityUpdateEvent { activity_id });
        }
        fn emit_reward_user_event(&self, user_address: AccountId, value: u128) {
            self.env().emit_event(RewardUserEvent {
                user_address,
                value,
            });
        }
    }
}
