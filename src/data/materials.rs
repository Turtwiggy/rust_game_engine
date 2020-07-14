use cgmath::{Matrix4, Vector3, vec3,  Deg, Rad, perspective};

pub enum AvailableMaterials {
    Orange,
    TakeDefault
}

struct FGMaterial {
    ambient : Vector3<f32>,
    diffuse : Vector3<f32>,
    specular : Vector3<f32>,
}

impl FGMaterial {

    pub fn create_material(material : AvailableMaterials) -> FGMaterial {

        match material {
            AvailableMaterials::Orange => {
                return FGMaterial{
                    ambient: vec3(1.0, 0.5, 0.31),
                    diffuse: vec3(1.0, 0.5, 0.31),
                    specular: vec3(0.5, 0.5, 0.5)
                }
            }
            AvailableMaterials::TakeDefault => {
                return FGMaterial{
                    ambient: vec3(1.0, 0.5, 0.31),
                    diffuse: vec3(1.0, 0.5, 0.31),
                    specular: vec3(0.5, 0.5, 0.5)
                }
            }
        }
    }
}