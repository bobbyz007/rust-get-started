use std::any::Any;
use std::f64::consts::PI;

#[allow(unused_variables)]
pub fn patterns() {
    let a = 2;
    match a {
        1 | 2 => println!("1 or 2 matched"),
        _ => println!("other matched"),
    }

    let favorite_color: Option<&str> = Some("sdf");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(s) = favorite_color {
        println!("using your favorite color, {}, as the background", s);
    } else if is_tuesday {
        println!("tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple as the background color");
        } else {
            println!("using orange as the background color");
        }
    } else {
        println!("using blue as the background color");
    }

    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(value) = stack.pop() {
        println!("pop: {}", value);
    }

    let v = vec!['a', 'b', 'c'];
    for (i, v) in v.iter().enumerate() {
        println!("{} is at index {}", v, i);
    }

    let (x, y, ..) = (1, 2, 3, 4);
    println!("x: {}, y: {}", x, y);

    let x = 5;
    match x {
        1 ..= 5 => println!("1~5 matched"),
        _ => println!("other matched"),
    }

    let p = Point { x: 0, y: 7 };
    let Point{x, y} = p;
    println!("x: {}, y: {}", x, y);

    match p {
        Point { x, y: 0 } => println!("on the x axis: {}", x),
        Point { x: 0, y } => println!("on the y axis: {}", y),
        Point { x, y } => println!("other "),
    }

    // let msg = Message::ChangeColor(0, 160, 255);
    let msg = Action::Move { x: 1, y: 2 };
    match msg {
        Action::Quit => {
            println!("The quit variant");
        }
        Action::Move { x, y } => {
            println!("move : {}, {}", x, y);
        }
        Action::Write(text) => println!("text message: {}", text),
        Action::ChangeColor(r, g, b) => println!("change color : {}, {}, {}", r, g, b),
    }

    // match 匹配array数组 或 切片
    let numbers = (2, 4, 8, 16, 32);
    match  numbers {
        (x, .., z) => println!("some number: {}, {}", x, z),
    }

    let number = Some(41);
    match number {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("other: {}", x),
        _ => println!("others"),
    }

    let msg = Message::Hello { id: 103 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => println!("found an id range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("found range between 10 and 12:"),
        Message::Hello { id } => println!("other range: {}", id),
    }

}
enum Message {
    Hello {id: i32}
}

#[allow(dead_code)]
enum Action {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct Point {
    x: i32,
    y: i32
}

// builder pattern in rust
pub fn pattern_builder() {
    let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
    println!("pattern builder area: {:?}", c.area());
}

#[allow(dead_code)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}
impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 0.0 }
    }
}
impl CircleBuilder {
    fn x(&mut self, x: f64) -> &mut Self {
        self.x = x;
        self
    }
    fn y(&mut self, y: f64) -> &mut Self {
        self.y = y;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut Self {
        self.radius = radius;
        self
    }
    fn build(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

// visitor pattern in rust
pub fn pattern_visitor() {
    let house = House::new();
    // simply print out the house elements
    house.accept(&HouseElementListVisitor::new());
    println!();
    // do something with the elements of a house
    house.accept(&HouseElementDemolishVisitor::new());
}
trait HouseElement {
    fn accept(&self, visitor: &dyn HouseElementVisitor);
    fn as_any(&self) -> &dyn Any;
}
trait HouseElementVisitor {
    fn visit(&self, element: &dyn HouseElement);
}
struct House {
    components: Vec<Box<dyn HouseElement>>,
}
impl House {
    fn new() -> Self {
        House {
            components: vec![Box::new(LivingRoom::new())],
        }
    }
}
impl HouseElement for House {
    fn accept(&self, visitor: &dyn HouseElementVisitor) {
        for component in self.components.iter() {
            component.accept(visitor);
        }
        visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any { self }
}

struct LivingRoom;
impl LivingRoom {
    fn new() -> Self { LivingRoom }
}
impl HouseElement for LivingRoom {
    fn accept(&self, visitor: &dyn HouseElementVisitor) {
        visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any { self }
}

struct HouseElementListVisitor;
impl HouseElementListVisitor {
    fn new() -> Self { HouseElementListVisitor }
}

impl HouseElementVisitor for HouseElementListVisitor {
    fn visit(&self, element: &dyn HouseElement) {
        match element.as_any() {
            house if house.is::<House>() => println!("Visiting the house..."),
            living if living.is::<LivingRoom>() => println!("Visiting the Living room..."),
            _ => {}
        }
    }
}
struct HouseElementDemolishVisitor;
impl HouseElementDemolishVisitor {
    pub fn new() -> Self {
        HouseElementDemolishVisitor
    }
}
impl HouseElementVisitor for HouseElementDemolishVisitor {
    fn visit(&self, element: &dyn HouseElement) {
        match element.as_any() {
            house if house.is::<House>() => println!("Annihilating the house...!!!"),
            living if living.is::<LivingRoom>() => println!("Bombing the Living room...!!!"),
            _ => {}
        }
    }
}
