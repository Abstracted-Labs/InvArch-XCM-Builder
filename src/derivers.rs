use codec::{Decode, Encode};
use core::marker::PhantomData;
use frame_support::{sp_runtime::traits::TrailingZeroInput, traits::OriginTrait};
use sp_core::H256;
use xcm::latest::{BodyId, BodyPart};

pub fn derive_tinkernet_multisig(index: u32) -> [u8; 32] {
    (
        H256([
            212, 46, 150, 6, 169, 149, 223, 228, 51, 220, 121, 85, 220, 42, 112, 244, 149, 243, 80,
            243, 115, 218, 162, 0, 9, 138, 232, 68, 55, 129, 106, 210,
        ]),
        index,
    )
        .using_encoded(sp_io::hashing::blake2_256)
}

pub trait ParachainPluralityOriginDeriver<Origin> {
    fn derive_account(para_id: u32, id: BodyId, part: BodyPart) -> Option<Origin>;
}

pub trait ParachainPluralityAccountIdDeriver<AccountId> {
    fn derive_account(para_id: u32, id: BodyId, part: BodyPart) -> Option<AccountId>;
}

pub struct TinkernetPluralityAccountIdDeriver<AccountId>(PhantomData<AccountId>);
impl<AccountId: From<[u8; 32]> + Decode> ParachainPluralityAccountIdDeriver<AccountId>
    for TinkernetPluralityAccountIdDeriver<AccountId>
{
    fn derive_account(para_id: u32, id: BodyId, part: BodyPart) -> Option<AccountId> {
        if para_id == 2125 {
            match (id, part) {
                (BodyId::Index(index), BodyPart::Voice) => Some(
                    AccountId::decode(&mut TrailingZeroInput::new(&derive_tinkernet_multisig(
                        index,
                    )))
                    .expect("infinite length input; no invalid inputs for type; qed"),
                ),

                _ => None,
            }
        } else {
            None
        }
    }
}

pub struct TinkernetPluralitySignedDeriver<Origin>(PhantomData<Origin>);
impl<Origin: OriginTrait> ParachainPluralityOriginDeriver<Origin>
    for TinkernetPluralitySignedDeriver<Origin>
where
    Origin::AccountId: Decode + From<[u8; 32]>,
{
    fn derive_account(para_id: u32, id: BodyId, part: BodyPart) -> Option<Origin> {
        if para_id == 2125 {
            match (id, part) {
                (BodyId::Index(index), BodyPart::Voice) => Some(Origin::signed(
                    Origin::AccountId::decode(&mut TrailingZeroInput::new(
                        &derive_tinkernet_multisig(index),
                    ))
                    .expect("infinite length input; no invalid inputs for type; qed"),
                )),

                _ => None,
            }
        } else {
            None
        }
    }
}
