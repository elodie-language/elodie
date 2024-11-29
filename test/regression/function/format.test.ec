test('Function formatting') {
    describe('A function can have comments before and after last expression'){
        fun some_fn() {
          // comment before
          42
          // comment after
        }
        should('Returns last expression'){ some_fn() == 42 }
    }

    describe('A function can have comments before and after return expression'){
        fun some_fn() {
          // comment before
          return 9924
          // comment after
        }
        should('Returns last expression'){ some_fn() == 9924 }
    }
}


