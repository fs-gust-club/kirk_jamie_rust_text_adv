use crate::room::Room;

pub struct Person<'a>{
    pub name: String,
    pub current_room: &'a mut Room<'a>
}

impl<'a> Person<'a>{
    pub fn new(name: String, room: &'a mut Room<'a>) -> Person<'a> {
        Person {
            name: name,
            current_room: room,
        }
    }

    pub fn change_room(&mut self, new_room: &'a mut Room<'a>){
        self.current_room = new_room;
    }
}