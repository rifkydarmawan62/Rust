fn main() {
    //Perulangan loop dengan range
    for number in 1..=5 {
        println!("Nomor = {}", number);
    }

    // Perulangan loop untuk iterator list
    let daftar_buah = ["apel", "pisang", "ceri"];
    for item_buah in daftar_buah.iter(){
        println!("Buah {}", item_buah);
    }
}
