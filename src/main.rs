use std::io;
mod person;
mod room;
mod item;
use std::collections::HashMap;

use crate::room::{Room,Door};
use crate::item::Item;
use crate::person::{Person};
use itertools::join;

fn go<'a> (mut commands: Vec<&str>, player: &mut Person, dungeon: &mut HashMap<String, Room>) {
    let direction: String = commands.remove(0).to_string().to_lowercase();
    if dungeon.contains_key::<str>(&direction) {
        player.change_room(&dungeon [&direction.to_string()].name);

        let curr_room = &dungeon [&direction.to_string()];
        let exits = curr_room.find_exits();
        let exit_str: String = join(exits.into_iter(),", ");
        let items = curr_room.find_items();
        
        let item_str: String = join(items.into_iter(),", ");
        println!("You are in {}, You can go to {}.", player.current_room, exit_str);
        println!("You can see {}", item_str);
    }
    else {
        println!("No exit that direction")
    }
}

fn where_am_i(player: &Person) {
    println!("You are in the {} room.", player.current_room);
}

fn get_item<'a> (mut commands: Vec<&str>, player: &mut Person, dungeon: &mut HashMap<String, Room>) {
    let item_name = commands[0];
    let room = &player.current_room;
    let item = dungeon.get(&room).unwrap().remove_Item(item_name);
    player.take_item(item)
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
    dungeon.get_mut("desert").unwrap().add_Item(Item::new("sword".to_string(),true, 10));
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

        let mut commands:Vec<&str> = command.split_whitespace().collect();
        let action = commands.remove(0);

        match action.trim() {
            "go" => go(commands, &mut player, &mut dungeon),
            "where" => where_am_i(&player),
            "take" => get_item(commands, &mut player, &mut dungeon),
            "quit" => break,
            _ => println!("Cannot do that: {}", action),
        }
    }
}
