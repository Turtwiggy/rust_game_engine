//Import an external crate
//Crate means "Library" or "Package"
//Each crate has an implicit root module that contains core cod
extern crate image;
extern crate tobj;

use image::{ImageBuffer, Rgb, RgbImage};

use std::convert::TryFrom;
use std::env;
use std::mem;

fn main() -> std::io::Result<()> {
    let _white = Rgb([255, 255, 255]);
    let _red = Rgb([255, 0, 0]);

    let imagex: u32 = 800;
    let imagey: u32 = 800;

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut imgbuf: RgbImage = ImageBuffer::new(imagex, imagey);

    // Iterate over the coordinates and pixels of the image
    //X iterates left to right
    //Y iterates top to bottom
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let percentagex = x as f32 / imagex as f32;
        let percentagey = y as f32 / imagey as f32;

        let x_rgb: u8 = 0;
        let y_rgb: u8 = percent_to_rgb(percentagey);
        //let y_rgb : u8 = percent_to_rgb(percentagey);
        //println!("{}  {}", x_rgb, y_rgb);
        // let r = (0.3 * x as f32) as u8;
        // let b = (0.3 * y as f32) as u8;
        let color = Rgb([x_rgb, 0, y_rgb]);
        *pixel = color;
    }

    // //Draw a line in the buffer
    // draw_line(&mut 50, &mut 20, &mut 80, &mut 40, &mut imgbuf, _white);
    // draw_line(&mut 20, &mut 13, &mut 40, &mut 80, &mut imgbuf, _red);
    // draw_line(&mut 80, &mut 40, &mut 13, &mut 20, &mut imgbuf, _red);

    let mut current_dir_path = env::current_dir()?;
    println!("The current directory is {}", current_dir_path.display());

    current_dir_path.push("src");
    current_dir_path.push("TinyRenderer");
    current_dir_path.push("obj");
    current_dir_path.push("african_head.obj");
    println!("Looking for object at: {}", current_dir_path.display());

    //Render model in wireframe
    let african_head = tobj::load_obj(&current_dir_path);
    let (models, materials) = african_head.unwrap();

    //Get model info
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!("Size of model[{}].indices: {}", i, mesh.indices.len());
        // for f in 0..mesh.indices.len() / 3
        // {
        //     println!("    idx[{}] = {}, {}, {}.", f,
        //         mesh.indices[3 * f],
        //         mesh.indices[3 * f + 1],
        //         mesh.indices[3 * f + 2]);
        // }
        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);
        // for v in 0..mesh.positions.len() / 3 {
        //     println!("    v[{}] = ({}, {}, {})", v,
        //         mesh.positions[3 * v],
        //         mesh.positions[3 * v + 1],
        //         mesh.positions[3 * v + 2]);
        // }

        //Draw model
        let n_faces = mesh.indices.len() / 3;
        println!("n_faces: {} ", n_faces);
        for i in 0..mesh.indices.len() / 3 {
            let face: [u32; 3] = [
                mesh.indices[3 * i],
                mesh.indices[3 * i + 1],
                mesh.indices[3 * i + 2],
            ];

            for j in 0..3 {
                let v0: [f32; 3] = [
                    mesh.positions[(3 * face[j] + 0) as usize],
                    mesh.positions[(3 * face[j] + 1) as usize],
                    mesh.positions[(3 * face[j] + 2) as usize],
                ];
                let v1: [f32; 3] = [
                    mesh.positions[(3 * face[(j + 1) % 3] + 0) as usize],
                    mesh.positions[(3 * face[(j + 1) % 3] + 1) as usize],
                    mesh.positions[(3 * face[(j + 1) % 3] + 2) as usize],
                ];
                //println!("v0: {} {} v1: {} {}", v0[0], v0[1], v1[0], v1[1]);

                let mut x0: i32 = ((v0[0] + 1.) * (imagex as f32 / 2.)) as i32;
                let mut y0: i32 = ((v0[1] + 1.) * (imagey as f32 / 2.)) as i32;
                let mut x1: i32 = ((v1[0] + 1.) * (imagex as f32 / 2.)) as i32;
                let mut y1: i32 = ((v1[1] + 1.) * (imagex as f32 / 2.)) as i32;

                println!("x0: {} {} y0: {} {}", x0, y0, x1, y1);
                draw_line(&mut x0, &mut y0, &mut x1, &mut y1, &mut imgbuf, _white);
            }
        }
    }

    imgbuf.save_with_format("test.png", image::ImageFormat::PNG);

    Ok(())
}

//Takes in 0->1 and returns 0->255
fn percent_to_rgb(percent: f32) -> u8 {
    (percent * 255.0) as u8
}

fn abs(number: i32) -> i32 {
    if number < 0 {
        return number * -1;
    }
    return number;
}

fn draw_line<'a>(
    x0: &'a mut i32,
    y0: &'a mut i32,
    x1: &'a mut i32,
    y1: &'a mut i32,
    imgbuf: &mut RgbImage,
    color: image::Rgb<u8>,
) {
    let mut steep = false;
    if abs( *x0 - *x1 ) < abs( *y0 - *y1 ) {
        mem::swap(x0, y0);
        mem::swap(x1, y1);
        steep = true;
    }
    if x0 > x1 {
        mem::swap(x0, x1);
        mem::swap(y0, y1);
    }

    let dx = *x1 - *x0;
    let dy = *y1 - *y0;
    println!("{} {}", *y1, *y0);

    let derror2 = abs(dy as i32) * 2;
    let mut error2: i32 = 0;
    let mut y = *y0;

    for x in *x0..*x1 {

        if y == 800 || x == 800
        {
            continue;
        }

        if steep {
            let pixel = imgbuf.get_pixel_mut(y as u32, x as u32);
            let image::Rgb(data) = *pixel;
            *pixel = color;
        } else {
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let image::Rgb(data) = *pixel;
            *pixel = color;
        }
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= (dx * 2);
        }
    }
}
