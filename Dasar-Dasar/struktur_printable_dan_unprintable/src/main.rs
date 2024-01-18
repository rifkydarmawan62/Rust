fn main() {
    /*struktur tidak dapat dicetak*/
    struct UnPrintable(i32);

    /*struktur dapat dicetak*/
    #[derive(Debug)]
    struct Printable(i32);

    //println!("struktur unprintable = {:?}", UnPrintable(20));
    println!("struktur printable = {:?}", Printable(20));
}
