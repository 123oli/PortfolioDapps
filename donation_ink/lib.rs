#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_without_default)]

#[ink::contract]
mod donation_ink {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    pub type DonationId = i32;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Donation {
        account: AccountId,
        amount: u128,
    }

    // Donation struct default implementations
    impl Default for Donation {
        fn default() -> Self {
            Self {
                account: zero_address(),
                amount: 0,
            }
        }
    }

    #[ink(storage)]
    pub struct DonationContract {
        owner: AccountId,
        beneficiary: AccountId,
        donations: Mapping<DonationId, Donation>,
        donation_id: i32,
    }

    impl DonationContract {
        #[ink(constructor)]
        pub fn new(beneficiary: AccountId) -> Self {
            let owner = Self::env().caller();
            Self {
                owner,
                beneficiary,
                donations: Mapping::default(),
                donation_id: 1,
            }
        }

        #[ink(message)]
        pub fn get_beneficiary(&self) -> Option<AccountId> {
            Some(self.beneficiary)
        }

        #[ink(message)]
        pub fn change_beneficiary(&mut self, new_beneficiary: AccountId) {
            let caller = self.env().caller();
            assert!(
                self.owner == caller,
                "Only owner can change the beneficiary account"
            );
            self.beneficiary = new_beneficiary;
        }

        #[ink(message, payable)]
        pub fn donation(&mut self) {
            // Account who is donating
            let caller = self.env().caller();
            let donation_id = self.next_donation_id();

            // Donation amount
            let donation_amount = self.env().transferred_value();
            let mut donated_so_far = self.donations.get(donation_id).unwrap_or_default();

            assert!(donation_amount > 0, "Cannot transfer 0 donation");

            // Total donation amount so far by caller
            donated_so_far.amount += donation_amount;
            donated_so_far.account = caller;

            // Insert total donation amount with respect to caller
            self.donations.insert(donation_id, &donated_so_far);

            // Send donation amount to the beneficiary account
            self.env()
                .transfer(self.beneficiary, donation_amount)
                .unwrap_or_default();
        }

        #[ink(message)]
        pub fn get_donation_amount_by_user(&self, donation_id: DonationId) -> (AccountId, u128) {
            (
                self.donations.get(donation_id).unwrap_or_default().account,
                self.donations.get(donation_id).unwrap_or_default().amount,
            )
        }

        #[ink(message)]
        pub fn get_donation(&mut self) -> Vec<Donation> {
            let mut donation: Vec<Donation> = Vec::new();
            for _donation in 0..self.donation_id {
                match self.donations.get(_donation) {
                    Some(value) => donation.push(value),
                    None => (),
                }
            }
            donation
        }

        #[inline]
        pub fn next_donation_id(&mut self) -> DonationId {
            let id = self.donation_id;
            self.donation_id += 1;
            id
        }
    }

    /// Helper for referencing the zero address (`0x00`). Note that in practice this address should
    /// not be treated in any special way (such as a default placeholder) since it has a known
    /// private key.
    fn zero_address() -> AccountId {
        [0u8; 32].into()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account_id)
                .expect("cannot get account balance")
        }

        #[ink::test]
        fn register_works() {
            set_next_caller(default_accounts().alice);
            let contract = DonationContract::new(default_accounts().bob);

            assert_eq!(contract.get_beneficiary(), Some(default_accounts().bob));
        }

        #[ink::test]
        fn change_beneficiary_works() {
            set_next_caller(default_accounts().alice);
            let mut contract = DonationContract::new(default_accounts().bob);

            assert_eq!(contract.get_beneficiary(), Some(default_accounts().bob));

            contract.change_beneficiary(default_accounts().charlie);

            assert_eq!(contract.get_beneficiary(), Some(default_accounts().charlie));
        }

        #[ink::test]
        fn donation_works() {
            set_next_caller(default_accounts().alice);
            let mut contract = DonationContract::new(default_accounts().bob);

            let donation_id = 1;
            set_balance(default_accounts().alice, 100);
            set_balance(default_accounts().bob, 0);
            set_sender(default_accounts().alice);

            // Balance benfore donation
            assert_eq!(get_balance(default_accounts().bob), 0);
            assert_eq!(get_balance(default_accounts().alice), 100);

            // Donate the balance
            let donate_balance = 90;
            ink::env::pay_with_call!(contract.donation(), donate_balance);

            let (account, amount) = contract.get_donation_amount_by_user(donation_id);
            assert_eq!(account, default_accounts().alice);
            assert_eq!(amount, 90);

            // Balance after donation
            assert_eq!(get_balance(default_accounts().bob), 90);
            // assert_eq!(get_balance(default_accounts().alice), 100 - 90);
        }
    }
}
