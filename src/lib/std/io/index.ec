export package io {

    export function print(message: String) {
        core::intrinsics::io::print(message)
    }

    export function print_line(message: String) {
        print(message)
        print('\n')
    }
}