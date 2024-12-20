use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    // let v = my_vec! {1,2,3};
    // let v = my_vec!(4, 5, 6);
    // let v = my_vec![7, 8, 9];
    // let v = my_vec![1; 5];

    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
        "7".parse()?,
        "8".parse()?,
        "9".parse()?,
    ];
    println!("{:?}", v);
    Ok(())
}
