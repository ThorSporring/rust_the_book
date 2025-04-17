
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn add_to_width(&mut self, adding: u32){
        self.width += adding;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.area() > other.area(){
            return true;
        }
        false
    }
    fn square(size: u32) -> Self{
        Self{
            width: size,
            height: size
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels", area(width1, height1));

    let rect1 = (30,50);
    println!("The area of the rectangle is {} square pixels", area_tuple(rect1));


    
    let mut rect2 = Rectangle{
        width: dbg!(30),
        height: 50
    };
    println!("The area of the rectangle is {} square pixels", area_rectangle(&rect2));

    println!("rect2 is {rect2:#?}");
    dbg!(&rect2);

    println!("The area of the rectangle is {} square pixels", rect2.area());
    rect2.add_to_width(10);
    println!("The area of the rectangle is {} square pixels", rect2.area());

    let rect3 = Rectangle{
        width: 15,
        height: 10
    };

    println!("can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let square = Rectangle::square(10);

    println!("The area of the square is {} square pixels", square.area());

    let mut num: i32 = 10;
    let x = &mut num;

    *x += 1;
    println!("{}{}",*x, num);


}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}