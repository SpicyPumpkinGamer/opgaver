fn print_pointer(p: &char) {
    println!("Pointer: {:p}", p);
    println!("Value in memory at pointer location: {}", p);
}

fn main() {
    let char1 = 'a'; //location af a bliver udskrevet 2 gange, samme værdi begge gange
    let char2 = 'b';

    let mut p = &char1;

    print_pointer(p);

    let mut p = &char2;

    print_pointer(p);
}
