
// struct with multiple generic types

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    fn display(&self)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("First: {}, Second: {}", self.first, self.second);
    }
}

fn demo() {
    let pair = Pair::new(10, "Ten");
    pair.display();
}
