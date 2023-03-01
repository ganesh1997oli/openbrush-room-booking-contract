pub use crate::{
    ensure,
    impls::room_book::types::{
        AgreementId, Data, HotelError, Rent, RentId, Room, RoomAgreement, RoomId,
    },
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

    #[modifiers(normal_user)]
    default fn sign_agreement(&mut self, room_id: RoomId) -> Result<(), HotelError> {
        // caller of the contract
        let caller = Self::env().caller();

        // value transfer while calling contract
        let value = Self::env().transferred_value();

        // get the romm of specific `room_id`
        let mut room = self.data::<Data>().room.get(&room_id).unwrap_or_default();

        // get the room `landlord`
        let room_landlord = room.landlord;

        // get the total to sign the agreement
        let total_fee = room.rent_per_month + room.security_deposit;

        // check if caller is paying enough `agreement_fee`
        ensure!(value >= total_fee, HotelError::NotEnoughAgreementFee);

        // transfer `total_fee` to `landlord`
        Self::env()
            .transfer(room_landlord, total_fee)
            .unwrap_or_default();

        // get the `next_room_agreement_id`
        let agreement_id = self.next_agreement_id();

        // update `room` data of `room_id`
        room.current_tenant = caller;
        room.vacant = false;
        room.time_stamp = Self::env().block_timestamp();
        room.agreement_id = agreement_id;

        // create new `RoomAgreement` object with given fields
        let agreement = RoomAgreement {
            room_id,
            agreement_id,
            room_name: room.room_name.clone(),
            room_address: room.room_address.clone(),
            rent_per_month: room.rent_per_month,
            security_deposit: room.security_deposit,
            lock_in_period: 1,
            time_stamp: room.time_stamp,
        };

        // insert room `sign_agreement` to the agreement mapping
        self.data::<Data>()
            .agreement
            .insert(&agreement_id, &agreement);

        // get the `next_rent_id`
        let rent_id = self.next_rent_id();

        // create new `Rent` object with the given fields
        let rent = Rent {
            rent_id,
            room_id,
            agreement_id,
            room_name: room.room_name,
            room_address: room.room_address,
            rent_per_month: room.rent_per_month,
            time_stamp: room.time_stamp,
            tenant_address: caller,
            land_lord_address: room_landlord,
        };

        // insert `Rent` in the rent mapping
        self.data::<Data>().rent.insert(&rent_id, &rent);

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

// modifier to check normal user
#[modifier_definition]
pub fn normal_user<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<HotelError>,
{
    if instance.data().land_lord == T::env().caller() {
        return Err(From::from(HotelError::CallerIsOwner));
    }
    body(instance)
}
