//created following this tutorial: https://www.youtube.com/watch?v=1H9FHhRntAk

#[link( name = "badmath", kind = "static")]
extern "C" {
    fn bad_add(v1:f32,v2:f32)->f32;
}

fn main() {
    println!("Hi from Rust!");
    let _res = unsafe{bad_add(9.0, 21.0)};
    println!("{}?? Are you sure that's right?", _res);
}
