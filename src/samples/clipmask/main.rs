/*!
 * This example demonstrates some masking and clipping functionality.
 * This is a rust adaptation of the code originally from here:
 *    http://members.shaw.ca/el.supremo/MagickWand/clipmask.htm
 */

extern mod wand_of_rust;

use wand_of_rust::{ MagickWandGenesis, MagickWandTerminus };
use wand_of_rust::MagickWand;
use wand_of_rust::composite_ops::OverCompositeOp;

fn main() {
  MagickWandGenesis();

  // RED 16Jan2014 16:46:29 - Using RAII for the wands causes problems, as attempting
  // to free a wand after MagickWandTerminus() has been called causes a segfault in
  // the ImageMagick library (assertion failure).
  {
    // instantiate the various wands we will need
    let destination = MagickWand::new();
    let mask = MagickWand::new();
    let source = MagickWand::new();

    let width = 100;
    let height = 100;

    destination.set_size(width, height);
    source.set_size(width, height);

    destination.read_image("tile:data/tile_water.jpg");
    mask.read_image("data/mask_bite.png");

    // When creating a mask, white is used for parts that should pass through,
    // while black is used to block parts. Internally, ImageMagick does the
    // inverse of this, so the mask must be negated.
    mask.negate_image(false);
    // Assign the mask
    destination.set_image_clip_mask(mask);

    source.read_image("tile:data/tile_disks.jpg");
    // The line below overlays source over the destination (background)
    destination.composite_image(source, OverCompositeOp, 0, 0);

    destination.write_image("clip_out.jpg");
  };
  MagickWandTerminus();
}
