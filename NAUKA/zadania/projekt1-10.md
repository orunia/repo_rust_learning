### Etap 1: Inicjalizacja i Moduły
* Wykonaj w konsoli komendę `cargo new magazyn_cli`.
* Utwórz nowe pliki w folderze `src/`: `produkt.rs`, `magazyn.rs` oraz `interfejs.rs`.
* Połącz pliki w `main.rs` używając słów kluczowych `mod` oraz `pub use`.



### Etap 2: Zmienne, Typy i Struktury
* W pliku `produkt.rs` zdefiniuj strukturę `Produkt`.
* Dodaj pola: `id` (typ `u32`), `nazwa` (typ `String`), `cena` (typ `f64`), `ilosc` (typ `u32`).
* Dodaj blok `impl Produkt` z funkcją konstrukcyjną `nowy(...) -> Self`.

### Etap 3: Kolekcje
* W pliku `magazyn.rs` zdefiniuj strukturę `Magazyn`.
* Dodaj pole `produkty` typu `std::collections::HashMap<u32, Produkt>`.
* Dodaj pole `historia_operacji` typu `Vec<String>` do zapisywania logów.



### Etap 4: Enumy i Dopasowywanie Wzorców
* Utwórz enum `Kategoria` (np. `Elektronika`, `Narzedzia`, `Inne`). Dodaj to pole do struktury `Produkt`.
* Utwórz enum `Komenda` określający intencje użytkownika: `Dodaj(String, f64, u32)`, `Wydaj(u32, u32)`, `Raport`, `Wyjscie`.
* Użyj instrukcji `match` w głównej pętli do interpretacji wybranej komendy.

### Etap 5: Własność i Metody
* Napisz metodę `dodaj_produkt(&mut self, produkt: Produkt)` w `impl Magazyn`.
* Napisz metodę `pobierz_nazwe(&self, id: u32) -> Option<&String>`, która jedynie pożycza nazwę produktu bez przejmowania własności.

### Etap 6: Pętla i Interakcja z Użytkownikiem
* W `interfejs.rs` stwórz funkcję uruchamiającą nieskończoną pętlę `loop`.
* Wczytuj tekst od użytkownika za pomocą `std::io::stdin().read_line()`.
* Parsuj wczytany ciąg znaków (np. instrukcja `Dodaj Mlotek 50.0 10`) i konwertuj go na wariant enuma `Komenda`.

### Etap 7: Obsługa Błędów
* Utwórz własny enum `BladMagazynu` z wariantami takimi jak `NieZnalezionoProduktu`, `BrakWystarczajacejIlosci`, `NieprawidlowyFormat`.
* Zmień metody w `Magazyn` tak, aby zwracały `Result<(), BladMagazynu>`.
* Zastosuj operator `?` do propagacji błędów podczas parsowania wejścia.

### Etap 8: Cechy (Traits)
* Zdefiniuj własny trait `FormatowanieRaportu`.
* Dodaj w nim sygnaturę metody `formatuj(&self) -> String`.
* Zaimplementuj ten trait dla struktury `Produkt`, definiując, jak ma wyglądać jej reprezentacja tekstowa w raporcie.

### Etap 9: Typy Generyczne
* Stwórz samodzielną funkcję generyczną `fn wydrukuj_element<T: FormatowanieRaportu>(element: &T)`.
* Funkcja ta powinna przyjmować dowolny obiekt implementujący nowo stworzony trait i wypisywać go na ekran.

### Etap 10: Czasy Życia (Lifetimes)
* Dodaj funkcjonalność wyszukiwania produktów po nazwie.
* Zdefiniuj strukturę `WynikWyszukiwania<'a>`, która przechowuje wycinek tekstu `&'a str` (szukaną frazę) oraz wektor referencji do znalezionych produktów `Vec<&'a Produkt>`.
* Oznacz precyzyjnie czasy życia, aby kompilator wiedział, że wyniki są powiązane z danymi w głównym słowniku `HashMap`.

### `Produkt::nowy(id: u32, nazwa: String, kategoria: Kategoria, cena: f64, ilosc: u32) -> Self`
* Przypisuje przekazane argumenty do odpowiednich pól struktury `Produkt` i zwraca jej nową instancję.

