//enumy pozwalaja okreslic ze dana wartosc moze byc
//tylko jedna z mozliwych opcji
//w rust mozemy przypisac dane bezposrednio do 
//wariantow enuma, co pozwlaa na pozbycie sie
//dodatkowej struktury

//kazdy wariant moze orzechowywac zupelnie inna ilosc 
//i typ danych
enum IpAddrKind{
    V4(u8, u8, u8, u8), //przechowuje 4 male liczby
    V6(String), //przechowuje ciag znakow
}

fn trasa(rodzaj_ip: IpAddrKind){

}

fn main(){
    //instancja, uzywamy :: zeby kompilator wiedzial
    //ze zarowno ipaddrkind::v4 i v6 naleza do tego
    //samego typu danych - IpAddrKind
    let cztery = IpAddrKind::V4;
    let szesc = IpAddrKind::V6;

    //tworzenie instacji po zaimplementowaniu danych
    let loopback = IpAddrKind::V6(String::from("::1"));
    let dom = IpAddrKind::V4(127, 0, 0, 1);
}*/

//===================================================

//moglibysmy stworzyc cztery rozne struktury dla kazdegi
//z tych typow wiadomosci, ale gdybysmy tak zrobili to 
//zdefiniowanie jedenj funkcji, ktora moglaby przyjac
//kazda z tych wiadomosci, byloby bardzo trudne
//dzieki enumowi wszystkie warianty traktowane sa jako
//jeden wspolny typ danych

enum Message{
    Quit, // brak powiazanych danych
    Move {x: i32, y: i32}, //dane nazwane
    Write(String), //pojedynczy tekst
    ChangeColor(i32, i32, i32); //trzy liczby (jak krotka)
}

impl Message{
    fn call(&self){
        //cialo
    }
}

fn main(){
    let m = Message::Write(String::from("Elo"));
    m.call();
}*/

//===================================================

//rust nie posiada nulli, wprowada bezpieczny koncept o tym
//ze wartosc moze byc obecna lub nie. wykorzystuje do tego
//specjalnego enuma o nazwie option

enum Option<T>{ //<T> to typ generyczny, oznacza to ze wariant some moze przechowywac dowolny typ danych
    None,     //brak wartosci
    Some(T),  //jest wartosc typu t
}

//option wymusza sprawdzenie czy wartosc istnieje lub nie
//uzywamy do tego match

fn plus_jeden(x: Option<i32>) -> Option<i32> {
    match x {
        None, //jesli nie ma to nic nie zwroci
        Some(i) => Some(i + 1) //jezeli jest to dodaje do niej 1
    }
}


//z racji ze option, some i none sa ladowane automatycznue
//nie musisz pisac option::some
fn main(){
    let jakas_liczba = Some(5);
    let jakis_znak = Some("e");

    //jesli tworzymy pusta wartosc, musimy powiedziec kompilatorowi
    //jakiego typu mialaby to byc wartosc, gdyby istniala
    let brak_liczby: Option<i32> = None;

    let piec = Some(5);
    let szesc = plus_jeden(piec); //zwroci Some(6)
    let puste = plus_jeden(None);


    let rzut_kostka = 9;

    match rzut_kostka {
        3 => dodaj_kapelusz(),
        7 => usun_kapelusz(),
        inna_liczba => rzut_kostka(inna_liczba) //lapie wszystko inne i przypisuje do zmiennej inna_liczba
        _ => rzuc_jeszcze_raz(), //lapie kazda wartosc, ale jej nie zapamietuje
        _ => (), //jesli to cokolwiek innego, zignoruj i nie wykonuj zadnego kodu
    }
}*/

//===================================================

//match bierze jakas wartosc, przepuszcza przez liste okreslonych
//wzorcow i gdy tylko wartosc dopasuje sie do wzorca, program 
//wykonuje przypisany do niego kod bloku

#[derive(Debug)]
enum StanUsa{
    Alabama,
    Alaska,
    Albaquerque,
    //..inne stany     
} 

enum Moneta{
    Penny,
    Nickel,
    Dime,
    Quarter(StansUsa), //teraz quarter trzyma w sobie informacje o stanie
}

