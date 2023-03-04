use crate::hotel::*;

use ink_e2e::build_message;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn add_new_room_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = HotelRef::new();
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

    let contract_account_id = client
        .instantiate("hotel", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let room_name = String::from("room one");
    let room_address = String::from("room address");
    let rent_per_month = 10;
    let security_deposit = 10;
    let time_stamp = 10;

    let add_room = build_message::<HotelRef>(contract_account_id.clone()).call(|hotel| {
        hotel.add_room(
            room_name,
            room_address,
            rent_per_month,
            security_deposit,
            time_stamp,
        )
    });
    let _ = client
        .call(&ink_e2e::alice(), add_room, 0, None)
        .await
        .expect("calling add_room failed");

    let get_room =
        build_message::<HotelRef>(contract_account_id.clone()).call(|hote| hote.get_room());

    let get_room_result = client
        .call_dry_run(&ink_e2e::alice(), &get_room, 0, None)
        .await;
    assert_eq!(
        get_room_result.return_value(),
        vec![Room {
            room_id: 0,
            agreement_id: 0,
            room_name,
            room_address,
            rent_per_month,
            security_deposit,
            time_stamp,
            vacant: false,
            landlord: alice,
            current_tenant: ZERO_ADDRESS.into(),
        }]
    );

    Ok(())
}
