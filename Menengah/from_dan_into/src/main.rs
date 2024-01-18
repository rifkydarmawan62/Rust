use std::convert::From;
use std::convert::Into;
#[derive(Debug)]
struct Integer(u32);
impl From<u32> for Integer{
    fn from(value: u32) -> Self {
        Integer(value)
    }
}
impl Into<u32> for Integer{
    fn into(self) -> u32 {
        self.0
    }
}
#[derive(Debug)]
struct Float(f32);
impl From<f32> for Float{
    fn from(value: f32) -> Self {
        Float(value)
    }
}
impl Into<f32> for Float{
    fn into(self) -> f32 {
        self.0
    }
}

fn main() {
    let str_hello_world = "str \"Hello World\"";
    let string_hello_world = String::from("string \"Hello World\"");
    println!("{}\n{}\n", str_hello_world, string_hello_world);

    let bilangan_bulat_integer = Integer::from(8);
    let bilangan_desimal_float = Float::from(8.1);
    println!("bilangan_bulat_integer = {:?}", bilangan_bulat_integer);
    println!("bilangan_desimal_float = {:?}\n", bilangan_desimal_float);

    let bilangan_bulat_u32 : u32 = bilangan_bulat_integer.into();
    let bilangan_deesimal_f32 : f32 = bilangan_desimal_float.into();
    println!("bilangan_bulat_u32 = {:?}", bilangan_bulat_u32);
    println!("bilangan_desimal_f32 = {:?}", bilangan_deesimal_f32);
}
