pub struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    // Method that borrows a Book and prints details
    pub fn describe(&self) {
        println!("Book: '{}' by {} ({} pages)", self.title, self.author, self.pages);
    }
}

// Function that takes ownership of a Book
pub fn consume_book(book: Book) {
    println!("\nConsuming the book:");
    book.describe();
    // After this function, `book` is dropped (ownership is moved)
}

pub fn struct_ownership_demo() {
    let my_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik & Carol Nichols"),
        pages: 550,
    };

    println!("\nStruct Ownership Demo:");
    my_book.describe();

    // Passing the book by ownership
    consume_book(my_book);

    // println!("{}", my_book.title); // ❌ ERROR: `my_book` was moved
}
