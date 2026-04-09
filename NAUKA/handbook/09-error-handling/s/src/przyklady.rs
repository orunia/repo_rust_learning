//dopasowywanie roznych rodzajow bledow za pomoca match
//czesto chcemy podjac rozne dzialania w zaleznosci od konkretnej
//przyczyny niepowodzenia, co wymaga zastosowania zagniezdzonego dopasowywania

use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) =>  fc,
                Err(e) => panic!("Blad z utworzeniem pliku {e:?}"),
            },
            _ => {
                panic!("Problem z otwarciem pliku: {error:?}");
            }
        },
    };
}

//uzywanie match dziala znakomicie, ale bywa dosc rozwlekle i nie zawsze dobrze
//komunikuje intecje programisty. z tego powodu typ result<t, e> posiada wiele wbudowanych metod
//pomocniczych, sposroc ktorych najpopularniejsze to unwrap i expect

use std::fs::File;

fn main(){
    //unwrap to skrotowe rozwiazanie wyrazenia match, jesli zwrocona wartosc jest Ok to unwrap natychmiast
    //wyciaga i zwraca ukryta wewnatrz niej zawartosc, natomiast jesli napotka Err, automatycznie uruchamia ona
    //makro panic! 
    let greeting_file = File::open("hello.txt").unwrap();
}

use std::fs::File;

fn main(){
    //metoda expect dziala identycznie jak unwrap, ale oferuje dodatkowa mozliwosc zdefiniowania
    //wlasnego komunikatu o bledzie, zalecana w kodzie produkcyjnym
    let greeting_file = File::open("hello.txt")
                        .expect("Plik powinien znajdowac sie w tym projekcie");
    
}


//kiedy implementacja danej funkcji wywoluje kod ktory moze zakonczyc sie niepowodzeniem, zamiast
//obslugiwac ten blad samodzielnie, mozna go przekazac wzej do kodu wywolujacego, proces ten nazywamy
//propagowaniem bledu i zapewnia znacznie wieksza elstaycznosc, oddajac pelna kontrole  i odpowiedzialnosc
// kodowi z wyzszego poziomu

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt");
    let mut username = String::new();
    //znak zapytania dziala prawie identycznie jak match
    //esli wartosc ok ukryta info jest wyciagana i przypisywana do zmiennej
    //a program kontynuuje dzialanie, jesli err, to caly proces wykonywania biezacej
    //funkcji zostaje natycmast rpzerywany, a blad jest zwracany do kodu wywolujacego w taki sam sposob
    //jakby uzyto return, kluczowa roznica jest to ze ? automatycznie wywoluje funkcje from, ktora
    //inteligentnie konwertuje specyficzny blad podrzedny na glowny typ bledu zadeklarowany w sygnaturze funkcji
    username_file.read_to_string(&mut username)?;
    Ok(username);
}

//mozmey uzywac ? na wartosciach typu result, wylacznie w funckjach zwracajacych typ result, a na wartosciah
//option wylacznie w funkcjach zwracajacych option, kompilator automatycznie zabrania mieszania i automatycznego
//konwertowanua typu option z typem result

//standardowo main domyslnie zwraca pusty typ jednostkowy oznacznay jako (), z tego powodu ? bezposrednio w main 
//skutkuje bledem kompilacji, ale da sie obejsc

use std::error::Error;
use std::fs::File;

//Box<dyn Error> oznacza dynamicznie absolutnie dowolny rodzaj bledu implementujacego ceche error, program zyskuje 
//prawo do korzystania z ?
//kiedy main zwraca na samym koncu Ok(()) procesor informuje system ze program zakonczyl sie
//poprawnym kodem wyjscia wynaszacym 0
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

//