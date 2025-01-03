test('call function with no arguments'){
    describe('invokes function with ()'){
        function test_function() -> Bool{ true }
        check('function was called') { test_function() }
    }
}

test('call function with single argument'){
    describe('call function with (99)'){
        function test_function(arg_1: Number) -> Bool{
          check('arg_1 == 99') { arg_1 == 99 }
          true
        }
        check('function was called') { test_function( 99 ) }
    }
}

test('call function with multiple arguments'){
    describe('call function with (100,0)'){
        function test_function(arg_1: Number, arg_2: Number) -> Bool{
          check('arg_1 == 100') { arg_1 == 100 }
          check('arg_2 ==   0') { arg_2 == 0 }
          true
        }
        check('function was called') { test_function( 100, 0) }
    }
}