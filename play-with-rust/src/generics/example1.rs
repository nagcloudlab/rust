

// without generics

struct Store {
    item_name: String,
    quantity: i32, // Fixed type
}

impl Store {
    fn new(item_name: String, quantity: i32) -> Self {
        Self { item_name, quantity }
    }

    fn display(&self) {
        println!("Store has {} of {}", self.quantity, self.item_name);
    }
}

fn demo() {
    let store = Store::new("Apples".to_string(), 100);
    store.display();
}
