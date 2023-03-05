use crate::impls::room_book::{
    room_book::Room,
    types::{AgreementId, HotelError, RentId, RoomId, RoomResult},
};
use ink::prelude::{string::String, vec::Vec};
use openbrush::traits::{AccountId, Timestamp};

#[openbrush::trait_definition]
pub trait RoomBook {
    /// Add room function where only landlord can call `add_room` function
    #[ink(message)]
    fn add_room(
        &mut self,
        room_name: String,
        room_address: String,
        rent_per_month: u128,
        security_deposit: u128,
        time_stamp: Timestamp,
    ) -> RoomResult;

    /// user other than `landlord` call the `sign_agreement` function
    #[ink(message, payable)]
    fn sign_agreement(&mut self, room_id: RoomId) -> RoomResult;

    /// room musn't be vacant and user should be tenant to call `pay_rent` function
    #[ink(message, payable)]
    fn pay_rent(&mut self, room_id: RoomId) -> RoomResult;

    /// If room is occupied by tenant and timeperiod of agreement complete then
    /// `landlord` allowed to call this function
    #[ink(message, payable)]
    fn agreement_completed(&mut self, room_id: RoomId) -> RoomResult;

    /// On behalf of any suspecious customer, `landlord` allowed to call this function
    #[ink(message, payable)]
    fn agreement_terminated(&mut self, room_id: RoomId) -> RoomResult;

    /// `landlord` is allowed to call this function to get all the room
    #[ink(message)]
    fn get_room(&mut self) -> Result<Vec<Room>, HotelError>;

    /// `customer` can view all available rooms
    #[ink(message)]
    fn get_available_room(&self) -> Vec<Room>;

    /// `landlord` of the contract
    #[ink(message)]
    fn get_landlord(&self) -> AccountId;

    /// get the `next_room_id`
    fn next_room_id(&mut self) -> RoomId;

    /// get the `next_agreement_id`
    fn next_agreement_id(&mut self) -> AgreementId;

    /// geht the `next_rent_id`
    fn next_rent_id(&mut self) -> RentId;
}
