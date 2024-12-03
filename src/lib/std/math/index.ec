export package math {
//    export function cos(x: F64) -> F64
//    export function cos(x: F32) -> F32
    export function cos(x: Number) -> Number {
        core::intrinsics::math::cos_f64(x)
    }
}