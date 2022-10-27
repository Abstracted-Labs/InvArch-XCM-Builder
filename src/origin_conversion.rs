use crate::derivers::ParachainPluralityOriginDeriver;
use core::{fmt::Debug, marker::PhantomData};
use frame_support::{sp_runtime::traits::AtLeast32BitUnsigned, traits::OriginTrait};
use frame_system::RawOrigin as SystemRawOrigin;
use pallet_inv4::INV4Origin;
use xcm::latest::{BodyId, BodyPart, Junction, Junctions, MultiLocation, NetworkId, OriginKind};
use xcm_executor::traits::ConvertOrigin;

pub struct ConvertSignedOrMultisig<Origin, IpId, AccountId>(
    core::marker::PhantomData<(Origin, IpId, AccountId)>,
);

impl<
        Origin: OriginTrait + Clone + Debug,
        IpId: AtLeast32BitUnsigned + Into<u32>,
        AccountId: Into<[u8; 32]>,
    > xcm_executor::traits::Convert<Origin, MultiLocation>
    for ConvertSignedOrMultisig<Origin, IpId, AccountId>
where
    Origin::PalletsOrigin: From<SystemRawOrigin<AccountId>>
        + TryInto<SystemRawOrigin<AccountId>, Error = Origin::PalletsOrigin>
        + From<INV4Origin<IpId, AccountId>>
        + TryInto<INV4Origin<IpId, AccountId>, Error = Origin::PalletsOrigin>,
{
    fn convert(o: Origin) -> Result<MultiLocation, Origin> {
        let result = o.try_with_caller(|caller| match caller.clone().try_into() {
            Ok(SystemRawOrigin::Signed(who)) => Ok(Junction::AccountId32 {
                network: NetworkId::Any,
                id: who.into(),
            }
            .into()),

            _ => match caller.clone().try_into() {
                Ok(INV4Origin::Multisig { id, .. }) => Ok(Junction::Plurality {
                    id: BodyId::Index(id.into()),
                    part: BodyPart::Voice,
                }
                .into()),

                _ => Err(caller),
            },
        });

        result
    }
}

pub struct DeriveOriginFromPlurality<Origin, PluralityDeriver>(
    PhantomData<(Origin, PluralityDeriver)>,
);
impl<Origin: OriginTrait, PluralityDeriver: ParachainPluralityOriginDeriver<Origin>>
    ConvertOrigin<Origin> for DeriveOriginFromPlurality<Origin, PluralityDeriver>
where
    Origin::AccountId: From<[u8; 32]>,
{
    fn convert_origin(
        origin: impl Into<MultiLocation>,
        kind: OriginKind,
    ) -> Result<Origin, MultiLocation> {
        let origin = origin.into();
        log::error!(
              target: "xcm::origin_conversion",
            "DeriveOriginFromPlurality origin: {:?}, kind: {:?}",
              origin, kind,
        );
        match (kind, origin.clone()) {
            (
                OriginKind::Native,
                MultiLocation {
                    parents: 1,
                    interior:
                        Junctions::X2(Junction::Parachain(para_id), Junction::Plurality { id, part }),
                },
            ) => PluralityDeriver::derive_account(para_id, id, part).ok_or(origin),
            (_, origin) => Err(origin),
        }
    }
}
