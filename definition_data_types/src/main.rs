fn main() {
    //scalar

    let i:u32=4;
    let f:f32=4.3;
    let c:char='m';
    let s:&str="melisa ateş";
    let _s:String=String::from("melisa ateşş");
    

    println!("Hello, world!");
    println!("{i}");
    println!("{f}");
    println!("{c}");
    println!("{s}");
    println!("{_s}");

    //compound
    
    //specific types, fixed size
    let arr:[u32;3]=[1,2,3];
    println!("arr = {:?}",arr[0]);//debug
    println!("arr_length = {}",arr.len());

    //specific types, resizable
    let mut vec:Vec<i32>=Vec::with_capacity(2);
    vec.push(5);
    vec.push(6);
    vec.push(8);
    println!("vec = {:?}",vec.get(0));
    println!("vec = {}",vec.len());

    //different types, fixed size
    let tup:(i32,f64)=(3,5.4);
    println!("tup = {tup:?}");
    println!("tup2 = {}",tup.1);

}
