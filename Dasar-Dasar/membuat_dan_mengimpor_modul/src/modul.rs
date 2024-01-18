pub fn fungsi_publik(){
    println!("Anda memanggil modul::fungsi_publik()");
}
fn fungsi_private(){
    println!("Anda memanggil modul::fungsi_private()");
}
pub fn panggil_fungsi_private(){
    println!("Anda memanggil modul::panggil_fungsi_private()");
    fungsi_private();
}
pub mod dalam{
    pub fn fungsi_publik_dalam(){
        println!("Anda memanggil modul::dalam::fungsi_publik_dalam()");
    }
    fn fungsi_private_dalam(){
        println!("Anda memanggil modul::dalam::fungsi_private_dalam()");
    }
}
pub(crate) fn fungsi_publik_crate(){
    println!("Anda memanggil modul::fungsi_publik_crate()");
}