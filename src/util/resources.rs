use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tobj;
use image;

use crate::threed::model::FGModel;
use sdl2::surface::Surface;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed get executable path")]
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;

        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn from_exe_path() -> Result<Resources, Error> {
        Resources::from_relative_exe_path(Path::new(""))
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    pub fn load_model(&self, resource_name: &str) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), tobj::LoadError> {
        
        let path = resource_name_to_path(&self.root_path, resource_name);
        println!("loading model from: {0}", path.display());

        let obj = tobj::load_obj(path, true);

        return obj;     
    }

    pub fn load_image(&self, resource_name: &str) -> Result<sdl2::surface::Surface, String> {

        let path = resource_name_to_path(&self.root_path, resource_name);
        println!("loading image from: {0}", path.display());

        use image::DynamicImage;
        use image::GenericImageView;
        use sdl2::pixels::PixelFormatEnum;

        //let img = image::open(path).unwrap();

        // // let mut imgbuf = image::ImageBuffer::new(img.dimensions().0, img.
        
        // let raw_image_data : Vec<u8>;

        // for x in 0..img.dimensions().0 {
        //     for y in 0..img.dimensions().1 {
        //         let rgba = img.get_pixel(x, y);

        //         raw_image_data.push()
        //     }
        // }
        
        // let icon = Surface::from_data(&buffer, img.width(), img.height(), , PixelFormatEnum::);

        let icon = Surface::load_bmp(path);
        return icon;
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}