fn main() {
    
    //slice str
    
    let x:String = String::from("hello melisa");
    let y:&str = &x[6..8];
    let new_x=my_slice(&x);
    
    //error code
    //let new_y=my_slice(&y);
    
    let _x=my_slice2(&x);
    let _y=my_slice2(&y);

    println!("x -> {}",x);
    println!("y -> {}",y);
    println!("new_x -> {new_x}");
    //println!("new_y -> {new_y}");
    println!("{_x}");
    println!("{_y}");


    fn my_slice (s:&String) -> &str{
        &s[..2]
    }

    fn my_slice2 (s:&str) -> &str{
        &s[..]
    }
}
