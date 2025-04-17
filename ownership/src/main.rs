fn main() {
    let mut a = [0, 1, 2, 3, 4];
    a[1] += 1;
    println!("{a:?}");

    let mut s: String = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{} {}", hello, world);

    s.push_str(" jens");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let jens: &str = &s[12..16];
    let slice: &str = &s[..];

    println!("{} {} {}", hello, world, jens);
    println!("{}", slice);

    let first = first_word(&s);
    println!("First: {}", first);


    let second = second_word(&s);
    println!("second: {}", second);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
           return &s[..i];
        }
    }
    &s[..]
}
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first = 0;
    let mut counter = 0;

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            counter += 1;
            if counter == 1
            {
                first = i + 1; //Compensating for space
            }
            if counter == 2
            {
                return &s[first..i];
            }
        }
    }
    &s[..]
}