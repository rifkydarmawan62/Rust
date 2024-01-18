#[allow(overflowing_literals)] //Memperbolehkan overflowing
fn main() {
    /*Tidak ada konversi tipe data secara implisit dari bahasa pemrograman Rust. Semua konversi tipe data dilakukan secara eksplisit*/
    let bilangan_desimal = 3.14;
    let bilangan_bulat = bilangan_desimal as u8;
    let karakter_tunggal = bilangan_bulat as char;
    println!("bilangan_desimal = {}", bilangan_desimal);
    println!("bilangan_bulat = {}", bilangan_bulat);
    println!("karakter_tunggal = {}\n", karakter_tunggal);

    println!("1000 as u16 = {}", 1000 as u16);
    println!("1000 as u8 = {}", 1000 as u8); //Menyebabkan overflowing (1000 - 256 - 256 - 256)
}
