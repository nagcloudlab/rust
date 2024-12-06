

pub fn filter_numbers() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // traditional way
    let odd_numbers = filter_odd_numbers(numbers.clone());
    let even_numbers = filter_even_numbers(numbers.clone());

    // functional way
    let odd_numbers = filter_numbers(numbers.clone(), |number| number % 2 != 0);
    let even_numbers = filter_numbers(numbers.clone(), |number| number % 2 == 0);

}

fn filter_numbers(numbers: Vec<i32>,predicate:Fn(i32)->bool) -> Vec<i32> {
    let mut filtered_numbers = Vec::new();
    for number in numbers {
        if predicate(number) {
            filtered_numbers.push(number);
        }
    }
    filtered_numbers
}


fn filter_odd_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut odd_numbers = Vec::new();
    for number in numbers {
        if number % 2 != 0 {
            odd_numbers.push(number);
        }
    }
    odd_numbers
}

fn filter_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut even_numbers = Vec::new();
    for number in numbers {
        if number % 2 == 0 {
            even_numbers.push(number);
        }
    }
    even_numbers
}