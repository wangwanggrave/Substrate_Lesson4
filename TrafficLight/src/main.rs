trait TrafficLight {
    fn duration(&self) -> u32;
}
    

enum LightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLight for LightColor{
    fn duration(&self) -> u32{
        match self{
            LightColor::Red => 10,
            LightColor::Yellow => 5,
            LightColor::Green => 15,
        }
    }
}

fn main(){
    let red_light = LightColor::Red;
    let yellow_light = LightColor::Yellow;
    let green_light = LightColor::Green;

    println!("Red light lasts {} seconds", red_light.duration());
    println!("Yellow light lasts {} seconds", yellow_light.duration());
    println!("Green light lasts {} seconds", green_light.duration());
}