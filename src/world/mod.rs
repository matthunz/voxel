mod chunk;
use std::{collections::HashMap, ops::{Index, Range}};

use cgmath::{InnerSpace, Vector2, Vector3, Zero};
pub use chunk::*;
use wgpu::{util::DeviceExt, Buffer, Device, RenderPass};

use crate::InstanceRaw;

#[derive(Default)]
pub struct World {
    chunks: HashMap<Vector2<usize>, (Chunk<16, 256>, Buffer, Vec<InstanceRaw>)>,
}

impl World {
    pub fn insert(&mut self, bottom_left: Vector2<usize>, chunk: Chunk<16, 256>, device: &Device) {
        let instances = chunk.instances(bottom_left.map(|i| i as f32));
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });
        self.chunks.insert(bottom_left, (chunk, buffer, instances));
    }

    pub fn render<'a: 'b, 'b>(&'a self, render_pass: &mut RenderPass<'b>, indices: Range<u32>) {
        for (chunk, instance_buffer, instances) in self.chunks.values(){
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.draw_indexed(indices.clone(), 0, 0..instances.len() as u32);
        }
    }

    pub fn chunk(&mut self, block: Vector2<usize>) -> Option<&mut Chunk<16, 256>> {
        let idx = block % 16;
        self.chunks.get_mut(&idx).map(|(chunk, _, _)| chunk)
    }

    pub fn block_mut(&mut self, block: Vector3<usize>) -> Option<&mut Block> {
        self.chunk(Vector2 {
            x: block.x,
            y: block.y,
        })
        .map(|chunk| &mut chunk.blocks[block.y][block.x][block.z])
    }
}
