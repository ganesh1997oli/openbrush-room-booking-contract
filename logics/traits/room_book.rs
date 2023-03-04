use crate::impls::room_book::{
    room_book::Room,
    types::{AgreementId, HotelError, RentId, RoomId},
};
use ink::prelude::{string::String, vec::Vec};
use openbrush::traits::{AccountId, Timestamp};

#[openbrush::trait_definition]
pub trait RoomBook {
    // only owner can create
    #[ink(message)]
    fn add_room(
        &mut self,
        room_name: String,
        room_address: String,
        rent_per_month: u128,
        security_deposit: u128,
        time_stamp: Timestamp,
    ) -> Result<(), HotelError>;

    #[ink(message, payable)]
    fn sign_agreement(&mut self, room_id: RoomId) -> Result<(), HotelError>;

    #[ink(message, payable)]
    fn pay_rent(&mut self, room_id: RoomId) -> Result<(), HotelError>;

    #[ink(message, payable)]
    fn agreement_completed(&mut self, room_id: RoomId) -> Result<(), HotelError>;

    #[ink(message, payable)]
    fn agreement_terminated(&mut self, room_id: RoomId) -> Result<(), HotelError>;

    #[ink(message)]
    fn get_room(&self) -> Vec<Room>;

    #[ink(message)]
    fn get_landlord(&self) -> AccountId;

    fn next_room_id(&mut self) -> RoomId;

    fn next_agreement_id(&mut self) -> AgreementId;

    fn next_rent_id(&mut self) -> RentId;
}
