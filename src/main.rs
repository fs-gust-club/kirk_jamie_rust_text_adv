use std::io;
mod person;
mod room;

use crate::room::{Room,Door};
use crate::person::{Person};


fn go(direction: &str, player: &mut Person, dungeon: &Vec<Room> ) {
    let mut rooms = IntoIterator::into_iter(dungeon);
    let curr_room = rooms.find(|&room| room.name == player.current_room).unwrap();
    let destinations = curr_room.find_exits(direction);

    player.change_room(destinations.to_string());

    println!("You are in {}, You can go to {}",curr_room.doors[0].name, destinations);

}

fn where_am_i(player: &Person) {
    println!("You are in the {} room.", player.current_room);
}


fn main() {
    let mut name = String::new(); 
        println!("What is your name: ");
        io::stdin().read_line(&mut name)
            .ok()
            .expect("failed to read line");
    let mut player = Person::new(name, "North".to_string());
    println!("Hi {}", player.name);

    let mut dungeon = Vec::new();
    dungeon.push(Room::new("North".to_string(), vec![Door::new("Forest","West"), Door::new("Desert","South")]));
    dungeon.push(Room::new("West".to_string(), vec![Door::new("Castle","East")]));
    dungeon.push(Room::new("East".to_string(), vec![Door::new("Desert","South")]));
    dungeon.push(Room::new("South".to_string(), vec![Door::new("Swamp","East"), Door::new("Forest","North")]));
    loop {
        let mut command = String::new(); 
        println!("Enter your choice");
        io::stdin().read_line(&mut command)
            .ok()
            .expect("failed to read line");

        let commands:Vec<&str> = command.split_whitespace().collect();
        let action = commands.get(0).unwrap_or(&"").to_lowercase();
        let object = commands.get(1).unwrap_or(&"");

        match action.trim() {
            "go" => go(object, &mut player, &dungeon),
            "where" => where_am_i(&player),
            "quit" => break,
            _ => println!("Cannot do that: {}", object),
        }
    }
}