### `Magazyn::nowy() -> Self`
* Inicjuje nowy obiekt `Magazyn` z pustym słownikiem `HashMap::new()` dla produktów oraz pustym wektorem `Vec::new()` dla historii operacji.

### `Magazyn::dodaj_produkt(&mut self, produkt: Produkt) -> Result<(), BladMagazynu>`
* Sprawdza, czy w `HashMap` istnieje już klucz odpowiadający `produkt.id`.
* Jeśli istnieje, zwraca błąd `Err(BladMagazynu::IdJuzIstnieje)`.
* Jeśli nie, wstawia produkt do `HashMap`, dodaje tekst "Dodano produkt ID: [id]" do wektora `historia_operacji` i zwraca `Ok(())`.

### `Magazyn::wydaj_produkt(&mut self, id: u32, ilosc: u32) -> Result<(), BladMagazynu>`
* Wyszukuje produkt w `HashMap` po podanym `id`.
* Jeśli go nie ma, zwraca `Err(BladMagazynu::NieZnalezionoProduktu)`.
* Sprawdza, czy aktualna ilość produktu jest większa lub równa żądanej `ilosc`.
* Jeśli brakuje sztuk, zwraca `Err(BladMagazynu::BrakWystarczajacejIlosci)`.
* Jeśli stan jest wystarczający, pomniejsza ilość produktu, dopisuje zdarzenie do `historia_operacji` i zwraca `Ok(())`.

### `Magazyn::pobierz_nazwe(&self, id: u32) -> Option<&String>`
* Odpytuje `HashMap` o wartość dla danego `id`.
* Zwraca `Some(&produkt.nazwa)` jeśli produkt istnieje.
* Zwraca `None`, jeśli produktu nie ma. Nie modyfikuje żadnych danych.

### `Magazyn::szukaj_po_nazwie<'a>(&'a self, fraza: &str) -> WynikWyszukiwania<'a>`
* Przechodzi (iteruje) przez wszystkie wartości w `HashMap`.
* Zatrzymuje tylko te produkty, których pole `nazwa` zawiera ciąg znaków `fraza`.
* Pakuje referencje znalezionych produktów (`&'a Produkt`) do wewnętrznego wektora struktury `WynikWyszukiwania` i zwraca ją.

### `interfejs::uruchom()`
* Rozpoczyna nieskończoną pętlę `loop`.
* Wypisuje dostępne komendy na ekran.
* Pobiera wejście od użytkownika poprzez `std::io::stdin().read_line()`.
* Przekazuje wejście do funkcji `analizuj_komende`.
* Używa instrukcji `match` na otrzymanym enumie `Komenda`, aby wywołać logikę na instancji `Magazyn` (np. `Magazyn::dodaj_produkt`).
* Przerywa działanie programu poleceniem `break`, gdy wariant to `Komenda::Wyjscie`.

### `interfejs::analizuj_komende(wejscie: &str) -> Result<Komenda, BladMagazynu>`
* Używa `split_whitespace()` na wejściu, aby rozdzielić ciąg znaków na słowa.
* Sprawdza pierwsze słowo i na jego podstawie dopasowuje oczekiwane argumenty.
* Konwertuje (parsuje za pomocą `.parse()`) kolejne słowa z tekstu na liczby (`u32`, `f64`).
* Jeśli brakuje argumentów lub ich konwersja się nie powiedzie, zwraca `Err(BladMagazynu::NieprawidlowyFormat)`.
* Przy prawidłowych danych zwraca odpowiedni wariant, np. `Ok(Komenda::Dodaj(id, nazwa, cena, ilosc))`.

### `FormatowanieRaportu::formatuj(&self) -> String`
* Metoda wymuszona przez trait.
* Dla struktury `Produkt` łączy jej pola w jeden sformatowany ciąg znaków, np. zwracając format: `"[ID: 1] Młotek - 50.50 PLN (Dostępne: 10)"`.

### `wydrukuj_element<T: FormatowanieRaportu>(element: &T)`
* Funkcja generyczna.
* Wywołuje na przekazanym obiekcie metodę `element.formatuj()` i wypisuje zwrócony tekst w konsoli za pomocą makra `println!()`.