use autocxx::prelude::*;

include_cpp! {
    #include "lensfun.h"
    safety!(unsafe_ffi)
    generate!("lfMount")
    generate!("lfCamera")
    generate!("lfLens")
    generate!("lfDatabase")
    generate!("lfModifier")
}
