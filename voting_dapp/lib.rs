#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// This is voting application which has three phases.
// 1. registration phase
// 2. voting phase
// 3. tally and display the vote phase

#[ink::contract]
mod voting_dapp {

    use ink_env::debug_print;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};

    pub type ProposalId = i32;
    pub type UserId = i32;
    
    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout, Default, PartialEq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Proposal {
        proposal_name: String,
        vote_aye: i32,
        vote_nye: i32,
        total_vote: i32,
        proposal_status: bool,
    }

    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout, Default, PartialEq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct User {
        user_name: String,
        user_account: AccountId,
        voted_proposal: Vec<Proposal>
    }
    
    #[derive(scale::Decode, scale::Encode, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    pub enum Vote {
        Aye,
        Nye,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct VotingDapp {
        owner: AccountId,
        proposal: Mapping<ProposalId, Proposal>,
        user: Mapping<UserId, User>,
        proposal_id: ProposalId,
        user_id: UserId,
        voted: Mapping<(AccountId, ProposalId), bool>,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ProposalError {
        NotOwner,
        ProposalNotFound,
        AccountNotRegistered
    }

    // Proposal Created Event
    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        pub proposal: Proposal
    }

    #[ink(event)]
    pub struct GetProposal {
        #[ink(topic)]
        pub proposal: Proposal
    }

    #[ink(event)]
    pub struct UserCreated {
        #[ink(topic)]
        pub user: User
    }

    #[ink(event)]
    pub struct ProposalStatusChanged {
        #[ink(topic)]
        pub proposal: Proposal
    }

    #[ink(event)]
    pub struct ProposalVoted {
        #[ink(topic)]
        pub proposal: String
    }

    impl VotingDapp {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            ink_lang::utils::initialize_contract(|contract: &mut VotingDapp|{
                contract.owner = caller;
                contract.proposal = Mapping::default();
                contract.proposal_id = Default::default();
            })
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, proposal_name: String) -> Result<(), ProposalError> {
            assert!(self.check_owner(self.env().caller()), "Not a owner");

            // Create proposal with given fields
            let proposal = Proposal {
                proposal_name,
                vote_aye: 0,
                vote_nye: 0,
                total_vote: 0,
                proposal_status: false,
            };

            // proposal id get from `get_next_id()`
            let proposal_id = self.get_next_id();

            // Insert proposal in the storage
            self.proposal.insert(proposal_id, &proposal);
            self.env().emit_event(ProposalCreated {proposal});
            Ok(())
        }

        #[ink(message)]
        pub fn change_proposal_status(&mut self, id: ProposalId) -> Result<(), ProposalError> {
            assert!(self.check_owner(self.env().caller()), "Not a owner");

            let proposal = self.proposal.get(id);
            match proposal {
                None => return Err(ProposalError::AccountNotRegistered),
                Some(v) => {
                    let p = Proposal {
                        proposal_name: v.proposal_name,
                        vote_aye: 0,
                        vote_nye: 0,
                        total_vote: 0,
                        proposal_status: true
                    };
                    self.proposal.insert(id, &p);
                }
            }
            // proposal.proposal_status = !proposal.proposal_status;

            // self.env().emit_event(ProposalStatusChanged {proposal: proposal.unwrap()});
            Ok(())
        }


        #[ink(message)]
        pub fn register_user(&mut self, user_account: AccountId, user_name: String) -> Result<(), ProposalError> {
            assert!(self.check_owner(self.env().caller()), "Not a owner");

            let user = User {
                user_name,
                user_account,
                voted_proposal: Default::default(),
            };

            let uid = self.get_next_userid();

            self.user.insert(uid, &user);
            self.env().emit_event(UserCreated{user});

            Ok(())
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, vote: Vote, id: ProposalId ) -> Result<(), ProposalError> {
            let proposal = self.proposal.get(id).unwrap_or_default();
            assert!(proposal.proposal_status, "Proposal status must be true");

            let caller = self.env().caller();
            assert!(self.check_register_user(caller), "Not register user");
            
            let is_voted = self.voted.get((caller, id)).unwrap_or_default();
            assert!(!is_voted, "Can't voted twice");
            debug_print!("is voted: {}",is_voted);
            
            match vote {
                Vote::Aye => {
                    let p = Proposal {
                        proposal_name: proposal.proposal_name,
                        vote_aye: proposal.vote_aye + 1,
                        vote_nye: proposal.vote_nye,
                        total_vote: proposal.total_vote + 1,
                        proposal_status: proposal.proposal_status,
                    };
                    self.proposal.insert(id, &p);
                    self.voted.insert((caller, id), &true);
                    // proposal.vote_aye += 1;
                },
                Vote::Nye => {
                    let p = Proposal {
                        proposal_name: proposal.proposal_name,
                        vote_aye: proposal.vote_aye,
                        vote_nye: proposal.vote_nye + 1,
                        total_vote: proposal.total_vote + 1,
                        proposal_status: proposal.proposal_status,
                    };
                    self.proposal.insert(id, &p);
                    self.voted.insert((caller, id), &true);
                    // proposal.vote_nye += 1;
                }
            }

            // self.env().emit_event(ProposalVoted{proposal: proposal.proposal_name});

            Ok(())
        }

        #[ink(message)]
        pub fn get_all_proposal(&mut self) -> Vec<Proposal>{
            let mut result: Vec<Proposal> = Vec::new();
            for i in 0..self.proposal_id {
                match self.proposal.get(i) {
                    Some(value) => result.push(value),
                    None => ()
                }
            }
            result
        }

        #[ink(message)]
        pub fn get_all_users(&mut self) -> Vec<User>{
            let mut result: Vec<User> = Vec::new();
            for i in 0..self.user_id {
                match self.user.get(i) {
                    Some(value) => result.push(value),
                    None => ()
                }
            }
            result
        }

        // Generate next proposal id by incrementing with 1
        pub fn get_next_id(&mut self) -> ProposalId {
            let id = self.proposal_id;
            self.proposal_id += 1;
            id
        }

        pub fn get_next_userid(&mut self) -> UserId {
            let id = self.user_id;
            self.user_id += 1;
            id
        }

        // Check owner
        pub fn check_owner(&self, user: AccountId) -> bool {
            if self.owner == user {
                true
            } else {
                false
            }
        }

        pub fn check_register_user(&mut self, caller: AccountId) -> bool {
            let user = self.get_all_users();
            let mut _ruser = false;

            for u in user.iter() {
                if u.user_account == caller {
                    _ruser = true;
                    break;
                }
            }

            _ruser
        }
    }
}
