
trait Incrementer{
    type V;
    fn increment(&mut self) -> Self::V;
}


struct Counter1{
    count: i32
}

impl Incrementer for Counter1{
    type V = i32;
    fn increment(&mut self) -> Self::V{
        self.count += 1;
        self.count
    }
    
}

struct Counter2{
    count: i64
}

impl Incrementer for Counter2{
    type V = i64;
    fn increment(&mut self) -> Self::V{
        self.count += 1;
        self.count
    }
    
}

fn main(){

    let mut counter1 = Counter1{count: 0};
    let mut counter2 = Counter2{count: 0};

    println!("Counter1: {}", counter1.increment());
    println!("Counter1: {}", counter1.increment());
    println!("Counter1: {}", counter1.increment());

    println!("Counter2: {}", counter2.increment());
    println!("Counter2: {}", counter2.increment());
    println!("Counter2: {}", counter2.increment());

}