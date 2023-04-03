use std::thread;
use std::thread::JoinHandle;

pub fn map(chunked_data: impl Iterator<Item = String>) -> Vec<JoinHandle<u32>> {
}

pub fn reduce(children_vector: Vec<JoinHandle<u32>>) -> u32 {
}
