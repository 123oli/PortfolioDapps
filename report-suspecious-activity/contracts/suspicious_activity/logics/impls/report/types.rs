use ink::prelude::vec::Vec;
use openbrush::{
    contracts::ownable::OwnableError,
    storage::Mapping,
    traits::{AccountId, String, Timestamp, ZERO_ADDRESS},
};

/// Activity Id & User Id
///
pub type ActivityId = i32;
pub type UserId = i32;

/// Function return type
///
pub type SuspeciousResult = Result<(), SuspeciousActivityError>;

/// Suspicious activity Category
///
#[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum SuspeciousActivityCategory {
    MoneyLaundering,
    NarcoticsTrafficking,
    HumanTrafficking,
    Homicide,
    CriminalAttempt,
    TheftCrime,
    TrafficOffenses,
    WhiteCollarCrimes,
}

/// User Type
///
#[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum UserType {}

/// Register User structure
///
#[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct User {
    pub user_id: UserId,
    pub wallet_address: AccountId,
    pub user_name: String,
    pub user_gmail: String,
}

/// Default implementation of `User` struct
///
impl Default for User {
    fn default() -> Self {
        User {
            user_id: Default::default(),
            wallet_address: ZERO_ADDRESS.into(),
            user_name: Default::default(),
            user_gmail: Default::default(),
        }
    }
}

/// Suspicious activity struct
///
#[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct SuspeciousActivity {
    pub activity_id: ActivityId,
    pub crime_category: SuspeciousActivityCategory,
    pub ipfs_image_url: String,
    pub crime_description: Vec<u8>,
    pub crime_location: String,
    pub is_valid: bool,

    pub activity_reporter: AccountId,
    pub reporting_time_stamp: Timestamp,
}

/// Default implementation of `SuspeciousActivity` struct
///
impl Default for SuspeciousActivity {
    fn default() -> Self {
        SuspeciousActivity {
            activity_id: Default::default(),
            crime_category: SuspeciousActivityCategory::CriminalAttempt,
            ipfs_image_url: Default::default(),
            crime_description: Default::default(),
            crime_location: Default::default(),
            is_valid: Default::default(),
            activity_reporter: ZERO_ADDRESS.into(),
            reporting_time_stamp: Default::default(),
        }
    }
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub contract_owner: AccountId,
    pub activity_id: ActivityId,

    pub suspecious_activity: Mapping<ActivityId, SuspeciousActivity>,
    pub user: Mapping<AccountId, User>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            contract_owner: ZERO_ADDRESS.into(),
            activity_id: Default::default(),
            suspecious_activity: Mapping::default(),
            user: Mapping::default(),
        }
    }
}

/// Contract Error
///
#[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SuspeciousActivityError {
    OwnableError(OwnableError),
    InvalidAccount,
    InvalidWalletAddress,
    InvalidUserName,
    UserNotfound,
    ActivityNotFound,
    NotEligibleForReward,
    ZeroTransferredValue,
}

impl From<OwnableError> for SuspeciousActivityError {
    fn from(error: OwnableError) -> Self {
        SuspeciousActivityError::OwnableError(error)
    }
}
