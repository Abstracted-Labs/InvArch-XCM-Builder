use core::marker::PhantomData;
use frame_support::{ensure, traits::Contains};

use xcm::latest::{
    Instruction, Junction, Junctions, MultiLocation, Weight,
    WeightLimit::{Limited, Unlimited},
    Xcm,
};
use xcm_executor::traits::ShouldExecute;

pub struct AllowPaidDescendedOriginFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowPaidDescendedOriginFrom<T> {
    fn should_execute<Call>(
        origin: &MultiLocation,
        message: &mut Xcm<Call>,
        max_weight: Weight,
        _weight_credit: &mut Weight,
    ) -> Result<(), ()> {
        log::error!(
            target: "xcm::barriers",
              "AllowPaidDescendedOriginFrom origin: {:?}, message: {:?}, max_weight: {:?}, weight_credit: {:?}",
            origin, message, max_weight, _weight_credit,
        );

        let mut iter = message.0.iter_mut();
        let i = iter.next().ok_or(())?;

        let descended_origin = match i {
            Instruction::DescendOrigin(junctions) => {
                let mut descended = origin.clone();
                for junction in junctions.clone().into_iter() {
                    descended.interior.push(junction).map_err(|_| ())?;
                }
                descended
            }
            _ => return Err(()),
        };

        log::error!("descended origin: {:?}", descended_origin);

        ensure!(T::contains(&descended_origin), ());

        let i = iter.next().ok_or(())?;

        match i {
            Instruction::WithdrawAsset(..) => (),
            _ => return Err(()),
        }
        let i = iter.next().ok_or(())?;
        match i {
            Instruction::BuyExecution {
                weight_limit: Limited(ref mut weight),
                ..
            } if *weight >= max_weight => {
                *weight = max_weight;
                Ok(())
            }
            Instruction::BuyExecution {
                ref mut weight_limit,
                ..
            } if weight_limit == &Unlimited => {
                *weight_limit = Limited(max_weight);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

pub struct TinkernetMultisigMultiLocation;
impl Contains<MultiLocation> for TinkernetMultisigMultiLocation {
    fn contains(t: &MultiLocation) -> bool {
        matches!(
            t,
            MultiLocation {
                parents: 1,
                interior: Junctions::X2(Junction::Parachain(2125), Junction::Plurality { .. })
            }
        )
    }
}
