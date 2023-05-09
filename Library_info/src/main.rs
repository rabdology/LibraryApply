mod book;
mod library;
mod calendar;
mod isbn;

use std::io;
use chrono::NaiveDate;
use book::Book;
use library::Library;
use calendar::add_return_event;
use isbn::fetch_book_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut library = Library::new();

    loop {
        println!("\nメニュー:");
        println!("1: 借りた本を追加");
        println!("2: 本を返却");
        println!("3: 返却予定日を更新");
        println!("4: 借りている本のリストを表示");
        println!("5: 終了");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                // 借りた本を追加
                let mut isbn = String::new();
                println!("ISBNを入力してください:");
                io::stdin().read_line(&mut isbn)?;
                let isbn = isbn.trim();

                let mut borrowed_date_input = String::new();
                println!("借用日を入力してください (YYYY-MM-DD):");
                io::stdin().read_line(&mut borrowed_date_input)?;
                let borrowed_date = NaiveDate::parse_from_str(&borrowed_date_input.trim(), "%Y-%m-%d")?;

                let book_info = fetch_book_info(&isbn)
                    .await
                    .expect("本の情報が取得できませんでした。ISBNが正しいか確認してください。");

                let due_date = borrowed_date + chrono::Duration::days(14);

                let book = Book {
                    isbn: isbn.to_string(),
                    title: book_info.title,
                    authors: book_info.authors,
                    borrowed_date,
                    due_date,
                };

                library.add_book(book);
                println!("本が追加されました。");

                // Google カレンダーへの返却予定日の反映
                add_return_event(&book).await?;
            }
            "2" => {
                // 本を返却
                let mut isbn = String::new();
                println!("返却する本のISBNを入力してください:");
                io::stdin().read_line(&mut isbn)?;
                let isbn = isbn.trim();

                library.remove_book(&isbn);
                println!("本が返却されました。");
            }
            "3" => {
                // 返却予定日を更新
                let mut isbn = String::new();
                println!("返却予定日を更新する本のISBNを入力してください:");
                io::stdin().read_line(&mut isbn)?;
                let isbn = isbn.trim();

                let mut new_due_date_input = String::new();
                println!("新しい返却予定日を入力してください (YYYY-MM-DD):");
                io::stdin().read_line(&mut new_due_date_input)?;
                let new_due_date = NaiveDate::parse_from_str(&new_due_date_input.trim(), "%Y-%m-%d")?;

                library.update_due_date(&isbn, new_due_date);
                println!("返却予定日が更新されました。");
            }
            "4" => {
                // 借りている本のリストを表示
                println!("借りている本のリスト:");
                for book in library.list_books() {
                    println!("ISBN: {}, タイトル: {}, 著者: {}, 借用日: {}, 返却予定日: {}",
                             book.isbn, book.title, book.authors.join(", "), book.borrowed_date, book.due_date);
                }
            }
            "5" => {
                println!("アプリケーションを終了します。");
                break;
            }
            _ => {
                println!("無効な選択です。もう一度お試しください。");
            }
        }
    }

    Ok(())
}
