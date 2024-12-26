test('intrinsics'){
    describe('math'){
        check('cos_f64'){
           core::intrinsics::math::cos_f64(0) == 1
        }
    }
}