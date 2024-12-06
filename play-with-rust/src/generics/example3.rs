
// generic enum

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn demo() {
    let success: Result<i32, &str> = Result::Ok(42);
    let failure: Result<i32, &str> = Result::Err("Out of stock");

    match success {
        Result::Ok(value) => println!("Success with value: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }

    match failure {
        Result::Ok(value) => println!("Success with value: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }
}
