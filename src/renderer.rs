extern crate gl;

use game::GameState;
use renderer_gl::*;
use threed::camera::*;
use util::resources::Resources;

use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Rad, Vector3};
use std::ffi::{CStr, CString};
use threed::mesh::{FGMesh, FGVertex};
use threed::model::FGModel;

pub fn create_renderer(gl: &gl::Gl, res: &Resources) -> Renderer {
    //Shaders available to the renderer
    let lit_shader =
        shader::Program::from_res(&gl, &res, "shaders/lit_multiple_lights_no_tex").unwrap();
    let light_shader = shader::Program::from_res(&gl, &res, "shaders/lit_directional").unwrap();
    let stencil_shader = shader::Program::from_res(&gl, &res, "shaders/stencil_border").unwrap();
    // shader_program.setInt(c_str!("texture1"), 0);

    // Configure OpenGL
    // ----------------
    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        gl.DepthFunc(gl::LESS);
        gl.Enable(gl::STENCIL_TEST);
        gl.StencilFunc(gl::NOTEQUAL, 1, 0xFF);
        gl.StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // gl::Disable(gl::DEPTH_TEST);
    }

    return Renderer {
        lit_shader: lit_shader,
        flat_shader: light_shader,
        border_shader: stencil_shader,
    };
}

pub struct Renderer {
    lit_shader: shader::Program,
    flat_shader: shader::Program,
    border_shader: shader::Program,
}

impl Renderer {
    pub fn render(
        &self,
        gl: &gl::Gl,
        window_size: (u32, u32),
        camera: &Camera,
        game_state: &GameState,
        cube_model: &FGModel,
        sponza_model: &FGModel,
    ) {
        unsafe {
            // render window contents here
            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl.StencilMask(0x00);
        }

        //View Projection
        let view = camera.GetViewMatrix();
        let projection: Matrix4<f32> = perspective(
            Deg(camera.Zoom),
            window_size.0 as f32 / window_size.1 as f32,
            0.1,
            100.0,
        );

        // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
        self.lit_shader.set_used();

        let view_pos = cgmath::Vector3 {
            x: camera.Position.x,
            y: camera.Position.y,
            z: camera.Position.z,
        };
        self.lit_shader.set_vector3(c_str!("viewPos"), &view_pos);
        self.lit_shader.set_mat4(c_str!("view"), &view);
        self.lit_shader.set_mat4(c_str!("projection"), &projection);

        //SHADER: lit_directional
        //-----------------------
        // let light_direction = cgmath::Vector3{x: -0.2, y: -1.0, z: -0.3};

        //SHADER: lit_point_noatten
        //-------------------------
        //self.lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
        // self.lit_shader.set_vector3(c_str!("light.direction"), &light_direction);

        //SHADER: lit_point_atten
        //-----------------------
        // self.lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
        // self.lit_shader.set_float(c_str!("light.constant"), 1.0);
        // self.lit_shader.set_float(c_str!("light.linear"), 0.09);
        // self.lit_shader.set_float(c_str!("light.quadratic"), 0.032);

        //SHADER: lit_flashlight
        //-----------------------
        // self.lit_shader.set_float(c_str!("light.constant"), 1.0);
        // self.lit_shader.set_float(c_str!("light.linear"), 0.09);
        // self.lit_shader.set_float(c_str!("light.quadratic"), 0.032);
        // //self.lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
        // let light_position = Vector3 {
        //     x: camera.Position.x,
        //     y: camera.Position.y,
        //     z: camera.Position.z
        // };
        // self.lit_shader.set_vector3(c_str!("light.position"), &light_position);
        // self.lit_shader.set_vector3(c_str!("light.direction"), &camera.Front);
        // self.lit_shader.set_float(c_str!("light.cutOff"), 12.5f32.to_radians().cos());
        // self.lit_shader.set_float(c_str!("light.outerCutOff"), 17.5f32.to_radians().cos());
        // //light properties
        // let light_colour = Vector3 {
        //     // x: ((1.0 * 2.0) as f32).sin(),
        //     // y: ((1.0 * 0.7) as f32).sin(),
        //     // z: ((1.0 * 1.3) as f32).sin(),
        //     x: 1.0,
        //     y: 1.0,
        //     z: 1.0
        // };
        // let diffuse_colour = light_colour * 0.5;
        // let ambient_colour = diffuse_colour * 0.2;
        // self.lit_shader.set_vector3(c_str!("light.ambient"), &ambient_colour);
        // self.lit_shader.set_vector3(c_str!("light.diffuse"), &diffuse_colour);
        // self.lit_shader.set_vec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

        //SHADER: lit_mutiple_lights_no_tex
        //---------------------------------
        /*
            Here we set all the uniforms for the 5/6 types of lights we have. We have to set them manually and index
            the proper PointLight struct in the array to set each uniform variable. This can be done more code-friendly
            by defining light types as classes and set their values in there, or by using a more efficient uniform approach
            by using 'Uniform buffer objects', but that is something we'll discuss in the 'Advanced GLSL' tutorial.
        */

        // directional light
        self.lit_shader
            .set_vec3(c_str!("dirLight.direction"), -0.2, -1.0, -0.3);
        self.lit_shader
            .set_vec3(c_str!("dirLight.ambient"), 0.05, 0.05, 0.05);
        self.lit_shader
            .set_vec3(c_str!("dirLight.diffuse"), 0.4, 0.4, 0.4);
        self.lit_shader
            .set_vec3(c_str!("dirLight.specular"), 0.5, 0.5, 0.5);

        // 4 point lights
        for i in 0..game_state.light_objects.len() {
            let position = format!("pointLights[{}].position", i);
            self.lit_shader.set_vector3(
                &CString::new(position).unwrap(),
                &game_state.light_objects[i],
            );

            let diffuse_colour = &game_state.light_colours[i];
            let diffuse = format!("pointLights[{}].diffuse", i);
            self.lit_shader.set_vec3(
                &CString::new(diffuse).unwrap(),
                diffuse_colour.x,
                diffuse_colour.y,
                diffuse_colour.z,
            );
            let ambient_colour = diffuse_colour * 0.2;
            let ambient = format!("pointLights[{}].ambient", i);
            self.lit_shader.set_vec3(
                &CString::new(ambient).unwrap(),
                ambient_colour.x,
                ambient_colour.y,
                ambient_colour.z,
            );

            let specular = format!("pointLights[{}].specular", i);
            self.lit_shader
                .set_vec3(&CString::new(specular).unwrap(), 1.0, 1.0, 1.0);
            let constant = format!("pointLights[{}].constant", i);
            self.lit_shader
                .set_float(&CString::new(constant).unwrap(), 1.0);
            let linear = format!("pointLights[{}].linear", i);
            self.lit_shader
                .set_float(&CString::new(linear).unwrap(), 0.09);
            let quadratic = format!("pointLights[{}].linear", i);
            self.lit_shader
                .set_float(&CString::new(quadratic).unwrap(), 0.032);
        }

        // spotLight
        self.lit_shader
            .set_vector3(c_str!("spotLight.position"), &camera.Position.to_vec());
        self.lit_shader
            .set_vector3(c_str!("spotLight.direction"), &camera.Front);
        self.lit_shader
            .set_vec3(c_str!("spotLight.ambient"), 0.0, 0.0, 0.0);
        self.lit_shader
            .set_vec3(c_str!("spotLight.diffuse"), 1.0, 1.0, 1.0);
        self.lit_shader
            .set_vec3(c_str!("spotLight.specular"), 1.0, 1.0, 1.0);
        self.lit_shader.set_float(c_str!("spotLight.constant"), 1.0);

        self.lit_shader.set_float(c_str!("spotLight.linear"), 0.09);
        self.lit_shader
            .set_float(c_str!("spotLight.quadratic"), 0.032);
        self.lit_shader
            .set_float(c_str!("spotLight.cutOff"), 12.5f32.to_radians().cos());
        self.lit_shader
            .set_float(c_str!("spotLight.outerCutOff"), 15.0f32.to_radians().cos());

        //cube's material properties
        self.lit_shader
            .set_vec3(c_str!("material.ambient"), 1.0, 0.5, 0.31);
        self.lit_shader
            .set_vec3(c_str!("material.diffuse"), 1.0, 0.5, 0.31);
        self.lit_shader
            .set_vec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
        self.lit_shader
            .set_float(c_str!("material.shininess"), 32.0);

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
            self.lit_shader.set_mat4(c_str!("model"), &model);

            //draw the mesh!
            //cube_model.Draw(gl, &self.lit_shader);
        }

