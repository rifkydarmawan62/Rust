fn main() {
    let variabel_non_lokal = 12;
    let mut variabel_mutable = 12;
    println!("variabel_non_lokal sebelum perubahan = {}", variabel_non_lokal);
    println!("variabel_mutable sebelum perubahan = {}\n", variabel_mutable);
    {
        let variabel_lokal = 12;
        let mut variabel_lokal_mutable = 12;
        //variabel_non_lokal = 1; Tidak dapat melakukan perubahan pada variabel immutable di dalam blok tanpa dideklarasikan kembali
        let variabel_non_lokal = 1; //Deklarasikan kembali di dalam blok
        variabel_mutable = 1;
        println!("variabel_lokal dideklarasikan = {}", variabel_lokal);
        println!("variabel_lokal_mutable dideklarasikan {}", variabel_lokal_mutable);
        println!("variabel_non_lokal setelah dideklarasikan kembali di dalam blok = {}", variabel_non_lokal);
        println!("variabel_mutable setelah diubah di dalam blok = {}\n", variabel_mutable);
    }
    //println!("variabel_lokal_mutable setelah keluar dari dalam blok = {}", variabel_lokal_mutable); Tidak dapat diakses di luar blok
    println!("variabel_non_lokal setelah keluar dari dalam blok = {}", variabel_non_lokal);
    println!("variabel_mutable setelah keluar dari dalam blok = {}", variabel_mutable);
}
