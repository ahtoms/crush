# Find a book

With this task, you will need to write a program that will find a book `(title: String)` within a vector. In the instance where you program cannot locate the book your function must be able to return a result contain no entry (`None`).

Utilise `stdin` for this task as a way of reading input from the user. Use the predefined list of strings provided and the function prototype.

Example:
```
> ./find_book
Animal Farm
Found: Animal Farm
```

* Implement a function that will search a vector of type `String` (`Vec<String>`) for a book title, this will need to return `None` if the book cannot be found or `Some(book)` if the book can be found.

* Implement standard input loop that will ask the user for input. Use the `read_line` example outlined in the Rust documentation [std::io::stdin](https://doc.rust-lang.org/std/io/fn.stdin.html) and [std::io::Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)

**As a hint, try using a `while let` or `match` when using the `read_line` method. Rust uses enums very heavily, in particularly `Result` and `Option`, Result is represented by `Ok(item)` or `Err(error_message)` and Option is represented by `Some(item)` and `None`.


