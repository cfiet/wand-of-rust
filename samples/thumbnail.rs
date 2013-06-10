/*!
 * The first code example from http://www.imagemagick.org/script/magick-wand.php
 * 
 * This program reads an image, creates a thumbnail, and writes out the result.
 * Usage:
 *    ./thumbnail inputfile [-o outputfile]
 */
extern mod extra;
extern mod rustwand;

use extra::getopts::*;
use rustwand::*;
use std::os;

fn print_usage(name: &str, _opts: &[Opt]) {
  println(fmt!("Usage: %s input_file [options]", name));
  println("-o\t\toutput_file");
  println("-h --help\tUsage");
}

fn main() {
  let args = os::args();
  let program_name = copy args[0];
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
  let output = opt_maybe_str(&matches, "o");
  // Ensure that at least the input file is specified
  let input: &str = if !matches.free.is_empty() { 
    copy matches.free[0]
  } else {
    print_usage(program_name, opts);
    return;
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

fn thumbnail_it(in: &str, out: Option<~str>) {
  let out_file = match out {
   None => generate_default_output_filename(in),
   Some(filename) => filename
  };

  MagickWandGenesis();

  MagickWand::borrow( |wand| {
    wand.read_image(in);
    wand.reset_iterator();
    wand.each_image(|| { // <-- this is ugly
      wand.resize_image(106, 80, LanczosFilter, 1.0);
    });
    wand.write_images(out_file, true);
  });

  MagickWandTerminus();
}
