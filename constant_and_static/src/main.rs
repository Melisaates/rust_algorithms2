fn main() {
    const MY_AGE:i32=35;
    static MY_NAME:&str="Melisa Ateş";

    let her_age:i32=MY_AGE;
    let his_age :i32=MY_AGE;

    println!("Hello, world!");
    println!("she : {}",her_age);
    println!("he : {}",his_age);

    //unsafe
    println!("name : {MY_NAME}");
}
