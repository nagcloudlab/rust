

// with generics

struct Store<T> {
    item_name: String,
    quantity: T, // Generic type
}

impl<T> Store<T> {
    fn new(item_name: String, quantity: T) -> Self {
        Self { item_name, quantity }
    }

    fn display(&self)
    where
        T: std::fmt::Display, // Trait bound to enable printing
    {
        println!("Store has {} of {}", self.quantity, self.item_name);
    }
}

fn demo() {
    let store_int = Store::new("Apples".to_string(), 100); // i32
    let store_float = Store::new("Oranges".to_string(), 25.5); // f64
    let store_string = Store::new("Bananas".to_string(), "a lot".to_string()); // String

    store_int.display();
    store_float.display();
    store_string.display();
}


