use codec::{Decode, Encode};
use core::marker::PhantomData;
use frame_support::{sp_runtime::traits::TrailingZeroInput, traits::OriginTrait};
use sp_core::H256;

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

pub trait ParachainPalletGeneralIndexOriginDeriver<Origin> {
    fn derive_account(para_id: u32, pallet_index: u8, id: u128) -> Option<Origin>;
}

pub trait ParachainPalletGeneralIndexAccountIdDeriver<AccountId> {
    fn derive_account(para_id: u32, palelt_index: u8, id: u128) -> Option<AccountId>;
}

pub struct TinkernetMultisigAccountIdDeriver<AccountId>(PhantomData<AccountId>);
impl<AccountId: Decode> ParachainPalletGeneralIndexAccountIdDeriver<AccountId>
    for TinkernetMultisigAccountIdDeriver<AccountId>
{
    fn derive_account(para_id: u32, pallet_index: u8, id: u128) -> Option<AccountId> {
        if para_id == 2125 && pallet_index == 71 {
            Some(
                AccountId::decode(&mut TrailingZeroInput::new(&derive_tinkernet_multisig(
                    id as u32,
                )))
                .expect("infinite length input; no invalid inputs for type; qed"),
            )
        } else {
            None
        }
    }
}

pub struct TinkernetMultisigSignedDeriver<Origin>(PhantomData<Origin>);
impl<Origin: OriginTrait> ParachainPalletGeneralIndexOriginDeriver<Origin>
    for TinkernetMultisigSignedDeriver<Origin>
where
    Origin::AccountId: Decode,
{
    fn derive_account(para_id: u32, pallet_index: u8, id: u128) -> Option<Origin> {
        if para_id == 2125 && pallet_index == 71 {
            Some(Origin::signed(
                Origin::AccountId::decode(&mut TrailingZeroInput::new(&derive_tinkernet_multisig(
                    id as u32,
                )))
                .expect("infinite length input; no invalid inputs for type; qed"),
            ))
        } else {
            None
        }
    }
}
