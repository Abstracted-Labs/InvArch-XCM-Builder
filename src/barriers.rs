use frame_support::traits::Contains;
use xcm::v3::{Junction, Junctions, MultiLocation};

pub struct TinkernetMultisigMultiLocation;
impl Contains<MultiLocation> for TinkernetMultisigMultiLocation {
    fn contains(t: &MultiLocation) -> bool {
        matches!(
            t,
            MultiLocation {
                parents: _,
                interior: Junctions::X3(
                    Junction::Parachain(2125),
                    Junction::PalletInstance(71),
                    Junction::GeneralIndex(_)
                )
            }
        )
    }
}
