#![allow(non_snake_case)]
// #![allow(dead_code)]

use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

use gl;
use gl::types::GLsizei;
use renderer_gl::*;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct FGVertex {
    #[location = "0"]
    pub pos: data::f32_f32_f32,
    #[location = "1"]
    pub nml: data::f32_f32_f32,
    #[location = "2"]
    pub tex: data::f32_f32,
    #[location = "3"]
    pub tangent : data::f32_f32_f32,
    #[location = "4"]
    pub bit_tangent : data::f32_f32_f32
    // #[location = "5"]
    // clr: data::u2_u10_u10_u10_rev_float,
}

impl Default for FGVertex {
    fn default() -> Self {
        FGVertex {
            pos: (0.0, 0.0, 0.0).into(),
            nml: (0.0, 0.0, 0.0).into(),
            tex: (0.0, 0.0).into(),
            tangent: (0.0, 0.0, 0.0).into(),
            bit_tangent: (0.0, 0.0, 0.0).into(),
        }
    }
}

#[derive(Clone)]
pub struct FGTexture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

pub struct FGMesh {
    /*  Mesh Data  */
    pub vertices: Vec<FGVertex>,
    pub indices: Vec<u32>,
    //pub textures: Vec<FGTexture>,

    pub VAO: buffer::VertexArray,
    /*  Render data  */
    VBO: buffer::ArrayBuffer,
    EBO: buffer::ElementArrayBuffer,
}

impl FGMesh {
    pub fn new(
        gl: &gl::Gl,
        vertices: Vec<FGVertex>, 
        indices: Vec<u32>, 
        // textures: Vec<FGTexture>
        ) -> FGMesh {

        let vao = buffer::VertexArray::new(gl);
        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);

        vao.bind();

        vbo.bind();
        vbo.static_draw_data(&vertices);

        ebo.bind();
        ebo.static_draw_data(&indices);

        FGVertex::vertex_attrib_pointers(gl);

        let mesh = FGMesh {
            vertices: vertices, indices: indices,
            VAO: vao, VBO: vbo, EBO: ebo
        };

