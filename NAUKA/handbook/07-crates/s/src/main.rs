// # Crates i Moduły w Rust
// Narzędzia do ogarniania chaotycznego kodu, gdy projekt mocno rośnie.

// ## Dlaczego ich używamy?
// 1. Enkapsulacja (prywatność): Ukrywasz "bebechy" systemu (np. fizykę gry). Użytkownik widzi tylko prostą funkcję (np. skocz()) i nie zepsuje reszty.
// 2. Unikanie konfliktów nazw: Możesz mieć dwie funkcje init(), jeśli rozdzielisz je modułami (silnik::init() i interfejs::init()).
// 3. Łatwy ekosystem: Dzięki paczkom pobierasz kod innych wpisując nazwę w Cargo.toml.

// ## Czym jest Crate?
// Najmniejsza jednostka, którą widzi kompilator Rusta.
// - Binary crate: Program do uruchomienia (musi mieć main.rs).
// - Library crate: Biblioteka dla innych (ma lib.rs).
//
// Crate root: Główny plik startowy. Od niego zaczyna się budowa drzewa.
// Słowo 'crate::' mówi: "zacznij szukać od czubka mojego aktualnego drzewa".
//
// 

// ## Relacja: main.rs vs lib.rs
// To są DWA OSOBNE kontenery, nawet gdy leżą w jednym folderze!
// - 'crate::' to szukanie u siebie w pliku.
// - Aby użyć biblioteki z poziomu main.rs, musisz wpisać nazwę projektu (np. restauracja::modul::funkcja).

// ==========================================================

// 1. Skróty ścieżek (use)
//tworzymy skrot, zeby moc pisac asparagus zamiast pelnej sciezki
use crate::garden::vegetables::Asparagus;

//zglaszamy kompilatorowi ze mamy tu publiczny modul garden
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("Im growing {plant:?}!");
}

// ==========================================================

// 2. Ścieżki bezwzględne, względne i rodzeństwo
//autorzy rust polecaja sciezke bezwzgledna, poniewaz
//czesciej przenosimy sama funkcje w inne miejsce, niz cala 
//strukture modulow naraz, sciezka bezwzgledna rzadziej sie psuje
mod front_of_house {
    //bedzie blad, poniewaz modul jest prywatny, trzeba dodac pub
    //w rust domyslnie wszystko jest prywatne, natomiast moduly wewnetrze "dzieci"
    //moga miec wglad do modulow rodzicow i wiedza co sie tam dzieje
   /* mod hosting {
        fn add_to_waitlist() {}
    }*/

    //upubliczniamy modul hosting
    pub mod hosting {
        //upubliczniamy sama funkcje
        pub fn add_to_waitlist(){}
         fn seat_at_table() {}
    }
    // ^ a wiec dzieki temu ponizsze sciezki w naszej funkcji
    //zadzialaja bez problemu

      // Submoduł - Obsługa kelnerska
    mod serving {
        fn take_order() {} // przyjmij zamówienie

        fn serve_order() {} // wydaj zamówienie

        fn take_payment() {} // przyjmij płatność
    }
}

//zauwaz ze nie dalismy pub do front_of_house, nie musimy tego zrobic
//poniewaz funkcja eat_at_restaurant znajduje sie na tym samym poziomie
//(sa rodzenstwem), wiec widza sie nawzajem

pub fn eat_at_restaurant() {
    // Ścieżka bezwzględna (zaczyna się od 'crate')
    crate::front_of_house::hosting::add_to_waitlist();

    // Ścieżka względna (zaczyna się od modułu obok)
    front_of_house::hosting::add_to_waitlist();
}

// ========================================================

// 3. Słowo kluczowe super
//czasami jestesmy glebok ow module i chcemy wywolac funkcje
//znajdujaca sie o jeden poziom wyzej u rodzica, zamiast pisac pelna
//sciezke od poczatku crate:: mozemy uzyc slowa kluczowego super

fn deliver_order(){} //funkcja w nadrzednym module, korzeniu

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        //uzywamy super aby wyjsc z back of house
        //i znalezc deliver_order poziom wyzej
        super::deliver_order();
    }

    fn cook_order(){}
}

// ========================================================

