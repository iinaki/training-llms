https://github.com/iinaki/rusty-merkle-tree/tree/main/src

To develop a node that will be part of a blockchain in Rust, I need to create a Merkle Tree in order to store hashes efficiently. I want to be able to add and remove hashes from the Merkle Tree and this  to be dynamically updated.
I also want to be able to get a proof of inclusion of a hash in the merkle tree, this is the path from the sibling of the leaf to the top of the tree that forms the merkle root.

Help me develop the MerkleTree struct with this form:
/// A Merkle Tree implementation
///
/// # Methods
/// - `new_from_hashes`: Creates a new MerkleTree from a list of hashes.
/// - `root`: Returns the root of the Merkle Tree, which is the Merkle Root.
/// - `verify`: Verifies that a given hash is contained in the Merkle Tree.
/// - `proof_of_inclusion`: Returns a proof of inclusion for a given hash in the Merkle Tree.
/// - `add_hash`: Adds a hash to the Merkle Tree, updating the tree structure.
/// - `remove_hash`: Removes a hash from the Merkle Tree, updating the tree structure.
pub struct MerkleTree {
    levels: Vec<Vec<MerkleHash>>,
}

Model A

-> no compila mismached types: 134 | let proof = tree.proof_of_inclusion(hash).unwrap();
| ^^^^ expected struct `std::string::String`, found str
|
= note: expected type `&std::string::String`
found type `&str`

// TESTING FOR MODEL A

// use std::collections::HashSet;
use std::fmt;

// Custom type for Merkle Hashes
pub type MerkleHash = String;

/// A Merkle Tree implementation
///
/// # Methods
/// - `new_from_hashes`: Creates a new MerkleTree from a list of hashes.
/// - `root`: Returns the root of the Merkle Tree, which is the Merkle Root.
/// - `verify`: Verifies that a given hash is contained in the Merkle Tree.
/// - `proof_of_inclusion`: Returns a proof of inclusion for a given hash in the Merkle Tree.
/// - `add_hash`: Adds a hash to the Merkle Tree, updating the tree structure.
/// - `remove_hash`: Removes a hash from the Merkle Tree, updating the tree structure.
pub struct MerkleTree {
    levels: Vec<Vec<MerkleHash>>,
}

impl MerkleTree {
    /// Creates a new MerkleTree from a list of hashes.
    pub fn new_from_hashes(hashes: Vec<MerkleHash>) -> MerkleTree {
        let mut tree = MerkleTree { levels: vec![hashes] };
        tree.build_tree();
        tree
    }

    /// Returns the root of the Merkle Tree, which is the Merkle Root.
    pub fn root(&self) -> Option<&MerkleHash> {
        self.levels.last().and_then(|level| level.get(0))
    }

    /// Verifies that a given hash is contained in the Merkle Tree.
    pub fn verify(&self, hash: &MerkleHash) -> bool {
        self.levels[0].contains(hash)
    }

    /// Returns a proof of inclusion for a given hash in the Merkle Tree.
    pub fn proof_of_inclusion(&self, hash: &MerkleHash) -> Option<Vec<(MerkleHash, bool)>> {
        let mut index = self.levels[0].iter().position(|h| h == hash)?;
        let mut proof = Vec::new();

        for i in 0..self.levels.len() - 1 {
            let level = &self.levels[i];
            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
            if sibling_index < level.len() {
                proof.push((level[sibling_index].clone(), index % 2 == 0));
            } else {
                // If the sibling does not exist, use the node itself
                proof.push((level[index].clone(), index % 2 == 0));
            }
            index /= 2;
        }

        Some(proof)
    }

    /// Adds a hash to the Merkle Tree, updating the tree structure.
    pub fn add_hash(&mut self, hash: MerkleHash) {
        self.levels[0].push(hash);
        self.build_tree();
    }

    /// Removes a hash from the Merkle Tree, updating the tree structure.
    pub fn remove_hash(&mut self, hash: &MerkleHash) {
        if let Some(index) = self.levels[0].iter().position(|h| h == hash) {
            self.levels[0].remove(index);
            self.build_tree();
        }
    }

