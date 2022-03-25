use std::vec;

fn main() {
    println!("Hello, world!");

    let result = another_function(0, '0');

    let mut _count = 0;

    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90];

    match a.get(9) {
        Some(index) => print!("Matched {} In Range", index),
        None => print!("OUT OF BOUNDS SELECTION"),
    }

    let s = Student {
        name: String::from("Andrew"),
        level: 1,
        remote: true,
        grades: Grades('A', 'B', 'C', 'D', 3.75),
    };

    println!("The returned value is: {}{}", result.0, result.1);

    'countin_up: loop {
        println!("count = {}", _count);

        _count += 1;

        if _count == 11 {
            break 'countin_up;
        }
    }

    _count = 0;

    let value = loop {
        // assigning to a variable from the result of a loop
        _count += 1;

        if _count == 10 {
            break _count * 2;
        }
    }; // semi colon needed at the end of the loop to end the statement.

    println!("Value = {}", value);

    for element in a {
        println!("element = {} ", element);
    }

    for num in (1..10).rev() {
        println!("number = {} ", num);
    }

    println!(
        "Student Name Is: {}, Student Level Is {}, Student Studies Remotely? {}, There Grades Are {} {} {} {} & The Mark Avergae Is {}",
        s.name, s.level, s.remote,s.grades.0,s.grades.1,s.grades.2,s.grades.3,s.grades.4);

    let we_load = WebEvent::WeLoad(true);
    //let click = MouseClick{x:100,y:250};
    let we_click = WebEvent::WeClick(MouseClick { x: 100, y: 250 });
    let we_keys = WebEvent::WeKeys(KeyPress(String::from("CTRL+"), 'N'));

    println!(
        "Mouse Click Location {:#?} \n Load? {:#?} \n Keypress {:#?}",
        we_click, we_load, we_keys
    );

    let new_car = car_factory();

    println!("Your Cars Colour Is {}, Your Cars Transmission Is {:#?}, Your Car Is Convertible? {}, Your Cars  Mileage Is {} "
            ,new_car.colour, new_car.transmission,new_car.convertible,new_car.milage);

    let three_nums = vec![15, 3, 69];
    println!("Initial Vector: {:?}", three_nums);

    let zeros = [0; 5];
    println!("Zero Vector: {:?}", zeros);

    let mut cars = Vec::<Car>::new();
    cars.push(Car {
        colour: "Blue".to_string(),
        transmission: Transmission::SemiAutomatic,
        convertible: true,
        milage: 15000,
    });

    cars.push(Car {
        colour: "Green".to_string(),
        transmission: Transmission::Automatic,
        convertible: false,
        milage: 55000,
    });

    println!(
        "{:?}{:?}{:?}{:?}",
        cars[0].colour, cars[0].transmission, cars[0].convertible, cars[0].milage
    );

    println!(
        "{:?}{:?}{:?}{:?}",
        cars[1].colour, cars[1].transmission, cars[1].convertible, cars[1].milage
    );
}

fn another_function(x: i32, label: char) -> (i32, char) {
    //println!("X's Value is: {}{} Units", x, label);

    let result = (x, label);

    if result.eq(&(0, '0')) {
        return (99, '9');
    } else {
        result
    }
}

struct Student {
    name: String,
    level: u8,
    remote: bool,
    grades: Grades,
}

struct Grades(char, char, char, char, f64);

#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}
#[derive(Debug)]
enum WebEvent {
    WeLoad(bool),
    WeKeys(KeyPress),
    WeClick(MouseClick),
}

struct Car {
    colour: String,
    transmission: Transmission,
    convertible: bool,
    milage: u32,
}
#[derive(Debug)]
enum Transmission {
    Manual,
    Automatic,
    SemiAutomatic,
}

fn car_factory() -> Car {
    let car = Car {
        colour: String::from("Red"),
        transmission: Transmission::Manual,
        convertible: true,
        milage: 20000,
    };

    car
}
