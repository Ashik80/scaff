pub trait StringCase {
    fn to_camel_case(&self) -> String;
    fn to_pascal_case(&self) -> String;
}

impl StringCase for str {
    fn to_camel_case(&self) -> String {
        let words = self.split('-');
        let mut result = String::new();
        let mut first_letter_avoided = false;
        for word in words {
            let first_letter = match !first_letter_avoided {
                true => {
                    first_letter_avoided = true;
                    word.chars().next().unwrap()
                }
                false => word.chars().next().unwrap().to_ascii_uppercase(),
            };
            result.push(first_letter);
            result.push_str(&word[1..])
        }
        result
    }

    fn to_pascal_case(&self) -> String {
        let words = self.split('-');
        let mut result = String::new();
        for word in words {
            let first_letter = &word.chars().next().unwrap().to_ascii_uppercase();
            let rest = &word[1..];
            result.push(*first_letter);
            result.push_str(rest)
        }
        result
    }
}
