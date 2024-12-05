

// io
// computation

use std::{thread, vec};


fn computation(){
    // Task 1: Compute and print numbers from 1 to 100
    for i in 1..=100 {
        println!("Number: {}", i);
    }
}
fn io(){
     // Task 2: Read user input and greet
    use std::io;
    println!("Enter your name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("Hello, {}!", input.trim());
}

pub fn thread_example() {

    //io();
    //computation();
    
    // let io_handle=thread::spawn(|| {
    //     io();
    // });
    // thread::spawn(|| {
    //     computation();
    // });
    // io_handle.join().unwrap();


    // let data=vec![1,2,3,4,5,6,7,8,9,10];
    // let handle1=thread::spawn(move || {
    //     let sum:i32=data.iter().sum();
    //     println!("Sum: {}", sum);
    // });

    // handle1.join().unwrap();


    // Arc - Atomic Reference Counter

    use std::sync::Arc;    
    let vec=[1,2,3];
    let data=Arc::new(vec); // smart pointer
    let mut handles=vec![];
    for _ in 0..3 {
        let data=data.clone();
        let handle=thread::spawn(move || {
            let sum:i32=data.iter().sum();
            println!("Sum: {}", sum);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);


    //-----------------------

    // Mutex - Mutual Exclusion

    use std::sync::Mutex;
    let data=Arc::new(Mutex::new(vec![]));
    let mut handles=vec![];
    for i in 1..=2 {
        let data=data.clone();
        let handle=thread::spawn(move || {
            let mut data=data.lock().unwrap();
            println!("length before: {}", data.len());
            //...
            // sleep for 5 seconds
            thread::sleep(std::time::Duration::from_secs(5));
            println!("length after: {}", data.len());
            data.push(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", data);




}