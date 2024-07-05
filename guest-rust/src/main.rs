use std::sync::atomic::AtomicU32;

use exports::test::example::my_interface::{Guest, GuestMyObject};

wit_bindgen::generate!({
    path: "../wit/simple.wit",
    world: "my-world",
});

export!(MyWorld);

struct MyWorld;

impl Guest for MyWorld {
    type MyObject = MyObject;
}

pub struct MyObject(AtomicU32);

impl GuestMyObject for MyObject {
    fn new(a: u32) -> Self {
        Self(AtomicU32::new(a))
    }

    fn set(&self, v: u32) {
        self.0.store(v, std::sync::atomic::Ordering::SeqCst);
    }

    fn get(&self) -> u32 {
        self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
}

fn main() {
    let o = test::example::my_interface::MyObject::new(42);
    o.set(17);
    o.set(o.get() * 2);
    println!("Object is {}", o.get());
}
