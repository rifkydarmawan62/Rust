use std::u8;

fn main() {
    let nilai : u8 = 10;
    print!("Nilai {} ", nilai);
    match nilai{
        nilai if nilai < 1 => println!("Terlalu Rendah"),
        1 => println!("Sangat rendah"),
        2 => println!("Rendah"),
        3 => println!("Cukup"),
        4 => println!("Tinggi"),
        5 => println!("Sangat Tinggi"),
        nilai if nilai > 5 => println!("Terlalu Tinggi"),
        _ => println!("Tidak Diketahui"),
    }
    //Percabangan match pada nilai variabel digit_biner
    let boolean = true;
    let digit_biner : u8 = match boolean{
        true => 1,
        false => 0,
    };
    println!("boolean \"{}\" = digit_biner \"{}\"\n", boolean, digit_biner);
}