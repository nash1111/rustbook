struct Color{
    r:i32,
    g:i32,
    b:i32,
}

fn main() {
    let a = Color{r:255, g:0, b:0};
    let b = a; // 所有権が譲渡される
    println!("{} {} {}", b.r, b.g, b.b);
    
}