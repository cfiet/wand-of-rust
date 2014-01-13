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
use wand_of_rust::colors::{ None, White };
use wand_of_rust::channels::Channels;

fn main() {
  MagickWandGenesis();

  MagickWand::borrow(|wand| {
    // Change 'logo:' to another file name to perform this on a different file.
    // The file you choose must be smaller than 1024 x 768, as we do no image 
    // bounds checking - same as the original reference code.
    wand.read_image("logo:");

    // Used to specify the fill color
    let fill_wand = PixelWand::new();
    fill_wand.set_color(None);


    // Used to specify the border region color
    let border_wand = PixelWand::new();
    border_wand.set_color(White);
    
    // Specifies how fuzzy border detection is
    let border_fuzz = 20.0;

    let channels = match Channels::from_str("rgba") {
      Some(channel_spec) => channel_spec,
      _ => fail!("Unacceptable channel spec provided!")
    };

    wand.floodfill_paint_image(
      channels, fill_wand, border_fuzz,
      border_wand, 0, 0, false
    );

    wand.write_image("logo_floodfill.jpg");
  });

  MagickWandTerminus();
}
