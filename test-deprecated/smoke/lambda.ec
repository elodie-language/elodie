function receiver(fn: function()){
    fn()
}

receiver() {
    std::io::println('lambda')
}

// out:lambda
