/*!
 * The first code example from http://www.imagemagick.org/script/magick-wand.php
 * 
 * This program reads an image, creates a thumbnail, and writes out the result.
 * Usage:
 *    ./thumbnail inputfile [-o outputfile]
 */
extern mod extra;
extern mod wand_of_rust;

use extra::getopts::{ Opt, getopts, optopt, optflag };
use wand_of_rust::LanczosFilter;
use wand_of_rust::MagickWand;
use wand_of_rust::MagickWandGenesis;
use wand_of_rust::MagickWandTerminus;
use std::os;

fn print_usage(name: &str, _opts: &[Opt]) {
  println!("Usage: {:s} input_file [options]", name);
  println!("-o\t\toutput_file");
  println!("-h --help\tUsage");
}

fn main() {
  let args = os::args();
  let program_name = args[0].clone();
  let opts = ~[ optopt("o"), optflag("h"), optflag("help") ];
  let matches = match getopts(args.tail(), opts) {
    Ok(m) => { m }
    Err(f) => { fail!(f.to_err_msg()) }
  };

  // Detect a help request
  if matches.opt_present("h") || matches.opt_present("help") {
    print_usage(program_name, opts);
    return;
  }
  // Ensure that at least the input file is specified
  let input = if !matches.free.is_empty() {
    matches.free[0].clone()
  } else {
    print_usage(program_name, opts);
    return;
  };

  let output = match matches.opt_str("o") {
    Some(path) => path.clone(),
    None => generate_default_output_filename(input)
  };

  thumbnail_it(input, output);
}

fn generate_default_output_filename(_input: &str) -> ~str {
  // TODO: Make this replace everything after the last dot occurring in
  // input rather than just using this hardcoded default
  // Meaning, I'd rather have turkey-thumbnail.png than thumbnail.png
  ~"thumbnail.png"
}

fn thumbnail_it(inbound: ~str, outbound: ~str) {
  MagickWandGenesis();

  do MagickWand::borrow |wand| {
    wand.read_image(inbound);
    wand.reset_iterator();
    wand.each_image(|| {
      wand.resize_image(106, 80, LanczosFilter, 1.0);
    });
    wand.write_images(outbound, true);
  };

  MagickWandTerminus();
}
