type Test_Result (
    description: String,
    passed: Bool,
    describe_results: List
)

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

fun test(description: String, body: fun()) {
    let test_result = Test_Result(description = description, passed = false, describe_results = std::collection::list::empty() )
    results.append( test_result )
    body()
}

fun describe(description: String, body: fun()) {
    let describe_result = Describe_Result( passed = false, it_results = std::collection::list::empty() )
    let test_result = results.get(1)
    std::io::print_line(test_result)

    // FIXME
    // test_result.describe_results.append(describe_result)
    let temp = test_result.describe_results
    temp.append(describe_result)

    body()
}

fun should(description: String, body: fun() -> Bool){
    let test_result = results.get(1)
    let temp = test_result.describe_results
    let describe_result = temp.get(1)
    
    let temp = describe_result.it_results

    // measure time
    let passed = body()

    if passed{
        std::io::print('Pass ')
    }else {
        std::io::print('Fail ')
    }

    std::io::print_line(description)
    // took - xyz ms
//
//    if passwd {
//        std::io::print_line('Test passed')
//        let result = It_Result(description = description, passed = true)
//        temp.append(result)
//    } else {
//        std::io::print_line('Test failed')
//        let result = It_Result(description = description, passed = false)
//        temp.append(result)
//    }
}

fun print_summary(){
    std::io::print_line('All in all...')
    std::io::print_line(results.length())

    let temp = results.get(1)
    let groups = temp.describe_results
    std::io::print_line(groups.length())

    let group = groups.get(1)
    let its = group.it_results
    std::io::print_line(its.length())

    let it = its.get(1)
    if it.passed{
        std::io::print('Pass ')
    }else {
        std::io::print('Fail ')
    }

    std::io::print_line(it.description)
}

fun a(){
    std::io::print_line('I will pass')
    return true
}

fun b(){
    std::io::print_line('I will fail')
    return false
}


fun two(){
//  should('test one', a)
    should('some failing test',  b)
}

fun one(){
    describe('desc', two)
}

test('test', one)

//print_summary()

// std::io::print_line(result.passing_count)
// std::io::print_line(result.failing_count)