export package io {

    export function print(s: String) {
        rt::intrinsics::io::print(s)
    }

    export function println(s: String) {
        print(s)
        print('\n')
    }
}