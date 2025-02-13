use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Shard {
    pub id: u64,
    pub data: Vec<u8>,
    //pub hash: Vec<u8>
}

impl Shard {
    pub fn split(data: &[u8], chunk_size: usize) -> Vec<Shard> {
        data.par_chunks(chunk_size)
            .enumerate()
            .map(|(id, chunk)| Shard {
                id: id as u64,
                data: chunk.to_vec(),
            })
        .collect()
    }

    pub fn reconstruct(shard: &[Shard]) -> Vec<u8> {
        let mut data = Vec::new();
        let mut sorted_shard = shard.to_vec();
        sorted_shard.sort_by_key(|shard| shard.id);

        // Iterating over shards to reconstruct them from their data
        for shard in sorted_shard {
            data.extend_from_slice(&shard.data);
        }

        data
    }
}
