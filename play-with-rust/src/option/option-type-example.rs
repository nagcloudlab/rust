

#[derive(Debug)]
struct Insurance {
    policy_number: String,
    provider: String,
}

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    insurance: Option<Insurance>,
}

#[derive(Debug)]
struct Person {
    name: String,
    car: Option<Car>,
}

// way-1 : using match expression
fn get_insurance_details_v1(person: &Person) {
    match &person.car {
        Some(car) => match &car.insurance {
            Some(insurance) => {
                println!("{}'s Car Insurance Details:", person.name);
                println!("Policy Number: {}", insurance.policy_number);
                println!("Provider: {}", insurance.provider);
            }
            None => println!("{} has a car but no insurance.", person.name),
        },
        None => println!("{} does not have a car.", person.name),
    }
}

// way-2: using functional style
fn get_insurance_details_v2(person: &Person) {
    person
        .car
        .as_ref()
        .and_then(|car| car.insurance.as_ref())
        .map(|insurance| {
            println!("{}'s Car Insurance Details:", person.name);
            println!("Policy Number: {}", insurance.policy_number);
            println!("Provider: {}", insurance.provider);
        })
        .unwrap_or_else(|| {
            if person.car.is_some() {
                println!("{} has a car but no insurance.", person.name);
            } else {
                println!("{} does not have a car.", person.name);
            }
        });
}


fn main() {
    let person_with_insured_car = Person {
        name: "Alice".to_string(),
        car: Some(Car {
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            insurance: Some(Insurance {
                policy_number: "12345ABC".to_string(),
                provider: "Awesome Insurance Co.".to_string(),
            }),
        }),
    };

    let person_with_uninsured_car = Person {
        name: "Bob".to_string(),
        car: Some(Car {
            make: "Ford".to_string(),
            model: "Focus".to_string(),
            insurance: None,
        }),
    };

    let person_without_car = Person {
        name: "Charlie".to_string(),
        car: None,
    };

    println!("Checking Alice:");
    get_insurance_details(&person_with_insured_car);

    println!("\nChecking Bob:");
    get_insurance_details(&person_with_uninsured_car);

    println!("\nChecking Charlie:");
    get_insurance_details(&person_without_car);
}
