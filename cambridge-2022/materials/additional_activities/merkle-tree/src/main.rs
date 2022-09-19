//! This is minimal Merkle tree implementation with proof checking

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

fn main() {
	let s = "";
	let h = hash(&s);
	println!("{}", h);
}

/// We'll use Rust's built-in hashing which returns a u64 type.
/// This alias just helps us understand when we're treating the number as a hash
type HashValue = u64;

/// Helper function that makes the hashing interface easier to understand.
fn hash<T: Hash>(t: &T) -> HashValue {
	let mut s = DefaultHasher::new();
	t.hash(&mut s);
	s.finish()
}

/// Given a vector of data blocks this function adds padding blocks to the end
/// until the length is a power of two which is needed for Merkle trees.
fn pad_base_layer(blocks: &mut Vec<&str>) {
	todo!()
}

/// Helper function to combine two hashes and compute the hash of the combination.
/// This will be useful when building the intermediate nodes in the Merkle tree.
///
/// There are many correct ways to do this, but the easiest one and the one that I recommend
/// is to convert the hashes to strings and concatenate them.
fn concatenate_hash_values(left: HashValue, right: HashValue) -> HashValue {
	todo!()
}

/// Calculates the Merkle root of a sentence. We consider each word in the sentence to
/// be one block. Words are separated by one or more spaces.
///
/// Example:
/// Sentence: "Trust me, bro!"
/// "Trust", "me," "bro!"
/// Notice that the punctuation like the comma and exclamation point are included in the words
/// but the spaces are not.
fn calculate_merkle_root(sentence: &str) -> HashValue {
	todo!()
}

/// A representation of a sibling node along the Merkle path from the data
/// to the root. It is necessary to specify which side the sibling is on
/// so that the hash values can be combined in the same order.
enum SiblingNode {
	Left(HashValue),
	Right(HashValue),
}

/// Generates a Merkle proof that one particular word is contained
/// in the given sentence. You provide the sentence and the index of the word
/// which you want a proof.
///
/// Panics if the index is beyond the length of the word.
///
/// Example: I want to prove that the word "Trust" is in the sentence "Trust me, bro!"
/// So I call generate_proof("Trust me, bro!", 0)
/// And I get back the merkle root and list of intermediate nodes from which the
/// root can be reconstructed.
fn generate_proof(sentence: &str, index: usize) -> (HashValue, Vec<SiblingNode>) {
	todo!()
}

/// Checks whether the given word is contained in a sentence, without knowing the whole sentence.
/// Rather we only know the merkle root of the sentence and a proof.
fn validate_proof(root: HashValue, word: &str, proof: Vec<SiblingNode>) -> bool {
	todo!()
}

#[test]
fn there_is_some_hash() {
	let h = calculate_merkle_root("Trust me, bro!");
	assert!(h > 0);
}

#[test]
fn padding() {
	let mut arr_4: Vec<&str> = "Trust me, bro!".split_ascii_whitespace().collect();
	let mut arr_8: Vec<&str> = "Trust me, bro bro bro!".split_ascii_whitespace().collect();
	pad_base_layer(&mut arr_4);
	pad_base_layer(&mut arr_8);
	assert_eq!(
		arr_4.len(),
		4,
		"You are not padding correctly. Expected 4 elements, found {}",
		arr_4.len()
	);
	assert_eq!(
		arr_8.len(),
		8,
		"You are not padding correctly. Expected 8 elements, found {}",
		arr_8.len()
	);
}

#[test]
fn different_hashes() {
	let h = calculate_merkle_root("Trust me, bro!");
	let h2 = calculate_merkle_root("Trust me, bro bro!");
	assert_ne!(h, h2, "Hashes for 3 word string is the same as for 4 word string");
}

#[test]
fn check_root_of_proof() {
	let h = calculate_merkle_root("Trust me, bro!");
  let result = generate_proof("Trust me, bro!", 1);
  assert_eq!(h, result.0, "Merkle root from generated proof is not the same as the original merkle root");
}

#[test]
fn check_different_roots_of_proof() {
	let h = calculate_merkle_root("Trust me, German!");
  let result = generate_proof("Trust me, bro!", 1);
  assert_ne!(h, result.0, "Merkle root from generated proof is the same as the merkle root for different sentence");
}

#[test]
fn check_merkle_proofs() {
  let proof = generate_proof("Trust me, bro!", 1);
  let result = validate_proof(proof.0, "me,", proof.1);
  assert!(result, "Your proof is incorrect. The word is in the sentence, but not identified by your solution");
}

#[test]
fn check_merkle_proofs_incorrect() {
  let proof = generate_proof("Trust me, bro!", 1);
  let result = validate_proof(proof.0, "hello", proof.1);
  assert!(!result, "Your proof is incorrect. The word is NOT in the sentence, but you validated it to be in");
}
