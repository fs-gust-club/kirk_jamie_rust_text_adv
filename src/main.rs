use std::io;
mod person;
mod room;
use std::collections::HashMap;

use crate::room::{Room,Door};
use crate::person::{Person};
use itertools::join;
#[macro_use]
extern crate nom;

use nom::{alpha, space};

fn go<'a> (direction: &str, player: &mut Person, dungeon: &mut HashMap<String, Room>) {
    
    if dungeon.contains_key::<str>(&direction) {
        player.change_room(&dungeon [&direction.to_string()].name);

        let curr_room = &dungeon [&direction.to_string()];
        let exits = curr_room.find_exits();
        let exit_str: String = join(exits.into_iter(),", ");

        println!("You are in {}, You can go to {}.", player.current_room, exit_str);
    }
    else {
        println!("No exit that direction")
    }
}

fn where_am_i(player: &Person) {
    println!("You are in the {} room.", player.current_room);
}


fn main() {
    let mut dungeon = HashMap::new();
    dungeon.insert(
        "forest".to_string(), 
        Room::new("Forest".to_string(), 
            vec![
                Door::new("Swamp","West"), 
                Door::new("Desert","South")
            ]),
    );
    dungeon.insert(
        "swamp".to_string(), 
        Room::new("Swamp".to_string(), 
            vec![
                Door::new("Forest","East"),
            ]),
    );
    dungeon.insert(
        "desert".to_string(), 
        Room::new("Desert".to_string(), 
            vec![
                Door::new("Forest","North"),
                Door::new("Arctic","East"),
            ]),
    );
    dungeon.insert(
        "arctic".to_string(), 
        Room::new("Arctic".to_string(), 
            vec![
                Door::new("Desert","West"),
            ]),
    );
    let mut name = String::new(); 
        println!("What is your name: ");
        io::stdin().read_line(&mut name)
            .ok()
            .expect("failed to read line");
    let mut player = Person::new(name, "forest".to_string());
    println!("Hi {}", player.name);

    loop {
        let mut command = String::new(); 
        println!("Enter your choice");
        io::stdin().read_line(&mut command)
            .ok()
            .expect("failed to read line");

        named!(pub name_parser(&str) -> &str,
            chain!
                ( tag_s!("hello")
                ~ space?
                ~ name: alpha
                , || name
            )
        );

        // let commands:Vec<&str> = command.split_whitespace().collect();
        // let action = commands.get(0).unwrap_or(&"").to_lowercase();
        // let object = commands.get(1).unwrap_or(&"");

        // match action.trim() {
        //     "go" => go(object, &mut player, &mut dungeon),
        //     "where" => where_am_i(&player),
        //     "quit" => break,
        //     _ => println!("Cannot do that: {}", object),
        // }
    }
}
