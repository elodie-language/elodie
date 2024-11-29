test('Type formatting') {
    describe('Type declaration can have comments'){
        type Some_Type (
            // comment before
            prop_1: Bool
            // comment in between 1
            // comment in between 2
            prop_2: Number
            // comment after
        )

        describe('Instantiation with Some_Type(prop_1 = true, prop_2 = 2)') {
            let t = Some_Type( prop_1 = true, prop_2 = 2 )
            should('t.prop_1 == true') { t.prop_1 == true }
            should('t.prop_2 == 2') { t.prop_2 == 2 }
        }
    }
}
