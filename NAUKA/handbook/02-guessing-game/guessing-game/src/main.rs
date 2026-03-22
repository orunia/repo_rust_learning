use std::io::stdin; // Importujemy "narzędzie" do obsługi klawiatury.
use rand::Rng;      // Importujemy "trait" (cechę), która pozwala generatorom losować liczby.
use rand::thread_rng; // Importujemy konkretny generator liczb losowych.
use std::cmp::Ordering; // Importujemy typ, który mówi czy coś jest Mniejsze, Większe czy Równe.

fn main() {
    println!("Guess the number!");

    // 1. thread_rng(): Daje nam lokalnego generatora liczb losowych.
    // 2. gen_range(1..=100): Generuje liczbę z zakresu (włącznie z 100).
    // Typ zmiennej to domyślnie i32 (liczba całkowita), Rust sam się domyślił.
    let secret_number = thread_rng().gen_range(1..=100);

    // loop: Nieskończona pętla. Program będzie tu krążył, dopóki brutalnie go nie przerwiemy (break).
    loop {
        println!("Please input your guess.");

        // Tworzymy NOWĄ zmienną (za każdym obrotem pętli).
        // mut: Oznacza, że to "otwarte wiadro" - możemy zmieniać jego zawartość.
        // String::new(): Tworzy pusty obiekt tekstowy w pamięci (na stercie).
        let mut guess = String::new();

        // 1. stdin(): Uzyskujemy uchwyt do wejścia standardowego (klawiatury).
        // 2. read_line(&mut guess):
        //    - &mut: To "Referencja Mutowalna". Nie dajemy funkcji całego wiadra na własność,
        //      tylko "pożyczamy" jej dostęp i pozwalamy wrzucić tam tekst.
        //    - Funkcja ta zwraca typ `Result` (Sukces lub Błąd).
        // 3. expect(...):
        //    - To "bramkarz". Sprawdza wynik read_line.
        //    - Jeśli jest błąd -> zabija program i wyświetla komunikat.
        //    - Jeśli sukces -> zwraca liczbę wczytanych bajtów (choć tu jej nie używamy).
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // SHADOWING (Cieniowanie):
        // Tworzymy nową zmienną o tej samej nazwie 'guess'. Stara zmienna (tekstowa) przestaje być widoczna.
        // Pozwala to zmienić typ danych (z tekstu na liczbę) bez wymyślania nowej nazwy typu 'guess_str'.
        
        // guess.trim(): Ucina "białe znaki" (np. Enter, który wcisnąłeś na końcu).
        // .parse(): Próbuje zamienić tekst "50" na liczbę. Zwraca "Pudełko Niespodziankę" (Result).
        
        let guess: u32 = match guess.trim().parse() {
            // MATCH otwiera pudełko od parse():
            
            Ok(num) => num,     // Wariant 1: W środku była liczba. Wyciągamy ją i przypisujemy do 'guess'.
            Err(_) => continue, // Wariant 2: W środku był błąd (wpisałeś litery).
                                // continue: "Olej resztę kodu w tej pętli, wróć na samą górę i pytaj od nowa".
                                // _ (podłoga): Oznacza "nie obchodzi mnie, jaki to konkretnie błąd".
        };

        println!("You guessed: {guess}");

        // cmp(): Porównuje dwie liczby. Zwraca jeden z trzech wariantów (Ordering).
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),   // Jeśli wynik to Less -> wypisz to.
            Ordering::Greater => println!("Too big!"), // Jeśli wynik to Greater -> wypisz to.
            Ordering::Equal => {
                println!("You win!");
                break; // Wygrana! break wyskakuje z pętli loop i kończy program.
            }
        }
    }
}