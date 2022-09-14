//https://dhghomon.github.io/easy_rust/Chapter_53.html

fn main(){
    box1();
}

fn box1(){
    let my_box = Box::new(1);
    let an_integer = *my_box;
    println!("{:?}", my_box);
    println!("{:?}", an_integer);
}