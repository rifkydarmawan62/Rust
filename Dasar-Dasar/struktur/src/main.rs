#[derive(Debug)]
struct Koordinat{
    x : i64,
    y : i64,
}
struct Persegi{
    sisi_atas_bawah : Koordinat,
    sisi_kiri_kanan : Koordinat,
}
fn main() {
    //
    let posisi = Koordinat{x : 20, y : 100};
    println!("Posisi x = {}\nPosisi y = {}\n", posisi.x, posisi.y);
    //Destruktur
    let Koordinat {x : sumbu_x, y : sumbu_y} = posisi;
    println!("Sumbu x = {}\nSumbu y = {}\n", sumbu_x, sumbu_y);
}
