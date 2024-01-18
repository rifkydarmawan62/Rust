fn main() {
    loop{
        println!("Perulangan loop tidak akan pernah berhenti tanpa break");
        break;
    }
    'blok_luar : loop{
        println!("Memasuki loop bagian luar");
        'blok_dalam : loop{
            println!("Memasuki loop bagian dalam");
            let break_luar : bool = false;
            if break_luar{
                println!("Loop bagian luar telah di break");
                break 'blok_luar;
            }else{
                println!("Loop bagian dalam telah di break");
                break 'blok_dalam;
            }
        }
        println!("Memasuki loop bagian luar kembali");
        break 'blok_luar
    }
    println!("Telah keluar dari perulangan loop");
    let nilai_return_dari_loop = loop{
        break 0
    };
    println!("nilai_return_dari_loop = {}", nilai_return_dari_loop);
}
