pub fn validate_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    let mut sum = 0;
    let mut weight = 10;

    if isbn.len() != 10 {
        return false;
    }

    for (i, c) in isbn.chars().enumerate() {
        match c.to_digit(10) {
            Some(digit) => {
                sum += digit * weight;
                weight -= 1;
            }
            None => {
                if i == 9 && (c == 'X' || c == 'x') {
                    sum += 10;
                } else {
                    return false;
                }
            }
        }
    }

    sum % 11 == 0
}
