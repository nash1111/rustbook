// 構造体にメソッドを追加
struct Person {
    name: String,
    age: u32
}

impl Person {
    fn say_name(&self) { 
        println!("My name is {}", self.name);
    }

    fn say_age(&self) {
        println!("I am {} years old", self.age);
    }
}

fn main() {
    let p = Person {
        name: "John".to_string(),
        age: 20
    };
    p.say_name();
    p.say_age();
}