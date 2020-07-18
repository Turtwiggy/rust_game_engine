extern crate gl;

use game::GameState;
use renderer_gl::shader_manager::*;
use renderer_gl::*;
use renderer_gl::Viewport;
use threed::camera::*;
use threed::mesh::{FGMesh, FGVertex};
use threed::model::FGModel;
use util::resources::Resources;

use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Rad, Vector3};
use std::ffi::{CStr, CString};

pub fn create_default(gl: &gl::Gl, res : &Resources, w : i32, h : i32) -> Renderer {

    let shader_manager = shader_manager::create_default(gl, res);
    let viewport = viewport::create_default(w, h);

    // Configure OpenGL
    // ----------------
    unsafe {
        gl.Enable(gl::DEPTH_TEST);

        //cull backward triangles
        gl.Enable(gl::CULL_FACE);
        gl.DepthFunc(gl::LESS);

        //stencil buffer
        gl.Enable(gl::STENCIL_TEST);
        gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
        gl.StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);

        //msaa
        gl.Enable(gl::MULTISAMPLE);
    }

    return Renderer {
        shader_manager, viewport
    };
}

pub struct Renderer {
    shader_manager: ShaderManager,
    pub viewport : Viewport,
}

impl Renderer {
    pub fn render(
        &self,
        gl: &gl::Gl,
        window_size: (u32, u32),
        camera: &Camera,
        game_state: &GameState,

        //models and textures below
        cube_model: &FGModel,
        sponza_model: &FGModel,
        plane_vao: &buffer::VertexArray,
        //cubemap_texture: &u32,
        //cubemap_vao: &buffer::VertexArray,
    ) {
        unsafe {
            // render window contents here
            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl.StencilMask(0x00);
        }

        //View Projection
        let mut view = camera.GetViewMatrix();
        let projection: Matrix4<f32> = perspective(
            Deg(camera.Zoom),
            window_size.0 as f32 / window_size.1 as f32,
            0.1,
            100.0,
        );

        { // Do lit shader business
            let lit_shader = self
                .shader_manager
                .get_shader("shaders/lit_multiple_lights_no_tex")
                .unwrap();

            // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
            lit_shader.set_used();

            let view_pos = cgmath::Vector3 {
                x: camera.Position.x,
                y: camera.Position.y,
                z: camera.Position.z,
            };
            lit_shader.set_vector3(c_str!("viewPos"), &view_pos);
            lit_shader.set_mat4(c_str!("view"), &view);
            lit_shader.set_mat4(c_str!("projection"), &projection);

            // directional light
            lit_shader.set_vec3(c_str!("dirLight.direction"), -0.2, -1.0, -0.3);
            lit_shader.set_vec3(c_str!("dirLight.ambient"), 0.05, 0.05, 0.05);
            lit_shader.set_vec3(c_str!("dirLight.diffuse"), 0.4, 0.4, 0.4);
            lit_shader.set_vec3(c_str!("dirLight.specular"), 0.5, 0.5, 0.5);

            // 4 point lights
            for i in 0..game_state.light_objects.len() {
                let position = format!("pointLights[{}].position", i);
                lit_shader.set_vector3(
                    &CString::new(position).unwrap(),
                    &game_state.light_objects[i],
                );

                let diffuse_colour = &game_state.light_colours[i];
                let diffuse = format!("pointLights[{}].diffuse", i);
                lit_shader.set_vec3(
                    &CString::new(diffuse).unwrap(),
                    diffuse_colour.x,
                    diffuse_colour.y,
                    diffuse_colour.z,
                );
                let ambient_colour = diffuse_colour * 0.2;
                let ambient = format!("pointLights[{}].ambient", i);
                lit_shader.set_vec3(
                    &CString::new(ambient).unwrap(),
                    ambient_colour.x,
                    ambient_colour.y,
                    ambient_colour.z,
                );

                let specular = format!("pointLights[{}].specular", i);
                lit_shader.set_vec3(&CString::new(specular).unwrap(), 1.0, 1.0, 1.0);
                let constant = format!("pointLights[{}].constant", i);
                lit_shader.set_float(&CString::new(constant).unwrap(), 1.0);
                let linear = format!("pointLights[{}].linear", i);
                lit_shader.set_float(&CString::new(linear).unwrap(), 0.09);
                let quadratic = format!("pointLights[{}].linear", i);
                lit_shader.set_float(&CString::new(quadratic).unwrap(), 0.032);
            }

            // spotLight
            lit_shader.set_vector3(c_str!("spotLight.position"), &camera.Position.to_vec());
            lit_shader.set_vector3(c_str!("spotLight.direction"), &camera.Front);
            lit_shader.set_vec3(c_str!("spotLight.ambient"), 0.0, 0.0, 0.0);
            lit_shader.set_vec3(c_str!("spotLight.diffuse"), 1.0, 1.0, 1.0);
            lit_shader.set_vec3(c_str!("spotLight.specular"), 1.0, 1.0, 1.0);
            lit_shader.set_float(c_str!("spotLight.constant"), 1.0);

            lit_shader.set_float(c_str!("spotLight.linear"), 0.09);
            lit_shader.set_float(c_str!("spotLight.quadratic"), 0.032);
            lit_shader.set_float(c_str!("spotLight.cutOff"), 12.5f32.to_radians().cos());
            lit_shader.set_float(c_str!("spotLight.outerCutOff"), 15.0f32.to_radians().cos());

            //cube's material properties
            lit_shader.set_vec3(c_str!("material.ambient"), 1.0, 0.5, 0.31);
            lit_shader.set_vec3(c_str!("material.diffuse"), 1.0, 0.5, 0.31);
            lit_shader.set_vec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
            lit_shader.set_float(c_str!("material.shininess"), 32.0);

            // create transformations
            // NOTE: cgmath requires axis vectors to be normalized!
            // let model: Matrix4<f32> = Matrix4::from_axis_angle(
            //     vec3(0.5, 1.0, 0.0).normalize(),
            //     Rad(1.0 as f32)
            // );

            // 1st. render pass, draw objects as normal, writing to the stencil buffer
            // --------------------------------------------------------------------
            // unsafe {
            //     gl.StencilFunc(gl::ALWAYS, 1, 0xFF);
            //     gl.StencilMask(0xFF);
            // }
            for (_, position) in game_state.game_objects.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let model: Matrix4<f32> = Matrix4::from_translation(*position);
                // let angle = 20.0 * i as f32;
                // model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                lit_shader.set_mat4(c_str!("model"), &model);

                //draw the mesh!
                cube_model.draw(gl, &lit_shader);
            }

            // 2nd. render pass: now draw slightly scaled versions of the objects, this time disabling stencil writing.
            // Because the stencil buffer is now filled with several 1s. The parts of the buffer that are 1 are not drawn, thus only drawing
            // the objects' size differences, making it look like borders.
            // -----------------------------------------------------------------------------------------------------------------------------
            // unsafe{
            //     gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            //     gl.StencilMask(0x00);
            //     gl.Disable(gl::DEPTH_TEST);
            // }
            // self.border_shader.set_used();
            // self.border_shader.set_mat4(c_str!("view"), &view);
            // self.border_shader.set_mat4(c_str!("projection"), &projection);

            // let scale = 1.1;
            // for (_, position) in game_state.game_objects.iter().enumerate() {
            //     // calculate the model matrix for each object and pass it to shader before drawing
            //     let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
            //     model = model * Matrix4::from_scale(scale);
            //     // let angle = 20.0 * i as f32;
            //     // model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
            //     self.border_shader.set_mat4(c_str!("model"), &model);
            //     unsafe {
            //         gl.DrawArrays(gl::TRIANGLES, 0, 36);
            //     }
            // }
            // unsafe {
            //     gl.StencilMask(0xFF);
            //     gl.StencilFunc(gl::ALWAYS, 0, 0xFF);
            //     gl.Enable(gl::DEPTH_TEST);
            // }

            //Draw the sponza model
            {
                let mut model: Matrix4<f32> =
                    Matrix4::from_translation(game_state.sponza_position[0]);
                let scale = 0.01;
                model = model * Matrix4::from_scale(scale);
                lit_shader.set_mat4(c_str!("model"), &model);
                sponza_model.draw(gl, &lit_shader);
            }
        }

