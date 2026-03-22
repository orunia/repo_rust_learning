## CZESC 7: JAK ROZWIĄZYWAĆ PROBLEMY W KODZIE (DEBUGOWANIE)

Napotkasz problemy. To pewne. Oto algorytm, jak sobie z nimi radzić, zanim poprosisz o pomoc.

## Krok 1: CZYTAJ BŁĘDY (Zwłaszcza w Rust)
Rust ma najlepsze komunikaty błędów w historii programowania.
* Nie panikuj na widok ściany tekstu.
* Spójrz na pierwszą linię błędu.
* Często kompilator mówi: `help: try adding & here`. On dosłownie mówi Ci, jak to naprawić! Czytaj sekcje `help`.

## Krok 2: Izolacja Problemu
Jeśli aplikacja nie działa, a błąd jest niejasny:
1.  Wyłącz ostatnią rzecz, którą dodałeś. Czy działa?
2.  Jeśli tak, włącz połowę nowej rzeczy.
3.  Metodą prób i błędów znajdź konkretną linijkę, która psuje kod.

## Krok 3: Print Debugging (Najprostsza metoda)
Nie wiesz, dlaczego zmienna ma złą wartość? Wypisz ją.
* **Rust:** `println!("Wartość x: {:?}", x);`
* **Flutter:** `debugPrint('Wartość x: $x');`
* *Ważne:* Jeśli aplikacja się wywala (crash), dodaj `println!` przed podejrznym miejscem. Jeśli tekst się pojawi, a potem aplikacja padnie, to błąd jest dalej.

## Krok 4: Wyszukiwanie w Google (Sztuka)
Jak szukać błędów? Skopiuj treść błędu, ale **usuń z niej nazwy swoich plików i zmiennych**.
* *Źle:* "Error in main.rs line 50 variable user_id invalid type"
* *Dobrze:* "Rust error distinct types expected struct found integer"

## Krok 5: Specyficzne dla Bridge (Rust + Flutter)
Połączenie dwóch języków jest trudne do debugowania.
* Jeśli błąd jest dziwny ("Panic"), uruchom aplikację z flagą: `RUST_BACKTRACE=1`. To pokaże Ci dokładnie, w której funkcji Rusta coś wybuchło.
* Sprawdź, czy typy danych w Rust i Dart do siebie pasują (np. czy nie wysyłasz `i32` tam, gdzie Dart oczekuje `String`).

## Krok 6: Gdzie szukać pomocy?
Jeśli powyższe zawiedzie:
1.  **ChatGPT:** Wklej kod i treść błędu. Zapytaj: "Dlaczego to nie działa i jak to naprawić?".
2.  **StackOverflow:** Sprawdź, czy ktoś miał ten problem.
3.  **Rust Discord / Reddit:** Społeczność Rusta jest bardzo pomocna dla początkujących.

---
