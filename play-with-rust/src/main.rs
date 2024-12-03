

trait DoorListener{
    fn on(&self);
    fn off(&self);
}

struct Light {}
impl DoorListener for Light {
    fn on(&self) {
        println!("Light is on");
    }
    fn off(&self) {
        println!("Light is off");
    }
}


struct Ac{}
impl DoorListener for Ac {
    fn on(&self) {
        println!("AC is on");
    }
    fn off(&self) {
        println!("AC is off");
    }
}

struct Door {
    listeners: Vec<Box<dyn DoorListener>>
}
impl Door {
    fn add_listener(&mut self, listener: Box<dyn DoorListener>) {
        self.listeners.push(listener);
    }
    fn remove_listener(&mut self, listener: Box<dyn DoorListener>) {
    }
    fn open(&self) {
        println!("Door is opened");
        for listener in &self.listeners {
            listener.on();
        }
    }
    fn close(&self) {
        println!("Door is closed");
        for listener in &self.listeners {
            listener.off();
        }
    }
}


fn main(){
    let mut door = Door{listeners: vec![]};
    // door.open();
    // door.close();

    let light= Light{};
    door.add_listener(Box::new(light));
    door.open();
    door.close();

}