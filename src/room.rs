pub struct Room<'a>{
    pub name: String,    
    pub doors: Vec<Door<'a>>,
}

impl<'a> Room<'a>{
    pub fn new(name: String, doors: Vec<Door<'a>>) -> Room <'a>{
        Room {
            name: name,
            doors: doors,
        }
    }

    pub fn find_exits(&self, direction: &str) -> &str{
        let mut destinations = IntoIterator::into_iter(self.doors.to_vec());
        let destination = destinations.find(move |door_dir| door_dir.direction == direction).unwrap();
        return destination.direction;
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
