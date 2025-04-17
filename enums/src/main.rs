enum IpAddr{
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Arkansas
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Arkansas => {
                    println!("arrrKANSAS");
                    25

                }
                UsState::Alabama => {
                    println!("Allabimmi");
                    10
                }
            }
        }
    }
}

fn decr_twice_v1(n: u32) -> Option<u32> {

    match n {
  
      0 => None,
  
      1 => None,
  
      n2 => Some(n2 - 2)
  
    }

  }
  
  
  fn decr_twice_v2(n: u32) -> Option<u32> {
  
    if n == 0 {
  
      None
  
    } else if n == 1 {
  
      None
  
    } else {
  
      Some(n - 2)
  
    }
  
  }

fn main() {
    let home = IpAddr::V4(String::from("12.1.1.1"));

    let cents = value_in_cents(Coin::Nickel);
    dbg!(cents);
    let cents = value_in_cents(Coin::Quarter(UsState::Alabama));
    dbg!(cents);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);

    let mut a = 10;
    let mut b = 10;
    
    let a = decr_twice_v1(a);
    let b = decr_twice_v2(b);

    dbg!(a);
    dbg!(b);


    // let cents = value_in_cents(Coin::Quarter(UsState::Arkansas));
    // dbg!(cents);

}
