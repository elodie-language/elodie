test('math'){
    describe('cos(x)'){
        describe('Number'){
            should('cos(0) == 1'){
                let input: Number = 0
                let result = std::math::cos(input)
                result == 1
            }
        }
        describe('F32'){
            should('cos(0) == 1'){
                let input: F32 = 0
                let result = std::math::cos(input)
                result == 1
            }
        }
        describe('F64'){
            should('cos(0) == 1'){
                let input: F64 = 0
                let result = std::math::cos(input)
                result == 1
            }
        }
    }
}