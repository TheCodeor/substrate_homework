pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait TrafficLightDuration {
    fn duration(&self) -> u32;
}

impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 60,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("红灯持续时间： {}秒", red.duration());
    println!("黄灯持续时间： {}秒", yellow.duration());
    println!("绿灯持续时间： {}秒", green.duration());
}
