pub struct Person{
    pub name: String,
    pub current_room: String
}

impl Person{
    pub fn new(name: String, room: String) -> Person {
        Person {
            name: name,
            current_room: room,
        }
    }

    pub fn change_room(&mut self, new_room: String){
        self.current_room = new_room;
    }
}