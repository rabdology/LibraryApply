use chrono::NaiveDate;

pub struct Book {
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub borrowed_date: NaiveDate,
    pub due_date: NaiveDate,
}
