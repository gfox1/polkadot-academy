#![cfg_attr(not(feature = "std"), no_std)]


use codec::MaxEncodedLen;
use frame_support::{ codec::{Decode, Encode}, traits::{Currency, EnsureOrigin, Get, ReservableCurrency}, BoundedVec, PalletId,};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::traits::{AccountIdConversion, Hash};
use sp_runtime::RuntimeDebug;
use sp_std::{convert::TryInto, vec, vec::Vec};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct Proposal<AccountId, BoundedString> {
    pub proposal_hash: Hash,
	pub total_votes: u128,
    pub quadratic_amount: u128,
    pub name: BoundedString,
    pub owner: AccountId,
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct VotingRound<AccountId, BoundedString> {
    pub name: BoundedString,
    pub ongoing: bool,
    pub admin: AccountId,
}

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type VoteBalance<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::*;
	use frame_support::traits::{Currency, ReservableCurrency};


	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		
		
		// The standard event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// Bring in currency and reservable currency - https://docs.rs/frame-support/latest/frame_support/traits/trait.ReservableCurrency.html
		type Currency: ReservableCurrency<Self::AccountId> + Currency<Self::AccountId>;

		#[pallet::constant]
        type PalletId: Get<Self::PalletId>;

		#[pallet::constant]
		type MaxProposalLength: Get<u32>;

	}

	// Did not add anything to this section
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	// Use getter method to access proposal list
	#[pallet::storage]
	#[pallet::getter(fn proposal_info)]
	
	pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, u32, 
	ProposalInfo<<T as frame_system::Config>::AccountId, BoundedVec<u8, T::NameMaxLength>>,>;


	#[pallet::storage]
	#[pallet::getter(fn proposal_votes)]
    pub type ProposalVotes<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::Hash, Blake2_128Concat, T::AccountId, u128>;

	// Use getter method to access proposal participants
	#[pallet::storage]
    #[pallet::getter(fn proposal_participants)]
    pub(super) type ProposalParticipants<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, bool>;

	


	// Events to inform the user of his action.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Submitted a proposal.
		ProposalSubmitted(Proposal<T>, T::AccountId),
		// Referendum starts
		ReferendumStarted(ReferendumIndex, T::BlockNumber),
		// Voted on a referendum.
		Voted(Proposal<T>, Vote, T::AccountId),
		// The referendum ended.
		ReferendumEnded(ReferendumIndex, Result<T::AccountId, VoteError>),

	}

	// Errors for the pallet
	#[pallet::error]
	pub enum Error<T> {
		/// Error for when a proposal is submitted that is too long.
		ProposalTooLong,
		/// Error for when a proposal is submitted that is too short.
		ProposalTooShort,
		/// Error for when a proposal is submitted that is empty.
		ProposalEmpty,
		/// Error for when a referendum is started with no proposals.
		NoProposalForReferendum,
		// Already voted on this referendum.
		AlreadyVoted,
		// Not a valid referendum.
		NotAValidReferendum,
		// No referendum to vote on
		NoReferendumToVoteOn,
		// Not enough tokens to vote on this referendum.
		NotEnoughTokens,
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
