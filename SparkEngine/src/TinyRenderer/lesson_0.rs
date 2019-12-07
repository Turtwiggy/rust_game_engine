//Import an external crate
//Crate means "Library" or "Package"
//Each crate has an implicit root module that contains core cod
extern crate nalgebra as na;
use na::{Rotation3, Vector4};

extern crate image;

fn main() {
    let _white = Vector4::new(255, 255, 255, 255);
    let _red = Vector4::new(255, 0, 0, 255);

    let imagex : u32 = 100;
    let imagey : u32 = 100;

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(imagex, imagey);
    
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let percentagex = x as f32 / imagex as f32;
        let percentagey = y as f32 / imagey as f32;

        let x_rgb : u8 = percent_to_rgb(percentagex);
        let y_rgb : u8 = percent_to_rgb(percentagey);
        println!("{}  {}", x_rgb, y_rgb);
        
        // let r = (0.3 * x as f32) as u8;
        // let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([x_rgb, 0, y_rgb]);
    }

    imgbuf.save_with_format("test.png", image::ImageFormat::PNG );

    // let mut img = Image::new(640, 480);
    // for y in 0u32..x {
    //     for x in 0u32..y {

    //         img.set_pixel(x as i32, y as i32, Color(255, 0, 0));

    //         // let r = ((x ^ y) % 256) as u8;
    //         // let g = ((x + y) % 256) as u8;
    //         // let b = ((y.wrapping_sub(x)) % 256) as u8;
    //         // img.set_pixel(x as i32, y as i32, Color(r, g, b));
    //     }
    // }
    // img.apply_gamma(2.2);
    // img.write_to_tga("test.tga").unwrap();
}

fn percent_to_rgb(percent : f32) -> u8
{
    (percent * 255.0) as u8
}
