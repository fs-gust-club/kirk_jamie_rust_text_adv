use super::item::Item;
use itertools::join;

pub struct Person{
    pub name: String,
    pub current_room: String,
    pub items: Vec<Item>
}

impl Person{
    pub fn new(name: String, room: String) -> Person {
        Person {
            name: name,
            current_room: room,
            items: Vec::new(),
        }
    }

    pub fn take_item(&mut self, item: Item) {
        self.items.push(item)
    }

    pub fn list_items(&self) -> Vec<String> {
        let items = IntoIterator::into_iter(self.items.to_vec());
        let items: Vec<String> = items.map(|x| x.name).collect();
        return items;
    }

    pub fn inventory(&self) {
        println!("You have: {}", join(self.list_items(),", "));
    }

    pub fn change_room(&mut self, new_room: &String){
        self.current_room = new_room.to_string();
    }
}