#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod cross_word {

    use ink::{
        prelude::{string::String, vec::Vec},
        storage::Mapping,
    };

    // Puzzle statuses
    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum PuzzleStatus {
        Unsolved,
        Solved { solver_pk: AccountId },
        // memo is basically message
        Claimed { memo: String },
    }

    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct CoordinatePair {
        x: u8,
        y: u8,
    }

    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum AnswerDirection {
        Across,
        Down,
    }

    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Answer {
        num: u8,
        start: CoordinatePair,
        direction: AnswerDirection,
        length: u8,
        clue: String,
    }

    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Puzzle {
        status: PuzzleStatus,
        reward: u128,
        creator: AccountId,
        dimension: CoordinatePair,
        answer: Vec<Answer>,
    }

    #[ink(storage)]
    pub struct CrossWord {
        puzzles: Mapping<AccountId, Puzzle>,
    }

    impl CrossWord {
        #[ink(constructor)]
        pub fn new() -> Self {
            let puzzles = Mapping::default();
            Self { puzzles }
        }

        #[ink(message)]
        pub fn get(&self) {}
    }
}
