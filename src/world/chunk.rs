use crate::{Instance, InstanceRaw};
use cgmath::{Quaternion, Vector3, Zero};

#[derive(Clone, Copy)]
pub enum Block {
    Air,
    Grass,
}

pub struct Chunk<const W: usize, const H: usize> {
    pub blocks: Box<[[[Block; W]; W]; H]>,
}

impl<const W: usize, const H: usize> Chunk<W, H> {
    pub fn instances(&self, bottom_left: Vector3<f32>) -> Vec<InstanceRaw> {
        (0..H)
            .flat_map(|y| {
                (0..W).flat_map(move |x| {
                    (0..W).filter_map(move |z| match self.blocks[y][z][x] {
                        Block::Air => None,
                        Block::Grass => {
                            let position = cgmath::Vector3 {
                                x: x as f32,
                                y: y as f32,
                                z: z as f32,
                            } * 2.
                                + bottom_left;
                            let rotation = Quaternion::zero();
                            let instance = Instance { position, rotation };

                            Some(instance.to_raw())
                        }
                    })
                })
            })
            .collect()
    }
}

impl<const W: usize, const H: usize> From<Block> for Chunk<W, H> {
    fn from(block: Block) -> Self {
        Self {
            blocks: Box::new([[[block; W]; W]; H]),
        }
    }
}
