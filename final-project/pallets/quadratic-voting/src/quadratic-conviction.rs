//! I tried reworking a few things from the conviction.rs file from the democracy pallet.
//! In the end I dont think it worked for my use case and ultimately I decided to move on.
//! leaving the file for now.
//! To be removed if lib.rs file is working.



use scale_info::TypeInfo;
use codec::{Encode, Decode};
use frame_support::RuntimeDebug;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
};

/// A value denoting the strength of conviction of a vote.
#[derive( Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, TypeInfo, MaxEncodedLen,)]
pub enum QuadraticConviction {
	/// No conviction, equals one vote.
	None,
	/// Votes multiplied on a quadratic basis.
	Quadratic,
}

impl Default for Conviction {
	fn default() -> Self {
		QuadraticConviction::None
	}
}

impl From<QuadraticConviction> for f64 {
	fn from(c: QuadraticConviction) -> f64 {
		match c {
			QuaraticConviction::None => 1,
			QuadraticConviction::Quadratic => f64,
		}
	}
}


impl TryFrom<u8> for Conviction {
	type Error = ();
	fn try_from(i: f64) -> Result<QuadraticConviction, ()> {
		Ok(match i {
			1 => QuadraticConviction::None,
			f64 => QuadraticConviction::Quadratic,
			_ => return Err(()),
		})
	}
}

impl QuadraticConviction {
	/// The amount of time (in number of periods) that our conviction implies a successful voter's
	pub fn quadratic_vote(self) -> f64 {
		match self {
			QuadraticConviction::None => 1,
            QuadraticConviction::Quadratic => sqrt(tokenamount::Balance::from(1).into()),
		}
	}

      /// The votes of a voter of the given the voters conviction.
    //     pub fn votes<B: From<u8> + Zero + Copy + CheckedMul + CheckedDiv + Bounded>(
    //         self,
    //         capital: B,
    //     ) -> Delegations<B> {
    //         let votes = match self {
    //             QuadraticConviction::None => capital.checked_div(&10u8.into()).unwrap_or_else(Zero::zero),
    //             x => capital.checked_mul(&u8::from(x).into()).unwrap_or_else(B::max_value),
    //         };
    //         Delegations { votes, capital }
    //     }
    // }

    // impl Bounded for Conviction {
    //     fn min_value() -> Self {
    //         Conviction::None
    //     }
    //     fn max_value() -> Self {
    //         Conviction::Locked6x
    //     }
    // }
}