// 4. Widoczność w strukturach (struct)
//ZASADA DODAWANIA PUB DZIALA NIECO INACZEJ DLA STRUKTUR I ENUMOW
//jesli dodajesz pub przed struktura, sama struktura stanie sie publiczna,
//ale wszystkie jej pola wewnatrz pozostana prywatne, musisz recznie zdecydowac
//ktore pola maja byc widoczne

mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,      // To pole jest publiczne
        seasonal_fruit: String, // To pole zostaje ukryte (prywatne)
    }

    impl Breakfast {
        // Ponieważ struktura ma prywatne pole, musimy stworzyć publiczną 
        // funkcję ("konstruktor"), która pozwoli nam stworzyć to śniadanie.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Zamawiamy śniadanie używając naszej funkcji budującej
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    
    // Możemy zmienić chleb, bo pole 'toast' jest publiczne
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Poniższa linijka wyrzuciłaby błąd - nie mamy dostępu do 'seasonal_fruit'
    // meal.seasonal_fruit = String::from("blueberries");
}

// ========================================================

// 5. Widoczność w Enumach (enum)
//Z ENUMAMI jest prosto, jesli zrobisz enuma publicznym
//wszystkie jego warianty automatycznie staja sie publiczne
//zostalo to zaprojektowane poniewaz enumy bez dostepu do swoich opcji
//bylby bezuzyteczny i wymog wpisania pub przy kazdej opcji bylaby irytujaca

mod back_of_house3 {
    // Wystarczy jedno 'pub' tutaj...
    pub enum Appetizer {
        Soup,  // ...aby to było publiczne
        Salad, // ...i to też!
    }
}

pub fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}

// ========================================================

// 6. Skracanie ścieżek i importowanie (use)
// dzieki use mozemy przyniesc modul do naszego obecnego zasiegu (zakresu)
mod front_of_house2{
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
//tworzymy skrot do modulu hosting
use crate::front_of_house2::hosting;

pub fn eat_at_restaurant4(){
    //zamiast calej sciezki, uzywamy tylko krotkiej nazwy modulu
    hosting::add_to_waitlist();
}

// Przy importowaniu struktur podajemy pełną ścieżkę do nazwy:
use std::collections::HashMap;
fn main2(){
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// ========================================================

// 7. Rozwiązywanie konfliktów (as), re-eksport i grupowanie
// slowo as tworzy lokalny alias
use std::fmt::Result;
use std::io::Result as IoResult;

fn fun_1() -> Result {}
fn fun_2() -> IoResult<()> {}

// pub use - reeksportowanie (wystawiasz interfejs na zewnątrz)
mod front_of_house3{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

pub use crate::front_of_house3::hosting as hosting_public;

pub fn eat_at_restaurant5(){
    hosting_public::add_to_waitlist();
}

// Zagnieżdżone ścieżki i operator Glob (*)
// zamiast: use std::cmp::Ordering; oraz use std::io;
use std::{cmp::Ordering, io};

// zamiast: use std::io; oraz use std::io::Write;
use std::io::{self, Write};

// Glob - ładuje wszystko, używaj ostrożnie (np. do testów)
use std::collections::*;

// ========================================================

// ## Podział modułów na osobne pliki
// "mod" służy tylko do ZGŁOSZENIA pliku kompilatorowi. Używasz go RAZ per moduł.
// 

// Krok 1 (Zamiast klamer, wstawiamy średnik):
// To wyśle kompilator do szukania pliku src/front_of_house_file.rs
mod front_of_house_file;

// pub use crate::front_of_house_file::hosting;
// pub fn eat_at_restaurant6() {
//     hosting::add_to_waitlist();
// }

// --- Zawartość src/front_of_house_file.rs ---
// pub mod hosting {
//     pub fn add_to_waitlist() {}
// }

// Krok 2 (Submoduły w folderach):
// Jeśli wyciągamy hosting do pliku, w src/front_of_house_file.rs piszemy:
// pub mod hosting;
// 
// Następnie tworzymy FOLDER 'src/front_of_house_file/' i w nim plik 'hosting.rs'
// --- Zawartość src/front_of_house_file/hosting.rs ---
// pub fn add_to_waitlist() {}
