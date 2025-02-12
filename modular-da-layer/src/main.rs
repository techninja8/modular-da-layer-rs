#![allow(dead_code)]

use sled;
use std::hash::{Hash, Hasher, DefaultHasher};

const SHARD_SIZE: usize = 4;

struct ShardDB {
    shards: Vec<sled::Db>,
}

impl ShardDB {
    fn new() -> Self {
        let mut shards = Vec::new();
        for i in 0..SHARD_SIZE {
            let db = sled::open(format!("Shard_{}", i)).expect("Failed to create shard!");
            shards.push(db)
        }

        Self { shards }
    }

    fn get_shard_index(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % SHARD_SIZE
    }

    fn insert_into_shard(&self, key: &str, value: &str) -> sled::Result<()> {
        let shard_idx = self.get_shard_index(key);
        let _ = self.shards[shard_idx].insert(key, value.as_bytes());
        println!("Placed {} in Shard_{}", key, shard_idx);
        Ok(())
    }

    fn retrieve_from_shard(&self, key: &str) -> sled::Result<Option<Vec<u8>>> {
        let shard_idx = self.get_shard_index(key);
        self.shards[shard_idx].get(key).map(|vec| {
            vec.map(|ivec| {
                ivec.iter().map(|&bytes| bytes as u8).collect()
            })
        })
    }

}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_shard = ShardDB::new(); // returns a variable number of shards

    let _ = new_shard.insert_into_shard("Key2", "Value2").unwrap();

    if let Some(data) = new_shard.retrieve_from_shard("Key2").unwrap() {
        println!("{}", String::from_utf8(data).unwrap());
    }

    Ok(())
    
}
