# Word Count

Given your new found power with command line arguments, we will now use it in conjunction with files.
Venturing back to the documentation provides us with helpful examples on how to read a file and introduces us the `?` sugar syntax.

To get familar with opening and using a file, you can visit the following link [std::fs::File](https://doc.rust-lang.org/std/fs/struct.File.html)

This will also need to handle `Option` types, if you can do this easily through `match` or `if let`.

Example Usage

```
> ./word_count gullivers_travels.txt obedience
Found obedience, 6 times
```


Note: We are simplying going to split by white, you may optionally attempt to remove characters impeding your search method.


* Retrieve the first two arguments from your program, first argument is the text file, second argument is the word to find and count the ocurrence of.

* Once you have provided the necessary checks, you can then open the file and read the contents, split the contents by whitespace and check each element for a matching word.

* If the word matches, increment the count

