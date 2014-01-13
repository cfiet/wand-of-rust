extern mod std;
use std::option::Option;
use std::libc::c_uint;

#[path="bindings.rs"]
mod bindings;

// TODO: Figure out a way to expose the channel constants from
// http://www.imagemagick.org/api/MagickCore/magick-type_8h_source.html#l00176
// I can't use enums, as there are duplicate values. Additionally, one of the 
// enum values is defined in terms of bitwise operations on other values - this
// is a no-no in Rust.

/**
 * Represents some set of image color channels. 
 */
pub struct Channels {
  bitflags: c_uint
}

impl Channels {
  /**
   * Create a Channels instance from a channel spec string, such as "rgba".
   * For a comprehensive list of accepted values, see:
   *  http://www.imagemagick.org/api/MagickCore/option_8c_source.html#l02282
   */
  pub fn from_str(channel_spec: &str) -> Option<Channels> {
    let bitflags = channel_spec.with_c_str(|buffer| {
      unsafe {
        bindings::ParseChannelOption(buffer) as c_uint
      }
    });
    match bitflags {
      -1 => None,
      flags => Some(Channels { bitflags: flags })
    }
  }

}
