# INSIDER PRO-TIPS: RZECZY, O KTÓRYCH NIKT CI NIE POWIE

## Wstęp
To nie są porady dla początkujących. To zbiór "min", na które wchodzą deweloperzy budujący aplikacje typu Signal/Session. Przeczytanie tego teraz zaoszczędzi Ci tygodni debugowania.

---

## 1. ARCHITEKTURA I MOST (BRIDGE)

### 💀 Pułapka: "Panic" w Rust zabija całą aplikację
Jeśli kod w Rust wyrzuci `panic!` (np. przez `unwrap()` na pustej wartości), a Ty tego nie złapiesz – **cała aplikacja Fluttera się zamknie bez ostrzeżenia**. Użytkownik zobaczy pulpit.
* **Protip:** Nigdy nie używaj `unwrap()` w kodzie produkcyjnym wystawionym do Fluttera.
* **Rozwiązanie:** Używaj `catch_unwind` na granicy API lub (lepiej) zwracaj `Result<T, Anyhow::Error>` wszędzie. Niech Flutter decyduje, co zrobić z błędem (wyświetlić komunikat), a nie Rust zabija proces.

### 🚀 Wydajność: Kopiowanie pamięci (Zero-Copy)
Przesyłanie obrazka 5MB z Fluttera do Rusta (żeby go zaszyfrować) przez proste skopiowanie bajtów zablokuje UI na wyczuwalny moment.
* **Protip:** Nie przesyłaj dużej zawartości (blobów) przez most, jeśli nie musisz.
* **Rozwiązanie:** Prześlij **ścieżkę do pliku** (String). Niech Rust sam otworzy plik z dysku, zaszyfruje go strumieniowo i zapisze nowy plik, zwracając tylko nową ścieżkę. Flutter nie powinien dotykać bajtów pliku, tylko zarządzać ścieżkami.

### 🥶 UI Freeze: Kryptografia jest ciężka
Hashing (Argon2) i asymetria (X25519) są zasobożerne. Jeśli zrobisz to na wątku, który obsługuje `flutter_rust_bridge`, możesz przyciąć animację przewijania czatu.
* **Protip:** Używaj `tokio::task::spawn_blocking` dla ciężkich obliczeń kryptograficznych w Rust. Oddziel "wątek komunikacji z Flutterem" od "wątku liczenia haszy".

---

## 2. KRYPTOGRAFIA I BEZPIECZEŃSTWO (SECURITY GEMS)

