

// generic struct with lifetimes

struct Store<'a, T> {
    item_name: &'a str,
    quantity: T,
}

impl<'a, T> Store<'a, T> {
    fn new(item_name: &'a str, quantity: T) -> Self {
        Self { item_name, quantity }
    }

    fn display(&self)
    where
        T: std::fmt::Display,
    {
        println!("Store has {} of {}", self.quantity, self.item_name);
    }
}

fn demo() {
    let item_name = "Apples";
    let store = Store::new(item_name, 100);
    store.display();
}
