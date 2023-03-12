use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, String};

use crate::impls::report::types::{
    ActivityId, SuspeciousActivity, SuspeciousActivityCategory, SuspeciousResult,
};

#[openbrush::trait_definition]
pub trait SuspeciousActivitiesTrait {
    /// Register user in order to eligible report activity
    ///
    #[ink(message)]
    fn register_user(
        &mut self,
        wallet_address: AccountId,
        user_name: String,
        user_gmail: String,
    ) -> SuspeciousResult;

    /// Create suspicious activity with given fields
    ///
    #[ink(message, payable)]
    fn report_suspecious_activity(
        &mut self,
        crime_category: SuspeciousActivityCategory,
        ipfs_image_url: String,
        crime_description: Vec<u8>,
        crime_location: String,
    ) -> SuspeciousResult;

    /// get suspecious activity
    ///
    #[ink(message)]
    fn get_suspecious_activity(&mut self) -> Vec<SuspeciousActivity>;

    /// Update suspicious activity if it is valid
    ///
    #[ink(message)]
    fn update_suspecious_activity(&mut self, activity_id: ActivityId) -> SuspeciousResult;

    /// Reward users on behalf of reporting valid crime
    ///
    #[ink(message, payable)]
    fn reward_user(&mut self, activity_id: ActivityId) -> SuspeciousResult;

    fn activity_next_id(&mut self) -> ActivityId;
}
