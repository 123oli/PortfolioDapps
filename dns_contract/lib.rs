#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod dns_contract {

    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    // type for domain id
    pub type DomainNameId = i32;

    // offer state for domain name
    #[derive(Debug, scale::Decode, scale::Encode, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum State {
        NotOffering,
        PrivateOffering,
        PublicOffering,
    }

    // struct for domain name
    #[derive(Debug, scale::Decode, scale::Encode, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct DomainName {
        name: String,
        offer_state: State,
        offer_price: u128,
        default_address: AccountId,
    }

    // Default implementation for Domain name
    impl Default for DomainName {
        fn default() -> Self {
            Self {
                name: Default::default(),
                offer_state: State::NotOffering,
                offer_price: Default::default(),
                default_address: zero_address(),
            }
        }
    }

    // define zero address function
    fn zero_address() -> AccountId {
        [0u8; 32].into()
    }

    #[ink(storage)]
    pub struct DnsContract {
        owner: AccountId,
        owner_name_count: Mapping<AccountId, i32>,
        domain_name: Mapping<DomainNameId, DomainName>,
        name_to_owner: Mapping<String, AccountId>,
        claimed: Mapping<DomainNameId, bool>,
        no_of_claimed_names: i32,
        domain_name_id: i32,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum DNSError {
        NameAlreadyExists,
        NotAOwner,
        CallerIsNotOwner,
        SameOwner,
        NameAlreadyClaimed,
        DomainAlreadyOwned,
    }

    // events message
    #[ink(event)]
    pub struct NewNameClaimed {
        #[ink(topic)]
        address: AccountId,
    }

    #[ink(event)]
    pub struct SetNewOwner {
        #[ink(topic)]
        address: AccountId,
    }

    impl DnsContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                owner_name_count: Mapping::default(),
                domain_name: Mapping::default(),
                name_to_owner: Mapping::default(),
                claimed: Mapping::default(),
                no_of_claimed_names: Default::default(),
                domain_name_id: 1,
            }
        }

        #[ink(message)]
        pub fn create_new_dns(
            &mut self,
            name: String,
            offer_state: State,
            offer_price: u128,
        ) -> Result<(), DNSError> {
            let name_id = self.next_domain_name_id();
            let claimed = self.claimed.get(name_id).unwrap_or_default();
            let caller = self.env().caller();

            if self.name_to_owner.contains(&name) {
                return Err(DNSError::DomainAlreadyOwned);
            }

            // insert name to owner
            self.name_to_owner.insert(&name, &caller);

            // check name mustn't be already claimed
            if claimed {
                return Err(DNSError::NameAlreadyClaimed);
            }

            let domain_name = DomainName {
                name,
                offer_state,
                offer_price,
                default_address: caller,
            };

            self.domain_name.insert(name_id, &domain_name);
            self.claimed.insert(name_id, &true);
            self.no_of_claimed_names += 1;

            let name_count = self.owner_name_count.get(caller).unwrap_or_default();
            self.owner_name_count.insert(caller, &(name_count + 1));

            self.env().emit_event(NewNameClaimed { address: caller });
            Ok(())
        }

        #[ink(message)]
        pub fn set_new_owner(
            &mut self,
            name_id: i32,
            new_owner: AccountId,
        ) -> Result<(), DNSError> {
            let name = self.domain_name.get(name_id);
            let caller = self.env().caller();

            match name {
                Some(value) => {
                    if value.default_address != caller {
                        return Err(DNSError::NotAOwner);
                    }
                    // make sure domain_name.owner != new_owner
                    if value.default_address == new_owner {
                        return Err(DNSError::SameOwner);
                    }

                    // owner transfer so owner_name_count descrease
                    let name_count = self.owner_name_count.get(caller).unwrap_or_default();
                    self.owner_name_count.insert(caller, &(name_count - 1));

                    // remove domain name claimed of this id
                    let name_claimed = self.claimed.get(name_id).unwrap_or_default();
                    self.claimed.insert(name_id, &!name_claimed);

                    let domain_name = DomainName {
                        name: value.name,
                        offer_state: value.offer_state,
                        offer_price: value.offer_price,
                        default_address: new_owner,
                    };

                    self.domain_name.insert(name_id, &domain_name);
                }
                None => (),
            }

            self.env().emit_event(SetNewOwner { address: new_owner });
            Ok(())
        }

        #[ink(message)]
        pub fn get_owner_domain_name(&self) -> Vec<DomainName> {
            let mut domain_name: Vec<DomainName> = Vec::new();
            let caller = self.env().caller();

            for _item in 0..self.domain_name_id {
                let name = self.domain_name.get(_item);
                match name {
                    Some(value) => {
                        if value.default_address == caller {
                            domain_name.push(value);
                        }
                    }
                    None => (),
                }
            }

            domain_name
        }

        // get a owner of contract
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner.clone()
        }

        #[ink(message)]
        pub fn get_no_of_name_claimed(&self) -> i32 {
            self.no_of_claimed_names.clone()
        }

        // get domain name count
        #[ink(message)]
        pub fn get_owner_name_count(&self, account_id: AccountId) -> i32 {
            self.owner_name_count.get(account_id).unwrap_or_default() as i32
        }

        #[ink(message)]
        pub fn is_claimed(&self, id: DomainNameId) -> bool {
            self.claimed.get(id).unwrap_or_default()
        }

        #[inline]
        fn next_domain_name_id(&mut self) -> DomainNameId {
            let id = self.domain_name_id;
            self.domain_name_id += 1;
            id
        }
    }
}
