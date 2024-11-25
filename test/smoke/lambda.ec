fun receiver(fn: fun()){
    fn()
}

receiver() {
    std::io::print_line('lambda')
}

// out:lambda
