#![allow(unused_variables,dead_code)]


//-------------------------------------
// PriceMatrix factory 
//-------------------------------------

fn get_price_matrix(pm_version:&str) -> Box<dyn PriceMatrix> {
    match pm_version {
        "v1" => Box::new(PriceMatrixV1::new()),
        "v2" => Box::new(PriceMatrixV2::new()),
        _ => Box::new(PriceMatrixV1::new()),
    }
}

//-------------------------------------
// PriceMatrix
//-------------------------------------

trait PriceMatrix {
    fn get_price(&self, item: &str) -> f64;
}


//-------------------------------------
// PriceMatrix_v1 component
//-------------------------------------

// Excelsheet based..
struct PriceMatrixV1 {
}

impl PriceMatrixV1 {
    fn new() -> Self {
        println!("PriceMatrixV1 instance created");
        PriceMatrixV1 {}
    }
    
}
impl PriceMatrix for PriceMatrixV1 {
    fn get_price(&self, item: &str) -> f64 {
        match item {
            "apple" => 10.00,
            "orange" => 10.000,
            _ => 0.0,
        }
    }
}


//-------------------------------------
// PriceMatrix_v2 component
//-------------------------------------

// Database based..

struct PriceMatrixV2 {
}
impl PriceMatrixV2 {
    fn new() -> Self {
        PriceMatrixV2 {}
    }
    
}

impl PriceMatrix for PriceMatrixV2 {
    fn get_price(&self, item: &str) -> f64 {
        match item {
            "apple" =>20.00,
            "orange" => 20.00,
            _ => 0.0,
        }
    }
    
}


//-------------------------------------
// Billing component
//-------------------------------------

// design issues

// ->dependent & dependency components tight coupled => maintenance problem
// ->unit not possible => dev/bug fix slow

// perfornace issues
// ->too many dependencies are created & droped => memory & performance issue   


// why these issues ?
// ->Billing component itself creating dependency instances

// solution: dont create dependency instances in Billing component, get from factory ( factory design pattern )

// limitation: dependent must know about factory

// better solution:

// dont create, don't find dependency instances, get/inject third party

// inversion of control (IoC) => dependency injection (DI)
// depndency inversion principle (DIP)


struct Billing {
    price_matrix: Box<dyn PriceMatrix>,
}
impl Billing {
    fn new(price_matrix:Box<dyn PriceMatrix>) -> Self {
        Billing {
            price_matrix
        }
    }
    fn calculate_total_price(&self, cart: Vec<&str>) -> f64 {
        let mut total = 0.0;
        for item in cart {
            //let price_matrix = PriceMatrixV1::new(); // dont create
            //let price_matrix = get_price_matrix("v1"); // dont find
            let price = self.price_matrix.get_price(item);
            total += price;
        }
        total
    }
    
}


fn main(){

    //-------------------------------------
    // init phase
    //-------------------------------------

    let price_matrix = get_price_matrix("v2");
    let billing = Billing::new(price_matrix);

    //-------------------------------------
    // run phase
    //-------------------------------------

    let cart = vec!["apple", "orange", "apple"];
    let total_price = billing.calculate_total_price(cart);
    println!("Total Price: {}", total_price);

    //-------------------------------------
    // clean phase
    //-------------------------------------


}