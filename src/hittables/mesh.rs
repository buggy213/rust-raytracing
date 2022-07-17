use std::path::Path;

use tobj::{
    LoadOptions, 
    LoadError
};

use crate::types::{
    vec3::Vec3, 
    vec2::{
        UV, 
        Vec2
    }
};

/// Enum representing whether a triangle should be shaded flat or 
/// smooth. If smooth, need each vertex normal separately so they can be interpolated
pub enum VertexNormals {
    Flat(Vec3),
    Smooth([Vec3; 3])
}

/// Simple 3D mesh, containing vertex positions, vertex normals,
/// triangles (along with corresponding vertex normals), and texture coordinates
/// Note: use with Blender's .obj exportter requires that mesh be triangulated,
/// and that the vertex data be included also
pub struct Tri {
    pub vertices: [Vec3; 3],
    pub vertex_normals: VertexNormals,
    pub texture_coordinates: [UV; 3],
}

pub struct Mesh {
    pub triangles: Vec<Tri>,
}

enum MeshError {
    MeshLoadError(LoadError),
    UntriangulatedError,
}

impl From<LoadError> for MeshError {
    fn from(x: LoadError) -> Self {
        MeshError::MeshLoadError(x)
    }
}

impl Mesh {
    pub fn from_file(filepath: &Path) -> Result<Mesh, MeshError> {
        let load_result = 
            tobj::load_obj(filepath, &LoadOptions::default())?;
        let (models, _) = load_result;
        if models.len() > 1 {
            eprintln!("warning: more than 1 model in .obj file");
        }

        let mesh = models[0].mesh;
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uv_coords = Vec::new();
        for chunk in mesh.positions.chunks(3) {
            vertices.push(Vec3(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
        }

        for chunk in mesh.normals.chunks(3) {
            normals.push(Vec3(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
        }
        
        for chunk in mesh.texcoords.chunks(2) {
            uv_coords.push(UV(Vec2(chunk[0] as f64, chunk[1] as f64)));
        }

        for (i, num_verts) in mesh.face_arities.iter().enumerate() {
            if *num_verts != 3 {
                return Err(MeshError::UntriangulatedError);
            }
            normals[]

            let triangle = Tri {
                vertices: [
                    vertices[mesh.indices[i * 3] as usize],
                    vertices[mesh.indices[i * 3 + 1] as usize],
                    vertices[mesh.indices[i * 3 + 2] as usize]
                ],
                vertex_normals: [
                    
                ]
                texture_coordinates: [],
            };
        }
        Ok()
    }
}