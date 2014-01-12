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
use wand_of_rust::Blue;

fn main() {
  MagickWandGenesis();

  do MagickWand::borrow |wand| {
    // Change 'logo:' to another file name to perform this on a different file.
    // The file you choose must be smaller than 1024 x 768, as we do no image 
    // bounds checking - same as the original reference code.
    wand.read_image("logo:");
    let width = wand.get_image_width();
    let height = wand.get_image_height();

    do PixelWand::borrow |pixel_wand| {
      pixel_wand.set_color(Blue);
      // The line below doesn't work yet, as I'm trying to determine a way to
      // expose the guts of the PixelWand structure to this MagickWand method.
      // Additionally, this closure can't fulfill Send if it captures the 
      // &MagickWand reference
      //wand.set_image_background_color(pixel_wand);
    };

    // Center the image on the newly extended canvas
    wand.extent_image(1024, 768, -(1024 - width) / 2, -(768 - height) / 2);
    wand.write_image("logo_extent.jpg");
  };

  MagickWandTerminus();
}
