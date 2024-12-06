

pub trait Wheel {
    fn rotate(&self) -> String;
}


pub struct WheelBrandA;

impl Wheel for WheelBrandA {
    fn rotate(&self) -> String {
        String::from("WheelBrandA is rotating smoothly!")
    }
}


pub struct WheelBrandB;

impl Wheel for WheelBrandB {
    fn rotate(&self) -> String {
        String::from("WheelBrandB is rotating powerfully!")
    }
}


pub struct Car<'a, W: Wheel> {
    wheels: Vec<&'a W>,
}

impl<'a, W: Wheel> Car<'a, W> {
    pub fn new(wheels: Vec<&'a W>) -> Self {
        Self { wheels }
    }

    pub fn move_car(&self) -> String {
        self.wheels
            .iter()
            .map(|wheel| wheel.rotate())
            .collect::<Vec<_>>()
            .join("\n")
    }
}



fn drive_demo() {
    // Wheels of Brand A
    let wheel_a1 = WheelBrandA;
    let wheel_a2 = WheelBrandA;
    let wheel_a3 = WheelBrandA;
    let wheel_a4 = WheelBrandA;

    // Wheels of Brand B
    let wheel_b1 = WheelBrandB;
    let wheel_b2 = WheelBrandB;
    let wheel_b3 = WheelBrandB;
    let wheel_b4 = WheelBrandB;

    // Car with Brand A wheels
    let car_a = Car::new(vec![&wheel_a1, &wheel_a2, &wheel_a3, &wheel_a4]);
    println!("Car with WheelBrandA:\n{}", car_a.move_car());

    // Car with Brand B wheels
    let car_b = Car::new(vec![&wheel_b1, &wheel_b2, &wheel_b3, &wheel_b4]);
    println!("Car with WheelBrandB:\n{}", car_b.move_car());

    // Car with mixed wheels
    let mixed_car = Car::new(vec![&wheel_a1, &wheel_b1, &wheel_a2, &wheel_b2]);
    println!("Car with Mixed Wheels:\n{}", mixed_car.move_car());
}
