//external function cos(x: Float64) -> Float64

// libc::math::cos()
// ffi::c::math::cos()
// std::math::cos

let result = core::intrinsics::math::cos_f64(1)
rt::io::println(result)

// out: 1