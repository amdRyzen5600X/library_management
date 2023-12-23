use crate::book::Book;

pub struct Library {
    pub available_books: Vec<Book>,
    pub borrowed_books: Vec<Book>,
}

impl Library {
    pub fn new(available_books: Vec<Book>) -> Library {
        Library{
            available_books,
            borrowed_books: Vec::new(),
        }
    }
    
    pub fn borrow_book(&mut self, title: &String) -> Option<u32>{
        self.borrowed_books.push(
            match self.available_books
            .clone()
            .into_iter()
            .find(|x| x.title.eq(title)){
                Some(book) => book,
                None => {
                    return None;
                },
            }
        );
        self.available_books
            .retain(|x| x.title.ne(title));
        return Some(1);
    }

    pub fn return_book(&mut self, title: &String) -> Option<u32> {
        self.available_books.push(
            match self.borrowed_books
            .clone()
            .into_iter()
            .find(|x| x.title.eq(title)){
                Some(book) => book,
                None => {
                    return None;
                },
            }
        );
        self.borrowed_books
            .retain(|x| x.title.ne(title));
        Some(1)
    }

    pub fn display_availabe_books(&self){
        for book in self.available_books.iter() {
            println!("{}", book.title);
        }
    }

    pub fn display_borrowed_books(&self){
        for book in self.borrowed_books.iter() {
            println!("{}", book.title);
        }
    }
}
