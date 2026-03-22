# STRATEGIA NAUKI, ZARZĄDZANIE WIEDZĄ I WALKA Z WYPALENIEM

## Wstęp
Ten dokument nie dotyczy kodu. Dotyczy Twojego mózgu – głównego narzędzia w tym projekcie. Rust jest językiem trudnym, który wymusza zmianę myślenia. Jeśli podejdziesz do niego ze złymi nawykami, wypalisz się po 2 tygodniach. Poniżej znajduje się kompletna strategia przetrwania.

---

## CZĘŚĆ 1: CO I JAK NOTOWAĆ? (BUDOWANIE "DRUGIEGO MÓZGU")

Błąd nowicjusza to przepisywanie dokumentacji do zeszytu. To strata czasu. Dokumentacja zawsze będzie w sieci, bardziej aktualna niż Twoje notatki.

### ❌ Czego NIE notować (Strata czasu):
1.  **Składni (Syntax):** Nie zapisuj: *"Żeby zrobić pętlę, piszemy `for i in 0..5`"*. To znajdziesz w Google w 3 sekundy. IDE (VS Code) i tak Ci to podpowie.
2.  **Definicji książkowych:** Nie pisz: *"Wskaźnik to zmienna przechowująca adres pamięci"*.
3.  **Kodu z tutoriali:** Nie kopiuj kodu z filmu do notatnika. To martwy kod.

### ✅ Co notować (Złoto):
1.  **Modele Mentalne (Mental Models):**
    * *Przykład:* Zamiast definicji `Mutex`, napisz: *"Mutex jest jak klucz do toalety w pociągu. Tylko jedna osoba (wątek) może mieć klucz. Inni muszą czekać w kolejce, aż ta osoba wyjdzie i odda klucz."*
    * Notuj **metafory**, które pomagają Ci zrozumieć, jak coś działa.
2.  **Rozwiązania Błędów (Bug Log):**
    * Kiedy spędzisz 2 godziny na naprawianiu błędu, ZAPISZ TO.
    * **Format:**
        * **Problem:** "Błąd `borrow of moved value` przy próbie użycia zmiennej `db` w wątku."
        * **Dlaczego:** "Przekazałem `db` do wątku, więc główny program stracił do niej prawo własności."
        * **Rozwiązanie:** "Muszę owinąć `db` w `Arc` (Atomic Reference Counting) i użyć `db.clone()` przed wysłaniem."
3.  **Decyzje ("Dlaczego?", a nie "Jak?"):**
    * *"Użyłem tutaj `Vec` zamiast tablicy, bo nie wiem, ile wiadomości przyjdzie z sieci."*
4.  **Szkielety myślowe (Snippets):**
    * Notuj schematy blokowe. Np. *"Flow szyfrowania w mojej apce: Klucz -> Hash -> Szyfr -> Base64 -> Wyślij"*.

### 💡 System Notowania (Protip)
Nie używaj papieru do kodu. Użyj **Obsidian**, **Notion** lub po prostu plików `.md` w folderze projektu (np. katalog `/docs/wiki`). Dzięki temu możesz przeszukiwać swoje notatki `Ctrl+F`.

---

## CZĘŚĆ 2: JAK SIĘ UCZYĆ? (TECHNIKI COGNITIVE SCIENCE)

### 1. Just-in-Time Learning (Nauka "na żądanie")
* **Złe podejście:** "Najpierw nauczę się całego Rusta, przeczytam 5 książek o kryptografii, a potem zacznę pisać". Nigdy nie zaczniesz.
* **Dobre podejście:** "Chcę wysłać wiadomość. Czego potrzebuję? Gniazda TCP. Ok, uczę się tylko tego, jak otworzyć gniazdo w Rust. Resztę ignoruję."
* Ucz się tylko tego, co jest Ci potrzebne do rozwiązania problemu, który masz **dzisiaj**.

### 2. Aktywne Przypominanie (Active Recall)
Czytanie jest pasywne. Mózg wtedy śpi.
* Po przeczytaniu rozdziału o "Ownership" w Rust, zamknij przeglądarkę.
* Otwórz pusty plik i spróbuj napisać kod, który **celowo wywoła błąd** związany z własnością, a potem go napraw.
* Jeśli musisz zerknąć do dokumentacji – to znaczy, że jeszcze nie rozumiesz. Zerknij, zamknij, spróbuj znowu.

### 3. Złamanie Iluzji Kompetencji
Kiedy oglądasz tutorial na YouTube, wszystko wydaje się proste. To iluzja.
* **Zasada:** Jeśli skopiowałeś kod (Ctrl+C, Ctrl+V), to go nie umiesz.
* **Ćwiczenie:** Przepisz kod ręcznie. Zmień nazwy zmiennych. Zmień jedną małą rzecz (np. liczbę wątków z 4 na 8) i zobacz, czy kod nadal działa. Jeśli nie wiesz, co się stanie po zmianie – nie rozumiesz kodu.

