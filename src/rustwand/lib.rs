//! Module: rustwand

#[crate_id="rustwand"]

#[crate_type = "dylib"]

#[feature(globs)]


extern mod std;
use std::libc::{c_uint, size_t, c_double};

#[path="bindings.rs"]
mod bindings;

pub enum FilterType {
  UndefinedFilter = 0,
  PointFilter = 1,
  BoxFilter = 2,
  TriangleFilter = 3,
  HermiteFilter = 4,
  HanningFilter = 5,
  HammingFilter = 6,
  BlackmanFilter = 7,
  GaussianFilter = 8,
  QuadraticFilter = 9,
  CubicFilter = 10,
  CatromFilter = 11,
  MitchellFilter = 12,
  JincFilter = 13,
  SincFilter = 14,
  SincFastFilter = 15,
  KaiserFilter = 16,
  WelshFilter = 17,
  ParzenFilter = 18,
  BohmanFilter = 19,
  BartlettFilter = 20,
  LagrangeFilter = 21,
  LanczosFilter = 22,
  LanczosSharpFilter = 23,
  Lanczos2Filter = 24,
  Lanczos2SharpFilter = 25,
  RobidouxFilter = 26,
  RobidouxSharpFilter = 27,
  CosineFilter = 28,
  SplineFilter = 29,
  LanczosRadiusFilter = 30,
  SentinelFilter = 31
}

pub struct MagickWand {
  priv wand: *mut bindings::MagickWand
}

impl MagickWand {
  pub fn borrow(block: |wand: &MagickWand|) {
    let wand = ~MagickWand { wand: unsafe { bindings::NewMagickWand() } };
    block(wand);
    unsafe { bindings::DestroyMagickWand(wand.wand); }
  }

  pub fn read_image(&self, path: &str) {
    // TODO: Deal with error conditions somehow - maybe return a Result<Something,Error>?
    path.with_c_str(|buffer| {
      unsafe { bindings::MagickReadImage(self.wand, buffer) }
    });
  }

  pub fn reset_iterator(&self) {
    // TODO: Again, deal with error conditions
    unsafe { bindings::MagickResetIterator(self.wand); }
  }

  pub fn each_image(&self, f: ||) {
    unsafe {
      while (bindings::MagickNextImage(self.wand) != bindings::MagickFalse) {
        f();
      }
    };
  }

  pub fn resize_image(&self, width: int, height: int, filter: FilterType, arg: f64) {
    unsafe {
      bindings::MagickResizeImage(
        self.wand, width as size_t, height as size_t, 
        filter as c_uint, arg as c_double
      );
    }
  }

  pub fn write_images(&self, path: &str, adjoin: bool) {
    let magic_version = match adjoin {
      false => bindings::MagickFalse,
      true => bindings::MagickTrue
    };
    unsafe {
      path.with_c_str(|buffer| {
        bindings::MagickWriteImages(self.wand, buffer, magic_version);
      })
    }
  }
}

pub fn MagickWandGenesis() {
  unsafe { bindings::MagickWandGenesis() }
}

pub fn MagickWandTerminus() {
  unsafe { bindings::MagickWandTerminus() }
}