fn wartosc_w_centach(moneta: Moneta) -> u8 {
    match moneta {
        Moneta::Penny => {
            println!("znalazlem szczesliwego pensa!");
            1
        },
        Moneta::Nickel => 5,
        Moneta::Dime => 10,

        //mozemy nie tylko sprawdzic czy moneta to quearter, ake od razu
        //wyciagnac z niej nazwe stanu i przypisac ja do nowej zmiennej stan
        //jesli podamy do funkcji Moneta::Quarter(StanUsa::Alaska) maszyna
        //sortujaca match ignoruje penny, nickel itd. lapie quarter a slowko
        //stan przyjmuje wartosc StanUas::Alaska, co pozwoli uzyc jej w println!
        Moneta::Quarter(stan) => {
            println!("Stan: {:?}", stan);
            25
        },
    }
}*/

//===================================================

//bywa ze match hest zbyt gadatliwy, zwlaszcza gdy interesuje nas jeden
//konkretny przypadek, tutaj przychodzi nam z pomoca if let oraz nowsze let...else
//To narzedzia ktore sprawiaja ze kod bedzie krotszy i przyjemniejszy w czytaniu

fn main(){
    let maksymalna_konfiguracja = Some(3u8);

    match maksymalna_konfiguraccja {
        Some(max) => println!("Maks wartosc:..."),
        _ => () //irytujacy kod zapychacz
    }

    //zamiast powyzszego kodu z match, mozemy uzyc if let
    //czytamy to nastepujaca: jesli wartosc zmiennej maksymalna_konfiguracja
    //pasuje do wzorca some, to wyciagnij z niej danej i nazwij je max i wykona ponizszy
    //blok kodu. znak = oznacza probe dopasowania wzorca z lewej do wartosci z prawej

    //UZYWAJ if let GDY SWIADOMIE CHCESZ ZIGNOROWAC CALA RESZTE
    if let Some(max) = maksymanlna_konfiguracja {
        println!("Maks wartosc to.... {}", max);
    }

    //mozemy dodac do tego else

    let mut licznik = 0;
    let moneta = Moneta::Quarter(StanUsa::Alaska);

    if let Moneta::Quarter(stan) = moneta {
        println!("To moneta ze stanu: {:?}", stan);
    }else{
        licznik += 1; //wykona sie tylko dla reszty, penny itd.
    }
}

//w programowaniu istnieje pojecie happy path, oznacz to pisanie kodu tak
//aby glowna poprawna logika funkcji nie byla gleboko schowana w dziesiatkach
//nawiasow klamrowych, a ewentualne bledy lub braki danych odrzucalo sie na samym
//poczatku

//zle podejscie:
fn opisz_monete(moneta: Moneta) -> Option<String> {
    if let Moneta::Quarter(stan) = moneta {
        // Główna logika jest schowana wewnątrz if let!
        if stan.istnial_w(1900) {
            Some(format!("{:?} to stary stan!", stan))
        } else {
            Some(format!("{:?} to nowy stan.", stan))
        }
    } else {
        None // Zwracamy błąd na samym końcu
    }
}

//to dziala, ale gdyby logika byla dluzsza kod bylby nieczytelny
//zeby to naprawic mamy let...else, pozwala na dopasowanie wzorca
//a jezeli sie nie uda - antychmisaat uciekniecie z funkcji
//dzieki temu program wie ze zmienna zostala w 100% poprawnie stworzona
fn opisz_monete_2(moneta: Moneta) -> Option<String> {
    //1. probujemy wyciagnac stan. jesli moneta nie ejst quearter...
    let Moneta::Quarter(stan) = moneta {
        return None; //natychmiast przerywamy funkcje!
    }

    //2. jestemy na szczesliwej sciezce
    //zmienna stan jest teraz dostepna normalnie bez zadnych
    //dodatkowych nawiasow

    if stan.istnial_w(1900){
        Some(format!("{:?} to stary stan!", stan));
    }else{
        Some(format!("{:?} to nowy stan", stan));
    }
}*/

/*
- **`enum`** – Tworzy typy danych, które mogą przyjmować różne, wykluczające się warianty (często przechowujące dane).
- **`Option<T>`** – Wbudowany enum chroniący przed "błędem wartym miliard dolarów" (Null).
- **`match`** – Potężna maszyna sortująca, idealna, gdy musisz rozważyć *każdy* możliwy wariant.
- **`if let`** – Skrótowy zapis, gdy interesuje Cię obsługa *tylko jednego* wariantu, a resztę chcesz zignorować.
- **`let...else`** – Idealne do wyciągania danych na samym początku funkcji i uciekania (wczesny `return`), jeśli dane są niepoprawne. */