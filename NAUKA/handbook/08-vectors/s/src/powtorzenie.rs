//kolekcje przechowuja wiele wartosci
//dane trzymane sa na sterce (heap), co pozwala im dynamicznie rosnac
//i malec w trakcie dzialania programu (nie musi byc znan podczas komplacji)

//3 najwazaniejsze kolekcje:
//vector - przechowuje zmienna liczbe wartosci ulozonych obok siebie
//string - kolekcja znakow
//hash map - pozwlaa na powiazanie wartosci z konkretnym kluczem (implementacja mapy)

//VECTORS
//moga przechowywac wartosci tylko tego samego typu, przydaja sie do list elementow
//np. linii tekstu lub cen w koszyku

//tworenie nowego wektora
let v: Vec<i32> = Vec::new();

//czesciej tworzy sie wektory z poczatkowymi wartosciami, sluzy do tego makro vec!
//rust sam wtedy wywnioskuje odpowiedni typ
let v = vec![1, 2, 3];

//aby dodac elementy na koniec wektora, uzywamy push, wektor musi byc mutowalny
let mut v = Vec::new();
v.push(5);
v.push(6);

//odczytywanie elementow wektora
let v = vec![1, 2, 3];
//1 sposob, & przed typem jest obowiazkowe, wywoluje panic jezeli ma sie zatrzymac w razie bledu
let third: &i32 = &v[2];
println!("Third element: {third}");

//v.get() zwraca none (brak awarii). uzywaj jezeli chcesz bezpiecznie obsluzyc brakujacy element
//za pomoca logiki programu
let third: Option<&i32> = v.get(2); //2 sposob
match third {
    Some(third) => println!("Third el: {third}"),
    None => println!("Third el dont exist");
}

//iterowanie po wartosciach wektora
let v = vec![100, 32, 56];
for i in &v {
    printlnt!{"{i}"};
}

//iteracja mutowalna pozwala na zmiane wartosci, uzywamy operatora dereferencji
//aby dostac sie do wartosci pod referencja i i ja zmodyfikowac
let mut v = vec!{100, 22, 33};
for i in &v {
    *i += 50;
}

//wektory moga przechowywac tylko jeden typ, mozna to obejsc uzywajac enum

enum SpreadsheetCell{
    Int(i32),
    Float(i32),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(3.32),
    SpreadsheetCell::Text(String::from("blue")),
]

//dzieki temu rsut wie w trakcie kompilacji, ile dokladnie pamieci zarezerwowac
//i sprawdza instrukcje match, czy poprawnie obsluzylismy kazdy z wariantow

//wektor jest zwalniany  pamieci, gdy wychodzi z zakresu swojego dzialania
//wraz z nim usuwane sa wszystkie elementy w nim przechowywane


//String
//konkatenacja

let s1 = String::from("elo ");
let s2 = String::from("byku");
let s3 = s1 + &s2; //s1 traci wlasnosc i przestaje byc wazny po tej linijce

//makro format! zalecane przy laczniu wielu elementow
let s = format!("{s2}-{s3}");

//proba odczytania pojedynczego znaku przez indeks skutkuje bledem kompilacji
//string to tak naprawde wektor bajtow Vec<u8>
//znak utf-8 moze zajmowac od 1 do 4 bajtow
//wyciagniecie &s[0] zwrociloby surowy bajt, a nie uzyteczna litere
//rust blokuje to dzizalanie w celu unikniecia cichych bledow logicznych

//zamiast precyzyjnego indeksu, mozemy podac zakres bajtow, aby wyciagnac wycinek
//stringa
let hello = "ebebebebebe";
let s = &hello[0..4]; //pobierasz pierwsze 4 bajty

//najlepszym sposobem na prace z tekstem jest zdecydowanie, jakiego typu
//danych oczekujesz: znakow czy bajtow

//zwraca poszczegolne znaki
for c in "Żó".chars() {
    println!("{c}"); //zwroci Ż, potem ó
}

//zwraca surowe bajty
for b in "Żó".bytes() {
    println!("{b}"); //zwroci 197, 187, 185, 179
}


//HashMap
//kolekcja HashMap<K, V> przechowuje mapowanie kluczy typu
//K na wartosci typu V, przy uzyciu funkcji hashujacej, ktora decyduje
//jak dane sa rozmieszczane w pamieci

//zastosowanie:
//gdy chcesz wyszukiwac dane nie po indeksie, ale po unikalnym kluczu dowolnego typu
//dane przechowywane sa na stercie
//wszystkie klucze musza byc tego samego typu, to samo dotyczy wartosci

//tworzenie hashmapy, trzeba zaiportowac
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

//odczytywanie wartosci
let team_name = String::from("Blue");
//get() zwraca option<&V>
//copied() zamienia Option<&i32> na Option<i32>
//unwrap_or(0) zwraca wartosc, a jesli klucza nie ma (none), zwraca 0
let score = scores.get(&team_name).copied().unwrap_or(0);

//iterowanie po calej mapie
for (key, value) in &scores {
    println!("{key}: {value}")
}

//wlasnosci w hash map
//typy implementujace trait sa kopiowane do mapy
//typy posiadane, tkaie jak string, sa przenoszone (moved), mapa staje sie ich wlascicielem

use std::collections::HashMap;
let field_name = String::from("Elo");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
//jezel do mapy wstawisz referencje, wartosci nie zostana przeniesione
//ale musza zyc co najmniej tak dlugo jak sama hashmapa

//aktualizowanie hashmapy
//kazdy klucz w mapie moze miec w danym momencie tylko jedna przypisana wartosc
//przy kazdej aktualizacji musisz zdecydowac jak potraktowac nowa i stara wartosc

//1. nadpisywanie starej wartosci
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); //nadpsuje 10
println!("{scores:?}"); // wynik {"Blue": 25}

//2. wstawianie, tylko jesli klucz nie istnieje
//sprawdza yellow, nie ma go, wiec dodaje 5-
scores.entry(String::from("Yellow")).or_insert(50);

//sprawdza blue, juz istnieje, wiec nic nie robi
scores.entry(String::from("Blue")).or_insert(50);

//3. aktualizacja na podstawie starej wartosci
//metoda or_insert zwraca mutowalna referencje &mut V do wartosci ukrytej pod kluczem
//mozna to wykorzystac np do zliczania wystapien slow w tekscie

use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    *count += 1; //wymagana dereferencja, aby zmodyfikowac wartosc
}
println!("{map:?}"); // Wynik: {"world": 2, "hello": 1, "wonderful": 1}

//domyslnie HashMap, uzywa algorytmu SipHash, ktory zapewnia ochrone przed atakami
//typu DoS wymierzonymi w tablice hashujace. nie jest to najszybszy algorytm, ale 
//gwaratnuje duze bezpieczenstwo