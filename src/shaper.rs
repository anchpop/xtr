use lyon::math::{rect, Point};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::simple_builder;
use lyon::tessellation::{FillOptions, VertexBuffers};

pub fn examples_shapes() -> VertexBuffers<Point, u16> {
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();

    let options = FillOptions::tolerance(0.1);

    fill_rounded_rectangle(
        &rect(0.0, 0.0, 100.0, 50.0),
        &BorderRadii {
            top_left: 10.0,
            top_right: 5.0,
            bottom_left: 20.0,
            bottom_right: 25.0,
        },
        &options,
        &mut simple_builder(&mut geometry),
    )
    .expect("Should never fail");

    geometry
}
