test('List') {

    describe('empty()') {
        describe('A newly created empty list') {
            let list = std::collection::list::empty()
            should('has a length of 0') { list.length() == 0 }
        }
    }

    describe('append()'){
        describe('To an empty list'){
            let list = std::collection::list::empty()
            describe('Append 2'){
                list.append(2)
                should('List has size of 1') { list.length() == 1 }
                should('1st element is 2') { list.get(1) == 2 }
            }
            describe('Append 4'){
                list.append(4)
                should('list has size of 2') { list.length() == 2 }
                should('1st element is 2') { list.get(1) == 2 }
                should('2nd second element is 4') { list.get(2) == 4 }
            }
            describe('Append 4'){
                list.append(4)
                should('List has size of 3') { list.length() == 3 }
                should('1st element is 2') { list.get(1) == 2 }
                should('2nd element is 4') { list.get(2) == 4 }
                should('3rd element is 4') { list.get(3) == 4 }
            }
            describe('Append 10'){
                list.append(10)
                should('List has size of 4') { list.length() == 4 }
                should('1st element is 2') { list.get(1) == 2 }
                should('2nd element is 4') { list.get(2) == 4 }
                should('3rd element is 4') { list.get(3) == 4 }
                should('4th element is 10') { list.get(4) == 10 }
            }
        }
    }
}