export package io {

    export function print(s: String) {
        core::intrinsics::io::print(s)
    }

    export function println(s: String) {
        print(s)
        print('\n')
    }
}