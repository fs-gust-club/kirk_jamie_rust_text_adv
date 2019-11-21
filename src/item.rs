#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub holdable: bool,
    pub hp: i8       
}
impl Item{
    pub fn new(name: String, holdable: bool, hp: i8) -> Item {
        Item {
            name: name,
            holdable: holdable,
            hp: hp,
        }
    }
}

