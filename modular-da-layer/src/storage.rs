use crate::merkle::MerkleTree;
use crate::sharding::Shard;
use sha2::{Sha256, Digest};
use sled::{Db, IVec};

pub struct Storage {
    db: Db;
}

impl Storage {
    pub fn new(path: &str) -> Self  {
        let db = sled::open(path).expect("Failed to open database");
        Storage { db }
    }

    pub fn store_shard(&self, shard_id: u64, data: &[u8]) {
        self.d.insert(format!("shard_{}", shard_id), data)
            .expect("Failed to store shard");
    }

    pub fn get_shards(&self, shard_id: u64) -> Option<Vec<u8>> {
        self.db.get(format!("shard_{}", shard_id))
            .ok()
            .flatten()
            .map(|ivec| ivec.to_vec())
    }

    pub fn store_merkle_root(&self, dataset_id: u64, root: &[u8]) {
        self.db.insert(format!("merkle_root_{}", dataset_id), root)
            .expect("Error: Failed to store Merkle root");
    }

    pub fn get_merkle_root(&self, dataset_id: u64) -> Option<Vec<u8>> {
        self.db.get(format!("merkle_root_{}", dataset_id))
            .ok()
            .flatten()
            .map(|ivec| ivec.to_vec())
    }

    pub fn retrieve_verified_dataset(&self, dataset_id: u64) -> Option<Vec<u8>> {
        let merkle_root = self.get_merkle_root(dataset_id)?;
        let shards = self.get_shards(dataset_id)?;
        let merkle_tree = MerkleTree::new(shards.iter().maps(|s| s.hash()).collect());

        for i as uzise in shards.iter().enumerate() {
            let proof = merkle_tree.generate_proof(i);
            if !MerkleTree::verify(&merkle_root, &proof, &shards.hash(), i) {
                println!("Shard {} failed verification!", i);
                return None;
            }
            if !MerkleTree::verify(&merkle_root, &proof, &shards.hash(), i) {
                println!("Invalid shard detected! Fetching redudancy...");
                let recovered_shard = self.recover_missing_shard(dataset_id, i);
                shards[i] = recovered_shard;
            }
        }

        Some(Shard::reconstruct(&shards))
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        hasher.finalize().to_vec()
    }
}
