use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: MerkleNode,
    pub leaf_nodes: Vec<MerkleNode>,
}

impl MerkleTree {
    // Constructing a Merkle tree
    pub fn new(hashes: Vec<Vec<u8>>) -> Self {
        if hashes.is_empty() {
            panic!("Merkle Tree Cannot Be Empty!");
        }

        let leaf_nodes: Vec<MerkleNode> = hashes.into_iter()
            .map(|hash| MerkleNode { hash })
            .collect();

        let mut nodes = leaf_nodes.clone();

        while nodes.len() > 1 {
            let mut parent_nodes = Vec::new();

            for chunk in nodes.chunks(2) {
                let parent_hash = if chunk.len() == 2 {
                    Self::hash_nodes(&chunk[0].hash, &chunk[1].hash)
                } else {
                    chunk[0].hash.clone() // Carry forward if odd
                };

                parent_nodes.push(MerkleNode { hash: parent_hash });
            }

            nodes = parent_nodes;
        }

        let root = nodes.pop().unwrap();
        MerkleTree { root, leaf_nodes }
    }

    fn hash_nodes(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Vec<Vec<u8>> {
        if leaf_index >= self.leaf_nodes.len() {
            panic!("Invalid leaf index");
        }

        let mut proof = Vec::new();
        let mut index = leaf_index;
        let mut level_nodes: Vec<Vec<u8>> = self.leaf_nodes.iter().map(|n| n.hash.clone()).collect();

        while level_nodes.len() > 1 {
            let mut parent_nodes = Vec::new();

            for i in (0..level_nodes.len()).step_by(2) {
                let left = &level_nodes[i];
                let right = if i + 1 < level_nodes.len() { &level_nodes[i + 1] } else { left };

                let parent_hash = Self::hash_nodes(left, right);
                parent_nodes.push(parent_hash.clone());

                if i == index || i + 1 == index {
                    let sibling_hash = if i == index { right } else { left };
                    if left != right {
                        proof.push(sibling_hash.clone());
                    }
                    index /= 2;
                }
            }

            level_nodes = parent_nodes;
        }

        proof
    }

    pub fn verify(root: &[u8], proof: &[Vec<u8>], target_hash: &[u8], leaf_index: usize) -> bool {
        let mut computed_hash = target_hash.to_vec();
        let mut index = leaf_index;

        for sibling in proof {
            if index % 2 == 0 {
                computed_hash = Self::hash_nodes(&computed_hash, sibling);
            } else {
                computed_hash = Self::hash_nodes(sibling, &computed_hash);
            }
            index /= 2;
        }

        computed_hash == root
    }
}
