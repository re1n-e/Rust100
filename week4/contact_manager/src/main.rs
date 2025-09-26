use std::fmt;
use std::io;

#[derive(Debug, Clone)]
struct Contact {
    id: usize,
    name: String,
    phone: String,
    email: String,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Contact #{}: {} | Phone: {} | Email: {}",
            self.id, self.name, self.phone, self.email
        )
    }
}

#[derive(Debug, Clone)]
struct TrieNode {
    next: Box<[Option<Box<TrieNode>>; 27]>, // Each letter → optional child node
    contacts: Option<Vec<Contact>>,         // Store contacts here
    is_contact: bool,
    freq: usize,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            next: Box::new(array_init::array_init(|_| None)),
            contacts: None,
            is_contact: false,
            freq: 0,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn add_contact(&mut self, name: &str, contact: Contact) {
        let mut temp = &mut self.root;
        for c in name.chars() {
            let index = if !c.is_whitespace() {
                (c.to_ascii_lowercase() as u8 - b'a') as usize
            } else {
                26
            };
            if temp.next[index].is_none() {
                temp.next[index] = Some(Box::new(TrieNode::new()));
            }
            temp.freq += 1;
            temp = temp.next[index].as_deref_mut().unwrap();
        }
        temp.is_contact = true;
        if let Some(ref mut list) = temp.contacts {
            list.push(contact);
        } else {
            temp.contacts = Some(vec![contact]);
        }
    }

    fn view_contacts(&self) {
        self.view(&self.root);
    }

    fn view(&self, temp: &TrieNode) {
        if temp.is_contact {
            if let Some(ref contacts) = temp.contacts {
                for contact in contacts {
                    println!("{contact}");
                }
            }
        }

        for child in temp.next.iter().filter_map(|c| c.as_deref()) {
            self.view(child);
        }
    }

    fn search(&mut self, name: &str) -> bool {
        let mut temp = &mut self.root;
        for ch in name.chars() {
            let index = if !ch.is_whitespace() {
                (ch.to_ascii_lowercase() as u8 - b'a') as usize
            } else {
                26
            };
            if temp.next[index].is_none() {
                return false;
            }
            temp = temp.next[index].as_deref_mut().unwrap();
        }
        if temp.is_contact {
            println!("Found following with name: {name}");
            if let Some(ref contacts) = temp.contacts {
                for contact in contacts {
                    println!("{contact}");
                }
            }
        }
        temp.is_contact
    }

    fn delete_contact(&mut self, name: &str, id: &usize) {}

    fn delete(temp: &mut TrieNode, name: &str, id: &usize) {
        if temp.is_contact {
            if let Some(ref mut contacts) = temp.contacts {}
        }
        for ch in name.chars() {
            let index = if !c.is_whitespace() {
                (c.to_ascii_lowercase() as u8 - b'a') as usize
            } else {
                26
            };
            if temp.next[index].is_none() {
                temp.next[index] = Some(Box::new(TrieNode::new()));
            }
        }
    }
}

fn main() {
    // loop {
    //     println!("\n Cotact Manager:");
    //     println!("1. Add Contact");
    //     println!("2. View Contacts");
    //     println!("3. Search Contacts");
    //     println!("4. Delete Contacts");
    //     println!("5. Exit");
    // }
    let mut trie = Trie::new();

    trie.add_contact(
        &"Ali",
        Contact {
            id: 1,
            name: "Ali".into(),
            phone: "1111".into(),
            email: "ali@example.com".into(),
        },
    );

    trie.add_contact(
        &"Alice",
        Contact {
            id: 2,
            name: "Alice".into(),
            phone: "2222".into(),
            email: "alice@example.com".into(),
        },
    );

    trie.view_contacts();
}
