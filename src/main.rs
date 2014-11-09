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

    fn value(&self) -> uint {
        match *self {
            Cons(x, _) => x,
            Nil => 0
        }
    }

    // FIXME: Any node you pass in transfers ownership. Need to figure out how
    // to pull a reference to tail out of &self.
    fn get_next(self) -> Node {
        match self {
            Cons(_, tail) => *tail,
            Nil => self
        }
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

    fn print_list(&self) {
        match *self {
            Cons(x, ref tail) => {
                print!("{} ", x);
                tail.print_list();
            },
            Nil => {
                println!("");
                return;
            }
        }
    }
}

fn main() {
    let mut head: Node = Cons(1, box Nil);
    head = head.append_to_front(2);
    head = head.append_to_front(3);

    println!("The length of the linked list is: {}", head.length());
    print!("The linked list contains: ");
    head.print_list();

    let mut val = head.contains(10);
    let mut val2 = head.contains(2);

    println!("List contains 10? {}. List contains 2? {}", val, val2);
}
