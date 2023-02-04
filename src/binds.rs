use autocxx::prelude::*;

include_cpp!(
    #include "open3d_wrapper.h"
    safety!(unsafe_ffi)
    generate!("test_fn")
);


pub use ffi::test_fn;