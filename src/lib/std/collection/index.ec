export package collection {
    from './list' export list

    export fun empty_list() -> List {
        return std::collection::list::empty()
    }
}