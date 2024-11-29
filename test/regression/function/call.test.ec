test('call function with no arguments'){
    describe('invokes function with ()'){
        fun test_function() -> Bool{ true }
        should('function was called') { test_function() }
    }
}

test('call function with single argument'){
    describe('call function with (99)'){
        fun test_function(arg_1: Number) -> Bool{
          should('arg_1 == 99') { arg_1 == 99 }
          true
        }
        should('function was called') { test_function( 99 ) }
    }
}

test('call function with multiple arguments'){
    describe('call function with (100,0)'){
        fun test_function(arg_1: Number, arg_2: Number) -> Bool{
          should('arg_1 == 100') { arg_1 == 100 }
          should('arg_2 ==   0') { arg_2 == 0 }
          true
        }
        should('function was called') { test_function( 100, 0) }
    }
}