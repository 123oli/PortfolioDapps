#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod todo_list {
    use ink_storage::{traits::{SpreadAllocate, PackedLayout, SpreadLayout}, Mapping};

    use ink_prelude::{
        string::String,
        vec::Vec,
    };

    pub type TodoItemId = i32;

    // Item priorities enum
    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Prioritise {
        HIGH,
        LOW,
        MEDIUM,
    }

    // item todo error
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TodoError {
        ItemNotExists
    }

    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TodoItem {
        owner: AccountId,
        item_name: String,
        is_completed: bool,
        priority: Prioritise
    }

    // Events for toto item
    #[ink(event)]
    pub struct ItemCreated {
        #[ink(topic)]
        item: TodoItem,
    }

    #[ink(event)]
    pub struct ItemUpdated {
        #[ink(topic)]
        item: TodoItem,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TodoList {
        owner: AccountId,
        item: Mapping<TodoItemId, TodoItem>,
        item_id: i32,
    }

    impl TodoList {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            ink_lang::utils::initialize_contract(|contract: &mut TodoList|{
                contract.owner = owner;
                contract.item = Mapping::default();
                contract.item_id = 1;
            })
        }

        #[ink(message)]
        pub fn create_todo(&mut self, item_name: String, priority: Prioritise ) -> Result<(), TodoError> {
            let caller = self.env().caller();
            let item_id = self.get_item_id();

            let item = TodoItem {
                owner: caller,
                item_name,
                is_completed: false,
                priority,
            };

            self.item.insert(item_id, &item);
            self.env().emit_event(ItemCreated{item});
            Ok(())
        }

        #[ink(message)]
        pub fn update_item(&mut self, item_id: TodoItemId) -> Result<(), TodoError> {
            let item = self.item.get(item_id);
            let caller = self.env().caller();
            match item {
                Some(value) => {
                    let item = TodoItem {
                        owner: caller,
                        item_name: value.item_name,
                        is_completed: true,
                        priority: value.priority,
                    };
                    self.item.insert(item_id, &item);
                    self.env().emit_event(ItemUpdated{item});
                },
                None => return Err(TodoError::ItemNotExists),
            }
            Ok(())
        }

        #[ink(message)]
        pub fn get_my_item(&self, account: AccountId) -> Vec<TodoItem> {
            let mut item: Vec<TodoItem> = Vec::new();
            for _item in 0..self.item_id {
                match self.item.get(_item) {
                    Some(value) => if value.owner == account {
                        item.push(value);
                    },
                    None => (),
                }
            }
            item
        }

        // Item next Id
        pub fn get_item_id(&mut self) -> TodoItemId {
            let id = self.item_id;
            self.item_id += 1;
            id
        }
    }
}
