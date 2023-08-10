use libuptest::error::Error;
use libuptest::test_helper::InputHelper;

fn main() -> Result<(), Error> {
    let rand_u128: u128 = InputHelper::get_u128();
    println!("u128: {}", rand_u128);
    let rand_u64: u64 = InputHelper::get_u64();
    println!("u64: {}", rand_u64);
    let rand_u32: u32 = InputHelper::get_u32();
    println!("u32: {}", rand_u32);
    let rand_u8: u8 = InputHelper::get_u8();
    println!("u8: {}", rand_u8);

    let rand_f64: f64 = InputHelper::get_f64();
    println!("f64: {}", rand_f64);
    let rand_f32: f32 = InputHelper::get_f32();
    println!("f32: {}", rand_f32);

    let rand_boolean: bool = InputHelper::get_boolean();
    println!("boolean: {}", rand_boolean);

    let rand_address = InputHelper::get_address();
    println!("Address: {}", rand_address.to_string());
    Ok(())
}