        mesh
    }

    /// render the mesh
    pub fn draw(&self, gl: &gl::Gl, shader: &shader::Program) {
        // bind appropriate textures
        // let mut diffuseNr  = 0;
        // let mut specularNr = 0;
        // let mut normalNr   = 0;
        // let mut heightNr   = 0;
        // for (i, texture) in self.textures.iter().enumerate() {

        //     unsafe{
        //         gl.ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
        //     }
        //     // retrieve texture number (the N in diffuse_textureN)
        //     let name = &texture.type_;
        //     let number = match name.as_str() {
        //         "texture_diffuse" => {
        //             diffuseNr += 1;
        //             diffuseNr
        //         },
        //         "texture_specular" => {
        //             specularNr += 1;
        //             specularNr
        //         }
        //         "texture_normal" => {
        //             normalNr += 1;
        //             normalNr
        //         }
        //         "texture_height" => {
        //             heightNr += 1;
        //             heightNr
        //         }
        //         _ => panic!("unknown texture type")
        //     };
        //     // now set the sampler to the correct texture unit
        //     let sampler = CString::new(format!("{}{}", name, number)).unwrap();
        //     shader.set_float(&sampler, i as f32);

        //     unsafe{
        //         // and finally bind the texture
        //         gl.BindTexture(gl::TEXTURE_2D, texture.id);
        //     }
        // }

        // draw mesh
        shader.set_used();
        self.VAO.bind();

        unsafe {
            gl.DrawElements(gl::TRIANGLES, self.indices.len() as GLsizei, gl::UNSIGNED_INT, ptr::null());

            self.VAO.unbind();
            // always good practice to set everything back to defaults once configured.
            gl.ActiveTexture(gl::TEXTURE0);
        }
    }

    pub fn create_plane(gl : &gl::Gl) -> (buffer::VertexArray, buffer::ArrayBuffer) {

        let vao = buffer::VertexArray::new(gl);
        let vbo = buffer::ArrayBuffer::new(gl);

        let mut plane_vertices : Vec<FGVertex> = Vec::new();
        plane_vertices.push( FGVertex {
            pos: (5.0, 0.0, 5.0).into(),
            tex: (2.0, 0.0).into(),
            ..FGVertex::default()
        });
        plane_vertices.push( FGVertex {
            pos: (-5.0, 0.0, 5.0).into(),
            tex: (0.0, 0.0).into(),
            ..FGVertex::default()
        });
        plane_vertices.push( FGVertex {
            pos: (-5.0, 0.0, -5.0).into(),
            tex: (0.0, 2.0).into(),
            ..FGVertex::default()
        });
        plane_vertices.push( FGVertex {
            pos: (5.0, 0.0, 5.0).into(),
            tex: (2.0, 0.0).into(),
            ..FGVertex::default()
        });
        plane_vertices.push( FGVertex {
            pos: (-5.0, 0.0, -5.0).into(),
            tex: (0.0, 2.0).into(),
            ..FGVertex::default()
        });
        plane_vertices.push( FGVertex {
            pos: (5.0, 0.0, -5.0).into(),
            tex: (2.0, 2.0).into(),
            ..FGVertex::default()
        });

        vao.bind();

        vbo.bind();
        vbo.static_draw_data(&plane_vertices);

        FGVertex::vertex_attrib_pointers(gl);

        (vao, vbo)
    }

    pub fn create_transparent_mesh(gl : &gl::Gl)  -> buffer::VertexArray {

        let vao = buffer::VertexArray::new(gl);
        let vbo = buffer::ArrayBuffer::new(gl);

        let mut transparent_verts : Vec<FGVertex> = Vec::new();
        transparent_verts.push( FGVertex {
            pos: (0.0, 5.0, 0.0).into(),
            tex: (0.0, 0.0).into(),
            ..FGVertex::default()
        });
        transparent_verts.push( FGVertex {
            pos: (0.0, -0.5, 0.0).into(),
            tex: (0.0, 1.0).into(),
            ..FGVertex::default()
        });
        transparent_verts.push( FGVertex {
            pos: (1.0, -0.5, 0.0).into(),
            tex: (1.0, 1.0).into(),
            ..FGVertex::default()
        });
        transparent_verts.push( FGVertex {
            pos: (0.0, 0.5, 0.0).into(),
            tex: (0.0, 0.0).into(),
            ..FGVertex::default()
        });
        transparent_verts.push( FGVertex {
            pos: (1.0, -0.5, 0.0).into(),
            tex: (1.0, 1.0).into(),
            ..FGVertex::default()
        });
        transparent_verts.push( FGVertex {
            pos: (1.0, -0.5, 0.0).into(),
            tex: (1.0, 0.0).into(),
            ..FGVertex::default()
        });
        
        vao.bind();

        vbo.bind();
        vbo.static_draw_data(&transparent_verts);

        FGVertex::vertex_attrib_pointers(gl);

        vao
    }

    pub fn create_skybox_vertices(gl : &gl::Gl) -> (buffer::VertexArray, buffer::ArrayBuffer) {
        let vao = buffer::VertexArray::new(gl);
        let vbo = buffer::ArrayBuffer::new(gl);

        let mut verts : Vec<FGVertex> = Vec::new();
        verts.push(FGVertex{pos:(-1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0, -1.0).into(),..FGVertex::default()});

        verts.push(FGVertex{pos:(-1.0, -1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0, -1.0,  1.0).into(),..FGVertex::default()});

        verts.push(FGVertex{pos:( 1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0, -1.0).into(),..FGVertex::default()});

        verts.push(FGVertex{pos:(-1.0, -1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0, -1.0,  1.0).into(),..FGVertex::default()});

        verts.push(FGVertex{pos:(-1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0,  1.0, -1.0).into(),..FGVertex::default()});

        verts.push(FGVertex{pos:(-1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0, -1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0, -1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:(-1.0, -1.0,  1.0).into(),..FGVertex::default()});
        verts.push(FGVertex{pos:( 1.0, -1.0,  1.0).into(),..FGVertex::default()});

        vao.bind();

        vbo.bind();
        vbo.static_draw_data(&verts);

        FGVertex::vertex_attrib_pointers(gl);

        (vao, vbo)
    }
}