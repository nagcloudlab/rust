
// LT
// - eat()
// - sleep()
// - work()

trait LivingThing {
    fn name(&self) -> String;
    fn eat(&self);
    fn work(&self);
    fn sleep(&self){
        println!("{} is sleeping",self.name());
    }
}

struct Human {
    name: String,
}

impl LivingThing for Human {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn eat(&self) {
        println!("{} is eating", self.name);
    }
    fn work(&self) {
        println!("{} is working", self.name);
    }
}


struct Animal {
    name: String,
}

impl LivingThing for Animal {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn eat(&self) {
        println!("{} is eating", self.name);
    }
    fn work(&self) {
        println!("{} is working", self.name);
    }
}


struct Robot {
    name: String,
}

impl LivingThing for Robot {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn eat(&self) {
        println!("{} is eating", self.name);
    }
    fn work(&self) {
        println!("{} is working", self.name);
    }
}


// Open for extension - Closed for modification Principle
struct God;
impl God {
    fn manage_livingthing_v1<T: LivingThing>(thing: T) {
        thing.eat();
        thing.sleep();
        thing.work();
    }
}

fn main(){

    let human = Human {
        name: "John".to_string(),
    };

    let animal = Animal {
        name: "Dog".to_string(),
    };

    let robot = Robot {
        name: "R2D2".to_string(),
    };

    // God::manage_human(&human);
    // God::manage_animal(&animal);

    God::manage_livingthing_v1(human);
    God::manage_livingthing_v1(animal);
    God::manage_livingthing_v1(robot);


}