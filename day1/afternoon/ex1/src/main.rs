use std::cmp::Ordering;

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: vec![],
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn print_books(&self) {
        if self.is_empty() {
            println!("[");
            return;
        }
        print!("[");
        for i in 0..&self.books.len() - 1  {
            let book = &self.books[i];
            print!("{} ({}), ", book.title, book.year);
        }
        let book = &self.books[&self.books.len() - 1];
        print!("{} ({})", book.title, book.year);
        println!("]");
    }

    fn oldest_book(&self) -> Option<&Book> {
        if self.is_empty() {
            return None;
        }
        (&self.books).iter().min_by(|x: &&Book, y: &&Book | -> Ordering {
            x.year.cmp(&y.year) })
    }
}

fn main() {
    let mut library = Library::new();

    println!("The library is empty: {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: {}", library.is_empty());


    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}
