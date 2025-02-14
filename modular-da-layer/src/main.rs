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
use network::*;
use shard::*;
use storage::*;
use verification::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        b"data1".to_vec(),
        b"data2".to_vec(),
        b"data3".to_vec(),
        b"data4".to_vec(),
    ];

    let index = 3;

    // let proof = compute_merkle_proof(&tree, index);

    let merkle_product = compute_merkle_product(data, index);

    let sigma = SigmaProtocol::new(&merkle_product);

    let (t, r) = sigma.commit();

    let e = SigmaProtocol::challenge();

    let z = sigma.response(&r, &e);
    let is_valid = sigma.verify(&t, &e, &z);

    if is_valid {
        println!("Verification Successful!");
    } else {
        println!("Verification Failed");
    }

    Ok(())

}

/*#[cfg(test)]
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
        let root  = tree.root.hash.clone();

        assert!(verify(&root, &proof, &sharded_hashes[3]));
    }
}*/
