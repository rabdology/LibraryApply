use crate::book::Book;
use chrono::NaiveDate;

pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_book(&mut self, isbn: &str) {
        self.books.retain(|book| book.isbn != isbn);
    }

    pub fn update_due_date(&mut self, isbn: &str, new_due_date: NaiveDate) {
        if let Some(book) = self.books.iter_mut().find(|book| book.isbn == isbn) {
            book.due_date = new_due_date;
        }
    }

    pub fn list_books(&self) -> &[Book] {
        &self.books
    }
}
