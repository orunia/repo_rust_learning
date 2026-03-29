#[derive(Debug)]
struct Material {
    nazwa: String,
    ilosc: u32,
    cena_jednostkowa: f64
}

struct PlacBudowy{
    magazyn: [Material; 5], //tablica majaca 5 miejsc typu struktur materialu
    liczba_pozycji: usize
}

impl PlacBudowy{
    fn nowy() -> Self {
        Self{
            //tablica w rust nie moze byc pusta, musi miec zarezerwowana
            //pamiec dla wszystkich swoich elementow
            //mamy tablice obiektow ktorych nie da sie latwo skopiowac
            //wiec obchodzimy system przy pomocy std::array:from_fn
            //jest to tasma produkcyjna przechodzaca po kazdym indeksie
            //tworzaca obiekty struktur
            // |_| to nasza funkcja anonimowa, kreski to miejsce na argumenty
            //a _ ignoruje numer indeksu, bo kazdy ma byc takim sam
            //wiec nas to nie obchodzi
            magazyn: std::array::from_fn(|_| Material{
                nazwa: String::from("Puste"),
                ilosc: 0,
                cena_jednostkowa: 0.0
            }),
            liczba_pozycji: 0
        }
    }

    fn dodaj(&mut self, nazwa: String, ilosc: u32, cena: f64){
        if self.liczba_pozycji < 5{
            self.magazyn[self.liczba_pozycji] = Material {
                nazwa: nazwa.clone(), 
                ilosc, //skrocony napisa bo nazwy takie same
                cena_jednostkowa: cena
            };

            self.liczba_pozycji += 1;
            println!{"Dodano {}, miejsce {} z 5", nazwa, self.liczba_pozycji};
        }else{
            println!{"Brak miejsca"};
        }
    }

    fn wydaj(&mut self, nazwa_szukana: &str, sztuki: u32){
        //wewnatrz fora nie uzywamy self, poniewaz nie mozemy
        //jednoczesnie posiadac mutowalnej referencji oraz odwolywac
        //sie do calego obiektu self, gdyz prowadzi to do konfliktow
        //pamieci oraz wyscigu danych
        for i in &mut self.magazyn[0..self.liczba_pozycji]{ 
            if i.nazwa == nazwa_szukana && i.ilosc >= sztuki { 
                i.ilosc -= sztuki;
            }
        }
    }

    fn raport(&self){
        //nie uzywamy wewnatrz self, poniewaz self jest do placu budowy
        //a on nie posiada pozycji z Material
        //wiec zeby ich uzyc z tablicy w ktorej sa te obiekty
        //musimy uzyc iteratora ktory przechodzi przez ta tablice
        //iter tworzy iterator, przechodzacy przez elementy
        //enumerate bierze iterator i pakuje kazdy element w pare
        //index - liczba, wynik dzialania iteratora
        //i - fizyczna zawartosc tego wiersza, cala strukture itd.
        //wiec uzywamy np i.nazwa zeby wyswietli prawdziwa zawartosc
        for (index, i) in self.magazyn[0..self.liczba_pozycji].iter().enumerate(){
            println!("{}. Nazwa: {}, ilosc: {}, cena: {}", 
                      index + 1, 
                      i.nazwa, 
                      i.ilosc, 
                      i.cena_jednostkowa);
        }

        oblicz_wartosc_calkowita(&self.magazyn[0..self.liczba_pozycji]);
    }
}

fn oblicz_wartosc_calkowita(elementy: &[Material]) -> f64{
    let mut suma = 0.0;

    for i in elementy {
        //musimy jawnie przekonwertowac przed mnozeniem
        suma += i.ilosc as f64 * i.cena_jednostkowa;
    }

    suma
}

fn analizuj_wpis(surowy_tekst: &str) -> (String, u32, f64) {
    let mut czesci = surowy_tekst.split_whitespace();

    //next pobiera kolejny fragment tekstu
    //unwrap mowi rustowi ze wie ze jest tam tekst i trzeba go wziac
    //prase parsuje typy
    let nazwa = czesci.next().unwrap().to_string();
    let ilosc = czesci.next().unwrap().parse::<u32>().unwrap();
    let cena = czesci.next().unwrap().parse::<f64>().unwrap();

    (nazwa, ilosc, cena)
}

fn main() {
    println!("Hello, world!");
}
