
 #[derive(Debug, Deserialize, Serialize)]
struct Book {
    title: String,
    author: String,
    due_date: DateTime<Utc>,
}

