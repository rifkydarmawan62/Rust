pub mod modul;
use modul::{fungsi_publik, fungsi_publik_crate, panggil_fungsi_private, dalam::{self, fungsi_publik_dalam}};

fn main(){
    fungsi_publik();
    fungsi_publik_crate();
    panggil_fungsi_private();
    fungsi_publik_dalam();
}
