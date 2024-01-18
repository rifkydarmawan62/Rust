fn main() {
    let mut nilai_desimal_byte : u8 = 0;
    while nilai_desimal_byte >= 0 && nilai_desimal_byte < 255{
        nilai_desimal_byte += 1;
        println!("nilai_desimal_byte = {}", nilai_desimal_byte);
    }
}
