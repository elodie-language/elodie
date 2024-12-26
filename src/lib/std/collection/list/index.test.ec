test('List') {

    describe('empty()') {
        describe('A newly created empty list') {
            let list = std::collection::list::empty()
            check('has a length of 0') { list.length() == 0 }
        }
    }

    describe('append()'){
        describe('To an empty list'){
            let list = std::collection::list::empty()
            describe('Append 2'){
                list.append(2)
                check('List has size of 1') { list.length() == 1 }
                check('1st element is 2') { list.get(1) == 2 }
            }
            describe('Append 4'){
                list.append(4)
                check('list has size of 2') { list.length() == 2 }
                check('1st element is 2') { list.get(1) == 2 }
                check('2nd second element is 4') { list.get(2) == 4 }
            }
            describe('Append 4'){
                list.append(4)
                check('List has size of 3') { list.length() == 3 }
                check('1st element is 2') { list.get(1) == 2 }
                check('2nd element is 4') { list.get(2) == 4 }
                check('3rd element is 4') { list.get(3) == 4 }
            }
            describe('Append 10'){
                list.append(10)
                check('List has size of 4') { list.length() == 4 }
                check('1st element is 2') { list.get(1) == 2 }
                check('2nd element is 4') { list.get(2) == 4 }
                check('3rd element is 4') { list.get(3) == 4 }
                check('4th element is 10') { list.get(4) == 10 }
            }
        }
    }
}