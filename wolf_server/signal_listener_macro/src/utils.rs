pub fn to_snake_case(pascal_case: &String) -> String {
    let words = split_by_case_and_lowercase(pascal_case.clone());
    words.join("_")
}

fn split_by_case_and_lowercase(pascal_case: String) -> Vec<String> {
    let mut ret = Vec::new();
    let mut first_word = true;
    let mut current_word = Vec::new();
    let mut chars = pascal_case.chars();
    loop {
        let next_char = chars.next();

        if next_char.is_none() || (!first_word && next_char.unwrap().is_uppercase()) {
            ret.push(current_word.join(""));
            current_word = Vec::new();
        }
        first_word = false;
        if let Some(c) = next_char {
            current_word.push(c.to_ascii_lowercase().to_string());
        } else {
            break;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::utils::to_snake_case;
    #[test]
    fn snake_case_test() {
        let to_test = String::from("TestSnakeCase");
        let snake_case = to_snake_case(&to_test);
        assert_eq!(snake_case, "test_snake_case");
    }
}
