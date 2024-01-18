use std::mem::size_of_val;
fn analisa_array(argumen_array : &[u8]){
    println!("Array {:?} ukuran memori {:?} byte", argumen_array, size_of_val(&argumen_array))
}
fn main() {
    //Deklarasi variabel
    const JUMLAH_ELEMEN_ARRAY: usize = 5usize;
    let array : [u8; JUMLAH_ELEMEN_ARRAY] = [1, 2, 3, 4, 5];
    let vector: Vec<u8> = vec![1, 2, 3, 4, 5];
    let tuple_tipe_data_acak : (u8, f32, String) = (115, 27.3f32, String::from("String"));

    //Mengindeks array
    println!("Array {:?} &[{:?}] = {:?}", array, 0, &array[0]);
    println!("Array {:?} &[{:?} .. {:?}] = {:?}", array, 0, 2, &array[0 .. 2]);
    analisa_array(&array);
    //Iterator array
    for item_array in array.iter(){
        println!("iterator array = {}", item_array);
    }
    println!("vector {:?}", vector);
    //Iterator vector
    for item_vector in vector.iter(){
        println!("iterator vector = {}", item_vector);
    }
    //Mengindeks tuple
    println!("tuple_tipe_data_acak {:?} tuple_tipe_data_acak.1 = {:?}", tuple_tipe_data_acak, tuple_tipe_data_acak.1);
    println!("tuple_tipe_data_acak {:?}", tuple_tipe_data_acak);
    println!("tuple_tipe_data_acak {:?} ukuran memori {:?} byte", tuple_tipe_data_acak, size_of_val(&tuple_tipe_data_acak));
}