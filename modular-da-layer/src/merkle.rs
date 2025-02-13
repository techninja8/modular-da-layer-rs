use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]

struct MerkleNode {
    pub hash: Vec<u8>,
}

struct MerkleTree {
    pub root: MerkleNode,
    pub leaf_nodes: Vec<MerkleNode>,
}

impl MerkleTree {
    pub fn new(hashes: Vec<Vec<u8>>) -> Self {
        let leaf_nodes: Vec<MerkleNode> = hashes
            .into_iter()
            .map(|hash| MerkleNode { hash })
            .collect();
        let root = Self::build_tree(&leaf_nodes);

        MerkleTree {
            root, leaf_nodes
        }
    }

    fn build_tree(leaf_nodes: &[MerkleNode]) -> MerkleNode {
        let mut nodes = leaf_nodes.to_vec();

        while nodes.len() > 1 {
            let mut parent_nodes = Vec::new();

            for chunk in nodes.chunks(2) {
                let parent_hash = if chunk.len() == 2 {
                    Self::hash_nodes(&chunk[0].hash, &chunk[1].hash)
                } else {
                    chunk[0].hash.clone()
                };

                parent_nodes.push(MerkleNode {
                    hash: parent_hash
                });
            }

            nodes = parent_nodes;
        }
        nodes.pop().unwrap()
    }

    fn hash_nodes(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Vec<Vec<u8>> {
        let mut proof  = Vec::new();
        let mut index = leaf_index;
        let mut level_nodes = self.leaf_nodes.clone();

        while level_nodes.len() > 1 {
            let mut parent_nodes = Vec::new();
            for chunk in level_nodes.chunks(2) {
                if chunk.len() == 2 {
                    let parent_hash = Self::hash_nodes(&chunk[0].hash, &chunk[1].hash);
                    parent_nodes.push(MerkleNode { hash: parent_hash });

                    if index % 2 == 0 {
                        proof.push(chunk[1].hash.clone());
                    } else {
                        proof.push(chunk[0].hash.clone());
                    }
                } else {
                    parent_nodes.push(chunk[0].clone());
                }
            }

            index /= 2;
            level_nodes = parent_nodes;
        }
        proof
    }

    pub fn verify(root: &[u8], proof: &[Vec<u8>], target_hash: &[u8]) -> bool {
        let mut computed_hash = target_hash.to_vec();

        for siblings in proof {
            computed_hash = if computed_hash < *siblings {
                Self::hash_nodes(&computed_hash, siblings)
            } else {
                Self::hash_nodes(siblings, &computed_hash)
            };
        }

        computed_hash == root
    }
}
