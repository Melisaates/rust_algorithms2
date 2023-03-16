fn main() {
    
    let mut x: i32=5;

    if x>10 {
        x=6;
    }else{
        x=3;
    }
    
    println!("x = {x}");

    let y : u32=if x<10 {4}else{1};
    println!("y = {y}");

    loop{
        println!("hellooo");
        break;
    }

    let m:u32=loop{
        break 14;
    };

    println!("m = {m}");

    'mel:loop{
        loop{
            break 'mel;
        }
    }
    println!("Hello, world!");

    let arr:[u32;2]=[3,19];
    for a in arr {
        println!("a = {a}");
    }
}
