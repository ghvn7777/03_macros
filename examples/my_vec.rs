use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3];
    println!("v1: {:?}", v);

    let v2: Vec<u8> = my_vec![];
    println!("v2: {:?}", v2);

    let v3 = my_vec![0; 5];
    println!("v3: {:?}", v3);

    let v4: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];
    println!("v4: {:?}", v4);

    Ok(())
}