    fn build_tree(&mut self) {
        let mut current_level = 0;
        while self.levels[current_level].len() > 1 {
            let level_hashes = &self.levels[current_level];
            let mut next_level_hashes = Vec::new();

            for i in (0..level_hashes.len()).step_by(2) {
                if i + 1 < level_hashes.len() {
                    let combined = combine_hashes(&level_hashes[i], &level_hashes[i + 1]);
                    next_level_hashes.push(combined);
                } else {
                    // If there's no pair, just move the hash up
                    next_level_hashes.push(level_hashes[i].clone());
                }
            }

            if current_level + 1 < self.levels.len() {
                self.levels[current_level + 1] = next_level_hashes;
            } else {
                self.levels.push(next_level_hashes);
            }

            current_level += 1;
        }

        // Remove any unnecessary levels
        while self.levels.len() > current_level + 1 {
            self.levels.pop();
        }
    }
}
//;
  //  let hash = digest(Algorithm::SHA256, combined.as_bytes());
//    hex::encode(hash)
fn combine_hashes(a: &MerkleHash, b: &MerkleHash) -> MerkleHash {
    format!("{}{}", a, b)
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, level) in self.levels.iter().enumerate() {
            write!(f, "Level {}: {:?}\n", i, level)?;
        }
        Ok(())
    }
}

fn main() {
    let hashes = vec![
        "hash1".to_string(),
        "hash2".to_string(),
        "hash3".to_string(),
        "hash4".to_string(),
    ];

    let mut tree = MerkleTree::new_from_hashes(hashes);
    println!("Merkle Tree:\n{}", tree);
    println!("Merkle Root: {:?}", tree.root());

    let hash = "hash3".to_string();
    let proof = tree.proof_of_inclusion(&hash).unwrap();
    println!("Proof of inclusion for {}: {:?}", hash, proof);

    tree.add_hash("hash5".to_string());
    println!("Updated Merkle Tree:\n{}", tree);

    tree.remove_hash(&"hash2".to_string());
    println!("Updated Merkle Tree after removal:\n{}", tree);
}

Response:
Std Out:

Merkle Tree:
Level 0: ["hash1", "hash2", "hash3", "hash4"]
Level 1: ["hash1hash2", "hash3hash4"]
Level 2: ["hash1hash2hash3hash4"]

Merkle Root: Some("hash1hash2hash3hash4")
Proof of inclusion for hash3: [("hash4", true), ("hash1hash2", false)] -> inclusion perfecta
Updated Merkle Tree:
Level 0: ["hash1", "hash2", "hash3", "hash4", "hash5"] -> tiene que agregar uno mas cuando es impar
Level 1: ["hash1hash2", "hash3hash4", "hash5"]
Level 2: ["hash1hash2hash3hash4", "hash5"]
Level 3: ["hash1hash2hash3hash4hash5"]

Updated Merkle Tree after removal:
Level 0: ["hash1", "hash3", "hash4", "hash5"] -> removal perfecto
Level 1: ["hash1hash3", "hash4hash5"]
Level 2: ["hash1hash3hash4hash5"]


Model B

use std::collections::HashSet;

// Define a type for the hash
type MerkleHash = Vec<u8>;

// A Merkle Tree implementation
///
/// # Methods
/// - `new_from_hashes`: Creates a new MerkleTree from a list of hashes.
/// - `root`: Returns the root of the Merkle Tree, which is the Merkle Root.
/// - `verify`: Verifies that a given hash is contained in the Merkle Tree.
/// - `proof_of_inclusion`: Returns a proof of inclusion for a given hash in the Merkle Tree.
/// - `add_hash`: Adds a hash to the Merkle Tree, updating the tree structure.
/// - `remove_hash`: Removes a hash from the Merkle Tree, updating the tree structure.
pub struct MerkleTree {
    levels: Vec<Vec<MerkleHash>>,
}

impl MerkleTree {
    /// Creates a new MerkleTree from a list of hashes.
    pub fn new_from_hashes(hashes: Vec<MerkleHash>) -> Self {
        let mut tree = MerkleTree { levels: vec![hashes] };
        tree.build_tree();
        tree
    }

    /// Returns the root of the Merkle Tree, which is the Merkle Root.
    pub fn root(&self) -> Option<&MerkleHash> {
        self.levels.last().and_then(|level| level.first())
    }

