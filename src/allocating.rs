use alloc::string::String;

use crate::traits::*;
use crate::options::FormatSizeOptions;
use crate::IFormatter;

pub fn format_size_i(input: impl ToF64, options: impl AsRef<FormatSizeOptions>) -> String {
  format!("{}", IFormatter{value: input, options: options})
}

pub fn format_size(input: impl ToF64 + Unsigned, options: impl AsRef<FormatSizeOptions>) -> String {
  format_size_i(input, &options)
}

pub fn make_format_i<T: ToF64>(options: impl AsRef<FormatSizeOptions>) -> impl Fn(T) -> String {
  return move |val | -> String {
      format_size_i(val, &options)
  }
}

pub fn make_format<T: ToF64 + Unsigned>(options: impl AsRef<FormatSizeOptions>) -> impl Fn(T) -> String {
  return make_format_i(options);
}