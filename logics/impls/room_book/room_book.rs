pub use crate::{
    ensure,
    impls::room_book::types::{AgreementId, Data, HotelError, RentId, Room, RoomId},
    traits::room_book::*,
};
use ink::prelude::string::String;
use openbrush::{
    contracts::ownable::*,
    modifier_definition, modifiers,
    traits::{Storage, Timestamp, ZERO_ADDRESS},
};

impl<T> RoomBook for T
where
    T: Storage<Data> + Storage<ownable::Data>,
{
    #[modifiers(only_owner)]
    default fn add_room(
        &mut self,
        room_name: String,
        room_address: String,
        rent_per_month: u128,
        security_deposit: u128,
        time_stamp: Timestamp,
    ) -> Result<(), HotelError> {
        // caller of the contract
        let caller = Self::env().caller();

        // get `room_id` & `agreement_id`
        let room_id = self.next_room_id();
        let agreement_id = self.next_agreement_id();

        // create a new `Room` object with the given fields
        let new_room = Room {
            room_id,
            agreement_id,
            room_name,
            room_address,
            rent_per_month,
            security_deposit,
            time_stamp,
            vacant: false,
            landlord: caller,
            current_tenant: ZERO_ADDRESS.into(),
        };

        // insert room in `Mapping` with respect to key `room_id`
        self.data::<Data>().room.insert(&room_id, &new_room);

        Ok(())
    }

    #[modifiers(only_fee_setter)]
    default fn sign_agreement(&mut self, room_id: RoomId) -> Result<(), HotelError> {
        Ok(())
    }

    default fn pay_rent(&mut self, room_id: RoomId) -> Result<(), HotelError> {
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn agreement_completed(&mut self, room_id: RoomId) -> Result<(), HotelError> {
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn agreement_terminated(&mut self, room_id: RoomId) -> Result<(), HotelError> {
        Ok(())
    }

    fn next_room_id(&mut self) -> RoomId {
        let room_id = self.data::<Data>().room_id;
        self.data::<Data>().room_id += 1;
        room_id
    }

    fn next_agreement_id(&mut self) -> AgreementId {
        let agreement_id = self.data::<Data>().agreement_id;
        self.data::<Data>().agreement_id += 1;
        agreement_id
    }

    fn next_rent_id(&mut self) -> RentId {
        let rent_id = self.data::<Data>().rent_id;
        self.data::<Data>().rent_id += 1;
        rent_id
    }
}

#[modifier_definition]
pub fn only_fee_setter<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<HotelError>,
{
    if instance.data().land_lord != T::env().caller() {
        return Err(From::from(HotelError::CallerIsNotOwner));
    }
    body(instance)
}