        { // Do flat shader business
            let flat_shader = self
                .shader_manager
                .get_shader("shaders/lit_directional")
                .unwrap();

            //Also draw lamp objects
            let light_direction = cgmath::Vector3 {
                x: -0.2,
                y: -1.0,
                z: -0.3,
            };

            //Shader: Lit Directional
            flat_shader.set_used();
            flat_shader.set_mat4(c_str!("projection"), &projection);
            flat_shader.set_mat4(c_str!("view"), &view);
            //cube's material properties
            flat_shader.set_vec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
            flat_shader.set_float(c_str!("material.shininess"), 32.0);
            //flat lighting
            flat_shader.set_vector3(c_str!("light.direction"), &light_direction);
            flat_shader.set_vec3(c_str!("light.ambient"), 0.2, 0.2, 0.2);
            flat_shader.set_vec3(c_str!("light.diffuse"), 0.5, 0.5, 0.5); // darken diffuse light a bit
            flat_shader.set_vec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

            for (i, position) in game_state.light_objects.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);

                model = model * Matrix4::from_scale(0.2); // a smaller cube
                flat_shader.set_mat4(c_str!("model"), &model);

                //Set material
                flat_shader.set_vector3(c_str!("material.diffuse"), &game_state.light_colours[i]);
                let ambient_colour = game_state.light_colours[i] * 0.2;
                flat_shader.set_vector3(c_str!("material.ambient"), &ambient_colour);

                unsafe {
                    cube_model.draw(gl, &flat_shader)
                    //gl.DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }

            {
                //Draw a plane
                plane_vao.bind();

                let model: Matrix4<f32> = Matrix4::from_translation(game_state.plane_position[0]);
                flat_shader.set_mat4(c_str!("model"), &model);

                unsafe {
                    gl.DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }
        }

        // draw skybox as last
        // unsafe {
        //     gl.DepthFunc(gl::LEQUAL); // change depth function so depth test passes when values are equal to depth buffer's content
        //     self.skybox_shader.set_used();
        //     // remove translation from the view matrix
        //     view.w[0] = 0.0;
        //     view.w[1] = 0.0;
        //     view.w[2] = 0.0;
        //     self.skybox_shader
        //         .set_mat4(c_str!("view"), &view);
        //     self.skybox_shader
        //         .set_mat4(c_str!("projection"), &projection);
        //     // skybox cube
        //     cubemap_vao.bind();
        //     gl.ActiveTexture(gl::TEXTURE0);
        //     gl.BindTexture(gl::TEXTURE_CUBE_MAP, *cubemap_texture);
        //     gl.DrawArrays(gl::TRIANGLES, 0, 36);
        //     cubemap_vao.unbind();
        //     gl.DepthFunc(gl::LESS); // set depth function back to default
        // }
    }
}
