export package list {

    export type List()

    export function empty() -> List {
        return List()
    }

   export define List {

        function append(value) {
             intrinsics.list_append(self, value)
        }

        function get(idx) {
            return intrinsics.list_get(self, idx)
        }

        function length() -> Number {
            return intrinsics.list_length(self)
        }

   }
}