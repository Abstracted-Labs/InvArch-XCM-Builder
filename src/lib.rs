#![cfg_attr(not(feature = "std"), no_std)]

pub mod barriers;
pub mod derivers;
pub mod location_conversion;
pub mod origin_conversion;

pub use barriers::TinkernetMultisigMultiLocation;
pub use location_conversion::TinkernetMultisigAsAccountId;
pub use origin_conversion::DeriveOriginFromTinkernetMultisig;

#[cfg(test)]
mod tests {
    use super::*;
    use codec::Encode;
    use frame_support::sp_runtime::AccountId32;
    use moonbeam_accountid20::AccountId20;
    use xcm::latest::{
        Junction::{GeneralIndex, PalletInstance, Parachain},
        Junctions::X3,
        MultiLocation,
    };
    use xcm_executor::traits::Convert;

    #[test]
    fn accountid32() {
        assert_eq!(
            TinkernetMultisigAsAccountId::<AccountId32>::convert(MultiLocation {
                parents: 1,
                interior: X3(Parachain(2125), PalletInstance(71), GeneralIndex(0)),
            })
            .unwrap()
            .encode(),
            hex::decode("61a0f43c8591aa1aca6ccb9c7274af1e9cc32b65f333c1a298bc1ea5f451465a")
                .unwrap()
        )
    }

    #[test]
    fn accountid20() {
        assert_eq!(
            TinkernetMultisigAsAccountId::<AccountId20>::convert(MultiLocation {
                parents: 1,
                interior: X3(Parachain(2125), PalletInstance(71), GeneralIndex(1)),
            })
            .unwrap()
            .encode(),
            hex::decode("f30971b7e3b1a6788e90b8ba50d441169869be9b").unwrap()
        )
    }
}
