wit_bindgen::generate!({
    path: "../wit/simple.wit",
    world: "my-world",
//    exports: { "component:demo/test-call": MyCalc },
});

fn main() {
    let o = test::example::my_interface::MyObject::new(42);
    o.set(17);
    o.set(o.get()*2);
//    println!("Object is {}", o.get());
}
