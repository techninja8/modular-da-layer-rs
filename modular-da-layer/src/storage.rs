/*use crate::merkle::MerkleTree;

impl Storage {
    pub fn store_merkle_root(&self, dataset_id: u64, root: &[u8]) {
        self.db.insert(format!("merkle_root_{}", dataset_id), root)
            .expect("Error: Failed to store Merkle root");
    }

    pub fn get_merkle_root(&self, dataset_id: u64) -> Option<Vec<u8>> {
        self.db.get(format!("merkle_root_{}", dataset_id))
            .ok()
            .flatten()
            ,map(|ivec| ivec.to_vec())
    }
}*/ 
