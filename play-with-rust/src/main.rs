
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
    fn create_livingthig(lt_type:String) -> Option<Box<dyn LivingThing>> {
        match lt_type.as_str() {
            "Human" => Some(Box::new(Human{name:"".to_string()})),
            "Animal" => Some(Box::new(Animal{name:"".to_string()})),
            "Robot" => Some(Box::new(Robot{name:"".to_string()})),
            _ => None,
        }
    }
    fn manage_livingthing_v1(thing: Box<dyn LivingThing>) {
        thing.eat();
        thing.sleep();
        thing.work();
    }
}

fn main(){

    let human = God::create_livingthig("Human".to_string());
    God::manage_livingthing_v1(human.unwrap());





}