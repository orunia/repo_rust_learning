fn main() {
    println!("Hello, world!");
    inna_funkcja();
    wypusz_liczbe(5);
    wypusz_wymiar(10, 'z');

    let y = {
        let x = 5; //instrukcja, ma srednik
        x + 1 //wyrazenie, nie ma srednika
    };

    println!("{y}"); // wypisze 6

    let h = plus_jeden(5);
    println!("{h}"); // wypisze 6
}

fn inna_funkcja(){
    println!("Elo";)
}

fn wypusz_liczbe(x: i32){
    println!("{x}")
}

fn wypusz_wymiar(x: i32, unit_label: char){
    println!("Wymiar to: {value}{unit_label}");
}

fn plus_jeden(x: i32) -> i32 {
    x + 1
}