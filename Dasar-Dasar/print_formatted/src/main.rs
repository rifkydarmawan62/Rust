fn main() {
    println!("{} hari", 31);
    println!("{0} {2} {1}", 1, 2021, "Januari");
    println!("{tanggal} {bulan} {tahun}", tanggal = 1, tahun = 2021, bulan = "Januari");
    /*Print formatted untuk tipe data bilangan*/
    println!("Bilangan heksadesimal 0X{:X}", 255);
    println!("Bilangan heksadesimal 0x{:x}", 255);
    println!("Bilangan desimal {}", 255);
    println!("Bilangan oktal 0o{:o}", 255);
    println!("Bilangan biner 0b{:b}", 255);
    //Menambahkan 4 spasi di sebelah kiri
    println!("{nomor:>5}", nomor = 1);
    //Menambahkan 4 spasi di sebelah kiri menggunakan variabel jumlah_karakter = 5
    println!("{nomor:0>jumlah_karakter$}", nomor=1, jumlah_karakter = 5);
    //Menambahkan angka 0 sebanyak 4 kali di sebelah kiri
    println!("{nomor:0>5}", nomor = 1);
    //Menambahkan angka 0 sebanyak 4 kali di sebelah kanan
    println!("{nomor:0<5}", nomor = 1);

}