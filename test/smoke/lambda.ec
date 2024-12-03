function receiver(fn: function()){
    fn()
}

receiver() {
    std::io::print_line('lambda')
}

// out:lambda
