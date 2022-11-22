fn main() {
    // result型
    // mut本当は不要
    let mut result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code: {}", code),
        Err(e) => println!("error: {}", e),
    }

    result = Ok(200);
    if let Ok(code) = result {
        println!("code: {}", code);
    }

    result = Ok(200);
    println!("code: {}", result.unwrap_or(-1));

    let v1 = vec![1, 2, 3];

    for elem in &v1 {
        println!("{}", elem);
    }

    println!("Hello, world!");
}
