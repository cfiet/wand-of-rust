/*!
 * This example reads the ImageMagick logo and centers it on a 1024x768 extended
 * canvas filled with a blue background. 
 *
 * This is a rust adaptation of the code originally from here:
 *    http://members.shaw.ca/el.supremo/MagickWand/extent.htm
 */

extern mod wand_of_rust;

use wand_of_rust::{ MagickWandGenesis, MagickWandTerminus };
use wand_of_rust::MagickWand;
use wand_of_rust::PixelWand;
use wand_of_rust::colors::Blue;

fn main() {
  MagickWandGenesis();

  MagickWand::borrow(|wand| {
    // Change 'logo:' to another file name to perform this on a different file.
    // The file you choose must be smaller than 1024 x 768, as we do no image 
    // bounds checking - same as the original reference code.
    wand.read_image("logo:");
    let width = wand.get_image_width();
    let height = wand.get_image_height();

    let pixel_wand = PixelWand::new();

    pixel_wand.set_color(Blue);
    wand.set_image_background_color(pixel_wand);

    let new_x = ( -(1024 - width) / 2 ) as int;
    let new_y = ( -(1024 - height) / 2 ) as int;
    // Center the image on the newly extended canvas
    wand.extent_image(1024, 768, new_x, new_y);
    wand.write_image("logo_extent.jpg");
  });

  MagickWandTerminus();
}
