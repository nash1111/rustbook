fn main() {
    enum Color{
        Red,
        Blue,
        Green,
    }
    let color = Color::Red;
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        //Color::Green => println!("green"),
    }
}