use std::{path::Path, sync::Arc};

use obj::{
    Obj, 
    ObjError, 
    MtlLibsLoadError
};

use crate::types::{
    vec3::Vec3, 
    vec2::{
        UV, 
        Vec2
    }, 
    ray::Ray, 
    aabb::AABB, 
    materials::Material, texture::SolidColor
};

use super::{
    hittable::{
        Hit, 
        HitRecord
    }, 
    tri::Triangle
};

/// Enum representing whether a triangle should be shaded flat or 
/// smooth. If smooth, need each vertex normal separately so they can be interpolated
pub enum VertexNormals {
    Flat(Vec3),
    Smooth([Vec3; 4])
}

/// Simple 3D mesh, containing vertex positions, vertex normals,
/// triangles (along with corresponding vertex normals), and texture coordinates
/// Note: use with Blender's .obj exportter requires that mesh be triangulated,
/// and that the vertex data be included also
pub struct MeshTri {
    pub vertex_indices: [usize; 3],
    pub vertex_normals: VertexNormals,
    pub texture_coordinates: [UV; 3],
    pub material_index: usize
}

pub struct Mesh {
    pub triangles: Vec<MeshTri>,
    pub vertices: Vec<Vec3>,
    pub bounding_box: AABB,
    pub materials: Vec<Material>
}

#[derive(Debug)]
pub enum MeshError {
    MeshLoadError(ObjError),
    MaterialLoadError(MtlLibsLoadError),
    UntriangulatedError,
}

impl From<ObjError> for MeshError {
    fn from(x: ObjError) -> Self {
        MeshError::MeshLoadError(x)
    }
}

impl From<MtlLibsLoadError> for MeshError {
    fn from(x: MtlLibsLoadError) -> Self {
        MeshError::MaterialLoadError(x)
    }
}

impl Mesh {
    pub fn from_file(filepath: &Path) -> Result<Mesh, MeshError> {
        let mut obj = Obj::load(filepath)?;
        obj.load_mtls()?;

        let obj_data = &obj.data;

        if obj_data.objects.len() > 1 {
            eprintln!("warning: more than 1 model in .obj file");
        }

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uv_coords = Vec::new();

        let mut x_min = f64::INFINITY;
        let mut y_min = f64::INFINITY;
        let mut z_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        let mut z_max = f64::NEG_INFINITY;
        for chunk in obj_data.position.iter() {
            let x = chunk[0] as f64;
            let y = chunk[1] as f64;
            let z = chunk[2] as f64;
            x_max = f64::max(x, x_max);
            y_max = f64::max(y, y_max);
            z_max = f64::max(z, z_max);
            x_min = f64::min(x, x_min);
            y_min = f64::min(y, y_min);
            z_min = f64::min(z, z_min);
            vertices.push(Vec3(x, y, z));
        }

        let bounding_box = AABB::new(
            Vec3(x_min, y_min, z_min),
            Vec3(x_max, y_max, z_max)
        );

        for chunk in obj_data.normal.iter() {
            normals.push(Vec3(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
        }
        
        for chunk in obj_data.texture.iter() {
            uv_coords.push(UV(Vec2(chunk[0] as f64, chunk[1] as f64)));
        }

        let mut tris = Vec::new();
        let object = &obj_data.objects[0];
        let mut materials = Vec::new();

        for (i, group) in object.groups.iter().enumerate() {
            let material = match &group.material {
                None => {
                    Material::Lambertian { 
                        albedo: Arc::new(SolidColor::from(Vec3(1.0, 0.0, 0.75)))
                    }
                }
                Some(_) => {
                    Material::Lambertian { 
                        albedo: Arc::new(SolidColor::from(Vec3(1.0, 0.0, 0.75))) 
                    }
                },
            };
            materials.push(material);
            for poly in group.polys.iter() {
                let index_tup_vec = &poly.0;
                if index_tup_vec.len() != 3 {
                    return Err(MeshError::UntriangulatedError);
                }

                let v0 = index_tup_vec[0];
                let v1 = index_tup_vec[1];
                let v2 = index_tup_vec[2];

                let n0 = normals[v0.2.unwrap()];
                let n1 = normals[v1.2.unwrap()];
                let n2 = normals[v2.2.unwrap()];

                let vertex_normals = if n0 == n1 && n1 == n2 {
                    VertexNormals::Flat(n0)
                } 
                else {
                    let v0 = vertices[v0.0];
                    let v1 = vertices[v1.0];
                    let v2 = vertices[v2.0];
                    let face_normal = Vec3::normalized(Vec3::cross(v1 - v0, v2 - v0));
                    VertexNormals::Smooth([n0, n1, n2, face_normal]) 
                };

                let t0 = uv_coords[v0.1.unwrap()];
                let t1 = uv_coords[v1.1.unwrap()];
                let t2 = uv_coords[v2.1.unwrap()];

                let triangle = MeshTri {
                    vertex_indices: [v0.0, v1.0, v2.0],
                    vertex_normals,
                    texture_coordinates: [t0, t1, t2],
                    material_index: i,
                };

                tris.push(triangle);
            }
        }
        Ok(Mesh {
            vertices,
            triangles: tris,
            bounding_box,
            materials
        })
    }
}

impl Hit for Mesh {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Meshes are quite expensive, so check if ray hits bounding box first
        let bounding_box = self.bounding_box(t_min, t_max).unwrap();
        if bounding_box.hit(r, t_min, t_max) {
            let mut t_max = t_max;
            let mut closest_so_far = None;
            for tri in &self.triangles {
                let v0 = self.vertices[tri.vertex_indices[0]];
                let v1 = self.vertices[tri.vertex_indices[1]];
                let v2 = self.vertices[tri.vertex_indices[2]];

                if let Some((t, u, v)) = Triangle::moller_trumbore(v0, v1, v2, r, t_min, t_max) {
                    if t < t_max {
                        t_max = t;

                        let front_face;
                        let normal;
                        match tri.vertex_normals {
                            VertexNormals::Flat(face_normal) => {
                                front_face = Vec3::dot(face_normal, r.direction) < 0.0;
                                normal = face_normal;
                            },
                            VertexNormals::Smooth(vertex_normals) => {
                                #[cfg(feature="ray_debug")]
                                {
                                    println!("{:?}", vertex_normals);
                                }
                                let w = 1.0 - u - v;
                                front_face = Vec3::dot(vertex_normals[3], r.direction) < 0.0;
                                normal = Vec3::normalized(w * vertex_normals[0] + u * vertex_normals[1] + v * vertex_normals[2]);
                            },
                        };

                        closest_so_far = Some(
                            HitRecord::construct_from_interpolated_normal(
                                r.at(t), 
                                normal,
                                front_face,
                                t, 
                                r, 
                                &self.materials[tri.material_index], 
                                u, 
                                v
                            )
                        )
                    }
                }
            }
            closest_so_far
        } else { None }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}