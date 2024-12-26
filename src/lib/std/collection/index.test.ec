test('collection') {
    describe('empty_list()') {
        describe('A newly created empty list') {
            let list = std::collection::empty_list()
            check('have a length of 0') { list.length() == 0 }
        }
    }
}