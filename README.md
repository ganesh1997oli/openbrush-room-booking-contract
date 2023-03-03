# Openbrush hotel room booking contract

This is smart contract written for hotel industry. It tracks the records like:

1. landlord
2. tenants
3. rooms
4. agreements
5. rents room

. `add_room` with this function you can add room with fields like [room_name, room_address, rent_per_month, security_deposit, time_stamp]. Only owner of the contract is supposed to add the room.

. `sign_agreement` with this function only user other that owner can sign agreement with `room_id`. User should pay fee more than `total_fee = rent_per_month + security_deposit` in order to sign agreement. Room must be vacant before sign agreement.

. `pay_rent` with this function user who is tenant of the room can `pay_rent` if time_stamp exceeds.

. `agreement_completed` with this function only owner of the contract can complete the agreement. To complete agreement room musn't be vacant. After complete transfer `security_deposit` back to tenant.

. `agreement_terminated` with this function agreement can be terminated by owner of the contract. Room must be occupied by tenant to execuate this function.
