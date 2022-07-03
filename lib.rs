#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]


extern crate libc;
pub mod src {
pub mod src {
pub mod api;
pub mod dumper;
pub mod emitter;
pub mod loader;
pub mod parser;
pub mod reader;
pub mod scanner;
pub mod writer;
} // mod src
pub mod tests {
pub mod example_deconstructor;
pub mod example_deconstructor_alt;
pub mod example_reformatter;
pub mod example_reformatter_alt;
pub mod run_dumper;
pub mod run_emitter;
pub mod run_emitter_test_suite;
pub mod run_loader;
pub mod run_parser;
pub mod run_parser_test_suite;
pub mod run_scanner;
pub mod test_reader;
pub mod test_version;
} // mod tests
} // mod src
