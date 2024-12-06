

// generic function

fn compare<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn demo() {
    let max_int = compare(10, 20);
    let max_float = compare(3.5, 2.8);

    println!("Max int: {}", max_int);
    println!("Max float: {}", max_float);
}
