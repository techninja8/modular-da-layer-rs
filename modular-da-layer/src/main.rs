#![allow(unused_imports)]

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, World");

    Ok(())
}