---

## CZĘŚĆ 3: ZARZĄDZANIE FRUSTRACJĄ (HACKOWANIE MÓZGU)

Rust jest specyficzny. Poczujesz się głupi. To normalny etap, nazywa się "Doliną Rozpaczy" (Valley of Despair).

### 1. Kompilator to nie Twój wróg
W Pythonie czy JavaScript błędy wyskakują, gdy program już działa (często u klienta). Rust "krzyczy" na Ciebie przed uruchomieniem.
* **Change Mindset:** Każdy czerwony błąd kompilatora `rustc` to zaoszczędzona godzina debugowania w nocy. Potraktuj kompilator nie jak wroga, ale jak **bardzo surowego senior developera**, który siedzi obok Ciebie. On Cię chroni.

### 2. Reguła 20 minut
Jeśli utkniesz na jednym problemie:
1.  Walcz z nim przez 20 minut sam (Google, StackOverflow).
2.  Jeśli po 20 minutach nie masz progresu – **PRZERWIJ**.
3.  Twój mózg wszedł w tryb "tunelowy". Nie widzisz oczywistych rozwiązań.
4.  Idź na spacer, umyj naczynia. Włącz "tryb rozproszony" (Diffuse Mode). Rozwiązanie przyjdzie, gdy będziesz robił herbatę.

### 3. Dopamina i "Małe Zwycięstwa"
Twój projekt jest ogromny. Jeśli będziesz czekał na dopaminę aż skończysz całość, wypalisz się.
* Musisz "oszukać" mózg i dawać mu nagrody częściej.
* Udało Ci się skompilować pusty projekt? **Sukces.**
* Udało Ci się wyświetlić jedną literę w konsoli? **Sukces.**
* Kończ dzień pracy (sesję kodowania) zawsze **jednym małym sukcesem**. Nie zostawiaj rozgrzebanego, niedziałającego kodu na koniec dnia, bo rano będziesz się bał do niego wrócić.

---

## CZĘŚĆ 4: MAPA ZAGROŻEŃ (PUŁAPKI)

### 💀 Tutorial Hell
Oglądasz 10 godzin kursów wideo, ale nie napisałeś ani linijki kodu samodzielnie. Czujesz, że umiesz, ale przed pustym edytorem masz pustkę w głowie.
* **Lekarstwo:** Projektuj. Nie oglądaj kursu "Jak zrobić chat". Oglądaj kurs "Jak obsługiwać sockety", a chat zrób sam.

### 💀 Premature Optimization (Przedwczesna optymalizacja)
Martwisz się, czy Twoja baza danych obsłuży milion użytkowników, podczas gdy nie masz jeszcze ani jednego.
* **Zasada:** Najpierw spraw, żeby działało (Make it work). Potem spraw, żeby było ładne (Make it right). Na końcu spraw, żeby było szybkie (Make it fast). W tej kolejności.

### 💀 Rabbit Holes (Królicze Nory)
Zaczynasz czytać o kryptografii krzywych eliptycznych i lądujesz na doktoratach z matematyki na Wikipedii.
* **Stop:** Zadaj sobie pytanie: "Czy muszę rozumieć matematykę ciał skończonych, żeby użyć biblioteki `x25519`?". Odpowiedź brzmi: NIE. Traktuj kryptografię jak czarną skrzynkę (Black Box).

---

## CZĘŚĆ 5: TECHNIKI PRACY (WORKFLOW)

### 1. Metoda Pomodoro
Ustaw timer na 25 minut głębokiej pracy (zero telefonu, zero YouTube). Potem 5 minut przerwy. Mózg uczy się lepiej w krótkich, intensywnych sesjach niż podczas 4 godzin "gapienia się w monitor".

### 2. No Zero Days
Obiecaj sobie, że codziennie zrobisz *cokolwiek*. Nawet jeśli to będzie napisanie jednej linijki kodu lub przeczytanie jednej strony dokumentacji. To buduje nawyk.

### 3. Dziennik Dewelopera (DevLog)
Załóż plik `DEVLOG.md`. Pisz w nim datę i jedno zdanie o tym, co dziś zrobiłeś.
* *Przykład: 2026-01-12: Walczyłem z instalacją Fluttera. W końcu zadziałało.*
Kiedy za miesiąc będziesz miał kryzys ("Nic nie umiem, stoję w miejscu"), otworzysz ten plik i zobaczysz, jak ogromną drogę przeszedłeś.