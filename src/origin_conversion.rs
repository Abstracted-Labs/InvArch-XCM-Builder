use super::derivers::{ParachainPalletGeneralIndexOriginDeriver, TinkernetMultisigSignedDeriver};
use core::marker::PhantomData;
use frame_support::traits::OriginTrait;
use xcm::v3::{Junction, Junctions, MultiLocation, OriginKind};
use xcm_executor::traits::ConvertOrigin;

pub struct DeriveOriginFromPalletInstanceGeneralIndex<Origin, Deriver>(
    PhantomData<(Origin, Deriver)>,
);
impl<Origin: OriginTrait, Deriver: ParachainPalletGeneralIndexOriginDeriver<Origin>>
    ConvertOrigin<Origin> for DeriveOriginFromPalletInstanceGeneralIndex<Origin, Deriver>
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
                    parents: _,
                    interior:
                        Junctions::X3(
                            Junction::Parachain(para_id),
                            Junction::PalletInstance(pallet_index),
                            Junction::GeneralIndex(id),
                        ),
                },
            ) => Deriver::derive_account(para_id, pallet_index, id).ok_or(origin),
            (_, origin) => Err(origin),
        }
    }
}

pub type DeriveOriginFromTinkernetMultisig<Origin> =
    DeriveOriginFromPalletInstanceGeneralIndex<Origin, TinkernetMultisigSignedDeriver<Origin>>;
