# Simple Data Structure

A stack is a LIFO (last-in-first-out) data structure. This kind of data structure is described in a way that the last elemented added will be the first element removed. This kind of data structure prescribes 3 methods, `peek`, `push` and `pop`.

* `push` adds an element onto the stack
* `peek` allows the programmer to inspect the last element added to the stack
* `pop` removes the last element added to the stack, moving the previously added element to the top

You can consider the structure like a **stack** of coffee coasters, where you will only be able to remove the top coaster.

This is not considered easy in Rust as you will need to acknowledge the ownership heirarchy within the data structure. 

As a few hints to help with your program

* Create two struct types, `Node` and `Stack`
  * `Node` will contain a link to the next `Node` and the current value stored on the stack `u32`.
  * `Stack` will hold the last node added and the size of the stack (`usize`)
  
* Use the `Option` type to let you set if a `Node` has a link to the next node.
* Consider the issue with using a `Option<Node>` and why you may want to use `Box` with your option type

* `Option` documentation will help, particularly when you want to *take* ownership of the interior item of an `Option` type [std::option](https://doc.rust-lang.org/std/option/index.html)

* *Extension* Try and implement a generic `Stack` type. Potentially you may want to provide some concessions like only allow `Sized` types in the stack. [Generics](https://doc.rust-lang.org/1.5.0/book/generics.html)

