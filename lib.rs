#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod contract {
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use logics::{
        impls::room_book::{room_book::HotelRoomBookingEvents, types::RoomId, *},
        traits::room_book::*,
    };
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

    #[ink(event)]
    pub struct AddRoomEvent {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct SignAgreementEvent {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        agreement_signer: AccountId,
    }

    #[ink(event)]
    pub struct RentPaymentEvent {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        rent_payment_signer: AccountId,
    }

    #[ink(event)]
    pub struct AgreementCompletedEvent {
        #[ink(topic)]
        room_id: RoomId,
    }

    #[ink(event)]
    pub struct AgreementTerminatedEvent {
        #[ink(topic)]
        room_id: RoomId,
    }

    impl Hotel {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance.hotel_data.land_lord = Self::env().caller();
            instance
        }
    }

    impl HotelRoomBookingEvents for Hotel {
        fn emit_add_room_event(&self, room_id: RoomId, owner: AccountId) {
            self.env().emit_event(AddRoomEvent { room_id, owner });
        }
        fn emit_sign_agreement_event(&self, room_id: RoomId, agreement_signer: AccountId) {
            self.env().emit_event(SignAgreementEvent {
                room_id,
                agreement_signer,
            });
        }
        fn emit_rent_payment_event(&self, room_id: RoomId, rent_payment_signer: AccountId) {
            self.env().emit_event(RentPaymentEvent {
                room_id,
                rent_payment_signer,
            });
        }
        fn emit_agreement_complete_event(&self, room_id: RoomId) {
            self.env().emit_event(AgreementCompletedEvent { room_id });
        }
        fn emit_agreement_terminated_event(&self, room_id: RoomId) {
            self.env().emit_event(AgreementTerminatedEvent { room_id });
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use crate::contract::types::Room;
        use ink_e2e::build_message;
        use logics::traits::room_book::roombook_external::RoomBook;
        use openbrush::traits::ZERO_ADDRESS;

        use super::*;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn new_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = HotelRef::new();
            let contract_acc_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 1000, None)
                .await
                .expect("failed to instantiate")
                .account_id;

            // when
            let get_owner = build_message::<HotelRef>(contract_acc_id.clone())
                .call(|_hotel| _hotel.get_landlord());
            let get_owner_res = client
                .call_dry_run(&ink_e2e::alice(), &get_owner, 0, None)
                .await;

            // check owner
            assert_eq!(
                get_owner_res.return_value(),
                ink_e2e::account_id(ink_e2e::AccountKeyring::Alice)
            );
            Ok(())
        }

        #[ink_e2e::test]
        async fn add_room_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

            let constructor = HotelRef::new();
            let contract_acc_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 1000, None)
                .await
                .expect("failed to instantiate")
                .account_id;

            let room_name = String::from("room one");
            let room_address = String::from("room address");
            let rent_per_month = 10;
            let security_deposit = 10;
            let time_stamp = 10;

            // Add room
            let add_room = build_message::<HotelRef>(contract_acc_id.clone()).call(|hotel| {
                hotel.add_room(
                    room_name.clone(),
                    room_address.clone(),
                    rent_per_month,
                    security_deposit,
                    time_stamp,
                )
            });

            let add_room = client
                .call(&ink_e2e::alice(), add_room, 0, None)
                .await
                .expect("calling add_room failed");

            // check event message
            let contract_emitted_event = add_room
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("Expect Event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted"
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // Decode the expected event type
            let event = contract_emitted_event.field_bytes();
            let decoded_event =
                <AddRoomEvent as scale::Decode>::decode(&mut &event[34..]).expect("Invalid data");

            let AddRoomEvent { room_id, owner } = decoded_event;

            // assert with expected value
            assert_eq!(owner, alice);
            assert_eq!(room_id, 0);

            // get the room
            let get_room =
                build_message::<HotelRef>(contract_acc_id.clone()).call(|hote| hote.get_room());

            let get_room_result = client
                .call_dry_run(&ink_e2e::alice(), &get_room, 0, None)
                .await;

            // check room add successfully
            assert_eq!(
                get_room_result.return_value().unwrap(),
                vec![Room {
                    room_id: 0,
                    agreement_id: 0,
                    room_name,
                    room_address,
                    rent_per_month,
                    security_deposit,
                    time_stamp,
                    vacant: true,
                    landlord: alice,
                    current_tenant: ZERO_ADDRESS.into(),
                    next_rent_due_date: 0,
                }]
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn sign_agreement_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = HotelRef::new();
            let contract_acc_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 1000, None)
                .await
                .expect("failed to instantiate")
                .account_id;

            let room_name = String::from("room one");
            let room_address = String::from("room address");
            let rent_per_month = 10;
            let security_deposit = 10;
            let time_stamp = 10;

            // Add room
            let room_id = build_message::<HotelRef>(contract_acc_id.clone()).call(|hotel| {
                hotel.add_room(
                    room_name.clone(),
                    room_address.clone(),
                    rent_per_month,
                    security_deposit,
                    time_stamp,
                )
            });

            let _ = client
                .call(&ink_e2e::alice(), room_id, 0, None)
                .await
                .expect("calling add_room failed");

            // Sign Agreement
            let sign_agreement = build_message::<HotelRef>(contract_acc_id.clone())
                .call(|hotel| hotel.sign_agreement(0));

            // since owner of contract cannot call `sign_agreement`
            // so caller change to `bob`
            let sign_agreement_response = client
                .call(&ink_e2e::bob(), sign_agreement, 100, None)
                .await
                .expect("calling sign agreement failed");

            // check event message for sign agreement
            let contract_emitted_event = sign_agreement_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("Expect Event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted"
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // Decode the expected event type
            let event = contract_emitted_event.field_bytes();
            let decoded_event = <SignAgreementEvent as scale::Decode>::decode(&mut &event[34..])
                .expect("Invalid data");

            Ok(())
        }
    }
}
