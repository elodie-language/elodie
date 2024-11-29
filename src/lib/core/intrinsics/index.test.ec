test('intrinsics'){
    describe('math'){
        should('cos_f64'){
           core::intrinsics::math::cos_f64(0) == 1
        }
    }
}