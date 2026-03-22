**Projekt: System Zarządzania Placem Budowy (Magazyn - Wersja z Tablicą)**

**Czas wykonania:** Minimum 3 godziny.

**Architektura i Wymagania Techniczne:**

1. **Struktury i Typy Danych:**
* Zdefiniuj strukturę `Material` z polami: `nazwa` (`String`), `ilosc` (`u32`), `cena_jednostkowa` (`f64`). Zadbaj o to, by można było stworzyć pusty materiał (np. puste pole tekstowe i zera).
* Zdefiniuj strukturę `PlacBudowy` z dwoma polami: `magazyn: [Material; 5]` (stała tablica pięciu elementów) oraz `liczba_pozycji: usize` (śledzi, ile materiałów faktycznie wprowadzono do tablicy).


2. **Funkcje Powiązane i Warsztat (`impl`):**
* Utwórz blok `impl PlacBudowy`.
* Zaimplementuj konstruktor `fn nowy() -> Self`, który zwraca plac budowy z wyzerowaną tablicą i `liczba_pozycji` równym 0. Wypełnij tablicę pustymi strukturami `Material`.
* Zaimplementuj metodę `fn dodaj(&mut self, nazwa: String, ilosc: u32, cena: f64)`. Metoda musi sprawdzić, czy `liczba_pozycji` jest mniejsza niż 5. Jeśli tak, nadpisuje element w tablicy pod indeksem równym `liczba_pozycji` i zwiększa ten licznik o 1. W przeciwnym razie wyświetla błąd o braku miejsca.
* Zaimplementuj metodę `fn wydaj(&mut self, nazwa_szukana: &str, sztuki: u32)`. Metoda iteruje tylko po zapisanych pozycjach tablicy (używając limitu `liczba_pozycji`). Jeśli znajdzie materiał, weryfikuje, czy stan magazynowy pozwala na wydanie i aktualizuje wartość.
* Zaimplementuj metodę `fn raport(&self)`, która pożycza dane tylko do odczytu i wypisuje wszystkie zapisane w tablicy pozycje.


3. **Wycinki (Slices):**
* Poza blokiem `impl` zdefiniuj osobną, wolną funkcję `fn oblicz_wartosc_calkowita(elementy: &[Material]) -> f64`. Funkcja ta przyjmuje wycinek tablicy i zwraca sumę wartości wszystkich materiałów w wycinku (ilość * cena jednostkowa).
* W metodzie `raport`, wywołując funkcję obliczającą wartość, stwórz wycinek odcinający puste miejsca w tablicy: `&self.magazyn[0..self.liczba_pozycji]`.


4. **Krotki (Tuples) i Parsowanie:**
* Zdefiniuj funkcję pomocniczą `fn analizuj_wpis(surowy_tekst: &str) -> (String, u32, f64)`.
* Funkcja ta otrzymuje ciąg znaków (np. "Pustaki 500 4.50"), rozdziela go (np. używając metody `.split_whitespace()`) i zwraca krotkę z odpowiednimi, przeparsowanymi typami.


5. **Pętla, Wejście Użytkownika i `match` (Obsługa Błędów):**
* W funkcji `main` stwórz instancję `PlacBudowy` i uruchom główną pętlę programu (`loop`).
* Wypisuj menu z opcjami:
1 - Dodaj materiał
2 - Wydaj materiał
3 - Wygeneruj raport
4 - Zakończ program
* Pobieraj wybór użytkownika za pomocą `std::io::stdin().read_line()`.
* Użyj instrukcji `match` do interpretacji wyboru oraz wyników z metody `.parse()`. Wyłapuj błędy typu `Err` (np. gdy ktoś wpisze litery zamiast cyfr). Program ma wypisać ostrzeżenie i za pomocą słowa kluczowego `continue` wrócić na początek pętli.
* Wpisanie opcji "4" musi wywołać instrukcję `break`.



**Scenariusz Testowy do zrealizowania:**

1. Uruchom program. Spróbuj wpisać litery tam, gdzie program oczekuje liczby w menu (sprawdzenie wychwytywania błędów przez `match` - program nie może się "wywalić").
2. Dodaj "Cement" (100 worków, cena 25.50). Program rozpakowuje tekst do krotki i zapisuje na indeksie 0 w tablicy.
3. Dodaj "Deski" (50 sztuk, cena 12.00). Zapisują się na indeksie 1.
4. Wygeneruj raport (metoda wypisze Cement i Deski, po czym przekaże wycinek dwuelementowy `&magazyn[0..2]` do funkcji `oblicz_wartosc_calkowita` i wypisze łączną wartość placu).
5. Wydaj "Cement" w ilości 150 sztuk (program musi zablokować akcję i wyświetlić błąd o braku wystarczającej ilości towaru).
6. Wydaj "Cement" w ilości 20 sztuk (sukces).
7. Wygeneruj raport (stan Cementu wynosi 80, całkowita wartość placu jest odpowiednio mniejsza).
8. Dodaj kolejne materiały, aż przekroczysz limit 5 elementów. Sprawdź, czy program poprawnie zablokuje próbę dodania szóstego materiału.
9. Zakończ program wpisując 4.