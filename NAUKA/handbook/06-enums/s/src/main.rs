/*ZAD 1
enum ZdarzenieSystemowe{
    Logowanie(String),
    ZrzutPamiecie(u32, u32),
    AwariaGlowna,
    NieznaneBledy([u16; 3])
}

fn obsluz_zdarzenie(zdarzenie: ZdarzenieSystemowe){
    match zdarzenie {
        ZdarzenieSystemowe::Logowanie(tekst) =>
                            println!("Moj tekst: {}", tekst),
        ZdarzenieSystemowe::ZrzutPamiecie(x, y) => 
                            println!("{} {}", x, y),
        ZdarzenieSystemowe::AwariaGlowna =>
                            println!("Awaria jest wariat"),
        ZdarzenieSystemowe::NieznaneBledy(tablica) => 
                            println!("{} {} {}", tablica[0], tablica[1], tablica[2])
    }
}

fn main() {
    let tekst = ZdarzenieSystemowe::Logowanie(String::from("eloo"));
    let zrzut = ZdarzenieSystemowe::ZrzutPamiecie(12, 3);
    let awaria = ZdarzenieSystemowe::AwariaGlowna;
    let nieznane = ZdarzenieSystemowe::NieznaneBledy([12, 3, 4]);

    obsluz_zdarzenie(tekst);
    obsluz_zdarzenie(zrzut);
    obsluz_zdarzenie(awaria);
    obsluz_zdarzenie(nieznane);
}*/

/*ZAD 2
fn podziel_i_znajdz(tekst: Option<&str>) -> Option<usize> {
    match tekst {
        None => None,
        Some(zawartosc) => { //podajemy nazwe zmiennej do ktorej zapiszemy wyciagniety tekst
            match zawartosc.find(' '){ //find leci przez caly tekst i zwraca nam indeks spacji opakowany w some
                None => None,
                Some(indeks) => Some(indeks)
            }
        }
    }
}

fn main(){
    let tekst = Some("Elo mordeczko");
    println!("{:?}", podziel_i_znajdz(tekst));
}*/

/*ZAD 3
fn main(){
    let dane: [Option<i32>; 6] = [Some(4), None, Some(7), Some(10), None, Some(15)];

    let mut suma = 0;
    for i in dane {
        if let Some(x) = i { //wylapuje tylko elementy ktore sa wariantem some, rozpakowuje liczbe i wklada ja do zmiennej x
            if x % 2 == 0 {
               suma += x;
            }
        }
    }

    println!("Wynik: {}", suma)
}*/

/*ZAD 4
enum Rola{
    Admin,
    Moderator,
    Uzytkownik
}

struct Konto{
    nazwa: String,
    rola: Option<Rola>
}

fn usun_komentarz(konto: Konto) -> bool {
    //sprawdzamy czy nasz some posiada role z enuma, jezeli tak to
    //zmienna rolaa dostaje wartosc naszej roli, inaczej instrukcja zwraca false
    let Some(rolaa) = konto.rola else {
        return false;
    };

    //jestesmy na naszej szczesliwej sciezce, zmienna rolaa jest dostepna
    //bez zadnych kropek itd..
    match rolaa {
        Rola::Admin => true,
        Rola::Moderator => true,
        Rola::Uzytkownik => false
    }
}

fn main() {
    let konto = Konto {
        nazwa: String::from("JanKowalski"),
        rola: Some(Rola::Admin),
    };

    println!("{:?}", usun_komentarz(konto));
}*/

/*ZAD 5
enum WynikSensora{
    Temperatura(f64),
    Cisnienie(u32),
    Wilgotnosc(u8),
    Awaria(String)
}

fn wynik_sensora(wynik: WynikSensora){
    match wynik {
        WynikSensora::Temperatura(x) => {
            if x > 100.0 {
                println!("Krytyczne");
            }
        },
        WynikSensora::Awaria(s)=> {
            println!("{}", s);
        },
        _ => (),
    }
}

fn main(){
    let temp = WynikSensora::Temperatura(101.0);
    let cis = WynikSensora::Cisnienie(21);
    let wil = WynikSensora::Wilgotnosc(2);
    let awa = WynikSensora::Awaria(String::from("Blaaaaaaad"));

    wynik_sensora(temp);
    wynik_sensora(cis);
    wynik_sensora(wil);
    wynik_sensora(awa);
}*/

/*ZAD 6
enum Operacja{
    Przelew {
        nadawca: String,
        kwota: Option<f64>
    },
    Wplata(f64),
    Wyplata(f64),
}

fn weryfikuj_przelew(op: Operacja) -> f64 {
    if let Operacja::Przelew{kwota: Some(x), ..} = op {
        return x
    }else{
        return 0.0;
    }
}

fn main(){
    let kwota = Operacja::Przelew{
        nadawca: String::from("Igor"),
        kwota: Some(100.0)
    };

    println!("{:?}", weryfikuj_przelew(kwota));
}
    */