export package intrinsics {
// FIXME move into sub package once compiler supports that
//       from './math' export math
    export package math {
        export external fun cos_f64(x: F64) -> F64
    }
}