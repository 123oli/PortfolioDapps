pub use crate::traits::report::*;
use crate::{
    ensure,
    impls::report::types::{SuspeciousActivityCategory, SuspeciousActivityError},
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::ownable::{self, only_owner},
    modifier_definition, modifiers,
    traits::{AccountId, AccountIdExt, Storage, String, Timestamp},
};

use super::types::{ActivityId, Data, SuspeciousActivity, SuspeciousResult, User};

// Events for SuspeciousActivity
//
pub trait SuspeciousActivityEvent {
    fn emit_register_user_event(&self, user_address: AccountId, user_name: String);
    fn emit_suspecious_activity_event(&self, activity_reporter: AccountId, time_stamp: Timestamp);
    fn emit_update_activity_event(&self, activity_id: ActivityId);
    fn emit_reward_user_event(&self, user_address: AccountId, value: u128);
}

pub trait Internal {
    // Checks if email is valid or not
    //
    // fn is_valid_email(email: &str) -> bool;

    fn _register(
        &mut self,
        wallet_address: AccountId,
        user_name: String,
        user_gmail: String,
    ) -> SuspeciousResult;

    fn _report_activity(
        &mut self,
        activity_id: ActivityId,
        crime_category: SuspeciousActivityCategory,
        ipfs_image_url: String,
        crime_description: Vec<u8>,
        crime_location: String,
    ) -> SuspeciousResult;

    fn _update(&mut self, activity_id: ActivityId) -> SuspeciousResult;
}

impl<T> SuspeciousActivitiesTrait for T
where
    T: Storage<Data> + Storage<ownable::Data>,
{
    #[modifiers(is_valid_user)]
    default fn register_user(
        &mut self,
        wallet_address: AccountId,
        user_name: String,
        user_gmail: String,
    ) -> SuspeciousResult {
        // Ensure `wallet_address` matches caller of the contract
        let caller = T::env().caller();
        ensure!(
            wallet_address == caller,
            SuspeciousActivityError::InvalidWalletAddress
        );

        // Ensure `user_name` is valid
        ensure!(
            user_name.len() >= 3,
            SuspeciousActivityError::InvalidUserName
        );

        // Register new user, call internal function
        self._register(wallet_address, user_name, user_gmail)
            .unwrap_or_default();
        Ok(())
    }

    #[modifiers(is_valid_user)]
    default fn report_suspecious_activity(
        &mut self,
        crime_category: SuspeciousActivityCategory,
        ipfs_image_url: String,
        crime_description: Vec<u8>,
        crime_location: String,
    ) -> SuspeciousResult {
        // check if user exists on the contract
        let caller = T::env().caller();
        match self.data::<Data>().user.get(&caller) {
            Some(value) => value,
            None => return Err(SuspeciousActivityError::UserNotfound),
        };

        let activity_id = self.activity_next_id();
        // report new suspecious activity, call internal function
        self._report_activity(
            activity_id,
            crime_category,
            ipfs_image_url,
            crime_description,
            crime_location,
        )
        .unwrap_or_default();
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn update_suspecious_activity(&mut self, activity_id: ActivityId) -> SuspeciousResult {
        match self.data::<Data>().suspecious_activity.get(&activity_id) {
            Some(value) => self._update(value.activity_id),
            None => return Err(SuspeciousActivityError::ActivityNotFound),
        };
        // self._update(activity_id).unwrap_or_default();
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn reward_user(&mut self, activity_id: ActivityId) -> SuspeciousResult {
        let reward_amount = T::env().transferred_value();
        let user_address = T::env().caller();

        match self.data::<Data>().suspecious_activity.get(&activity_id) {
            Some(value) => {
                ensure!(
                    value.is_valid,
                    SuspeciousActivityError::NotEligibleForReward
                );

                ensure!(
                    reward_amount > 0,
                    SuspeciousActivityError::ZeroTransferredValue
                );

                T::env()
                    .transfer(value.activity_reporter, reward_amount)
                    .unwrap_or_default();
            }
            None => return Err(SuspeciousActivityError::ActivityNotFound),
        };

        self.emit_reward_user_event(user_address, reward_amount);
        Ok(())
    }

    default fn get_suspecious_activity(&mut self) -> Vec<SuspeciousActivity> {
        let mut activity: Vec<SuspeciousActivity> = Vec::new();
        for activity_id in 0..self.data::<Data>().activity_id {
            match self.data::<Data>().suspecious_activity.get(&activity_id) {
                Some(value) => activity.push(value),
                None => (),
            }
        }

        activity
    }

    default fn activity_next_id(&mut self) -> ActivityId {
        let activity_id = self.data::<Data>().activity_id;
        self.data::<Data>().activity_id += 1;
        activity_id
    }
}

// Event implementation
impl<T> SuspeciousActivityEvent for T
where
    T: Storage<Data>,
{
    default fn emit_register_user_event(&self, _user_address: AccountId, _user_name: String) {}
    default fn emit_suspecious_activity_event(
        &self,
        _activity_reporter: AccountId,
        _time_stamp: Timestamp,
    ) {
    }
    default fn emit_update_activity_event(&self, _activity_id: ActivityId) {}
    default fn emit_reward_user_event(&self, _user_address: AccountId, _value: u128) {}
}

// Internal struct Implementation
//
impl<T> Internal for T
where
    T: Storage<Data>,
{
    fn _register(
        &mut self,
        wallet_address: AccountId,
        user_name: String,
        user_gmail: String,
    ) -> SuspeciousResult {
        let new_user = User {
            user_id: 1,
            wallet_address,
            user_name: user_name.clone(),
            user_gmail,
        };

        self.data::<Data>().user.insert(&wallet_address, &new_user);
        self.emit_register_user_event(wallet_address, user_name);
        Ok(())
    }

    fn _report_activity(
        &mut self,
        activity_id: ActivityId,
        crime_category: SuspeciousActivityCategory,
        ipfs_image_url: String,
        crime_description: Vec<u8>,
        crime_location: String,
    ) -> SuspeciousResult {
        let caller = T::env().caller();
        let time_stamp = T::env().block_timestamp();

        let activity = SuspeciousActivity {
            activity_id,
            crime_category,
            ipfs_image_url,
            crime_description,
            crime_location,
            is_valid: false,
            activity_reporter: caller,
            reporting_time_stamp: time_stamp,
        };

        self.data::<Data>()
            .suspecious_activity
            .insert(&1, &activity);
        self.emit_suspecious_activity_event(caller, time_stamp);
        Ok(())
    }

    fn _update(&mut self, activity_id: ActivityId) -> SuspeciousResult {
        match self.data::<Data>().suspecious_activity.get(&activity_id) {
            Some(value) => {
                let activity = SuspeciousActivity {
                    activity_id: value.activity_id,
                    crime_category: value.crime_category,
                    ipfs_image_url: value.ipfs_image_url,
                    crime_description: value.crime_description,
                    crime_location: value.crime_location,
                    is_valid: true,
                    activity_reporter: value.activity_reporter,
                    reporting_time_stamp: value.reporting_time_stamp,
                };

                self.data::<Data>()
                    .suspecious_activity
                    .insert(&value.activity_id, &activity);
                self.emit_update_activity_event(activity_id);
            }
            None => return Err(SuspeciousActivityError::ActivityNotFound),
        };
        Ok(())
    }
}

// modifier to check normal user
#[modifier_definition]
pub fn is_valid_user<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<SuspeciousActivityError>,
{
    ensure!(
        !T::env().caller().is_zero(),
        SuspeciousActivityError::InvalidAccount
    );
    body(instance)
}
