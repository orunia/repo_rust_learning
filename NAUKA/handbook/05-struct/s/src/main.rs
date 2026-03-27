/*ZAD1
#[derive(Debug)] // mowi kompilatorowi, zeby automatycznie generowal kod ktory pozwala wypluc cala strukture do konsoli
struct Zamowienie {
    produkt: String,
    ilosc: u32,
    cena_jednostkowa: f64,
}

fn main() {
    // instancja struktury
    let produkt1 = Zamowienie {
        produkt: String::from("Chuj"),
        ilosc: 3,
        cena_jednostkowa: 65.99,
    };

    //instancja struktury, ale tym razem zmieniamy jedna wartosc, a reszte zostaje przeniesiona z pierwszej instancji
    // co oznacza ze produkt1 nie mozna juz uzywac
    let produkt2 = Zamowienie {
        ilosc: 5,
        ..produkt1
    };

    println!("{:?}", produkt2);
}*/

/*ZAD2
#[derive(Debug)]
struct Wektor3d(f64, f64, f64);

struct PustySygnal;

fn main(){
    let wektor = Wektor3d(4.3, 4.4, 4.5);
    let sygnal = PustySygnal;

    let Wektor3d(x, y, z) = wektor;

    println!("{} {}  {}", x, y, z);
}*/

/*ZAD3
#[derive(Debug)] // mowi kompilatorowi, zeby wygenerowal kod, ktory da sie wypluc cala strukture
struct KontoBankowe {
    wlasciciel: String,
    saldo: i32,
}

// metody naszej struktury
// wplac: metoda ma wejsc do srodka "skrzynki" i zmodyfikowac dane
// wypiszsaldo: dostaje referencje do odczytu tego co jest w tej skrzynce wlasnej skrzynki
impl KontoBankowe {
    fn wplac(&mut self, kwota: i32){
        self.saldo += kwota
    }

    fn wypiszsaldo(&self){
        println!("{:?}", self)
    }
}

fn main(){
    let mut konto = KontoBankowe{
        wlasciciel: String::from("Igor"),
        saldo: 44,
    };

    //dobry zapis, ale nadmiarowy. w rust metody ktore sa w 
    //impl wywolujemy za pomoca kropki

    //let noweSaldo = KontoBankowe::wplac(&mut konto, 10);
    //let wypisz = KontoBankowe::wypiszsaldo(&mut konto);

    konto.wplac(10);
    konto.wypiszsaldo();
}*/

/*ZAD4
#[derive(Debug)]
struct KontoBankowe {
    wlasciciel: String,
    saldo: i32,
}

impl KontoBankowe {
    fn wplac(&mut self, kwota: i32){
        self.saldo += kwota
    }

    fn wypiszsaldo(&self){
        println!("{:?}", self)
    }

    fn otworz(imie: String) -> Self {
        Self{
         wlasciciel: imie,
         saldo: 0,
       }
    }
}

fn main(){
    let mut konto = KontoBankowe{
        wlasciciel: String::from("Igor"),
        saldo: 44,
    };

    konto.wplac(10);
    konto.wypiszsaldo();
    
    let im: String = "Chuj".to_string();
    let mut nowa = KontoBankowe::otworz(im);

    println!("{:?}", nowa);
}*/

/*ZAD5
#[derive(Debug)]
struct Silnik {
    model: String,
    moc: u32,
}

fn main(){
    let mnoznik: u32 = 2;

    let instancja = Silnik {
        model: "okej".to_string(),
        moc: dbg!(150 * mnoznik) //println na sterydach, przejmuje wartosc, wypisuje i zwraca z powrotem, wstrzykujemy ja w "srodek" kodu bez
        // potrzeby przerywania go, jak jest git to idzie dalej i si
    };

    println!("{:#?}", instancja);
}*/

/*ZAD6
struct Magazyn {
    pojemnosc: u32,
    zajete_miejsce: u32
}

impl Magazyn {
    fn czy_zmiesci(&self, towar: u32) -> bool {
        self.pojemnosc >= self.zajete_miejsce &&
        self.pojemnosc >= towar
    }
}

fn main(){
    let mag1 = Magazyn {
        pojemnosc: 44,
        zajete_miejsce: 22,
    };

    let prawda: bool = mag1.czy_zmiesci(50);
    println!("{prawda}");
}*/

/*ZAD7 I 7.5
#[derive(Debug)]
struct TajnyDokument {
    tresc: String,
}

impl TajnyDokument {
    fn zniszcz_i_przeczytaj(&self) -> String {
        self.tresc.clone()
    }
}

fn main(){
    let instancja = TajnyDokument {
        tresc: "Chuuuuuuj".to_string(),
    };

    instancja.zniszcz_i_przeczytaj();
    println!("{:?}", instancja);
}
    
impl TajnyDokument {
    // Usuwamy '&' przed self -> funkcja teraz "pożera" instancję
    fn zniszcz_i_przeczytaj(self) -> String {
        self.tresc // Przenosimy własność Stringa na zewnątrz
    } // Tutaj instancja dokumentu zostaje usunięta z pamięci (drop)
}

fn main() {
    let instancja = TajnyDokument {
        tresc: "Tajne dane".to_string(),
    };

    let dane = instancja.zniszcz_i_przeczytaj();
    
    // println!("{:?}", instancja); // To by teraz wywaliło BŁĄD - dokument nie istnieje!
    println!("Odebrane dane: {}", dane);
}*/

/*ZAD 8
struct Zgloszenie{
    opis: String,
}

impl Zgloszenie{
    fn pierwsze_slowo(&self) -> &str {
        match self.opis.find(' ') { // przechodzi przez tekst w poszukiwaniu spacji
            Some(i) => &self.opis[..i], //wycinek do spacji
            None => &self.opis, // jezeli brak spacji, to tekst zostaje przekazany
        }
    }
}

fn main(){
    let zgloszenie = Zgloszenie {
        opis: String::from("Haaaj guuurl"),
    };

    let wynik = zgloszenie.pierwsze_slowo();

    //zgloszenie.opis.clear();

    println!("{wynik}");
}*/

/*ZAD9
#[derive(Debug)]
struct Gracz{
    pseudonim: String,
    poziom: u32,
}

impl Gracz{
    //funkcja stowarzyszona, sluzy do budowania obiektu od zera, zebys w main nie musial pamietac jakie pole ma strukture
    //zwraca nam nowa strukture, dlatego w main zamiast implementacji, uzywamy tej funkcji
    fn nowy(nick: &str) -> Self { // Self to alias na nazwe struktury
        Self{
            pseudonim: nick.to_string(),
            poziom: 1
        }
    }
}

fn awansuj(gracz: &mut Gracz){
        gracz.poziom += 1
}

fn main(){
    let mut gracz = Gracz::nowy("Igor"); // tworzy struct gracza o nazwie igor

    awansuj(&mut gracz); // przekazujemy gracza Igor do funkcji awansuj, ktora wchodzi do pamieci i zmienia jego poziom przy zachowaniu imienia

    println!("{:#?}", gracz);
}*/
