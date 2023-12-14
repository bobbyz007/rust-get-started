use std::fmt::Debug;

#[derive(Debug)]
pub struct Foo;

pub trait Bar {
    fn baz(&self);
}
impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self)
    }
}

pub fn static_dispatch<T>(t: &T) where T: Bar {
    t.baz();
}
// trait object, fat pointer and allocated in heap
pub fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}

/// static dispatch: impl trait
pub trait Fly {
    fn fly(&self) -> bool;
}
#[derive(Debug)]
pub struct Duck;
#[derive(Debug)]
pub struct Pig;

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
pub fn fly_static(s: impl Fly + Debug) -> bool {
    s.fly()
}
// impl used as return type
pub fn can_fly(s: impl Fly + Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s)
    } else {
        println!("{:?} can't fly", s)
    }
    s
}
