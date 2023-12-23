#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author_name: String,
}

impl Book {
    pub fn new(title: String, author_name: String) -> Book{
        Book{
            title,
            author_name,
        }
    }
}
