struct Character {
    name: String,
    class: Class,
    level: u8,
}

enum Class {
    Mage,
    Hunder,
    Warrior,
}






impl Character {
    fn new(name: String, class: Class) -> Self {
        Person {
            name: name,
            level: 1,
            class: class,
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
    }

    fn say_hi(&self) {
        println!("Hi, I'm {}", &self.name);
    }
}
