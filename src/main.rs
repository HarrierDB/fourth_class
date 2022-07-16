struct TrafficLight{
    time: String,
    color: String,
}

impl ReturnTime for TrafficLight {
    fn returntime(&self) {
        println!("{}", self.time)
    }
}

pub trait ReturnTime {
    fn returntime(&self);
}

fn main() {
    let light = TrafficLight{time: String::from("20s"), color:String::from("Yellow")};
    light.returntime();
}
