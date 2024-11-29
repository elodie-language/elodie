export package io {

    export fun print(message: String) {
        core::intrinsics::io::print(message)
    }

    export fun print_line(message: String) {
        print(message)
        print('\n')
    }
}