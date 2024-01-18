fn fungsi_sebelum_main(){
    println!("Anda memanggil fungsi sebelum main");
}
fn main() {
    fungsi_sebelum_main();
    fungsi_setelah_main();
    let a = ambil_sepuluh();
    println!("a = {}", a);
}
fn fungsi_setelah_main(){
    println!("Anda memanggil fungsi setelah main");
}
fn ambil_sepuluh() -> i8{
    return 10;
}