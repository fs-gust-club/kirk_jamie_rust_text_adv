use super::item::Item;

pub struct Room<'a>{
    pub name: String,    
    pub doors: Vec<Door<'a>>,
    pub items: Vec<Item>
}

impl<'a> Room<'a>{
    pub fn new(name: String, doors: Vec<Door<'a>>) -> Room <'a>{
        Room {
            name: name,
            doors: doors,
            items: Vec::new(),
        }
    }

    pub fn add_Item(&mut self, item: Item) {
        self.items.push(item)
    }

    pub fn find_exits(&self) -> Vec<&str> {
        let destinations = IntoIterator::into_iter(self.doors.to_vec());
        let destinations: Vec<&str> = destinations.map(|x| x.name).collect();
        return destinations;
    }
    pub fn find_items(&self) -> Vec<String> {
        let items = IntoIterator::into_iter(self.items.to_vec());
        let items: Vec<String> = items.map(|x| x.name).collect();
        return items;
    }
}
#[derive(Clone, Copy)]
pub struct Door<'a>{
    pub name: &'a str,
    pub direction: &'a str
}

impl<'a> Door<'a>{
    pub fn new(name: &'a str, direction: &'a str) -> Door<'a> {
        Door {
            name: name,
            direction: direction,
        }
    }
}
