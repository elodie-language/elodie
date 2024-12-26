function receiver(fn: function()){
    fn()
}

receiver() {
    rt::io::println('lambda')
}

// out:lambda
