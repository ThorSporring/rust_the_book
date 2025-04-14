
fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition true");
    } else {
        println!("Condition false")
    }
    

    let number;
    number = 10;
    {
        println!("{number}")
    }

    let mut counter = 0;
    let thread_result = loop {
        some_func();
        counter += 1;
        if counter == 10{
            break "done";
        }
    };
    println!("{thread_result}");

    let a = [1, 2, 3, 4, 5, 6];
    let mut index = 0;

    while index < a.len() {
        println!("a[{index}] = {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("{}", element);
    }

    println!("With range: ");
    for element in (1..=4).rev() {
        println!("{element}");
    }

}

fn some_func (){
    println!("some function");
}