use crate::derivers::ParachainPluralityAccountIdDeriver;
use core::marker::PhantomData;
use xcm::latest::{Junction, Junctions, MultiLocation};
use xcm_executor::traits::Convert;

pub struct PluralityAsAccountId<AccountId, PluralityDeriver>(
    PhantomData<(AccountId, PluralityDeriver)>,
);
impl<
        AccountId: From<[u8; 32]> + Clone,
        PluralityDeriver: ParachainPluralityAccountIdDeriver<AccountId>,
    > Convert<MultiLocation, AccountId> for PluralityAsAccountId<AccountId, PluralityDeriver>
{
    fn convert(location: MultiLocation) -> Result<AccountId, MultiLocation> {
        let id = match location.clone() {
            MultiLocation {
                parents: 1,
                interior:
                    Junctions::X2(Junction::Parachain(para_id), Junction::Plurality { id, part }),
            } => PluralityDeriver::derive_account(para_id, id, part).ok_or(location)?,
            _ => return Err(location),
        };
        Ok(id)
    }
}
