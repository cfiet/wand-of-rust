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
use composite_ops::CompositeOperator;

// This module is just for internal use
#[path="bindings.rs"]
mod bindings;

// Expose the other modules that are meant for public consumption
pub mod channels;
pub mod colors;
pub mod filters;
pub mod composite_ops;


/**
 * The MagickWand struct is used in image manipulation functions.
 */
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
  /**
   * Create a new MagickWand instance. This instance will be properly
   * cleaned up once it falls out of scope.
   */
  pub fn new() -> MagickWand {
    MagickWand { wand: unsafe { bindings::NewMagickWand() } }
  }

  /**
   * Use a temporary MagickWand for a block of code. The MagickWand
   * will be cleaned up immediately after the provided closure completes.
   */
  pub fn borrow(block: |&MagickWand|) {
    let wand = ~MagickWand { wand: unsafe { bindings::NewMagickWand() } };
    block(wand);
    // Destructor fires here
  }

  /**
   * If any part of the image canvas has no color information, it will
   * be treated as 'background'. This method sets the color to whatever
   * the provided PixelWand is using.
   */
  pub fn set_image_background_color(&self, fill: PixelWand) {
    unsafe {
      bindings::MagickSetImageBackgroundColor(
        self.wand, 
        fill.wand as *std::libc::types::common::c95::c_void
      );
    }
  }

  /**
   * Compose overlay_img atop the current image using the specified composite
   * operator.
   */
  pub fn composite_image(&self, overlay_img: MagickWand, composeOp: CompositeOperator, 
                         x: int, y: int) -> bool {
    unsafe {
      bindings::MagickCompositeImage(
        self.wand, overlay_img.wand as *std::libc::types::common::c95::c_void, 
        composeOp as u32, x as i64, y as i64
      ) == bindings::MagickTrue
    }
  }

  /**
   * Negates the colors in the current active image, if only_gray is specified,
   * then only grayscale values will be negated.
   */
  pub fn negate_image(&self, only_gray: bool) -> bool {
    unsafe {
      let im_only_gray = match only_gray {
        true => bindings::MagickTrue,
        false => bindings::MagickFalse
      };
      bindings::MagickNegateImage(self.wand, im_only_gray) == bindings::MagickTrue
    }
  }

  /**
   * Assign / replace the current image clip mask.
   */
  pub fn set_image_clip_mask(&self, mask_wand: MagickWand) {
    unsafe {
      bindings::MagickSetImageClipMask(
        self.wand, mask_wand.wand as *std::libc::types::common::c95::c_void
      );
    }
  }

  /**
   * This method will analyze the specified channels, matching anything 
   * within a contiguous region of 'border' color and fill it with 'fill'
   * color. The border_fuzz param controls how loosely the detection logic
   * matches border regions. Finally, the x and y coordinates specify where
   * the fill should begin (think clicking a bucket fill button).
   *
   * If reverse is specified, then everything that is *not* the border 
   * color will be filled instead.
   */
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
      ) == bindings::MagickTrue
    }
  }

  /**
   * Read an image in for use with subsequent MagickWand operations.
   */
  pub fn read_image(&self, path: &str) {
    // TODO: Deal with error conditions somehow - maybe return a Result<Something,Error>?
    path.with_c_str(|buffer| {
      unsafe { bindings::MagickReadImage(self.wand, buffer) }
    });
  }

  /**
   * Retrieve the width of the image
   */
  pub fn get_image_width(&self) -> uint {
    unsafe { bindings::MagickGetImageWidth(self.wand) as uint }
  }

  /**
   * Retrieve the height of the image
   */
  pub fn get_image_height(&self) -> uint {
    unsafe { bindings::MagickGetImageHeight(self.wand) as uint }
  }

  /**
   * Resize the image to the specified width and height, using
   * the specified filter type with the specified blur / sharpness
   * factor.
   *
   * blur_factor values greater than 1 create blurriness, while values
   * less than 1 create sharpness.
   */
  pub fn resize_image(&self, width: uint, height: uint, 
                      filter: FilterType, blur_factor: f64) {
    unsafe {
      bindings::MagickResizeImage(
        self.wand, width as size_t, height as size_t, 
        filter as c_uint, blur_factor as c_double
      );
    }
  }

  /**
   * Sets the size of the MagickWand. To be used prior to reading a raw image.
   */
  pub fn set_size(&self, width: uint, height: uint) -> bool {
    unsafe {
      bindings::MagickSetSize(
        self.wand, width as size_t, height as size_t
      ) == bindings::MagickTrue
    } 
  }

  /**
   * Change the canvas size of an image to the specified width and 
   * height, centering the existing image on the specified coordinates.
   */
  pub fn extent_image(&self, width: uint, height: uint, x: int, y: int) {
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

  /**
   * When the MagickWand is working on multiple images, this method
   * will invoke the provided closure once for each image.
   */
  pub fn each_image(&self, block: ||) {
    unsafe {
      while (bindings::MagickNextImage(self.wand) != bindings::MagickFalse) {
        // TODO: This counts as multiple calls
        block();
      }
    };
  }

  /**
   * Specify the compression quality to use when writing an image.
   */
  pub fn set_image_compression_quality(&self, quality: u64) {
    unsafe { bindings::MagickSetImageCompressionQuality(self.wand, quality); }
  }

  /**
   * Write the current image to the provided path.
   */
  pub fn write_image(&self, path: &str) -> bool {
    path.with_c_str(|buffer| {
      unsafe { bindings::MagickWriteImage(self.wand, buffer) }
    }) == bindings::MagickTrue
  }

  /**
   * Writes an image or image sequence to the specified path.
   *
   * If adjoin is true, a single multi-image file will be created.
   *
   */
  pub fn write_images(&self, path: &str, adjoin: bool) -> bool {
    let magic_version = match adjoin {
      false => bindings::MagickFalse,
      true => bindings::MagickTrue
    };
    unsafe {
      path.with_c_str(|buffer| {
        bindings::MagickWriteImages(self.wand, buffer, magic_version)
      }) == bindings::MagickTrue
    } 
  }
}

/**
 * The PixelWand struct is used for pixel and color based operations.
 */
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

  /**
   * Set the current color of this wand.
   */
  pub fn set_color(&self, color: ColorName) {
    color_to_str(color).with_c_str(|buffer| {
      unsafe { bindings::PixelSetColor(self.wand, buffer); }
    })
  }
}

/**
 * This function must be called before any other ImageMagick operations
 * are attempted.
 */
pub fn MagickWandGenesis() {
  unsafe { bindings::MagickWandGenesis() }
}

/**
 * This function should be called when ImageMagick is no longer needed.
 */
pub fn MagickWandTerminus() {
  unsafe { bindings::MagickWandTerminus() }
}

