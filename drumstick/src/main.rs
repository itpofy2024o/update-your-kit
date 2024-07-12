mod crud;
mod models;

use crate::crud::RhythmStore;
use std::io::{self, Write};

fn main() {
    let mut store = RhythmStore::new();
    loop {
        println!("1. Create A Song");
        println!("2. List All Songs");
        println!("3. Update A Song");
        println!("4. Access A Song");
        println!("5. Delete A Song");
        println!("6. Exit This Program");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                print!("Enter the name of the new song which to be added to the list: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                print!("Enter song writer(s) of the to be added song: ");
                io::stdout().flush().unwrap();
                let mut author = String::new();
                io::stdin().read_line(&mut author).unwrap();
                let author = author.trim().to_string();

                let song = store.create(name, author);
                println!("Created Song Be: {:?}", song);
            }
            2 => {
                for sound in store.list() {
                    println!("{:?}", sound);
                }
            }
            3 => {
                print!("Enter an ID to update: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<u32>().unwrap();

                print!("Enter new name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                print!("Enter new auhor: ");
                io::stdout().flush().unwrap();
                let mut author = String::new();
                io::stdin().read_line(&mut author).unwrap();
                let author = author.trim().to_string();

                match store.update(id, name, author) {
                    Some(music) => println!("Updated Music: {:?}", music),
                    None => println!("Not found"),
                }
            }
            4 => {
                print!("Enter the ID to read: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<u32>().unwrap();

                match store.read(id) {
                    Some(music) => println!("The Music: {:?}", music),
                    None => println!("Not found"),
                }
            }
            5 => {
                print!("Enter the ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<u32>().unwrap();

                if store.delete(id) {
                    println!("Deleted A Song");
                } else {
                    println!("Not found");
                }
            }
            6 => break,
            _ => println!("Invalid choice"),
        }
    }
}
