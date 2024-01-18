fn main() {
    let variabel_immutable : u8 = 21;
    let mut variabel_mutable : u8 = 21;
    let pointer_immutable: &u8 = &variabel_immutable;
    let pointer_mutable = &mut variabel_mutable;
    println!("variabel_immutable = {}", variabel_immutable);
    println!("variabel_mutable = {}", variabel_mutable);
    println!("*pointer_immutable = {}", *pointer_immutable);
    //println!("*pointer_mutable = {}", *pointer_mutable);
    //*pointer_mutable = 10;
    println!("variabel_mutable = {}", variabel_mutable);
}
