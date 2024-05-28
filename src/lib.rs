
//! # LibYML
//!
//! [`LibYML`][00] is a Rust library for parsing and emitting YAML data in a safe and efficient manner. The library is designed to be easy to use and provides a comprehensive set of tools for working with YAML data.
//!
//! [![LibYML Logo](https://kura.pro/libyml/images/banners/banner-libyml.svg)](https://libyml.one "LibYML: Unleashing the Power of Safe and Efficient YAML Parsing in Rust")
//!
//! ## Seamless YAML Serialization for [Rust][01].
//!
//! [![Made With RustRust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//! [![Crates.io](https://img.shields.io/crates/v/libyml.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/libyml "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.3-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/libyml "Lib.rs")
//! [![Github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/sebastienrousseau/libyml "Github")
//! [![License](https://img.shields.io/crates/l/libyml.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](https://opensource.org/license/apache-2-0/ "MIT or Apache License, Version 2.0")
//!
//! ## Features
//!
//! - **Serialization and Deserialization**: LibYML provides easy-to-use APIs for serializing Rust structs and enums into YAML format and deserializing YAML data into Rust types.
//! - **Custom Struct and Enum Support**: The library allows you to work with custom structs and enums, enabling seamless serialization and deserialization of your own data types.
//! - **Error Handling**: LibYML provides comprehensive error handling mechanisms, including detailed error messages and the ability to handle and recover from parsing and emission errors.
//! - **Streaming Support**: The library supports streaming of YAML data, allowing you to efficiently process large YAML documents incrementally.
//! - **Alias and Anchor Support**: LibYML handles YAML aliases and anchors, enabling you to work with complex YAML structures that involve references and duplicated data.
//! - **Tag Handling**: The library provides support for custom tags, allowing you to serialize and deserialize YAML data with specific type information.
//! - **Configurable Emitter**: LibYML allows you to customize the emitter settings, such as indentation, line width, and scalar style, to generate YAML output according to your preferences.
//! - **Extensive Documentation**: The library comes with detailed documentation and examples, making it easy to get started and understand how to use its various features effectively.
//! - **Safety and Efficiency**: LibYML is designed with safety and efficiency in mind. It minimizes the use of unsafe code and provides an interface that helps prevent common pitfalls and errors.
//!
//! ## Rust Version Compatibility
//!
//! This library is compatible with Rust 1.60 and above.
//!
//! [00]: https://libyml.com "LibYML: Unleashing the Power of Safe and Efficient YAML Parsing in Rust"
//! [01]: https://www.rust-lang.org "Rust Programming Language"
//!

#![no_std]
#![doc(
    html_favicon_url = "https://kura.pro/libyml/images/favicon.ico",
    html_logo_url = "https://kura.pro/libyml/images/logos/libyml.svg",
    html_root_url = "https://docs.rs/libyml"
)]
#![crate_name = "libyml"]
#![crate_type = "lib"]

extern crate alloc;

use core::mem::size_of;

mod libc {
    pub(crate) use core::ffi::c_void;
    pub(crate) use core::primitive::{
        i32 as c_int, i64 as c_long, i8 as c_char, u32 as c_uint,
        u64 as c_ulong, u8 as c_uchar,
    };
}

#[macro_use]
mod externs {
    use crate::libc;
    use crate::ops::{die, ForceAdd as _, ForceInto as _};
    use alloc::alloc::{self as rust, Layout};
    use core::mem::{self, MaybeUninit};
    use core::ptr;
    use core::slice;

    const HEADER: usize = {
        let need_len = mem::size_of::<usize>();
        // Round up to multiple of MALLOC_ALIGN.
        (need_len + MALLOC_ALIGN - 1) & !(MALLOC_ALIGN - 1)
    };

    // `max_align_t` may be bigger than this, but libyaml does not use `long
    // double` or u128.
    const MALLOC_ALIGN: usize = {
        let int_align = mem::align_of::<libc::c_ulong>();
        let ptr_align = mem::align_of::<usize>();
        if int_align >= ptr_align {
            int_align
        } else {
            ptr_align
        }
    };

    pub(crate) unsafe fn malloc(
        size: libc::c_ulong,
    ) -> *mut libc::c_void {
        let size = HEADER.force_add(size.force_into());
        let layout = Layout::from_size_align(size, MALLOC_ALIGN)
            .ok()
            .unwrap_or_else(die);
        let memory = rust::alloc(layout);
        if memory.is_null() {
            rust::handle_alloc_error(layout);
        }
        memory.cast::<usize>().write(size);
        memory.add(HEADER).cast()
    }

