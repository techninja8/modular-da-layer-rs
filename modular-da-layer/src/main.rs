#![allow(unused_imports)]
#![allow(dead_code)]

mod merkle;
mod network;
mod shard;
mod storage;
mod verification;
mod crypto_utils;

use crypto_utils::*;
use merkle::*;
use merkle::MerkleTree;
use network::*;
use shard::*;
use storage::*;
use verification::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())

}

#[cfg(test)]
mod test{
    
    use super::*;

    #[test]
    fn test_merkle_proof_verification() {
        let sharded_hashes = vec![
            vec![1, 2, 3], 
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];

        let tree = merkle::MerkleTree::new(sharded_hashes.clone());
        let proof = tree.generate_proof(2);
        let root = tree.root.hash.clone();

        assert!(MerkleTree::verify(&root, &proof, &sharded_hashes[2], 2));
    }
}
