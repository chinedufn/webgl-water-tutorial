use bincode;
use blender_mesh::BlenderMesh;
use std::collections::HashMap;

#[derive(Default)]
pub struct Assets {
    meshes: HashMap<String, BlenderMesh>,
}

impl Assets {
    pub fn new() -> Assets {
        // FIXME: XmlHttpRequest request instead of including in binary
        // Deserializing on the client
        let meshes = include_bytes!("../../../meshes.bytes");
        let mut meshes: HashMap<String, BlenderMesh> = bincode::deserialize(meshes).unwrap();

        for (_mesh_name, mesh) in meshes.iter_mut() {
            mesh.combine_vertex_indices();
            mesh.triangulate();
            mesh.y_up();
        }

        Assets { meshes }
    }

    pub fn get_mesh(&self, mesh_name: &str) -> Option<&BlenderMesh> {
        self.meshes.get(mesh_name)
    }
}