    pub(crate) unsafe fn realloc(
        ptr: *mut libc::c_void,
        new_size: libc::c_ulong,
    ) -> *mut libc::c_void {
        let mut memory = ptr.cast::<u8>().sub(HEADER);
        let size = memory.cast::<usize>().read();
        let layout =
            Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        let new_size = HEADER.force_add(new_size.force_into());
        let new_layout =
            Layout::from_size_align(new_size, MALLOC_ALIGN)
                .ok()
                .unwrap_or_else(die);
        memory = rust::realloc(memory, layout, new_size);
        if memory.is_null() {
            rust::handle_alloc_error(new_layout);
        }
        memory.cast::<usize>().write(new_size);
        memory.add(HEADER).cast()
    }

    pub(crate) unsafe fn free(ptr: *mut libc::c_void) {
        let memory = ptr.cast::<u8>().sub(HEADER);
        let size = memory.cast::<usize>().read();
        let layout =
            Layout::from_size_align_unchecked(size, MALLOC_ALIGN);
        rust::dealloc(memory, layout);
    }

    pub(crate) unsafe fn memcmp(
        lhs: *const libc::c_void,
        rhs: *const libc::c_void,
        count: libc::c_ulong,
    ) -> libc::c_int {
        let lhs =
            slice::from_raw_parts(lhs.cast::<u8>(), count as usize);
        let rhs =
            slice::from_raw_parts(rhs.cast::<u8>(), count as usize);
        lhs.cmp(rhs) as libc::c_int
    }

    pub(crate) unsafe fn memcpy(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        count: libc::c_ulong,
    ) -> *mut libc::c_void {
        ptr::copy_nonoverlapping(
            src.cast::<MaybeUninit<u8>>(),
            dest.cast::<MaybeUninit<u8>>(),
            count as usize,
        );
        dest
    }

    pub(crate) unsafe fn memmove(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        count: libc::c_ulong,
    ) -> *mut libc::c_void {
        ptr::copy(
            src.cast::<MaybeUninit<u8>>(),
            dest.cast::<MaybeUninit<u8>>(),
            count as usize,
        );
        dest
    }

    pub(crate) unsafe fn memset(
        dest: *mut libc::c_void,
        ch: libc::c_int,
        count: libc::c_ulong,
    ) -> *mut libc::c_void {
        ptr::write_bytes(dest.cast::<u8>(), ch as u8, count as usize);
        dest
    }

    pub(crate) unsafe fn strcmp(
        lhs: *const libc::c_char,
        rhs: *const libc::c_char,
    ) -> libc::c_int {
        let lhs = slice::from_raw_parts(
            lhs.cast::<u8>(),
            strlen(lhs) as usize,
        );
        let rhs = slice::from_raw_parts(
            rhs.cast::<u8>(),
            strlen(rhs) as usize,
        );
        lhs.cmp(rhs) as libc::c_int
    }

    pub(crate) unsafe fn strdup(
        src: *const libc::c_char,
    ) -> *mut libc::c_char {
        let len = strlen(src);
        let dest = malloc(len + 1);
        memcpy(dest, src.cast(), len + 1);
        dest.cast()
    }

    pub(crate) unsafe fn strlen(
        str: *const libc::c_char,
    ) -> libc::c_ulong {
        let mut end = str;
        while *end != 0 {
            end = end.add(1);
        }
        end.offset_from(str) as libc::c_ulong
    }

    pub(crate) unsafe fn strncmp(
        lhs: *const libc::c_char,
        rhs: *const libc::c_char,
        mut count: libc::c_ulong,
    ) -> libc::c_int {
        let mut lhs = lhs.cast::<u8>();
        let mut rhs = rhs.cast::<u8>();
        while count > 0 && *lhs != 0 && *lhs == *rhs {
            lhs = lhs.add(1);
            rhs = rhs.add(1);
            count -= 1;
        }
        if count == 0 {
            0
        } else {
            (*lhs).cmp(&*rhs) as libc::c_int
        }
    }

    macro_rules! __assert {
        (false $(,)?) => {
            $crate::externs::__assert_fail(
                stringify!(false),
                file!(),
                line!(),
            )
        };
        ($assertion:expr $(,)?) => {
            if !$assertion {
                $crate::externs::__assert_fail(
                    stringify!($assertion),
                    file!(),
                    line!(),
                );
            }
        };
    }

