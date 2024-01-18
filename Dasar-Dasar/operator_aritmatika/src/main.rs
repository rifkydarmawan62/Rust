fn main() {
    //Penjumlahan unsigned integer 32 bit
    println!("1 + 2 = {}", 1u32 + 2);
    //Pengurangan signed integer 32 bit
    println!("1 - 2 = {}", 1i32 - 2);

    //Operator logika boolean
    println!("true = {}", true);
    println!("!true = {}", !true);
    println!("Logika AND");
    println!("true && true = {}", true && true);
    println!("true && false = {}", true && false);
    println!("false && true = {}", false && true);
    println!("false && false = {}\n", false && false);

    println!("Logika OR");
    println!("true || true = {}", true || true);
    println!("true || false = {}", true || false);
    println!("false || true = {}", false || true);
    println!("false || false = {}\n", false || false);

    //Operator bitwise
    println!("0011 AND 0101 = {:04b}", 0b0011u8 & 0b0101u8);
    println!("0011 OR 0101 = {:04b}", 0b0011u8 | 0b0101u8);
    println!("0011 XOR 0101 = {:04b}", 0b0011u8 ^ 0b0101u8);
}