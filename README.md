# assert_into

For when writing .try_into().unwrap() feels to long.

´´´
fn main() {
    let a: u32 = (-1i32).assert_into();
}
´´´

Gives you: thread 'main' panicked at '-1 is out of range for type u32 (TryFromIntError(()))', src/main.rs:19:13