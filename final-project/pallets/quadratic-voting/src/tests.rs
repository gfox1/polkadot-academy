//! Added tests for the functions I created in lib.rs
//! Did not add tests for the functions I created in votes.rs
//! Did not add tests for the functions I created in quadratic-conviction.rs
//! Could add test for error handling


use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_proposal_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Pallet::propose(Origin::signed(1), "proposal".into()));
		// Read pallet storage and assert an expected result.
		assert_eq!(Pallet::proposals().unwrap(), vec![(1, "proposal".into())]);
	});
}

#[test]
fn voting_round_starts() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			Pallet::start_voting_round(Origin::signed(1), 0),
			Error::<Test>::NoProposalForReferendum
		);
	});
}
	// TODO: test that the voting works 
	// #[test]
	// fn vote_works() {
	// 	new_test_ext().execute_with(|| {
	// 		// Vote for the proposal
	// 		assert_ok!(Pallet::vote(Origin::signed(1), 1, true));
	// 		// Read pallet storage and assert an expected result.
	// 		assert_eq!(Pallet::votes().unwrap(), vec![(1, 1, true)]);
	// 	});
	// }

#[test]
fn end_referendum_works() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when round ends
		assert_noop!(
			Pallet::end_referendum(Origin::signed(1), 0),
			Error::<Test>::ReferendumAlreadyEnded
		);
	});
}