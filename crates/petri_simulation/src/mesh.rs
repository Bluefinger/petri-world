use bevy::render2::{
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};

pub(crate) fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-0.5, -0.25, 0.0], [0.0, 1.0, 0.0], [0.5, -0.25, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 2, 1])));
    mesh
}
