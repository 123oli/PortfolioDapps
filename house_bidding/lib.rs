#![cfg_attr(not(feature = "std"), no_std)]

#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            return Err($y.into());
        }
    }};
}

#[ink::contract]
mod house_bidding {
    use ink::{
        prelude::{string::String, vec, vec::Vec},
        storage::Mapping,
    };

    pub type HouseId = i32;
    pub type BidderId = i32;

    fn zero_address() -> AccountId {
        [0; 32].into()
    }

    #[derive(scale::Decode, scale::Encode, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum HouseError {
        HouseNotFound,
        CantBidFurther,
        StillBidding,
        CantBidTwice,
        ValueTooSmall,
        BiddingLimitNotFulfill,
    }

    /// Bidder struct
    #[derive(scale::Decode, scale::Encode, Eq, PartialEq, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Bidder {
        bidder_id: BidderId,
        bidder_account: AccountId,
        bidder_amount: Balance,
    }

    impl Default for Bidder {
        fn default() -> Self {
            Bidder {
                bidder_id: Default::default(),
                bidder_account: zero_address(),
                bidder_amount: Default::default(),
            }
        }
    }

    /// House struct
    #[derive(scale::Decode, scale::Encode, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct House {
        house_id: HouseId,
        house_owner: AccountId,
        house_title: String,
        house_description: String,
        rooms: i32,
        special_features: Vec<String>,
        initial_price: Balance,
        bidder: Vec<Bidder>,
        max_bid_price: Balance,
        winner: AccountId,
    }

    impl Default for House {
        fn default() -> Self {
            House {
                house_id: Default::default(),
                house_owner: zero_address(),
                house_title: Default::default(),
                house_description: Default::default(),
                rooms: Default::default(),
                special_features: Vec::new(),
                initial_price: Default::default(),
                bidder: Vec::new(),
                max_bid_price: Default::default(),
                winner: zero_address(),
            }
        }
    }

    #[ink(storage)]
    pub struct HouseBidding {
        owner: AccountId,
        house_id: HouseId,
        bidder_id: BidderId,
        house: Mapping<HouseId, House>,
    }

    impl Default for HouseBidding {
        fn default() -> Self {
            HouseBidding {
                owner: zero_address(),
                house_id: Default::default(),
                bidder_id: Default::default(),
                house: Mapping::default(),
            }
        }
    }

    impl HouseBidding {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self::default();
            instance
        }

        #[ink(message)]
        pub fn add_house(
            &mut self,
            house_title: String,
            house_description: String,
            rooms: i32,
            initial_price: Balance,
            special_features: Vec<String>,
        ) -> Result<(), HouseError> {
            let house_owner = self.env().caller();
            let house_id = self.next_house_id();

            let house = House {
                house_id,
                house_owner,
                house_title,
                house_description,
                rooms,
                special_features,
                initial_price,
                bidder: vec![],
                max_bid_price: 0,
                winner: zero_address(),
            };

            self.house.insert(&house_id, &house);
            Ok(())
        }

        #[ink(message, payable)]
        pub fn bid(&mut self, house_id: HouseId) -> Result<(), HouseError> {
            let caller = self.env().caller();
            let bidder_id = self.next_bidder_id();
            let bidder_amount = self.env().transferred_value();

            let mut house = match self.house.get(&house_id) {
                None => return Err(HouseError::HouseNotFound),
                Some(house) => house,
            };

            ensure!(
                bidder_amount >= house.initial_price,
                HouseError::ValueTooSmall
            );

            for bid in house.bidder.clone() {
                if bid.bidder_account == caller {
                    return Err(HouseError::CantBidTwice);
                }
            }

            let bidder = Bidder {
                bidder_id,
                bidder_account: caller,
                bidder_amount,
            };

            let bidder_len = house.bidder.len() as i32;
            ensure!(bidder_len < 5, HouseError::CantBidFurther);

            house.bidder.push(bidder);

            self.house.insert(&house_id, &house);
            Ok(())
        }

        #[ink(message)]
        pub fn get_winner(&mut self, house_id: HouseId) -> Result<(), HouseError> {
            match self.house.get(&house_id) {
                None => return Err(HouseError::HouseNotFound),
                Some(mut house) => {
                    if house.bidder.len() == 5 {
                        for bid in house.bidder.clone() {
                            if bid.bidder_amount > house.max_bid_price {
                                house.max_bid_price = bid.bidder_amount;
                                house.winner = bid.bidder_account;

                                self.house.insert(&house_id, &house);
                            } else {
                                return Err(HouseError::StillBidding);
                            }
                        }
                    } else {
                        return Err(HouseError::BiddingLimitNotFulfill);
                    }
                }
            };
            Ok(())
        }

        #[ink(message)]
        pub fn get_house(&self) -> Vec<House> {
            let mut house_vec: Vec<House> = Vec::new();
            for id in 0..self.house_id {
                match self.house.get(&id) {
                    None => (),
                    Some(house) => house_vec.push(house),
                }
            }
            house_vec
        }

        pub fn next_house_id(&mut self) -> HouseId {
            let id = self.house_id;
            self.house_id += 1;
            id
        }

        pub fn next_bidder_id(&mut self) -> HouseId {
            let id = self.bidder_id;
            self.bidder_id += 1;
            id
        }
    }
}
