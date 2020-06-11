use lyon::math::{rect, Point};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::simple_builder;
use lyon::tessellation::{FillOptions, VertexBuffers};

pub fn examples_shapes() -> VertexBuffers<Point, u16> {
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();

    let options = FillOptions::tolerance(0.1);

    fill_rounded_rectangle(
        &rect(0.0, 0.0, 0.9, 0.9),
        &BorderRadii {
            top_left: 0.05,
            top_right: 0.05,
            bottom_left: 0.05,
            bottom_right: 0.05,
        },
        &options,
        &mut simple_builder(&mut geometry),
    )
    .expect("Should never fail");

    geometry
}

pub fn point_to_vertex(point: &Point) -> VectorVertex {
    VectorVertex {
        position: [point.x, point.y, 0.],
        color: [0.9, 0.1, 0.1, 1.],
    }
}

type VertexPosition = [f32; 3];
type VertexColor = [f32; 4];

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VectorVertex {
    pub position: VertexPosition,
    pub color: VertexColor,
}

impl VectorVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<VectorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<VertexPosition>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

unsafe impl bytemuck::Pod for VectorVertex {}
unsafe impl bytemuck::Zeroable for VectorVertex {}
