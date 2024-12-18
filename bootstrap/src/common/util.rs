pub fn is_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_ascii_lowercase() {
            return false;
        }
    }

    let mut previous_char = None;
    for c in chars {
        if c == '_' {
            if previous_char == Some('_') {
                return false;
            }
        } else if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
            return false;
        }
        previous_char = Some(c);
    }

    !s.ends_with('_')
}

pub fn is_pascal_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_ascii_uppercase() {
            return false;
        }
    }

    let mut previous_char = None;
    for c in chars {
        if c == '_' {
            if previous_char == Some('_') {
                return false;
            }
        } else if previous_char == Some('_') {
            if !c.is_ascii_uppercase() && !c.is_ascii_digit() {
                return false;
            }
        } else if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
            return false;
        }
        previous_char = Some(c);
    }

    !s.ends_with('_')
}

#[cfg(test)]
mod tests {
    mod is_snake_case {
        use crate::common::util::is_snake_case;

        #[test]
        fn ok() {
            assert!(is_snake_case("elodie"));
            assert!(is_snake_case("this_is_snake_case"));
            assert!(is_snake_case("simple_example"));
            assert!(is_snake_case("one_word"));
            assert!(is_snake_case("with_numbers_123"));
        }

        #[test]
        fn invalid_capitals() {
            assert!(!is_snake_case("This_Is_Not_Snake_Case"));
            assert!(!is_snake_case("notSnakeCase"));
            assert!(!is_snake_case("snake_Case_With_Capitals"));
        }

        #[test]
        fn invalid_symbols() {
            assert!(!is_snake_case("not snake case"));
            assert!(!is_snake_case("snake-case-with-dashes"));
            assert!(!is_snake_case("snake@case!"));
        }

        #[test]
        fn invalid_numbers() {
            assert!(!is_snake_case("123snake_case"));
        }

        #[test]
        fn empty_or_edge_cases() {
            assert!(!is_snake_case(""));
            assert!(!is_snake_case("_leading_underscore"));
            assert!(!is_snake_case("trailing_underscore_"));
            assert!(!is_snake_case("double__underscore"));
        }
    }

    mod is_pascal_snake_case {
        use crate::common::util::is_pascal_snake_case;

        #[test]
        fn ok() {
            assert!(is_pascal_snake_case("Elodie_C"));
            assert!(is_pascal_snake_case("This_Is_Pascal_Snake_Case"));
            assert!(is_pascal_snake_case("With_Numbers_123"));
        }

        #[test]
        fn invalid_capitals() {
            assert!(!is_pascal_snake_case("this_Is_Not_Pascal_Snake_Case"));
            assert!(!is_pascal_snake_case("Pascal_snake_Case"));
            assert!(!is_pascal_snake_case("Pascal_Snake_case"));
        }

        #[test]
        fn invalid_symbols() {
            assert!(!is_pascal_snake_case("Pascal Snake Case"));
            assert!(!is_pascal_snake_case("Pascal-Snake-Case"));
            assert!(!is_pascal_snake_case("Pascal@Case!"));
            assert!(!is_pascal_snake_case("SingleWord"));
        }

        #[test]
        fn invalid_numbers() {
            assert!(!is_pascal_snake_case("123Pascal_Snake_Case"));
        }

        #[test]
        fn empty_or_edge_cases() {
            assert!(!is_pascal_snake_case(""));
            assert!(!is_pascal_snake_case("_Leading_Underscore"));
            assert!(!is_pascal_snake_case("Trailing_Underscore_"));
            assert!(!is_pascal_snake_case("Double__Underscore"));
        }
    }
}
