
    // //A Shader
    // // let vert_shader = shader::Shader::from_vert_source(
    // //     &CString::new(include_str!("data/shaders/triangle.vert")).unwrap(),
    // // )
    // // .unwrap();
    // // let frag_shader = shader::Shader::from_frag_source(
    // //     &CString::new(include_str!("data/shaders/triangle.frag")).unwrap(),
    // // )
    // // .unwrap();
    // // let shader_program = shader::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    // // shader_program.set_used();

    // //A triangle
    // let vertices: Vec<f32> = vec![
    //     // positions      // colors
    //     0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
    //     -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
    //     0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    // ];

    // //VBO
    // let mut vbo: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut vbo);
    // }
    // unsafe {
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,                                                       // target
    //         (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
    //         vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
    //         gl::STATIC_DRAW,                               // usage
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // }
    // //VAO
    // let mut vao: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    // }
    // unsafe {
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    //     gl::VertexAttribPointer(
    //         0,         // index of the generic vertex attribute ("layout (location = 0)")
    //         3,         // the number of components per generic vertex attribute
    //         gl::FLOAT, // data type
    //         gl::FALSE, // normalized (int-to-float conversion)
    //         (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
    //         std::ptr::null(),                                     // offset of the first component
    //     );
    //     gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
    //     gl::VertexAttribPointer(
    //         1,         // index of the generic vertex attribute ("layout (location = 0)")
    //         3,         // the number of components per generic vertex attribute
    //         gl::FLOAT, // data type
    //         gl::FALSE, // normalized (int-to-float conversion)
    //         (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
    //         (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    //     gl::BindVertexArray(0);
    // }

    // unsafe {
    //     println!("setting viewport");
    //     gl::Viewport(0, 0, width as GLint, height as GLint );
    // }