use super::derivers::{
    ParachainPalletGeneralIndexAccountIdDeriver, TinkernetMultisigAccountIdDeriver,
};
use core::marker::PhantomData;
use xcm::v2::{Junction, Junctions, MultiLocation};
use xcm_executor::traits::Convert;

pub struct PalletInstanceGeneralIndexAsAccountId<AccountId, Deriver>(
    PhantomData<(AccountId, Deriver)>,
);
impl<
        AccountId: From<[u8; 32]> + Clone,
        Deriver: ParachainPalletGeneralIndexAccountIdDeriver<AccountId>,
    > Convert<MultiLocation, AccountId>
    for PalletInstanceGeneralIndexAsAccountId<AccountId, Deriver>
{
    fn convert(location: MultiLocation) -> Result<AccountId, MultiLocation> {
        let id = match location.clone() {
            MultiLocation {
                parents: _,
                interior:
                    Junctions::X3(
                        Junction::Parachain(para_id),
                        Junction::PalletInstance(pallet_index),
                        Junction::GeneralIndex(id),
                    ),
            } => Deriver::derive_account(para_id, pallet_index, id).ok_or(location)?,
            _ => return Err(location),
        };
        Ok(id)
    }
}

pub type TinkernetMultisigAsAccountId<AccountId> =
    PalletInstanceGeneralIndexAsAccountId<AccountId, TinkernetMultisigAccountIdDeriver<AccountId>>;