        {
            let mut model: Matrix4<f32> = Matrix4::from_translation(game_state.sponza_position[0]);

            let scale = 0.01;
            model = model * Matrix4::from_scale(scale);

            self.lit_shader.set_mat4(c_str!("model"), &model);
            sponza_model.draw(gl, &self.lit_shader);
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

        //Also draw lamp objects
        let light_direction = cgmath::Vector3{x: -0.2, y: -1.0, z: -0.3};
        //Shader: Lit Directional
        self.flat_shader.set_used();
        self.flat_shader.set_mat4(c_str!("projection"), &projection);
        self.flat_shader.set_mat4(c_str!("view"), &view);
        //cube's material properties
        self.flat_shader.set_vec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
        self.flat_shader.set_float(c_str!("material.shininess"), 32.0);
        //flat lighting
        self.flat_shader.set_vector3(c_str!("light.direction"), &light_direction);
        self.flat_shader.set_vec3(c_str!("light.ambient"),  0.2, 0.2, 0.2);
        self.flat_shader.set_vec3(c_str!("light.diffuse"),  0.5, 0.5, 0.5); // darken diffuse light a bit
        self.flat_shader.set_vec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

        for(i, position) in game_state.light_objects.iter().enumerate(){
            // calculate the model matrix for each object and pass it to shader before drawing
            let mut model: Matrix4<f32> = Matrix4::from_translation(*position);

            model = model * Matrix4::from_scale(0.2);  // a smaller cube
            self.flat_shader.set_mat4(c_str!("model"), &model);

            //Set material
            self.flat_shader.set_vector3(c_str!("material.diffuse"), &game_state.light_colours[i]);
            let ambient_colour = game_state.light_colours[i] * 0.2;
            self.flat_shader.set_vector3(c_str!("material.ambient"), &ambient_colour);

            unsafe {
                cube_model.draw(gl, &self.flat_shader)
                //gl.DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

   

    }
}
