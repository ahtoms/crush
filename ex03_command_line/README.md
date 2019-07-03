# Command Line

We typically want to assemble programs that can utilise command line arguments. We will start using the std::env module to retrieve command line arguments.

You can enter command arguments through cargo by specifying them after `cargo run` like so `cargo run 1 2 3 4`.


This will be our first dip into the documentation to understand how we can retrieve the information.
[std::env::args](https://doc.rust-lang.org/std/env/fn.args.html).

Try the following
* Print out all command line arguments to standard output
* Visit [Args type](https://doc.rust-lang.org/std/env/struct.Args.html) and search for a trait that provides an implementation to collect all elements. Consider the trait name as a major hint.
You will need to provide type annotation for the variable, provide the type `Vec<String>`, example

```
let args: Vec<String> = //Assignment here
```



