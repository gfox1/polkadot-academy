#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;




#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::*;

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
	
		// The standard event type
		type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;



	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// Use getter methos to access proposal count 
	// https://docs.substrate.io/main-docs/build/runtime-storage/#:~:text=a%20storage%20item.-,Getter%20methods,-The%20%23%5Bpallet%3A%3Agetter
	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub type ProposalCount<T: Config> = StorageValue<_, u32>; //May need to add a few more items here



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
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
	}

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