    /// Verifies that a given hash is contained in the Merkle Tree.
    pub fn verify(&self, hash: &MerkleHash) -> bool {
        self.levels
            .first()
            .unwrap()
            .iter()
            .any(|h| h == hash)
    }

    /// Returns a proof of inclusion for a given hash in the Merkle Tree.
    pub fn proof_of_inclusion(&self, hash: &MerkleHash) -> Option<Vec<MerkleHash>> {
        let mut proof = Vec::new();
        let mut index = self
            .levels
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, h)| h == hash)
            .map(|(i, _)| i)?;

        for level in &self.levels[1..] {
            let sibling_index = if index % 2 == 0 {
                index + 1
            } else {
                index - 1
            };
            proof.push(level[sibling_index].clone());
            index /= 2;
        }

        Some(proof)
    }

    /// Adds a hash to the Merkle Tree, updating the tree structure.
    pub fn add_hash(&mut self, hash: MerkleHash) {
        self.levels.first_mut().unwrap().push(hash);
        self.build_tree();
    }

    /// Removes a hash from the Merkle Tree, updating the tree structure.
    pub fn remove_hash(&mut self, hash: &MerkleHash) {
        if let Some(pos) = self.levels.first_mut().unwrap().iter().position(|h| h == hash) {
            self.levels.first_mut().unwrap().remove(pos);
            self.build_tree();
        }
    }

    fn build_tree(&mut self) {
        let mut current_level = self.levels.last().unwrap().clone();
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..current_level.len()).step_by(2) {
                let hash1 = &current_level[i];
                let hash2 = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    &vec![0; hash1.len()]
                };
                next_level.push(combine_hashes(hash1, hash2));
            }
            self.levels.push(next_level);
            current_level = self.levels.last().unwrap().clone();
        }
    }
}

fn combine_hashes(hash1: &MerkleHash, hash2: &MerkleHash) -> MerkleHash {
    // Here you would implement the actual hash combination function
    // For example, using SHA-256:
    let mut combined = hash1.clone();
    combined.extend(hash2);
    return combined;
}

fn main() {
    let hashes = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];
    let mut tree = MerkleTree::new_from_hashes(hashes);

    println!("Root: {:?}", tree.root());

    let hash = vec![1, 2, 3];
    println!("Verify: {}", tree.verify(&hash));

    let proof = tree.proof_of_inclusion(&hash).unwrap();
    println!("Proof of inclusion: {:?}", proof);

    let new_hash = vec![13, 14, 15];
    tree.add_hash(new_hash.clone());
    println!("Verify after add: {}", tree.verify(&new_hash));

    tree.remove_hash(&new_hash);
    println!("Verify after remove: {}", tree.verify(&new_hash));
}

error[E0277]: can't compare `&std::vec::Vec` with `std::vec::Vec`
--> main.rs:49:30
|
49 | .find(|(_, h)| h == hash)
| ^^ no implementation for `&std::vec::Vec == std::vec::Vec`

in

let mut index = self
            .levels
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, h)| h == hash)
            .map(|(i, _)| i)?;

            -> much harder to debug than in model A

Ideal response

use sha3::{Digest, Sha3_256};

use crate::direction::Direction;
use crate::merkle_tree_error::MerkleTreeError;
use crate::proof_of_inclusion::ProofOfInclusion;

use super::merkle_hash::MerkleHash;

/// A Merkle Tree implementation
///
/// # Methods
/// - `new_from_hashes`: Creates a new MerkleTree from a list of hashes.
/// - `root`: Returns the root of the Merkle Tree, which is the Merkle Root.
/// - `verify`: Verifies that a given hash is contained in the Merkle Tree.
/// - `proof_of_inclusion`: Returns a proof of inclusion for a given hash in the Merkle Tree.
/// - `add_hash`: Adds a hash to the Merkle Tree, updating the tree structure.
/// - `remove_hash`: Removes a hash from the Merkle Tree, updating the tree structure.
#[derive(Debug)]
pub struct MerkleTree {
    levels: Vec<Vec<MerkleHash>>,
}

impl MerkleTree {
    /// Creates a new MerkleTree from a list of hashes.
    pub fn new_from_hashes(hashes: Vec<MerkleHash>) -> Result<MerkleTree, MerkleTreeError> {
        let mut tree = MerkleTree { levels: vec![] };
        MerkleTree::build_tree(&mut tree, hashes)?;
        Ok(tree)
    }