### 🕵️‍♂️ Timing Attacks (Ataki czasowe)
To jest poziom NSA, ale musisz to wiedzieć. Jeśli porównujesz dwa hasze zwykłym `==`:
```rust
if user_hash == database_hash { ... } // ❌ BŁĄD
To haker może zmierzyć czas odpowiedzi. Jeśli pierwszy znak się zgadza, procesor "myśli" nanosekundę dłużej. Mierząc te różnice, można odgadnąć hasło.

Protip: Używaj porównań Constant Time (stały czas).

Rozwiązanie: Biblioteka subtle lub funkcje verify z bibliotek krypto. One zawsze trwają tyle samo czasu, niezależnie od tego, czy hasło jest poprawne, czy nie.

🗑️ Secure Wipe (Zerowanie pamięci)
Gdy Rust zwalnia zmienną z kluczem prywatnym (Drop), system operacyjny po prostu oznacza pamięć jako "wolną", ale bajty klucza tam zostają. Inna aplikacja może je odczytać.

Protip: Klucze prywatne muszą być nadpisane zerami przed usunięciem.

Rozwiązanie: Używaj biblioteki secrecy lub zeroize. Typ Secret<String> automatycznie wyzeruje pamięć przy usuwaniu.

📱 Powiadomienia Push to dziura w prywatności
Jeśli wyślesz powiadomienie przez Google (FCM) z treścią: "Masz wiadomość od Ali: Cześć", to Google to widzi.

Protip: Powiadomienia muszą być puste.

Rozwiązanie: Wysyłasz "Silent Push" (tylko sygnał "obudź się"). Aplikacja wstaje w tle, łączy się z Oxen, pobiera wiadomość, odszyfrowuje ją lokalnie i dopiero wtedy sama wyświetla lokalne powiadomienie użytkownikowi.

3. BAZA DANYCH I DANE
🔒 SQLite "Database is Locked"
Rust jest wielowątkowy. SQLite domyślnie średnio lubi wiele wątków piszących naraz.

Protip: Włącz tryb WAL (Write-Ahead Logging).

Kod: PRAGMA journal_mode=WAL; przy starcie połączenia. To pozwala na to, by jeden wątek pisał, a inne czytały w tym samym czasie bez blokowania.

🕒 Problem Zegara (Time Skew)
Użytkownik A ma zegar ustawiony na 12:00. Użytkownik B ma 11:55. Jeśli używasz czasu lokalnego do sortowania wiadomości lub wygasania (TTL), będziesz miał chaos.

Protip: Nie ufaj zegarowi urządzenia w krytycznych funkcjach.

Rozwiązanie: W sieciach P2P/Blockchain często używa się "Block Height" (numer bloku) jako czasu, lub relatywnego czasu ("wygasa za 5 minut od momentu odebrania", a nie "o godzinie 14:00").

4. SIECI OXEN & P2P
🧅 Onion Requesty są wolne i zawodne
Wysyłanie wiadomości przez 3 węzły (cebula) trwa. Czasem pakiet zginie.

Protip: Nie zakładaj, że send() oznacza "dostarczono".

Rozwiązanie: Musisz zaimplementować ACK (Acknowledge).

Wysyłasz wiadomość.

Czekasz na zwrotkę "Odebrałem".

Jeśli nie przyjdzie w 10s -> ponawiasz wysyłkę (Retry Policy).

To musi dziać się automatycznie w tle.

📦 Kolejność wiadomości (Ordering)
W sieciach rozproszonych wiadomość "Cześć" (wysłana pierwsza) może przyjść po wiadomości "Jak leci?" (wysłanej drugiej).

Protip: Sortowanie po czasie odebrania to błąd.

Rozwiązanie: Używaj Lamport Timestamps lub po prostu licznika sekwencyjnego dla każdej konwersacji (1, 2, 3...). Jeśli dostaniesz 3, a nie masz 2 -> wiesz, że masz dziurę i musisz czekać.

5. ROZWÓJ I NARZĘDZIA (WORKFLOW HACKS)
🐳 Reproducible Builds (Budowanie w Dockerze)
Jeśli tworzysz "bezpieczny komunikator", użytkownicy (paranoicy) zapytają: "Skąd mam wiedzieć, że plik .apk w sklepie to ten sam kod co na GitHubie?".

Protip: Od początku staraj się, aby build był powtarzalny.

Rozwiązanie: Używaj Dockera do kompilacji wydania produkcyjnego. To gwarantuje, że biblioteki systemowe są zawsze te same.

🧪 Testy Integracyjne "Grey Box"
Nie testuj tylko Rusta (Unit Tests) i tylko Fluttera (Widget Tests).

Protip: Najwięcej błędów jest na styku (Bridge).

Rozwiązanie: Napisz test, który wywołuje funkcję w Dart, która woła Rust, który szyfruje coś i zwraca do Dart. To sprawdzi, czy typy danych się nie rozjechały po aktualizacji biblioteki.

📱 Android Background Limits
Android nienawidzi aplikacji działających w tle. Zabije Twoje połączenie z Oxen po kilku minutach, żeby oszczędzać baterię.

Protip: To największa zmora komunikatorów P2P na mobile.

Rozwiązanie: Musisz używać WorkManager (Android) i odpowiednich trybów "Foreground Service" (z ikonką na pasku), jeśli pobierasz duże pliki. W przeciwnym razie system ubije proces w połowie pobierania.

Mój Osobisty Tip na Koniec
"Zrób to brzydko, ale bezpiecznie". Na początku Twoje UI będzie brzydkie. Wiadomości będą znikać. Ale jeśli kryptografia będzie solidna (dobre biblioteki, brak wycieków kluczy), to masz fundament. UI można poprawić w jeden weekend. Złamanej kryptografii nie naprawisz bez zmuszania wszystkich użytkowników do aktualizacji i zmiany kluczy.

Priorytety:

Dane nie wyciekają (Memory safety, encryption).

Dane nie giną (Persistence, Retry logic).

Wygląda ładnie (Flutter UI).