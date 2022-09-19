//! Could not get file to compile in the end...
//! Did not finish all of the funtions
//! Need to spend mroe time with ti and make sure that i was using each section of this file correctly

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

mod votes;
mod quadratic-conviction;



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

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Votes<AccountId, BlockNumber> {
	/// The proposal's unique index.
	index: ProposalIndex,
	/// The number of approval votes that are needed to pass the motion.
	threshold: MemberCount,
	/// The current set of voters that approved it.
	ayes: Vec<AccountId>,
	/// The current set of voters that rejected it.
	nays: Vec<AccountId>,
	/// The hard end time of this vote.
	end: BlockNumber,
}

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type VoteBalance<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::*;
	use frame_support::traits::{Currency, ReservableCurrency};
}

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
	Proposal<<T as frame_system::Config>::AccountId, BoundedVec<u8, T::NameMaxLength>>,>;


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
		/// Valid account for proposal 
		ValidAccount,
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
		// Referendum already ended.
		ReferendumAlreadyEnded,
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		// Left weights as was in example

		/// Create a new proposal
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_proposal(origin: OriginFor<T>, new_proposal: Vec<u32>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;

			// Check that the account is valid.
			ensure!(T::PalletId::get().is_signer(who), Error::<T>::ValidAccount);
			
			// Check that the proposal is not too long.
			ensure!(new_proposal.len() <= T::MaxProposalLength::get(), Error::<T>::ProposalTooLong);
			
			// Check that the proposal is not too short.
			ensure!(new_proposal.len() > 0, Error::<T>::ProposalTooShort);
			
			// Check that the proposal is not empty.
			ensure!(new_proposal.len() > 0, Error::<T>::ProposalEmpty);
						

			// Update storage with new proposal
			<Proposal<T>>::put(proposal_info::<T>::next_id(), new_proposal);

			// Emit an event - ProposalSubmitted
			Self::deposit_event(Event::ProposalSubmitted(new_proposal, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Start a voting Round with a given proposal
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn start_voting_round(origin: OriginFor<T>, block_numebr: T::BlockNumber) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Check that the referendum is not already started.
			if <ReferendumInfo<T>>::get(block_numebr).is_some() {
				return Err(Error::<T>::NotAValidReferendum.into());
			}
			// Check that there is at least one proposal.
			if <Proposal<T>>::iter().count() == 0 {
				return Err(Error::<T>::NoProposalForReferendum.into());
			}
			let referendum_index = <ReferendumInfo<T>>::next_id();
			
			// Update storage with new referendum
			<ReferendumInfo<T>>::put(referendum_index, ReferendumInfo {
				end: block_numebr + T::BlockPeriod::get(),
				proposal_count: <Proposal<T>>::iter().count(),
			});

			// Emit an event - ReferendumStarted
			Self::deposit_event(Event::ReferendumStarted(referendum_index, block_numebr));

		}

		/// Vote on a referendum.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn vote(origin, referendum_index: ReferendumIndex, vote: Vote) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check that the referendum is valid.
			ensure!(<ReferendumInfo<T>>::get(referendum_index).is_some(), Error::<T>::NotAValidReferendum);
			
			// Check that the referendum is not ended.
			ensure!(<ReferendumInfo<T>>::get(referendum_index).unwrap().end > <system::Module<T>>::block_number(), Error::<T>::NotAValidReferendum);
			
			// Check that the referendum is not already voted on.
			ensure!(<Votes<T>>::get(&(who, referendum_index)).is_none(), Error::<T>::AlreadyVoted);
			
			let referendum_votes = <Votes<T>>::get(&(who, referendum_index)).unwrap_or_default();

			//  TODO: Finish voting logic
			for vote in vote.iter() {
				referendum_votes.push(vote.clone());
			}

			<Votes<T>>::insert(&(who, referendum_index), referendum_votes);

			// Emit an event - VoteCast
			Self::deposit_event(Event::VoteCast(who, referendum_index, vote));

		}

		/// End a referendum.
		// Does not need weight as it should end kbased on blocknumber
		pub fn end_referendum(origin, referendum_index: ReferendumIndex) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Check that the referendum is valid.
			ensure!(<ReferendumInfo<T>>::get(referendum_index).is_some(), Error::<T>::NotAValidReferendum);
			
			// Check that the referendum is not ended.
			ensure!(<ReferendumInfo<T>>::get(referendum_index).unwrap().end > <system::Module<T>>::block_number(), Error::<T>::NotAValidReferendum);
			
			// Check that the referendum is not already voted on.
			ensure!(<Votes<T>>::get(&(who, referendum_index)).is_none(), Error::<T>::AlreadyVoted);
			
			// Check that the referendum is not already ended.
			ensure!(<ReferendumInfo<T>>::get(referendum_index).unwrap().end > <system::Module<T>>::block_number(), Error::<T>::NotAValidReferendum);
			
			// Update storage with new referendum
			<ReferendumInfo<T>>::put(referendum_index, ReferendumInfo {
				end: <system::Module<T>>::block_number(),
				proposal_count: <Proposal<T>>::iter().count(),
			});
			// Emit an event - ReferendumEnded
			Self::deposit_event(Event::ReferendumEnded(referendum_index));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


	}



