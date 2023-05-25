pub struct Book {
    pub isbn: String,
    pub due_date: String,
}

impl Book {
    pub fn new(isbn: &str, due_date: &str) -> Book {
        Book {
            isbn: isbn.to_string(),
            due_date: due_date.to_string(),
        }
    }
}
