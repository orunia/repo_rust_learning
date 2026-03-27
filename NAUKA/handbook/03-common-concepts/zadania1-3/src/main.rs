/*use std::io;

fn main() {
    //deklarujemy tablice i32 ktora ma 5 miejsc
    let arr: [i32; 5] = [1,2,3,4,5];
    let wynik = skrajne_wartosci(arr);
    println!("{:?}", wynik);
}

//funkcja przyjmuje tablice i32 z 5 miejscami i zwraca tuple pierwszego i ostatniego indeksu
fn skrajne_wartosci(arr: [i32; 5]) -> (i32, i32){
    (arr[0], arr[4])
} */


/*use std::io::stdin; 
use rand::Rng;     
use rand::thread_rng; 
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number!");

    //thread_rng daje nam lokalnego generatora liczb losowych
    //gen_range tworzy nam zakres od 1 do 100 wlacznie
    let secret_number = thread_rng().gen_range(1..=100);

    let mut licznik_prob = 0;

    loop {
        println!("Please input your guess.");

        //mutowalna zmienna ktora nam posluzy w dalszym kodzie
        let mut guess = String::new();

        //stdin daje nam dostep do klawiatury
        //read_line bierze tekst, dajemy &mut guess poniewaz pozyczamy dostep do niej, aby wlozyc tekst
        //expect pwooduje panike, program sie wylacza
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    
        //shadowing, tworzymy druga zmienna o tej samej nazwie, po to aby zmienic jej typ
        // guess trim wycina biale znaki, a parse parsuje
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     //w srodku byla liczba, wyciagamy ja i wpisujemy do guess
            Err(_) => continue, //nie bylo liczby, wiec poprzez continue powtarza sie caly kod od gory 
        };

        println!("You guessed: {guess}");
         licznik_prob += 1;

        
        // funkcja compare, sprawdza czy dwie liczby sa takie same
        // dajemy jej referencje do zmiennej (daje nam to zabezpieczenie, poniewaz dzieki referencji nie odapala sie funkcja drop)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),   
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }

        if licznik_prob == 5 {
            println!("Przegrales!");
            break;
        }
    }
} */

/*use std::io::stdin;

fn main(){
    let mut liczba = String::new();
    stdin()
        .read_line(&mut liczba)
        .expect("Failed");

    let liczba: i32 = liczba.trim().parse().expect("chuj");

    let wynik = pomnoz_przez_dwa(liczba);
    println!("{wynik}");
}

fn pomnoz_przez_dwa(x: i32) -> i32 {
    x * 2
}*/

/*fn main(){
    let wynik = stworz_krotke(7.1, 5.3, 4.4);
    println!("{:?}", wynik);

    let (x, y, z) = wynik;

    println!("{x}")
}

//przekazujemy 3 liczby do funkcji, ktora zwraca nam krotke w ktorej sie one zawieraja
//nastepnie dochodzi do destrukturyzacji podczas ktorej liczby sa wkladane do osobnych zmiennych
fn stworz_krotke(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    (x, y, z)
}*/

/*use std::io::stdin;

fn main(){
    loop{
        let mut wpisz_liczbe = String::new();

        stdin()
            .read_line(&mut wpisz_liczbe)
            .expect("Fail");

        let wpisz_liczbe: i32 = match wpisz_liczbe.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if wpisz_liczbe == 0 {
            break;
        };

        let wynik = dodaj_dziesiec(wpisz_liczbe);

        println!("wynik: {wynik}");

    }
}

fn dodaj_dziesiec(x: i32) -> i32 {
    x + 10
}
    */

/*fn main(){
    let liczba = 1;

    let wynik = if liczba > 0 {"dodatnia"} else {"ujemna"}; 
    println!("wynik: {wynik}");

    let mut x = 0;
    let wynik1 = loop {
        x += 2;
        if x == 10 {
            break x * 5;
        }
    };

    println!("wynik2: {wynik1}");

    let mut y = 10;

    while y != 0 {
        if y % 2 == 0 {
            println!("{y}");
        }
        y -= 1;
    }

    let mnozniki = [2, 3, 4, 5];

    for element in mnozniki {
        let w = element * 10;
        println!("{w}");
    }

    for element in (1..=5).rev() {
        println!("{element}");
    }
}*/
