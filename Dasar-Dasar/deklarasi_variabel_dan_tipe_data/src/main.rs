fn main() {
    //Deklarasi Variabel Tipe Data Skalar
    let variabel_boolean : bool;
    /*Unsigned integer*/
    let variabel_u8 : u8; //unsigned 8 bit
    let variabel_u16 : u16; //unsigned 16 bit
    let variabel_u32 : u32; //unsigned 32 bit
    let variabel_u64 : u64; //unsigned 64 bit
    let variabel_u128 : u128; //unsigned 128 bit
    /*Signed integer*/
    let variabel_i8 : i8; //signed 8 bit
    let variabel_i16 : i16; //signed 16 bit
    let variabel_i32 : i32; //signed 32 bit (default)
    let variabel_i64 : i64; //signed 64 bit
    let variabel_i128 : i128; //signed 128 bit
    /*Float*/
    let variabel_f32 : f32; //32 bit
    let variabel_f64 : f64; //64 bit (default)

    //variabel_mutable. Nilai variabel mutable dapat diubah, tetapi tipe data variabel mutable tidak dapat diubah tanpa dideklarasikan kembali
    let mut variabel_mutable : i32 = 10;
    let _variabel_immutable : u8 = 12;
    println!("let mut variabel_mutable = {}", variabel_mutable);
    println!("let _variabel_immutable = {}", _variabel_immutable);
    //Mendeklarasikan variabel_mutable kembali
    let variabel_mutable = true;
    println!("let variabel_mutable = {}", variabel_mutable);
}
