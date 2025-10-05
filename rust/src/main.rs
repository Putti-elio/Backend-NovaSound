fn main() {
    println!("Hello, world!");
    call_a();

    while true {
        println!("Hello, world!");
    }
}


fn call_a(){
    println!("call B...");
    call_b();
}

fn call_b(){
    println!("A calling me....");
}