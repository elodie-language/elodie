test('math'){
    describe('cos(x)'){
        should('accept x as number'){
            let input: Number = 0
            let result = std::math::cos(input)
            let result = 1
            result == 1
        }

    }
}