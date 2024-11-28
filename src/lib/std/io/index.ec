export package io {
    export fun print(message: String) {
        ec_io_print(message)
    }

    export fun print_line(message: String) {
        print(message)
        print('\n')
    }
}