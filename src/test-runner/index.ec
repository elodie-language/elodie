function println(message: String) {
    rt::io::println(message)
}

function print(message: String) {
    std::io::print(message)
}

type Test_Result (
    name: String,
    passed: Bool,
    describe_results: List
)

define Test_Result {
    function summarize(){
//        println('All in all...')
    }
}

type Describe_Result (
    description: String,
    passed: Bool,
    it_results: List
)

type It_Result (
    description: String,
    passed: Bool
)

let results = std::collection::list::empty()

function test(name: String, body: function()) {
    println(' ' + name)
    let test_result = Test_Result(name = name, passed = false, describe_results = std::collection::list::empty() )
    results.append( test_result )
    body()
    test_result.summarize()
}

function describe(description: String, body: function()) {
    println('  ' + description)


    let describe_result = Describe_Result( passed = false, it_results = std::collection::list::empty() )
    let test_result = results.get(1)
//    rt::io::println(test_result)

    // FIXME
    // test_result.describe_results.append(describe_result)
    let temp = test_result.describe_results
    temp.append(describe_result)

    body()
}


function check(description: String, body: function() -> Bool){
    let test_result = results.get(1)
    let temp = test_result.describe_results
    let describe_result = temp.get(1)

    let temp = describe_result.it_results

    // measure time
    let passed = body()

    if passed{
        print('    \x1b[0;32mPass\x1b[0m -')
    } else {
        print('    \x1b[0;31mFail\x1b[0m -')
        intrinsics.report_test_failure()
    }

    rt::io::println(description)
    // took - xyz ms
//
//    if passwd {
//        rt::io::println('Test passed')
//        let result = It_Result(description = description, passed = true)
//        temp.append(result)
//    } else {
//        rt::io::println('Test failed')
//        let result = It_Result(description = description, passed = false)
//        temp.append(result)
//    }
}

