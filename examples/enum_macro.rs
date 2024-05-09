use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 }, // 不是 unnamed，所以不会生成 From impl
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    println!("{:?}", up);

    let left: Direction = 42.into();
    println!("{:?}", left);
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}
