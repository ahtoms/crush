use std::result::Result;

struct Node<T> {
    obj: T,
    next: Option<Box<Node<T>>>
}


struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize,
}

impl <T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {top: None, size: 0}
    }

    fn push(&mut self, element: T) {
        if let Some(n) = self.top.take() {
            let node = Node { obj: element, next: Some(n) };
            self.top = Some(Box::new(node));
        } else {
            self.top = Some(Box::new(Node { obj: element, next: None }));
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Result<T, &str> {
        match self.top.take() {
            Some(v) => {
                self.size -= 1;
                let res = v.obj;
                self.top = v.next;
                Ok(res)
            },
            None => { Err("No elements on stack") }
        }
    }

    fn peek(&self) -> Result<&T, &str> {
        match &self.top {
            Some(v) => { Ok(&v.obj) },
            None => { Err("No elements on stack") }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn empty(&self) -> bool {
        self.size == 0
    }
}


fn main() {
    //Simple test
    let mut stack : Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    println!("Size {}", stack.size());

    while !stack.empty() {
        if let Ok(r) = stack.peek() {
            println!("Peeked Item {}", r);
        }

        if let Ok(item) = stack.pop() {
            println!("Popped Item {}\n", item);
        }
    }


}
