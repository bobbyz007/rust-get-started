use std::fmt::Debug;

fn main() {
    // trait object
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);

    // impl trait
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);
    can_fly(Pig);
    can_fly(Duck);
}

#[derive(Debug)]
struct Foo;

trait Bar {
    fn baz(&self);
}
impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self)
    }
}

fn static_dispatch<T>(t: &T) where T: Bar {
    t.baz();
}
// trait object, fat pointer and allocated in heap
fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}

/// static dispatch: impl trait
trait Fly {
    fn fly(&self) -> bool;
}
#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}
impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

// impl used as parameter
fn fly_static(s: impl Fly + Debug) -> bool {
    s.fly()
}
// impl used as return type
fn can_fly(s: impl Fly + Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s)
    } else {
        println!("{:?} can't fly", s)
    }
    s
}
