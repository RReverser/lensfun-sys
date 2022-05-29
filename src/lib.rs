#![allow(rustdoc::bare_urls)]

use autocxx::prelude::*;

include_cpp! {
    #include "lensfun.h"
    safety!(unsafe_ffi)
    generate!("lfMount")
    generate!("lfCamera")
    generate!("lfLens")
    generate!("lfDatabase")
    generate!("lfModifier")
    extern_cpp_opaque_type!("lfError", crate::substitutes::lfError)
}

mod substitutes {
    #[must_use]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    /// liblensfun error codes: negative codes are -errno, positive are here
    pub struct lfError {
        pub code: i32,
    }

    impl lfError {
        const NO_ERROR: Self = Self { code: 0 };
        const WRONG_FORMAT: Self = Self { code: 1 };
        const NO_DATABASE: Self = Self { code: 2 };

        pub fn as_result(self) -> std::io::Result<()> {
            Err(match self {
                Self::NO_ERROR => return Ok(()),
                Self::WRONG_FORMAT => {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "wrong format")
                }
                Self::NO_DATABASE => {
                    std::io::Error::new(std::io::ErrorKind::NotFound, "no database")
                }
                Self { code } if code < 0 => std::io::Error::from_raw_os_error(-code),
                Self { code } => std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("unknown error code: {}", code),
                ),
            })
        }
    }

    unsafe impl cxx::ExternType for lfError {
        type Id = cxx::type_id!("lfError");
        type Kind = cxx::kind::Trivial;
    }
}

pub use ffi::*;
