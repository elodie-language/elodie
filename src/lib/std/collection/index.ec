export package collection {
    from './list' export list

    export function empty_list() -> List {
        return std::collection::list::empty()
    }
}