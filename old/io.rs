use image;
use image::DynamicImage::*;
use image::GenericImage;
use image::{open, DynamicImage};
use std::ffi;
use std::ffi::CStr;
use std::fs;
use std::io::{self, Read};
use std::mem;
use std::os::raw::c_void;
use std::path::{Path, PathBuf};
use std::ptr;
use tobj;

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

  pub fn load_model(
    &self,
    resource_name: &str,
  ) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), tobj::LoadError> {
    let path = resource_name_to_path(&self.root_path, resource_name);
    println!("loading model from: {0}", path.display());

    let obj = tobj::load_obj(path, true);

    return obj;
  }

  pub fn load_bmp_image(&self, resource_name: &str) -> Result<sdl2::surface::Surface, String> {
    let path = resource_name_to_path(&self.root_path, resource_name);
    println!("loading image from: {0}", path.display());

    let icon = Surface::load_bmp(path);

    icon
  }

  /// loads a cubemap texture from 6 individual texture faces
  /// order:
  /// +X (right)
  /// -X (left)
  /// +Y (top)
  /// -Y (bottom)
  /// +Z (front)
  /// -Z (back)
  /// -------------------------------------------------------
  pub fn load_cubemap(&self, gl: &gl::Gl, faces: &[&str]) -> u32 {
    unsafe {
      let mut texture_id = 0;

      gl.GenTextures(1, &mut texture_id);
      gl.BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

      for (i, face) in faces.iter().enumerate() {
        let path = resource_name_to_path(&self.root_path, face);
        println!("trying to load cubemap from: {}", face);
        let img = image::open(path).expect("Cubemap texture failed to load");

        let data = img.raw_pixels();
        gl.TexImage2D(
          gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
          0,
          gl::RGB as i32,
          img.width() as i32,
          img.height() as i32,
          0,
          gl::RGB,
          gl::UNSIGNED_BYTE,
          &data[0] as *const u8 as *const c_void,
        );
      }

      gl.TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as i32,
      );
      gl.TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as i32,
      );
      gl.TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as i32,
      );
      gl.TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as i32,
      );
      gl.TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_R,
        gl::CLAMP_TO_EDGE as i32,
      );

      println!("texId: {}", texture_id);
      texture_id
    }
  }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
  let mut path: PathBuf = root_dir.into();

  for part in location.split("/") {
    path = path.join(part);
  }

  path
}
