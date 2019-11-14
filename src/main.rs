use std::io;
mod person;
mod room;

use crate::room::{Room,Door};
use crate::person::{Person};


fn go<'a> (direction: &str, &mut player: &mut Person<'a>, mut dungeon: &[room::Room<'a>; 4]) {
    // let mut rooms = IntoIterator::into_iter(dungeon);
    let curr_room = dungeon.iter().find(|room| room.name == player.current_room.name).unwrap();
    let destinations = curr_room.find_exits(direction);
    let mut room_to_move = dungeon.iter_mut().find(|room| room.name == destinations).unwrap();

    player.change_room(room_to_move);

    println!("You are in {}, You can go to {}", player.current_room.name, destinations.to_string());

}

fn where_am_i(player: &Person) {
    println!("You are in the {} room.", player.current_room.name);
}


fn main() {
    let dungeon = [
        Room::new("North".to_string(), vec![Door::new("Forest","West"), Door::new("Desert","South")]),
        Room::new("West".to_string(), vec![Door::new("Castle","East")]),
        Room::new("East".to_string(), vec![Door::new("Desert","South")]),
        Room::new("South".to_string(), vec![Door::new("Swamp","East"), Door::new("Forest","North")])
        ];
    let mut name = String::new(); 
        println!("What is your name: ");
        io::stdin().read_line(&mut name)
            .ok()
            .expect("failed to read line");
    let mut player = Person::new(name, &mut dungeon[0]);
    println!("Hi {}", player.name);

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
            "go" => go(object, &player, &dungeon),
            "where" => where_am_i(&player),
            "quit" => break,
            _ => println!("Cannot do that: {}", object),
        }
    }
}
