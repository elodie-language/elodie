export package io {

    export function print(s: String) {
        sysroot::rt::io::print(s)
    }

    export function println(s: String) {
        print(s)
        print('\n')
    }
}