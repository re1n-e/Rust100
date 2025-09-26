use std::fmt;
use std::io::{self, Write};

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
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            next: Box::new(array_init::array_init(|_| None)),
            contacts: None,
            is_contact: false,
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

    fn search(&self, name: &str) -> bool {
        let mut temp = &self.root;
        for ch in name.chars() {
            let index = if !ch.is_whitespace() {
                (ch.to_ascii_lowercase() as u8 - b'a') as usize
            } else {
                26
            };
            if temp.next[index].is_none() {
                return false;
            }
            temp = temp.next[index].as_deref().unwrap();
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

    fn delete_contact(&mut self, name: &str, id: usize) {
        Self::delete(&mut self.root, name, 0, id);
    }

    fn delete(node: &mut TrieNode, name: &str, pos: usize, id: usize) -> bool {
        if pos == name.len() {
            if node.is_contact {
                if let Some(ref mut contacts) = node.contacts {
                    contacts.retain(|c| c.id != id);
                    if contacts.is_empty() {
                        node.contacts = None;
                        node.is_contact = false;
                    }
                }
            }
        } else {
            let c = name.chars().nth(pos).unwrap();
            let index = if !c.is_whitespace() {
                (c.to_ascii_lowercase() as u8 - b'a') as usize
            } else {
                26
            };

            if let Some(child) = node.next[index].as_deref_mut() {
                let should_delete = Self::delete(child, name, pos + 1, id);

                if should_delete {
                    node.next[index] = None;
                }
            }
        }

        node.contacts.is_none() && !node.is_contact && node.next.iter().all(|c| c.is_none())
    }
}

fn get_input(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut trie = Trie::new();
    let mut id_counter = 1;

    loop {
        println!("\nContact Manager:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search Contacts");
        println!("4. Delete Contacts");
        println!("5. Exit");

        let choice = get_input("Enter choice");

        match choice.as_str() {
            "1" => {
                let name = get_input("Enter name");
                let phone = get_input("Enter phone");
                let email = get_input("Enter email");

                let contact = Contact {
                    id: id_counter,
                    name: name.clone(),
                    phone,
                    email,
                };
                trie.add_contact(&name, contact);
                println!("Contact added successfully!");
                id_counter += 1;
            }
            "2" => {
                println!("All Contacts:");
                trie.view_contacts();
            }
            "3" => {
                let name = get_input("Enter name to search");
                if !trie.search(&name) {
                    println!("No contact found with that name.");
                }
            }
            "4" => {
                let name = get_input("Enter name of contact to delete");
                let id: usize = get_input("Enter contact ID to delete").parse().unwrap_or(0);
                if id == 0 {
                    println!("Invalid ID!");
                } else {
                    trie.delete_contact(&name, id);
                    println!("Contact deleted (if it existed).");
                }
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
