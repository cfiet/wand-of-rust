//! Module: wand-of-rust

#[crate_id = "wand_of_rust"];
#[crate_type = "dylib"];

#[desc = "ImageMagick's MagickWand bindings"];
#[license = "MIT"];

#[feature(globs)];

#[allow(dead_code)];

extern mod std;
use std::libc::{c_uint, size_t, c_double};
use channels::Channels;
use filters::FilterType;
use colors::ColorName;
use colors::color_to_str;

#[path="bindings.rs"]
mod bindings;

pub mod channels;
pub mod colors;
pub mod filters;


pub struct MagickWand {
  priv wand: *mut bindings::MagickWand
}

/**
 * Automate safe cleanup for MagickWand instances
 */
impl Drop for MagickWand {
  fn drop(&mut self) {
    unsafe { bindings::DestroyMagickWand(self.wand); }
  }
}

impl MagickWand {
  pub fn new() -> MagickWand {
    MagickWand { wand: unsafe { bindings::NewMagickWand() } }
  }

  pub fn borrow(block: |&MagickWand|) {
    let wand = ~MagickWand { wand: unsafe { bindings::NewMagickWand() } };
    block(wand);
    // Destructor fires here
  }

  pub fn set_image_background_color(&self, fill: PixelWand) {
    unsafe {
      bindings::MagickSetImageBackgroundColor(
        self.wand, 
        fill.wand as *std::libc::types::common::c95::c_void
      );
    }
  }

  pub fn floodfill_paint_image(&self, channels: Channels, fill: PixelWand, 
                               border_fuzz: f64, border: PixelWand, 
                               x: int, y: int, reverse: bool) -> bool {
    unsafe {
      bindings::MagickFloodfillPaintImage(
        self.wand, 
        channels.bitflags, 
        fill.wand as *std::libc::types::common::c95::c_void, 
        border_fuzz, 
        border.wand as *std::libc::types::common::c95::c_void, 
        x as i64, y as i64, 
        reverse as u32
      ) == 0
    }
  }

  pub fn read_image(&self, path: &str) {
    // TODO: Deal with error conditions somehow - maybe return a Result<Something,Error>?
    path.with_c_str(|buffer| {
      unsafe { bindings::MagickReadImage(self.wand, buffer) }
    });
  }

  pub fn get_image_width(&self) -> int {
    unsafe { bindings::MagickGetImageWidth(self.wand) as int }
  }

  pub fn get_image_height(&self) -> int {
    unsafe { bindings::MagickGetImageHeight(self.wand) as int }
  }

  pub fn resize_image(&self, width: int, height: int, filter: FilterType, arg: f64) {
    unsafe {
      bindings::MagickResizeImage(
        self.wand, width as size_t, height as size_t, 
        filter as c_uint, arg as c_double
      );
    }
  }

  pub fn extent_image(&self, width: int, height: int, x: int, y: int) {
    unsafe { 
      bindings::MagickExtentImage(
        self.wand, width as u64, height as u64, x as i64, y as i64
      ); 
    }
  }

  pub fn reset_iterator(&self) {
    // TODO: Again, deal with error conditions
    unsafe { bindings::MagickResetIterator(self.wand); }
  }

  pub fn each_image(&self, block: ||) {
    unsafe {
      while (bindings::MagickNextImage(self.wand) != bindings::MagickFalse) {
        // TODO: This counts as multiple calls
        block();
      }
    };
  }

  pub fn set_image_compression_quality(&self, quality: u64) {
    unsafe { bindings::MagickSetImageCompressionQuality(self.wand, quality); }
  }

  pub fn write_image(&self, path: &str) {
    path.with_c_str(|buffer| {
      unsafe { bindings::MagickWriteImage(self.wand, buffer); }
    })
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

pub struct PixelWand {
  priv wand: *mut bindings::PixelWand
}

/*
 * Trying an RAII scheme for the pixel wand.
 */
impl Drop for PixelWand {
  fn drop(&mut self) {
    unsafe { bindings::DestroyPixelWand(self.wand); }
  }
}

impl PixelWand {

  /**
   * Construct a new PixelWand and return it in an owned box.
   */
  pub fn new() -> PixelWand {
    PixelWand { wand: unsafe { bindings::NewPixelWand() } }
  }

  /**
   * Retrieve a temporary pixelwand, cleaning up its resources
   * immediately after the provided closure completes.
   */
  pub fn borrow(block: |&PixelWand|) {
    let wand = ~PixelWand { wand: unsafe { bindings::NewPixelWand() } };
    block(wand);
    // Destructor fired here
  }

  pub fn set_color(&self, color: ColorName) {
    color_to_str(color).with_c_str(|buffer| {
      unsafe { bindings::PixelSetColor(self.wand, buffer); }
    })
  }
}

pub fn MagickWandGenesis() {
  unsafe { bindings::MagickWandGenesis() }
}

pub fn MagickWandTerminus() {
  unsafe { bindings::MagickWandTerminus() }
}

