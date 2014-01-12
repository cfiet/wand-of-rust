/*!
 * This example resizes the ImageMagick logo to 50% of its original size, sharpens
 * it, and saves it as a high quality JPEG.
 * 
 * This code is a rust adaptation of the resize example seen here:
 *  http://members.shaw.ca/el.supremo/MagickWand/resize.htm
 *
 *  Usage:
 *    ./resize
 */

extern mod wand_of_rust;

use wand_of_rust::{ MagickWandGenesis, MagickWandTerminus };
use wand_of_rust::LanczosFilter;
use wand_of_rust::MagickWand;

fn main() {
  MagickWandGenesis();

  MagickWand::borrow(|wand| {
    // To convert a different file, change 'logo:' to a different file name
    wand.read_image("logo:");
    // Halve the dimensions, ensuring that the values are at least 1
    let halfwidth = match wand.get_image_width() {
      1 => 1,
      width => width / 2
    };

    let halfheight = match wand.get_image_height() {
      1 => 1,
      height => height / 2
    };

    // Resize using a Lanczos filter
    wand.resize_image(halfwidth, halfheight, LanczosFilter, 1.0);
    // Set the image compression to a high quality setting
    wand.set_image_compression_quality(95);
    // Save it
    wand.write_image("logo_resized.jpg");
  });

  MagickWandTerminus();
}
