//Import an external crate
//Crate means "Library" or "Package"
//Each crate has an implicit root module that contains core cod
extern crate image;
extern crate tobj;

use image::{ImageBuffer, Rgb, RgbImage};

use std::convert::TryFrom;
use std::env;
use std::mem;
use std::cell::Cell;

struct Vec2 {
    x: f32,
    y: f32,
}

fn main() {
    let _white = Rgb([255, 255, 255]);
    let _red = Rgb([255, 0, 0]);

    let imagex: u32 = 200;
    let imagey: u32 = 200 ;

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

    //Render model in wireframe
    draw_object("african_head.obj", imagex, imagey, &mut imgbuf, _white);

    // let mut t0: Vec<Vec2> = Vec::new();
    // t0.push(Vec2 );
    // t0.push(Vec2 );
    // t0.push(Vec2 );

    // let mut t0: [Vec2; 3] = [
    //     Vec2 { x: 10., y: 70. },
    //     Vec2 { x: 50., y: 160. },
    //     Vec2 { x: 70., y: 80. }
    // ];
    // let mut t1: [Vec2; 3] = [
    //     Vec2 { x: 180., y: 50. },
    //     Vec2 { x: 150., y: 1. },
    //     Vec2 { x: 70., y: 180. },
    // ];
    // let mut t2: [Vec2; 3] = [
    //     Vec2 { x: 180., y: 150. },
    //     Vec2 { x: 120., y: 160. },
    //     Vec2 { x: 130., y: 180. },
    // ];

    // let (a, b, c) = get_mut_points(&mut t0);
    // draw_triangle(a, b, c, &mut imgbuf, _red);

    // let (c, d, e) = get_mut_points(&mut t1);
    // draw_triangle(c, d, e, &mut imgbuf, _white);

    // let (f, g, h) = get_mut_points(&mut t2);
    // draw_triangle(f, g, h, &mut imgbuf, _white);
    
    imgbuf.save_with_format("test.png", image::ImageFormat::PNG);
}

fn get_mut_points( array : &mut [Vec2] ) -> (&mut Vec2, &mut Vec2, &mut Vec2)
{    
    let (a, tail) = array.split_at_mut(1);
    let (b, c) = tail.split_at_mut(1);
    println!("{} {} {}", a[0].y, b[0].y, c[0].y);
    return (&mut a[0], &mut b[0], &mut c[0]);
}

fn draw_object(
    name: &str,
    imagex: u32,
    imagey: u32,
    imgbuf: &mut RgbImage,
    color: image::Rgb<u8>,
) -> std::io::Result<()> {
    let mut current_dir_path = env::current_dir()?;
    println!("The current directory is {}", current_dir_path.display());
    current_dir_path.push("src");
    current_dir_path.push("TinyRenderer");
    current_dir_path.push("obj");
    current_dir_path.push(name);

    println!("Looking for object at: {}", current_dir_path.display());
    let african_head = tobj::load_obj(&current_dir_path);
    let (models, materials) = african_head.unwrap();
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

                let mut p0 : Vec2 = Vec2 {
                    x: ((v0[0] + 1.) * (imagex as f32 / 2.))
                    , y: ((v0[1] + 1.) * (imagey as f32 / 2.)) };

                let mut p1 : Vec2 = Vec2 {
                    x: ((v1[0] + 1.) * (imagex as f32 / 2.))
                    , y: ((v1[1] + 1.) * (imagex as f32 / 2.))};
            
                // let (a, b, c) = get_mut_points(&mut t0);
                // draw_triangle(a, b, c, &mut imgbuf, _red);

                //println!("x0: {} {} y0: {} {}", x0, y0, x1, y1);
                draw_line_vecs(&mut p0, &mut p1, imgbuf, color);
            }
        }
    }
    Ok(())
}

fn draw_triangle<'a>(
    t0: &'a mut Vec2,
    t1: &'a mut Vec2,
    t2: &'a mut Vec2,
    imgbuf: &mut RgbImage,
    color: image::Rgb<u8>,
) {
    draw_line_vecs(t0, t1, imgbuf, color);
    draw_line_vecs(t1, t2, imgbuf, color);
    draw_line_vecs(t2, t0, imgbuf, color);
}

fn draw_line_vecs<'a>(
    p0: &'a mut Vec2,
    p1: &'a mut Vec2,
    imgbuf: &mut RgbImage,
    color: image::Rgb<u8>,
) {
    let x0 = p0.x as i32;
    let y0 = p0.y as i32;
    let x1 = p1.x as i32;
    let y1 = p1.x as i32;

    let mut steep = false;
    if abs(x0 - x1) < abs(y0 - y1) {
        mem::swap(&mut p0.x, &mut p0.y);
        mem::swap(&mut p1.x, &mut p1.y);
        steep = true;
    }
    if p0.x > p1.x{
        mem::swap(p0, p1);
    }
    //println!("{} {}", *y1, *y0);

    for x in x0..( x1 + 1 ) 
    {
        let t : f32 = (x as f32 - p0.x) / (p1.x - p0.x);
        let y : i32 = ((p0.y * (1. - t)) + (p1.y * t)) as i32;

        if steep
        {
            let pixel = imgbuf.get_pixel_mut(y as u32, x as u32);
            let image::Rgb(data) = *pixel;
            *pixel = color;
        } else
        {
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let image::Rgb(data) = *pixel;
            *pixel = color;
        }
    }
}

    // for x in *x0..*x1 {
    //     if y >= 100 || x >= 100 {
    //         continue;
    //     }

    //     if steep {
    //         let pixel = imgbuf.get_pixel_mut(y as u32, x as u32);
    //         let image::Rgb(data) = *pixel;
    //         *pixel = color;
    //     } else {
    //         let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
    //         let image::Rgb(data) = *pixel;
    //         *pixel = color;
    //     }
    //     error2 += derror2;
    //     if error2 > dx {
    //         y += if y1 > y0 { 1 } else { -1 };
    //         error2 -= (dx * 2);
    //     }
    // }

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
