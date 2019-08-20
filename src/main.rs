#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Location {
    pub fn new() -> Self {
        Location { x: 0, y: 0, z: 0 }
    }

}

struct Chunk {
    data: Box<[u8]>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: Box::new([0_u8; 64]),
        }
    }


    pub fn copy_some_data(from: &Chunk, to: &mut Chunk) {
        for i in 0..16 {
            to.data[i] = from.data[i];
        }
    }

}

struct Volume {
    chunks: HashMap<Location, Chunk>,
}

impl Volume {
    pub fn new() -> Self {
        Volume {
            chunks: HashMap::new(),
        }
    }


    // .. Methods to add / remove data to chunks

    // Then this
    pub fn maintain(&mut self) {
        let chunk_locs: Vec<_> = self.chunks.keys().cloned().collect();
        for (_, group) in &(&chunk_locs).iter().group_by(|loc| (loc.x, loc.y)) {
            let group: Vec<_> = group.collect();
            {
                let lte: Vec<_> = group
                    .clone()
                    .into_iter()
                    .filter(|loc| loc.z <= 0)
                    .sorted_by(|l, r| l.z.abs().cmp(&r.z.abs()))
                    .collect();
                for i in 1..lte.len() {
                    // Remove both from the hashmap.
                    let from_chunk = self.chunks.remove(lte[i - 1]).unwrap();
                    let mut to_chunk = self.chunks.remove(lte[i]).unwrap();

                    // Copy the voxels
                    Chunk::copy_some_data(&from_chunk, &mut to_chunk);

                    // Re-add them to the HashMap
                    self.chunks.insert(*lte[i - 1], from_chunk);
                    self.chunks.insert(*lte[i], to_chunk);
                }
            }
            {
                let gte: Vec<_> = group
                    .clone()
                    .into_iter()
                    .filter(|loc| loc.z >= 0)
                    .sorted_by(|l, r| l.z.abs().cmp(&r.z.abs()))
                    .collect();
                for i in 1..gte.len() {
                    // Remove both from the hashmap.
                    let from_chunk = self.chunks.remove(&gte[i - 1]).unwrap();
                    let mut to_chunk = self.chunks.remove(&gte[i]).unwrap();

                    // Copy the voxels
                    Chunk::copy_some_data(&from_chunk, &mut to_chunk);

                    // Re-add them to the HashMap
                    self.chunks.insert(*gte[i - 1], from_chunk);
                    self.chunks.insert(*gte[i], to_chunk);
                }
            }
        }
    }
}