    pub(crate) unsafe fn __assert_fail(
        __assertion: &'static str,
        __file: &'static str,
        __line: u32,
    ) -> ! {
        struct Abort;
        impl Drop for Abort {
            fn drop(&mut self) {
                panic!();
            }
        }
        let _abort_on_panic = Abort;
        panic!(
            "{}:{}: Assertion `{}` failed.",
            __file, __line, __assertion
        );
    }
}

mod fmt {
    use crate::yaml::yaml_char_t;
    use core::fmt::{self, Write};
    use core::ptr;

    pub(crate) struct WriteToPtr {
        ptr: *mut yaml_char_t,
    }

    impl WriteToPtr {
        pub(crate) unsafe fn new(ptr: *mut yaml_char_t) -> Self {
            WriteToPtr { ptr }
        }

        pub(crate) fn write_fmt(&mut self, args: fmt::Arguments<'_>) {
            let _ = Write::write_fmt(self, args);
        }
    }

    impl Write for WriteToPtr {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            unsafe {
                ptr::copy_nonoverlapping(s.as_ptr(), self.ptr, s.len());
                self.ptr = self.ptr.add(s.len());
            }
            Ok(())
        }
    }
}

trait PointerExt: Sized {
    fn c_offset_from(self, origin: Self) -> isize;
}

impl<T> PointerExt for *const T {
    fn c_offset_from(self, origin: *const T) -> isize {
        (self as isize - origin as isize) / size_of::<T>() as isize
    }
}

impl<T> PointerExt for *mut T {
    fn c_offset_from(self, origin: *mut T) -> isize {
        (self as isize - origin as isize) / size_of::<T>() as isize
    }
}

#[macro_use]
mod macros;

pub mod api;
mod dumper;
mod emitter;
mod loader;
mod ops;
mod parser;
mod reader;
mod scanner;
mod success;
mod writer;
pub mod yaml;

pub use crate::api::{
    yaml_alias_event_initialize, yaml_document_add_mapping,
    yaml_document_add_scalar, yaml_document_add_sequence,
    yaml_document_append_mapping_pair,
    yaml_document_append_sequence_item, yaml_document_delete,
    yaml_document_end_event_initialize, yaml_document_get_node,
    yaml_document_get_root_node, yaml_document_initialize,
    yaml_document_start_event_initialize, yaml_emitter_delete,
    yaml_emitter_initialize, yaml_emitter_set_break,
    yaml_emitter_set_canonical, yaml_emitter_set_encoding,
    yaml_emitter_set_indent, yaml_emitter_set_output,
    yaml_emitter_set_output_string, yaml_emitter_set_unicode,
    yaml_emitter_set_width, yaml_event_delete,
    yaml_mapping_end_event_initialize,
    yaml_mapping_start_event_initialize, yaml_parser_delete,
    yaml_parser_initialize, yaml_parser_set_encoding,
    yaml_parser_set_input, yaml_parser_set_input_string,
    yaml_scalar_event_initialize, yaml_sequence_end_event_initialize,
    yaml_sequence_start_event_initialize,
    yaml_stream_end_event_initialize,
    yaml_stream_start_event_initialize, yaml_token_delete,
};
pub use crate::dumper::{
    yaml_emitter_close, yaml_emitter_dump, yaml_emitter_open,
};
pub use crate::emitter::yaml_emitter_emit;
pub use crate::loader::yaml_parser_load;
pub use crate::parser::yaml_parser_parse;
pub use crate::scanner::yaml_parser_scan;
pub use crate::writer::yaml_emitter_flush;
pub use crate::yaml::{
    YamlAliasDataT, YamlBreakT, YamlDocumentT, YamlEmitterStateT,
    YamlEmitterT, YamlEncodingT, YamlErrorTypeT, YamlEventT,
    YamlEventTypeT, YamlMappingStyleT, YamlMarkT, YamlNodeItemT,
    YamlNodePairT, YamlNodeT, YamlNodeTypeT, YamlParserStateT,
    YamlParserT, YamlReadHandlerT, YamlScalarStyleT,
    YamlSequenceStyleT, YamlSimpleKeyT, YamlStackT, YamlTagDirectiveT,
    YamlTokenT, YamlTokenTypeT, YamlVersionDirectiveT,
    YamlWriteHandlerT,
};
#[doc(hidden)]
pub use crate::yaml::{
    YamlBreakT::*, YamlEmitterStateT::*, YamlEncodingT::*,
    YamlErrorTypeT::*, YamlEventTypeT::*, YamlMappingStyleT::*,
    YamlNodeTypeT::*, YamlParserStateT::*, YamlScalarStyleT::*,
    YamlSequenceStyleT::*, YamlTokenTypeT::*,
};