    /// Recursive function that builds the Merkle Tree from a list of hashes.
    fn build_tree(
        tree: &mut MerkleTree,
        mut hashes: Vec<MerkleHash>,
    ) -> Result<(), MerkleTreeError> {
        if hashes.len() == 1 {
            tree.levels.push(hashes);
            return Ok(());
        }

        if hashes.len() % 2 != 0 {
            let last = match hashes.last() {
                Some(last) => last.clone(),
                None => {
                    return Err(MerkleTreeError::FailedToBuild(
                        "No last element in hashes".to_string(),
                    ));
                }
            };
            hashes.push(last);
        }

        let len = hashes.len();
        tree.levels.push(hashes.clone());

        let mut next_hashes = vec![];
        for i in (0..len).step_by(2) {
            let left = &hashes[i];
            let right = &hashes[i + 1];

            next_hashes.push(MerkleTree::combine_hashes(left, right));
        }

        MerkleTree::build_tree(tree, next_hashes)
    }

    /// Concatenates two hashes and returns the hash of the concatenation.
    // fn combine_hashes(mut left: MerkleHash, right: &MerkleHash) -> MerkleHash {
    //     left = left + right;

    //     let mut hasher = Sha3_256::new();
    //     hasher.update(left);
    //     let result = hasher.finalize();

    //     MerkleTree::bytes_to_hex(&result)
    // }
    fn combine_hashes(left: &MerkleHash, right: &MerkleHash) -> MerkleHash {
        let mut combined = left.clone();
        combined.push_str(right);

        let mut hasher = Sha3_256::new();
        hasher.update(combined);
        let result = hasher.finalize();

        MerkleTree::bytes_to_hex(&result)
    }

    /// Returns the root of the Merkle Tree, which is the Merkle Root.
    fn root(&self) -> &MerkleHash {
        &self.levels[self.levels.len() - 1][0]
    }

    /// Verifies that a given hash is contained in the Merkle Tree, in O(log n) time, with n = number of leaf hashes.
    ///
    /// # Parameters
    /// - `leaf`: The hash to verify
    /// - `index`: The index of the hash in the bottom level of the tree
    pub fn verify_with_index(&self, leaf: &MerkleHash, index: u32) -> bool {
        if self.levels[0][index as usize] != *leaf {
            return false;
        }

        let proof = match self.proof_of_inclusion_with_index(leaf, index) {
            Ok(proof) => proof,
            Err(_) => return false,
        };

        let mut computed_root = leaf.clone();

        for (hash, direction) in proof.iter() {
            computed_root = match direction {
                Direction::Left => MerkleTree::combine_hashes(hash, &computed_root),
                Direction::Right => MerkleTree::combine_hashes(&computed_root, hash),
            };
        }

        &computed_root == self.root()
    }

    /// Verifies that a given hash is contained in the Merkle Tree, in O(n) time, with n = number of leaf hashes.
    ///
    /// # Parameters
    /// - `leaf`: The hash to verify
    pub fn verify(&self, leaf: &MerkleHash) -> bool {
        let hash_index = match self.levels[0].iter().position(|h| h == leaf) {
            Some(index) => index,
            None => return false,
        };

        self.verify_with_index(leaf, hash_index as u32)
    }

    /// Returns the hash of the given data
    ///
    /// # Parameters
    /// - `data`: An object that can be converted to a byte slice
    pub fn get_hash_of(data: &impl AsRef<[u8]>) -> MerkleHash {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        MerkleTree::bytes_to_hex(&result)
    }

