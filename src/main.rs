use std::io;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn search_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }

    fn search_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|book| book.author == author).collect()
    }

    fn search_by_year(&self, year: u32) -> Vec<&Book> {
        self.books.iter().filter(|book| book.year == year).collect()
    }

    fn remove_book(&mut self, book_title: &str) -> bool {
        if let Some(index) = self.books.iter().position(|book| book.title == book_title) {
            self.books.remove(index);
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut library = Library::new();

    loop {
        println!("1. Add a book");
        println!("2. Search by title");
        println!("3. Search by author");
        println!("4. Search by year");
        println!("5. Remove a book");
        println!("6. Exit");

        let choice: u32 = match get_user_input("Enter your choice: ").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                let title = get_user_input("Enter the title: ");
                let author = get_user_input("Enter the author: ");
                let year: u32 = match get_user_input("Enter the year: ").trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for year. Please enter a valid number.");
                        continue;
                    }
                };
                let book = Book {
                    title,
                    author,
                    year,
                };
                library.add_book(book);
            }
            2 => {
                let title = get_user_input("Enter the title to search: ");
                if let Some(book) = library.search_by_title(&title) {
                    println!("Found book: {:?}", book);
                } else {
                    println!("Book not found");
                }
            }
            3 => {
                let author = get_user_input("Enter the author to search: ");
                let found_books = library.search_by_author(&author);
                if !found_books.is_empty() {
                    println!("Found books by {}: {:?}", author, found_books);
                } else {
                    println!("No books found by {}", author);
                }
            }
            4 => {
                let year: u32 = match get_user_input("Enter the year to search: ").trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input for year. Please enter a valid number.");
                        continue;
                    }
                };
                let found_books = library.search_by_year(year);
                if !found_books.is_empty() {
                    println!("Found books published in {}: {:?}", year, found_books);
                } else {
                    println!("No books found published in {}", year);
                }
            }
            5 => {
                let title = get_user_input("Enter the title to remove: ");
                if library.remove_book(&title) {
                    println!("Book '{}' removed successfully", title);
                } else {
                    println!("Failed to remove book '{}'", title);
                }
            }
            6 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a valid number."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
