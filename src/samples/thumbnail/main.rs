/*!
 * The first code example from http://www.imagemagick.org/script/magick-wand.php
 * 
 * This program reads an image, creates a thumbnail, and writes out the result.
 * Usage:
 *    ./thumbnail inputfile [-o outputfile]
 */
extern mod extra;
extern mod wand_of_rust;

use extra::getopts::{ Opt, optopt, optflag };
use wand_of_rust::{ MagickWand, LanczosFilter, MagickWandGenesis, MagickWandTerminus };
use std::os;

fn print_usage(name: &str, _opts: &[Opt]) {
  println(format!("Usage: {:s} input_file [options]", name));
  println("-o\t\toutput_file");
  println("-h --help\tUsage");
}

fn main() {
  let args = os::args();
  let program_name = args[0].clone();
  let opts = ~[ optopt("o"), optflag("h"), optflag("help") ];
  let matches = match getopts(args.tail(), opts) {
    Ok(m) => { m }
    Err(f) => { fail!(fail_str(f)) }
  };
  // Detect a help request
  if opt_present(&matches, "h") || opt_present(&matches, "help") {
    print_usage(program_name, opts);
    return;
  }
  // Ensure that at least the input file is specified
  let input: &str = match matches.free {
    [head, _] => head,
    _ => {
      print_usage(program_name, opts);
      return;
    }
  };
  
  let output = match opt_maybe_str(&matches, "o") {
    Some(path) => path,
    None => generate_default_output_filename(input)
  };

  thumbnail_it(input, output);
}

fn generate_default_output_filename(input: &str) -> ~str {
  // see if the filename has a period in it. If so, I assume that
  // the last occurance of the period denotes the file extension
  let append_index = match std::str::rfind_char(input, '.') {
    Some(index) => index,
    None => input.len()
  };
  // TODO: Figure out wtf with these boxes
  std::str::concat_slices(&[
    std::str::slice(input, 0, append_index),
    &"-thumbnail.png"
  ])
}

fn thumbnail_it(inbound: &str, outbound: &str) {
  MagickWandGenesis();

  do MagickWand::borrow |wand| {
    wand.read_image(inbound);
    wand.reset_iterator();
    do wand.each_image {
      wand.resize_image(106, 80, LanczosFilter, 1.0);
    };
    wand.write_images(outbound, true);
  };

  MagickWandTerminus();
}
