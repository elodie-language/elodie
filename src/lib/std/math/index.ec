export package math {
//    export fun cos(x: F64) -> F64
//    export fun cos(x: F32) -> F32
    export fun cos(x: Number) -> Number {
        core::intrinsics::math::cos_f64(x)
    }
}