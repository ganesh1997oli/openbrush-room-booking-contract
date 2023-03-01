#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod hotel {

    use logics::{impls::room_book::*, traits::room_book::*};
    use openbrush::{contracts::ownable::*, traits::Storage};

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct Hotel {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        hotel_data: types::Data,
    }

    impl RoomBook for Hotel {}

    impl Hotel {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance
        }
    }
}
