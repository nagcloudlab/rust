

// Trait for PriceMatrix
pub trait PriceMatrix {
    fn get_price(&self, item_code: &str) -> f64;
}


pub struct PriceMatrixV1;

impl PriceMatrix for PriceMatrixV1 {
    fn get_price(&self, item_code: &str) -> f64 {
        match item_code {
            "ITEM001" => 100.0,
            "ITEM002" => 200.0,
            "ITEM003" => 300.0,
            _ => 0.0,
        }
    }
}


pub struct PriceMatrixV2;

impl PriceMatrix for PriceMatrixV2 {
    fn get_price(&self, item_code: &str) -> f64 {
        match item_code {
            "ITEM001" => 90.0,
            "ITEM002" => 180.0,
            "ITEM003" => 270.0,
            _ => 0.0,
        }
    }
}


pub struct BillingComponent<'a, T: PriceMatrix> {
    price_matrix: &'a T,
}

impl<'a, T: PriceMatrix> BillingComponent<'a, T> {
    pub fn new(price_matrix: &'a T) -> Self {
        Self { price_matrix }
    }

    pub fn calculate_total(&self, cart_items: &[(&str, u32)]) -> f64 {
        cart_items
            .iter()
            .map(|(item_code, quantity)| self.price_matrix.get_price(item_code) * (*quantity as f64))
            .sum()
    }
}


fn shop_demo() {
    // Use PriceMatrixV1
    let price_matrix_v1 = PriceMatrixV1;
    let billing_v1 = BillingComponent::new(&price_matrix_v1);

    let cart = vec![("ITEM001", 2), ("ITEM002", 3), ("ITEM003", 1)];
    println!("Total with PriceMatrixV1: {}", billing_v1.calculate_total(&cart));

    // Use PriceMatrixV2
    let price_matrix_v2 = PriceMatrixV2;
    let billing_v2 = BillingComponent::new(&price_matrix_v2);

    println!("Total with PriceMatrixV2: {}", billing_v2.calculate_total(&cart));
}