    /// Returns a proof of inclusion for a given hash in the Merkle Tree. The proof generated conains the hashes of the siblings of the nodes in the path from the leaf to the root, and their directions. In O(log n) time, with n = number of leaf hashes..
    ///
    /// # Parameters
    /// - `leaf`: The hash to generate the proof for
    /// - `index`: The index of the hash in the bottom level of the tree
    ///
    /// # Returns
    /// A Result that, if the hash given is included in the tree, contains a `ProofOfInclusion` containing the proof of inclusion for the given hash. If the hash is not included in the tree, an error message is returned.
    pub fn proof_of_inclusion_with_index(
        &self,
        leaf: &MerkleHash,
        mut index: u32,
    ) -> Result<ProofOfInclusion, MerkleTreeError> {
        if self.levels[0][index as usize] != *leaf {
            return Err(MerkleTreeError::InvalidHash(
                "Hash is not part of the tree".to_string(),
            ));
        }

        let mut proof = vec![];

        for level in self.levels.iter() {
            if level.len() == 1 {
                break;
            }

            if index % 2 == 0 {
                if index + 1 < level.len() as u32 {
                    proof.push((level[(index + 1) as usize].clone(), Direction::Right));
                } else {
                    proof.push((level[index as usize].clone(), Direction::Right));
                }
            } else {
                proof.push((level[(index - 1) as usize].clone(), Direction::Left));
            }

            index /= 2;
        }

        Ok(ProofOfInclusion::new_from(leaf.clone(), proof))
    }

    /// Returns a proof of inclusion for a given hash in the Merkle Tree. The proof generated conains the hashes of the siblings of the nodes in the path from the leaf to the root, and their directions. In O(n) time, with n = number of leaf hashes.
    ///
    /// # Parameters
    /// - `leaf`: The hash to generate the proof for
    ///
    /// # Returns
    /// A Result that, if the hash given is included in the tree, contains a `ProofOfInclusion` containing the proof of inclusion for the given hash. If the hash is not included in the tree, an error message is returned.
    pub fn proof_of_inclusion(
        &self,
        leaf: &MerkleHash,
    ) -> Result<ProofOfInclusion, MerkleTreeError> {
        let hash_index = match self.levels[0].iter().position(|h| h == leaf) {
            Some(index) => index,
            None => {
                return Err(MerkleTreeError::InvalidHash(
                    "Hash is not part of the tree".to_string(),
                ))
            }
        };

        self.proof_of_inclusion_with_index(leaf, hash_index as u32)
    }

    /// Adds a hash to the Merkle Tree, updating the tree structure.
    ///
    /// # Parameters
    /// - `hash`: The hash to add to the tree
    pub fn add_hash(&mut self, hash: MerkleHash) -> Result<(), MerkleTreeError> {
        let len = self.levels[0].len();

        if self.verify(&hash) {
            return Err(MerkleTreeError::HashAlreadyExists(
                "Hash is already contained in the tree".to_string(),
            ));
        }

        if len >= 2 && self.levels[0][len - 1] == self.levels[0][len - 2] {
            self.levels[0][len - 1] = hash;
        } else {
            self.levels[0].push(hash);
        }

        let mut new_tree = MerkleTree { levels: vec![] };

        MerkleTree::build_tree(&mut new_tree, self.levels[0].clone())?;

        self.levels = new_tree.levels;
        Ok(())
    }
    
    /// Removes a hash from the Merkle Tree, updating the tree structure.
    pub fn remove_hash(&mut self, hash: &MerkleHash) {
        if let Some(index) = self.levels[0].iter().position(|h| h == hash) {
            self.levels[0].remove(index);
            self.build_tree();
        }
    }

    /// Converts a byte slice to a hexadecimal string.
    fn bytes_to_hex(bytes: &[u8]) -> String {
        let hex_chars: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
        hex_chars.join("")
    }

