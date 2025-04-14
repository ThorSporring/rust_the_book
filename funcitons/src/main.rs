fn main() {
    println!("Hello, world!");
    let x: i8 = 1;

    another_function(x.try_into().unwrap());
    let y = {
        let x = 4;
        x + (4 * x)
    };
    let u ={ 
        y + 1
    };
    println!("y = {y}");
    println!("u = {u}");

    let z: i16 = 10;
    number(z);
    println!("z = {z}")
}

fn another_function(x: i8){
    println!("Another function {x}");
}

fn number(x: i16) -> i32 {
    x.try_into().expect("DIE")
}
