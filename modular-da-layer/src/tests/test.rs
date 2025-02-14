#![allow(unused_imports)]
#![allow(dead_code)]

mod merkle;
mod network;
mod shard;
mod storage;
mod verification;

use merkle::*;
use network::*;
use shard::*;
use storage::*;
use verification::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn testing_sharding_and_reconstruction() {
        let original_data = b"This is a data that would be sharded and unsharded";
        let chunk_size = 10;

        let shards = Shard::split(original_data, chunk_size);
        assert!(shards.len() > 1, "The data should have splitted into more than 1 shards");

        let reconstructed_data = Shard::reconstruct(&shards);

        assert_eq!(original_data.as_ref(), reconstructed_data.as_slice());
    }
    fn test_merkle_proof_verification() {
        let sharded_hashes = vec![
            vec![1, 2, 3], 
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];

        let tree = merkle::MerkleTree::new(sharded_hashes.clone());
        let proof = tree.generate_proof(2);
        let root  = tree.root.hash.clone();

        assert!(verify(&root, &proof, &sharded_hashes[3]));
    }
}

