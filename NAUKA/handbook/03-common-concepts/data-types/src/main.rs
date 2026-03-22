use std::io::stdin;

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = x + y;
    println!("{sum}");

    let z: char = 'Z'; //char 4 bajty
    let c = 'c'; //char 4 bajty

    // grupowanie wielu wartosci w jeden typ (tuple type)
    let tup: (i32, f32, u8) = (500, 6.4, 1);

    //destrukturyzacja tuple type
    let tt = (500, 6.4, 1);
    let (x, y, z) = tt;
    println!{"the value of y is: {y}"};

    //dostep przez indeks z kropka

    let dd: (i32, f64, u8) = (500, 6.4, 1);
    let piecset = dd.0;
    let szesccztery = dd.1;
    let jeden = dd.2;

    //tablice musza byc tego samego typu
    let tab = [1, 2, 3, 4, 5];
    let months = ["asdasd", "asdas", "asdads", "asasd", "a"];

    //jawna deklaracja typu i dlugosci
    let a: [i32; 5] = [1,2,3,4,5];

    //inicjalizacja tym samym elementem
    let aa: [3; 5];

    //dostep do elementow
    let first = a[0];
    let last = a[4];

    let mut index = String::new();
    stdin()
        .read_line(&mut index)
        .expect("Failed");

    let index: u32 = index.trim().parse().expect("Not a number");

    let element = a[index];
}
