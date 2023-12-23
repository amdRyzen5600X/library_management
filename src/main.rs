mod book;
mod library;

use book::Book;
use library::Library;

fn main() {
    let mut books: Vec<Book> = Vec::new();
    for i in 0..15 {
        let book = Book::new(String::from(format!("title{}", i)), String::from(format!("author{}", i)));
        books.push(book);
    }
    let mut lib = Library::new(books);

    loop {
        println!("wellcome to our library choose one of thees options");
        println!("1. borrow a book");
        println!("2. return a book");
        println!("3. display all available books");
        println!("4. display all borrowed books");

        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("failed to read a line");

        let answer: u32 = match answer.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        match answer {
            1 => {
                let mut title = String::new();
                std::io::stdin()
                    .read_line(&mut title)
                    .expect("failed to read a title");
                let title = title.trim().parse().expect("");
                match lib.borrow_book(&title) {
                    Some(_) => println!("book borrowed just fine"),
                    None => {
                        println!("no such a book({}) try to display available books", title);
                        continue;
                    },
                }
            }
            2 => {
                let mut title = String::new();
                std::io::stdin()
                    .read_line(&mut title)
                    .expect("failed to read a title");
                let _ = title.trim();
                match lib.return_book(&title) {
                    Some(_) => println!("book returned just fine"),
                    None => {
                        println!("no such a book try to display borrowed books");
                        continue;
                    },
                }
            }
            3 => {
                lib.display_availabe_books();
            }
            4 => {
                lib.display_borrowed_books();
            }
            _ => {
                println!("type a number between 1 and 4");
                continue;
            }

            
        }
    }


}
