struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn push_back(&mut self, value: i32) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { value, next: None }));
    }

    fn delete(&mut self, index: i32) {
        if index == 0 {
            if self.head.is_none() {
                println!("The index dosen't exist");
                return;
            }
            self.head = self.head.take().and_then(|node| node.next);
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                println!("The index dosen't exist");
                return;
            }
        }

        if let Some(node) = current {
            if node.next.is_none() {
                println!("The index dosen't exist");
                return;
            }
            node.next = node.next.take().and_then(|n| n.next);
        } else {
            println!("The index dosen't exist");
        }
    }

    fn print_list(&mut self) {
        let mut current = &mut self.head;
        print!("head->");
        while let Some(node) = current {
            print!("{}->", node.value);
            current = &mut node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_back(40);
    list.print_list();

    list.push_front(21);
    list.push_front(22);
    list.push_front(15);
    list.print_list();

    list.delete(2);
    list.print_list();
    list.delete(0);
    list.print_list();
}
