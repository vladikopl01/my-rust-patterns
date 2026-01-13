trait Toy {
    fn log(&self);
}

struct Car;
struct Robot;

impl Toy for Car {
    fn log(&self) {
        println!("This is a car toy");
    }
}

impl Toy for Robot {
    fn log(&self) {
        println!("This is a robot toy");
    }
}

enum ToyType {
    Car,
    Robot,
}

struct ToyFactory;
impl ToyFactory {
    fn create_toy(toy_type: ToyType) -> Box<dyn Toy> {
        match toy_type {
            ToyType::Car => Box::new(Car),
            ToyType::Robot => Box::new(Robot),
        }
    }
}

pub fn factory_pattern() {
    let car = ToyFactory::create_toy(ToyType::Car);
    car.log();

    let robot = ToyFactory::create_toy(ToyType::Robot);
    robot.log();
}
