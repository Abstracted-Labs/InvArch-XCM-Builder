use super::derivers::{
    ParachainPalletGeneralIndexAccountIdDeriver, TinkernetMultisigAccountIdDeriver,
};
use core::marker::PhantomData;
use xcm::v3::{Junction, Junctions, MultiLocation};
use xcm_executor::traits::ConvertLocation;

pub struct PalletInstanceGeneralIndexAsAccountId<AccountId, Deriver>(
    PhantomData<(AccountId, Deriver)>,
);
impl<AccountId: Clone, Deriver: ParachainPalletGeneralIndexAccountIdDeriver<AccountId>>
    ConvertLocation<AccountId> for PalletInstanceGeneralIndexAsAccountId<AccountId, Deriver>
{
    fn convert_location(location: &MultiLocation) -> Option<AccountId> {
        match *location {
            MultiLocation {
                parents: _,
                interior:
                    Junctions::X3(
                        Junction::Parachain(para_id),
                        Junction::PalletInstance(pallet_index),
                        Junction::GeneralIndex(id),
                    ),
            } => Deriver::derive_account(para_id, pallet_index, id),
            _ => None,
        }
    }
}

pub type TinkernetMultisigAsAccountId<AccountId> =
    PalletInstanceGeneralIndexAsAccountId<AccountId, TinkernetMultisigAccountIdDeriver<AccountId>>;
