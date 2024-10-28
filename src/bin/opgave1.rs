fn print_pointer(p: &char) {
    println!("Pointer: {:p}", p);
    println!("Value in memory at pointer location: {}", p);
}

fn main() {
    let char1 = 'a';
    let char2 = 'b';

    let mut p = &char1;

    print_pointer(p);

    // Din kode her

    print_pointer(p);
}