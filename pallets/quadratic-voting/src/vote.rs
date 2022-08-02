//! Somthing I was trying out
//! I wanted to move all of the voting elements to a seperate file to not pollute the lib.rs file.
//! I think this is a good idea, but I'm not sure if it will work.
//! Leaving in for now until I understand if I can/should keep everything in the lib.rs file.


use scale_info::TypeInfo;
use codec::{Encode, Decode};
use frame_support::RuntimeDebug;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
};



/// Quadratic voting conviction, plus a vote, one way or the other.
#[derive(Copy, Clone, Eq, PartialEq, Default, RuntimeDebug, MaxEncodedLen)]
pub struct Vote {
	pub aye: bool,
	pub quadratic_conviction: Conviction,
}

impl Encode for Vote {
	fn encode_to<T: Output + ?Sized>(&self, output: &mut T) {
		output.push_byte(u8::from(self.conviction) | if self.aye { 0b1000_0000 } else { 0 });
	}
}




/// Info for keeping track of a motion being voted on.
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


impl<AccountId, BlockNumber> Votes<AccountId, BlockNumber> {
    /// Create a new vote in the given `proposal` with the given `threshold` and `end` time.
    pub fn new(index: ProposalIndex, threshold: MemberCount, end: BlockNumber) -> Self {
        Votes {
            index,
            threshold,
            ayes: Default::default(),
            nays: Default::default(),
            end,
        }
    }
    /// Add an `AccountId` as an `ayes` vote.
    pub fn add_ayes(&mut self, who: AccountId) {
        self.ayes.push(who);
    }
    /// Add an `AccountId` as a `nays` vote.
    pub fn add_nays(&mut self, who: AccountId) {
        self.nays.push(who);
    }
    /// Remove an `AccountId` as a `ayes` vote.
    pub fn remove_ayes(&mut self, who: AccountId) {
        self.ayes.retain(|i| i != &who);
    }
    /// Remove an `AccountId` as a `nays` vote.
    pub fn remove_nays(&mut self, who: AccountId) {
        self.nays.retain(|i| i != &who);
    }
    /// Return the current vote count.
    pub fn vote_count(&self) -> u32 {
        self.ayes.len() as u32 + self.nays.len() as u32
    }
    /// Return the current approval count.
    pub fn approval_count(&self) -> u32 {
        self.ayes.len() as u32
    }
    /// Return the current approval threshold.
    pub fn approval_threshold(&self) -> MemberCount {
        self.threshold
    }
    /// Return the current set of `ayes`.
    pub fn ayes(&self) -> &[AccountId] {
        &self.ayes
    }
    /// Return the current set of `nays`.
    pub fn nays(&self) -> &[AccountId] {
        &self.nays
    }
    /// Return the current end time.
    pub fn end(&self) -> BlockNumber {
        self.end
    }
    /// Return the current proposal index.
    pub fn index(&self) -> ProposalIndex {
        self.index
    }
}
