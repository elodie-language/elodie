export package list {

    export type List()

    export fun empty() -> List {
        return List()
    }

   export define List {

        fun append(value) {
             intrinsics.list_append(self, value)
        }

        fun get(idx) {
            return intrinsics.list_get(self, idx)
        }

        fun length() -> Number {
            return intrinsics.list_length(self)
        }

   }
}