    /// Prints the Merkle Tree structure.
    pub fn print(&self) {
        for i in (0..self.levels.len()).rev() {
            println!("LEVEL {}:", self.levels.len() - i - 1);
            for hash in self.levels[i].iter() {
                println!("- {:?}", hash);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use sha3::{Digest, Sha3_256};

    use crate::merkle_tree::MerkleTree;

    #[test]
    fn build_simple_tree() {
        let data = vec![[1; 32], [2; 32], [3; 32], [4; 32]];

        let tree = MerkleTree::new_from_hashes(data).unwrap();

        println!("LEVEL 1: {:?}", tree.levels[0]);
        println!("LEVEL 2: {:?}", tree.levels[1]);
        println!("LEVEL 3: {:?}", tree.levels[2]);

        let mut hasher = Sha3_256::new();
        hasher.update([1; 32]);
        let result = hasher.finalize();

        let hash: [u8; 32] = result.into();
        println!("HASH 1: {:?}", hash);

        assert_eq!(tree.levels.len(), 3);
        assert_eq!(tree.levels[0].len(), 4);
        assert_eq!(tree.levels[1].len(), 2);
        assert_eq!(tree.levels[2].len(), 1);
    }

    #[test]
    fn build_simple_tree_from_strings() {
        let data = vec!["something00", "something01", "something02", "something03"];

        let tree = MerkleTree::new_from_hashes(data).unwrap();

        tree.print();

        assert_eq!(tree.levels.len(), 3);
        assert_eq!(tree.levels[0].len(), 4);
        assert_eq!(tree.levels[1].len(), 2);
        assert_eq!(tree.levels[2].len(), 1);
    }

    #[test]
    fn verify_inclusion_in_simple_tree_from_strings() {
        let data = vec![
            "something00",
            "something01",
            "something02",
            "something03",
            "something04",
        ];

        let tree = MerkleTree::new_from_hashes(data).unwrap();

        let hash = MerkleTree::get_hash_of(&"something04");
        println!("HASH: {:?}", hash);

        assert!(tree.verify_with_index(&hash, 4));
        tree.print()
    }

    #[test]
    fn verify_inclusion_in_big_tree_from_strings() {
        let data = vec![
            "something00",
            "something01",
            "something02",
            "something03",
            "something04",
            "something05",
            "something06",
            "something07",
            "something08",
            "something09",
            "something010",
            "something011",
            "something012",
            "something013",
            "something014",
            "something015",
            "something016",
            "something017",
            "something018",
            "something019",
            "something020",
            "something021",
            "something022",
            "something023",
            "something024",
            "something025",
            "something026",
            "something027",
            "something028",
            "something029",
            "something030",
            "something031",
        ];

        let tree = MerkleTree::new_from_hashes(data).unwrap();

        let hash = MerkleTree::get_hash_of(&"something017");

        assert!(tree.verify_with_index(&hash, 17));

        tree.print()
    }

    #[test]
    fn proof_of_inclusion_in_big_tree_from_strings() {
        let data = vec![
            "something00",
            "something01",
            "something02",
            "something03",
            "something04",
            "something05",
            "something06",
            "something07",
            "something08",
            "something09",
            "something010",
            "something011",
            "something012",
            "something013",
            "something014",
            "something015",
            "something016",
            "something017",
            "something018",
            "something019",
            "something020",
            "something021",
            "something022",
            "something023",
            "something024",
            "something025",
            "something026",
            "something027",
            "something028",
            "something029",
            "something030",
            "something031",
        ];

        let tree = MerkleTree::new_from_hashes(data).unwrap();

        let hash = MerkleTree::get_hash_of(&"something017");

        let proof = tree.proof_of_inclusion(&hash).unwrap();

        proof.print();
    }

    #[test]
    #[should_panic]
    fn proof_of_inclusion_fails() {
        let data = vec![
            "something00",
            "something01",
            "something02",
            "something03",
            "something04",
            "something05",
            "something06",
            "something07",
            "something08",
            "something09",
            "something010",
            "something011",
            "something012",
            "something013",
            "something014",
            "something015",
            "something016",
            "something017",
            "something018",
            "something019",
            "something020",
            "something021",
            "something022",
            "something023",
            "something024",
            "something025",
            "something026",
            "something027",
            "something028",
            "something029",
            "something030",
            "something031",
        ];

        let tree = MerkleTree::new_from_hashes(data).unwrap();

        let hash = MerkleTree::get_hash_of(&"not in the tree");

        let _proof = tree.proof_of_inclusion(&hash).unwrap();
    }

    #[test]
    fn add_to_tree() {
        let data = vec![
            "something00",
            "something01",
            "something02",
            "something03",
            "something04",
            "something05",
            "something06",
            "something07",
            "something08",
            "something09",
            "something010",
            "something011",
            "something012",
            "something013",
            "something014",
            "something015",
            "something016",
        ];

        let mut tree = MerkleTree::new_from_hashes(data).unwrap();
        println!("TREE BEFORE ADDING:");
        tree.print();

        let new_data = MerkleTree::get_hash_of(&"something099");
        let _ = tree.add_hash(new_data.clone());

        assert!(tree.verify(&new_data));

        let proof = tree.proof_of_inclusion(&new_data).unwrap();
        println!("PROOF OF ADDED:");
        proof.print();

        println!("TREE AFTER ADDING:");
        tree.print()
    }
}
