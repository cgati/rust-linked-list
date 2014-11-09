pub enum Node {
    Cons(uint, Box<Node>),
    Nil
}

impl Node {
    fn new_node() -> Node {
        Nil
    }

    fn append_to_front(self, elem: uint) -> Node {
        Cons(elem, box self)
    }

    fn contains(&self, elem: uint) -> bool {
        match *self {
            Cons(x, ref tail) => if x == elem { true } else { tail.contains(elem) },
            Nil => false
        }
    }

    fn length(&self) -> uint {
        match *self {
            Cons(_, ref tail) => 1 + tail.length(),
            Nil => 0
        }
    }
}

fn main() {
    let mut head: Node = Cons(1, box Nil);
    head = head.append_to_front(2);

    println!("The length of the linked list is: {}", head.length());
}
