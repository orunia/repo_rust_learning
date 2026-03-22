## PROJEKTY TRENINGOWE (PLAN PRAKTYKI)

Wykonanie tych zadań da Ci solidne fundamenty. Nie musisz robić ich idealnie, ważne, żeby działały.

Oto plik z dodanymi linkami do sekcji "Materiały", bez zmieniania treści zadań:

```markdown
# KOMPLETNY PLAN NAUKI RUST: SESSION BACKEND CORE

Plan skupia się wyłącznie na technologiach backendowych: Rust, Kryptografia, Bazy Danych i Sieci P2P (Oxen/Onion). Zero frontendu.

---

## 1. FUNDAMENTY RUST (CORE LOGIC)
*Cel: Opanowanie języka, zarządzania pamięcią i typów danych niezbędnych do pisania bezpiecznego kodu.*

### 🟢 Poziom Łatwy

#### 1. Generator Hasła CLI (Secure)
* **Opis:** Napisz narzędzie w konsoli, które generuje bezpieczne hasła. Musi przyjmować flagi (np. `--length 20`, `--no-special`).
    * **Rozwinięcie:** Nie używaj zwykłego generatora liczb losowych (`rand::thread_rng` jest ok, ale zrozum dlaczego). Zaimplementuj obliczanie entropii hasła (siły w bitach) i wyświetl ją użytkownikowi.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `clap` (parsowanie argumentów), `rand`.
    * **Teoria:** Różnica między `PRNG` (pseudo-losowość) a `CSPRNG` (kryptograficzna losowość). Entropia haseł.
> **Materiały:**
> * [Rust Cookbook: Command Line Arguments (Clap)](https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html)
> * [The Rust Rand Book: Cryptographic Security](https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-rngs)
> * [Teoria: Jak obliczyć entropię hasła?](https://www.omnicalculator.com/other/password-entropy)

#### 2. Analizator Logów (Panic-Proof)
* **Opis:** Program wczytuje plik tekstowy, zlicza wystąpienia IP i zapisuje raport.
    * **Rozwinięcie:** Program musi być "kuloodporny". Jeśli plik nie istnieje, ma złe kodowanie lub brak uprawnień – program ma wypisać ładny błąd, a nie wywalić "panic". Użyj `BufReader` dla wydajności.
* **Czego to uczy (Teoria & Praktyka):**
    * **Rust:** `std::fs`, `std::io::BufReader` (praca na strumieniach, nie ładowanie całego pliku do RAM).
    * **Error Handling:** `Result`, `Option`, biblioteka `anyhow` lub `thiserror`.
> **Materiały:**
> * [Rust By Example: File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)
> * [Dokumentacja: BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
> * [Blog: Error Handling with anyhow & thiserror](https://blog.burntsushi.net/rust-error-handling/)

#### 3. Kalkulator ONP (BigInt)
* **Opis:** Kalkulator działający na stosie (Odwrotna Notacja Polska).
    * **Rozwinięcie:** Kryptografia wymaga operacji na ogromnych liczbach. Zamiast `i32`, użyj biblioteki do "Wielkich Liczb" (BigInt), które nie mieszczą się w procesorze (np. 2048-bitowe).
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `num-bigint`.
    * **Algorytmy:** Struktura danych Stos (`Vec` jako LIFO).
> **Materiały:**
> * [Dokumentacja Crate `num-bigint`](https://docs.rs/num-bigint/latest/num_bigint/)
> * [Wideo: Reverse Polish Notation (Computerphile)](https://www.youtube.com/watch?v=7ha78yWRDlE)

### 🟡 Poziom Średni

#### 1. Szyfrator Plików (XOR Stream)
* **Opis:** Program bierze plik wejściowy i klucz, wykonuje operację XOR i zapisuje wynik.
    * **Rozwinięcie:** Kluczowe jest strumieniowanie. Program musi umieć zaszyfrować plik 10GB mając tylko 100MB RAM, czytając i szyfrując po kawałku (chunk).
* **Czego to uczy (Teoria & Praktyka):**
    * **Rust:** Zarządzanie pamięcią i buforami (`buffer`).
    * **Teoria:** Operacje bitowe (XOR). Dlaczego "Stream Cipher" jest szybki.
> **Materiały:**
> * [Rust By Example: File I/O (Open Options)](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html)
> * [Teoria: Stream Ciphers (XOR)](https://www.youtube.com/watch?v=sMOZf4GN3oc)

#### 2. Wielowątkowy Pobieracz (Thread Pool)
* **Opis:** Pobierz dane z 10 serwerów jednocześnie.
    * **Rozwinięcie:** Stwórz pulę wątków (np. 4 wątki robocze), które biorą zadania z kolejki. To symuluje pobieranie wiadomości z wielu węzłów sieci Oxen naraz.
* **Czego to uczy (Teoria & Praktyka):**
    * **Rust:** `std::thread`, `mpsc` (kanały komunikacyjne).
    * **Concurrency:** Jak nie doprowadzić do wyścigu (Race Condition).
> **Materiały:**
> * [Rust Book: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
> * [Dokumentacja: Tokio Channels](https://tokio.rs/tokio/tutorial/channels)

### 🔴 Poziom Trudny

#### 1. Serwer TCP (Własny Protokół)
* **Opis:** Serwer async, który odbiera wiadomości binarne.
    * **Rozwinięcie:** Zaprojektuj własny protokół binarny: `[4 bajty długości][1 bajt typu][treść]`. Serwer musi odczytać nagłówek, a potem dokładnie tyle bajtów treści, ile zadeklarowano.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `tokio`, `bytes`.
    * **Sieci:** Framing w TCP (TCP to strumień, nie paczki!). Endianness (Big Endian vs Little Endian).
> **Materiały:**
> * [Tokio Tutorial: I/O](https://tokio.rs/tokio/tutorial/io)
> * [Dokumentacja Crate `bytes`](https://docs.rs/bytes/latest/bytes/)
> * [Beej's Guide to Network Programming](https://beej.us/guide/bgnet/)

---

Oto zaktualizowana sekcja "Integracja" z rozwiniętymi opisami, celami edukacyjnymi uwzględniającymi generowanie UI przez AI, oraz materiałami do nauki.

### 2. INTEGRACJA (RUST + FLUTTER BRIDGE)

*Cel: Połączenie mocy obliczeniowej Rusta z wyglądem Fluttera. Ty piszesz logikę w Rust, definiujesz API, a AI generuje kod Dart/Flutter, który to wyświetla.*

#### 🟢 Poziom Łatwy

1.  **Hello World z Rusta**
    * **Opis:** Przycisk we Flutterze wywołuje funkcję `say_hello()` w Ruście, która zwraca string "Hello from Rust!". Flutter wyświetla ten tekst.
        * **Rozwinięcie:** Twoim zadaniem jest poprawne skonfigurowanie środowiska (`flutter_rust_bridge_codegen`). Musisz zdefiniować prostą funkcję w `lib.rs`, wygenerować kod wiążący (bindings) i sprawdzić, czy aplikacja na emulatorze nie wyrzuca błędów linkowania.
    * **Czego uczy:** Konfiguracja toolchaina (Rust + Android NDK/iOS SDK), podstawowy przepływ danych (FFI), generowanie kodu.

> **Materiały:**
> * [Oficjalny Tutorial: Flutter Rust Bridge](https://www.google.com/search?q=https://cjycode.com/flutter_rust_bridge/guides/quick-start)
> * [Wideo: Rust + Flutter Setup Guide](https://www.google.com/search?q=https://www.youtube.com/watch%3Fv%3Dn-JtM9tHqT8)
> 
> 
2.  **Prosty Kalkulator w Rust**
    * **Opis:** Flutter ma dwa pola tekstowe na liczby. Przycisk "Dodaj" wysyła liczby do Rusta. Rust dodaje i zwraca wynik.
        * **Rozwinięcie:** Skup się na typach danych. Jak Rustowe `i32` lub `f64` mapuje się na Darta? Co się stanie, jak wyślesz liczbę spoza zakresu? Zdefiniuj funkcję `pub fn add(a: i32, b: i32) -> i32`.
    * **Czego uczy:** Przekazywanie typów prostych przez FFI, konwersja typów między językami.

> **Materiały:**
> * [FRB: Passing Arguments](https://www.google.com/search?q=https://cjycode.com/flutter_rust_bridge/guides/types/index)
> * [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
> 
> 
3.  **Odwracacz Obrazka (Hash)**
    * **Opis:** Wybierz zdjęcie we Flutterze, wyślij jego ścieżkę do Rusta. Rust ma policzyć sumę kontrolną (MD5/SHA) pliku i zwrócić hash do Fluttera.
        * **Rozwinięcie:** Aplikacja mobilna (Flutter) podaje tylko ścieżkę (`String`). Rust musi otworzyć ten plik systemowy. Pamiętaj, że operacje na plikach mogą być wolne – Rust nie powinien tego robić na głównym wątku (FRB domyślnie wrzuca to na ThreadPool, ale warto to wiedzieć).
    * **Czego uczy:** `std::fs` (operacje na plikach), biblioteki haszujące (`sha2`), obsługa błędów (co jeśli plik nie istnieje?).

> **Materiały:**
> * [Crate `sha2` Documentation](https://www.google.com/search?q=%5Bhttps://docs.rs/sha2/latest/sha2/%5D(https://docs.rs/sha2/latest/sha2/))
> * [Rust By Example: File Read](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html)
> 
> 

#### 🟡 Poziom Średni

1.  **Generator Frazy Seed (Mnemonic)**
    * **Opis:** Klikasz przycisk we Flutterze. Rust generuje 12 losowych słów (BIP39) i zwraca je jako listę `Vec<String>`.
        * **Rozwinięcie:** Tutaj zwracasz złożony obiekt. `flutter_rust_bridge` automatycznie zamieni Rustowy `Vec<String>` na Dartowe `List<String>`, które AI łatwo wyświetli w UI. Użyj crate'a `bip39`.
    * **Czego uczy:** Przekazywanie wektorów i tablic, serializacja danych między językami, standardy krypto portfeli.

> **Materiały:**
> * [Crate `bip39` Documentation](https://www.google.com/search?q=%5Bhttps://docs.rs/bip39/latest/bip39/%5D(https://docs.rs/bip39/latest/bip39/))
> * [FRB: Vectors and Arrays](https://cjycode.com/flutter_rust_bridge/guides/types/arbitrary)
> 
> 
2.  **Asynchroniczny Timer**
    * **Opis:** Uruchom długie zadanie w Rust (np. `sleep` na 5 sekund). Flutter nie może się zaciąć.
        * **Rozwinięcie:** Użyj `tokio::time::sleep` i funkcji `async fn`. Dzięki temu Flutter otrzyma `Future`, na który może czekać (await), pokazując w międzyczasie spinner, a UI pozostanie płynne (60 FPS).
    * **Czego uczy:** Różnica między `std::thread::sleep` (blokuje wątek) a `tokio::time::sleep` (zwalnia zasoby), asynchroniczność na linii Dart-Rust.

> **Materiały:**
> * [Rust Async Book: Why Async?](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html)
> * [Tokio Docs: Sleep](https://docs.rs/tokio/latest/tokio/time/fn.sleep.html)
> 
> 
3.  **Szyfrowanie Wiadomości (AES/Chacha)**
    * **Opis:** Wpisujesz tekst i hasło. Rust szyfruje i zwraca Base64.
        * **Rozwinięcie:** Flutter wysyła `String` (tekst). Rust zamienia go na bajty (`Vec<u8>`), szyfruje, a wynik (też bajty) musi zakodować do Base64, żeby łatwo przesłać go z powrotem jako tekst.
    * **Czego uczy:** Praca na surowych bajtach, kodowanie Base64, praktyczne użycie `chacha20poly1305` w moście.

> **Materiały:**
> * [Crate `base64](https://www.google.com/search?q=%5Bhttps://docs.rs/base64/latest/base64/%5D(https://docs.rs/base64/latest/base64/))`
> * [Rust String vs &str vs Vec<u8>](https://doc.rust-lang.org/book/ch08-02-strings.html)
> 
> 
#### 🔴 Poziom Trudny

1.  **Menedżer Haseł (Mini-Projekt)**
    * **Opis:** Rust zarządza bazą SQLite, Flutter jest tylko "widokiem".
        * **Rozwinięcie:** Musisz utrzymać stan bazy danych po stronie Rusta (np. używając `lazy_static` albo wzorca State w FRB). Flutter wywołuje `init_db(master_password)`, a potem `get_passwords()`. Baza jest otwierana tylko raz.
    * **Czego uczy:** Zarządzanie stanem (State Management) wewnątrz Rusta, bezpieczne przekazywanie hasła głównego, architektura "Headless".

> **Materiały:**
> * [FRB: State Management (Holding Rust objects)](https://www.google.com/search?q=https://cjycode.com/flutter_rust_bridge/guides/types/opaque)
> * [Pattern: Global State in Rust](https://www.google.com/search?q=https://blog.logrocket.com/rust-design-patterns-singleton/)
> 
> 
2.  **Monitor Zasobów Systemowych (Stream)**
    * **Opis:** Rust co sekundę wysyła użycie CPU/RAM przez `Stream`.
        * **Rozwinięcie:** Zamiast funkcji "request-response", używasz `StreamSink`. Rust pcha dane (push) do UI. To identyczny mechanizm, jakiego użyjesz w Session do wyświetlania przychodzących wiadomości w czasie rzeczywistym.
    * **Czego uczy:** `StreamSink` w FRB, biblioteka `sysinfo`, architektura reaktywna.

> **Materiały:**
> * [FRB: Streams (Sending data to Dart)](https://www.google.com/search?q=https://cjycode.com/flutter_rust_bridge/guides/stream)
> * [Crate `sysinfo](https://www.google.com/search?q=%5Bhttps://docs.rs/sysinfo/latest/sysinfo/%5D(https://docs.rs/sysinfo/latest/sysinfo/))`
> 
> 
3.  **Chat P2P w sieci lokalnej (LAN)**
    * **Opis:** Komunikacja UDP między telefonami w Wi-Fi.
        * **Rozwinięcie:** Rust otwiera `UdpSocket` i nasłuchuje w pętli. Gdy przyjdzie pakiet, przekazuje go przez `Stream` do Fluttera. Flutter wyświetla dymek. AI zrobi UI czatu, Ty musisz dostarczyć strumień wiadomości `Stream<Message>`.
    * **Czego uczy:** Łączenie asynchronicznego networkingu (`tokio`) ze strumieniami UI, obsługa uprawnień sieciowych na Androidzie.

> **Materiały:**
> * [Tokio: UDP Socket](https://tokio.rs/tokio/tutorial/io)
> * [Android Permissions for Network (teoria)](https://developer.android.com/training/basics/network-ops/connecting)
> 
>
---
## 3. BAZY DANYCH I TRWAŁOŚĆ (SLED & KEY-VALUE)
*Cel: Zrozumienie, jak przechowywać dane w szybkiej bazie Key-Value (bez SQL) i jak zapewnić im bezpieczeństwo.*

### 🟢 Poziom Łatwy

#### 1. Szyfrowany Magazyn (Basic KV)
* **Opis:** Stwórz prosty zapis: `Klucz (String)` -> `Wartość (Zaszyfrowane Bajty)`.
    * **Rozwinięcie:** Użyj `sled::open`. Przed zapisem wartości, zaszyfruj ją (ChaCha20/AES). W bazie `sled` wartości to surowe bajty (`IVec`), co idealnie pasuje do kryptografii. Kluczem niech będzie np. "notatka_1", a wartością zaszyfrowana treść + nonce.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `sled`, `chacha20poly1305`.
    * **Teoria:** Model Key-Value. Dlaczego bazy KV są szybsze od SQL? Praca na surowych bajtach (`IVec`).
> **Materiały:**
> * [Dokumentacja `sled`](https://docs.rs/sled/latest/sled/)
> * [Sled: Basic Operations Example](https://github.com/spacejam/sled/tree/master/examples)
> * [Artykuł: Why Key-Value Stores?](https://www.freecodecamp.org/news/key-value-stores-explained/)

#### 2. Bezpieczne Otwieranie (KDF + Sled)
* **Opis:** Zabezpiecz dostęp do bazy hasłem użytkownika.
    * **Rozwinięcie:** `sled` nie ma wbudowanego szyfrowania "całego pliku" jak SQLCipher. Musisz to obsłużyć logicznie.
        1. Użytkownik podaje hasło.
        2. KDF (Argon2) generuje `MasterKey`.
        3. Ten klucz służy do szyfrowania wartości *przed* wysłaniem ich do `sled`. Klucze (np. ID wiadomości) mogą pozostać jawne, ale treść musi być tajna.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `argon2`, `zeroize` (czyszczenie pamięci).
    * **Bezpieczeństwo:** Application-Level Encryption (szyfrowanie w warstwie aplikacji, a nie bazy).
> **Materiały:**
> * [Dokumentacja `argon2`](https://docs.rs/argon2/latest/argon2/)
> * [Best Practices: Data Encryption at Rest](https://cloud.google.com/docs/security/encryption/default-encryption)

### 🟡 Poziom Średni

#### 1. Pobieranie Historii Czatów (Range Scan)
* **Opis:** Pobierz wszystkie wiadomości dla konkretnego czatu, posortowane chronologicznie.
    * **Rozwinięcie:** W SQL zrobiłbyś `SELECT * WHERE chat_id=X`. W `sled` nie ma `WHERE`. Musisz sprytnie zaprojektować klucze.
        * Format klucza: `chat_id:timestamp:msg_id`.
        * Użyj funkcji `scan_prefix(chat_id)`, aby błyskawicznie pobrać wszystkie wiadomości należące do tego czatu. To jest sekret wydajności KV.
* **Czego to uczy (Teoria & Praktyka):**
    * **Sled:** Iteratory i funkcja `scan_prefix`.
    * **Architektura:** Lexicographical Key Sorting (jak bajty klucza wpływają na kolejność sortowania).
> **Materiały:**
> * [Sled Docs: scan_prefix](https://docs.rs/sled/latest/sled/struct.Tree.html#method.scan_prefix)
> * [Blog: Modeling Data in Key-Value Stores](https://hackernoon.com/modeling-data-in-nosql-key-value-stores-8d4e9f3)

#### 2. Blind Indexing w KV (Wyszukiwanie)
* **Opis:** Znajdź użytkownika po numerze telefonu, nie trzymając numeru w bazie.
    * **Rozwinięcie:**
        * Klucz: `HMAC(numer_telefonu)` (hash, np. 32 bajty).
        * Wartość: `Zaszyfrowany_Profil_Uzytkownika`.
        * Aby wyszukać, klient liczy hash szukanego numeru i robi `db.get(hash)`. Baza nigdy nie widzi numeru telefonu, widzi tylko losowe ciągi znaków jako klucze.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `hmac`, `sha2`.
    * **Teoria:** Hash-based lookups. Zaleta KV: wyszukiwanie po kluczu głównym (Primary Key) jest operacją O(1) lub O(log N).
> **Materiały:**
> * [Artykuł: Blind Indexing Strategy](https://www.freecodecamp.org/news/how-to-search-encrypted-data/)
> * [RustCrypto: HMAC Examples](https://docs.rs/hmac/latest/hmac/)

### 🔴 Poziom Trudny

#### 1. Transakcje Atomowe (Batch Operations)
* **Opis:** Odbierasz wiadomość: musisz zapisać treść ORAZ zaktualizować wskaźnik "ostatnia wiadomość". Obie rzeczy muszą się udać, albo żadna.
    * **Rozwinięcie:** Użyj `sled::Batch`. Dodaj do batcha operację `insert(wiadomosc)` i `insert(last_msg_ptr)`. Wykonaj `db.apply_batch()`. Jeśli prąd padnie w trakcie, baza nie zostanie w stanie niespójnym.
* **Czego to uczy (Teoria & Praktyka):**
    * **Sled:** `Batch` i gwarancje atomowości.
    * **Teoria:** ACID w bazach NoSQL.
> **Materiały:**
> * [Sled Docs: Batch](https://docs.rs/sled/latest/sled/struct.Batch.html)
> * [Wikipedia: Atomicity (database systems)](https://en.wikipedia.org/wiki/Atomicity_(database_systems))

#### 2. Własny Mechanizm TTL (Garbage Collector)
* **Opis:** Wiadomości mają znikać po 24h (autodestrukcja). `sled` nie ma wbudowanego TTL (Time-To-Live). Napisz go sam.
    * **Rozwinięcie:**
        * Zapisując wiadomość, dodaj ją też do osobnego "drzewa" (Tree) indeksu czasowego, gdzie kluczem jest `timestamp_usuniecia`.
        * Uruchom osobny wątek w tle (Rust thread), który co minutę skanuje to drzewo, znajduje klucze z czasem < `teraz` i usuwa właściwe wiadomości.
* **Czego to uczy (Teoria & Praktyka):**
    * **Rust:** Zarządzanie wątkami w tle (`std::thread`, `tokio::spawn`).
    * **Algorytmy:** Projektowanie własnego Garbage Collectora danych.
> **Materiały:**
> * [Sled Docs: Trees (Namespaces)](https://docs.rs/sled/latest/sled/struct.Db.html#method.open_tree)
> * [Design Pattern: Log-structured merge-tree](https://en.wikipedia.org/wiki/Log-structured_merge-tree)


## 4. KRYPTOGRAFIA STOSOWANA (SIGNAL PROTOCOL)
*Cel: Implementacja mechanizmów Session/Signal. Nie wymyślasz matematyki, implementujesz standardy.*

### 🟢 Poziom Łatwy

#### 1. Podpisywanie Wiadomości (Ed25519)
* **Opis:** Stwórz system podpisu cyfrowego.
    * **Rozwinięcie:** Wygeneruj parę kluczy (Prywatny/Publiczny). Podpisz wiadomość kluczem prywatnym. Napisz funkcję weryfikującą, która przyjmuje wiadomość, podpis i klucz publiczny, zwracając `true/false`.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `ed25519-dalek`.
    * **Teoria:** Integralność i Niezaprzeczalność (Non-repudiation). Różnica między szyfrowaniem a podpisem.
> **Materiały:**
> * [Dokumentacja Crate `ed25519-dalek`](https://docs.rs/ed25519-dalek/latest/ed25519_dalek/)
> * [Wideo: Digital Signatures (Computerphile)](https://www.youtube.com/watch?v=s22eJ1eVOrU)

#### 2. Wymiana Kluczy (ECDH)
* **Opis:** Symulacja "Alice i Bob".
    * **Rozwinięcie:** Alice i Bob generują swoje pary kluczy na krzywych eliptycznych (X25519). Wymieniają się TYLKO kluczami publicznymi. Każde z nich musi wyliczyć ten sam "Wspólny Sekret".
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `x25519-dalek`.
    * **Teoria:** Diffie-Hellman Key Exchange. Podstawa bezpiecznej komunikacji.
> **Materiały:**
> * [Dokumentacja Crate `x25519-dalek`](https://docs.rs/x25519-dalek/latest/x25519_dalek/)
> * [Artykuł: Primer on Elliptic Curve Cryptography](https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/)

### 🟡 Poziom Średni

#### 1. Symulacja X3DH (Initial Handshake)
* **Opis:** Zaimplementuj protokół nawiązywania sesji (jak w Signal).
    * **Rozwinięcie:** Alice chce wysłać wiadomość do Boba (offline). Pobiera jego "PreKey" z serwera, łączy ze swoim kluczem tymczasowym (Ephemeral Key) i tworzy sesję bez udziału Boba.
* **Czego to uczy (Teoria & Praktyka):**
    * **Teoria:** Asynchroniczna wymiana kluczy. Forward Secrecy (dlaczego stare klucze są usuwane).
> **Materiały:**
> * [Specyfikacja Signal: X3DH Key Agreement Protocol](https://signal.org/docs/specifications/x3dh/)

#### 2. Szyfrowanie z Uwierzytelnieniem (AEAD)
* **Opis:** Napisz wrapper na funkcje szyfrujące.
    * **Rozwinięcie:** Samo AES to za mało. Użyj `AES-GCM` lub `ChaCha20-Poly1305`. Musisz obsłużyć "Additional Associated Data" (AAD) – dane, które nie są szyfrowane (np. nagłówek), ale są chronione przed zmianą.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `aead`, `aes-gcm`.
    * **Teoria:** Dlaczego szyfrowanie bez autoryzacji jest niebezpieczne (ataki bit-flipping).
> **Materiały:**
> * [Dokumentacja: RustCrypto AEAD Traits](https://docs.rs/aead/latest/aead/)
> * [Teoria: Authenticated Encryption (Wikipedia)](https://en.wikipedia.org/wiki/Authenticated_encryption)

### 🔴 Poziom Trudny

#### 1. Double Ratchet (Grzechotka)
* **Opis:** Implementacja mechanizmu zmieniającego klucze po każdej wiadomości.
    * **Rozwinięcie:** Stwórz łańcuch kluczy (Root Chain, Sending Chain). Po wysłaniu wiadomości wygeneruj nowy klucz z poprzedniego (KDF) i skasuj stary. To uniemożliwia odczytanie starych wiadomości po kradzieży klucza.
* **Czego to uczy (Teoria & Praktyka):**
    * **Algorytmy:** Double Ratchet Algorithm (Signal).
    * **Bezpieczeństwo:** Perfect Forward Secrecy (PFS) i Post-Compromise Security.
> **Materiały:**
> * [Specyfikacja Signal: Double Ratchet Algorithm](https://signal.org/docs/specifications/doubleratchet/)
> * [Wideo: Double Ratchet Lecture (Trevor Perrin)](https://www.youtube.com/watch?v=7W8G89f81GQ)

---

## 5. SIECI OXEN I ONION ROUTING (INFRASTRUKTURA)
*Cel: Zrozumienie jak wysłać pakiet anonimowo, bez centralnego serwera.*

### 🟢 Poziom Łatwy

#### 1. Parser Session ID
* **Opis:** Session ID to po prostu klucz publiczny (05...). Napisz parser.
    * **Rozwinięcie:** Napisz walidator, który sprawdza czy ID ma odpowiednią długość, czy zaczyna się od `05` (standard Session) i konwertuje string hex na bajty potrzebne do kryptografii.
* **Czego to uczy (Teoria & Praktyka):**
    * **Crates:** `hex`.
    * **Oxen:** Struktura identyfikatora w sieci Session.
> **Materiały:**
> * [Oxen Docs: Session ID](https://docs.oxen.io/products-built-on-oxen/session/session-id)
> * [Dokumentacja Crate `hex`](https://docs.rs/hex/latest/hex/)

#### 2. Mock Service Node (UDP)
* **Opis:** Prosty serwer symulujący węzeł sieci.
    * **Rozwinięcie:** Węzeł nasłuchuje na porcie UDP. Odbiera pakiet, wypisuje jego rozmiar i odsyła potwierdzenie. To wstęp do budowania sieci P2P.
* **Czego to uczy (Teoria & Praktyka):**
    * **Rust:** `tokio::net::UdpSocket`.
    * **Sieci:** Różnica między TCP a UDP (Session używa "Onion Requests" często po UDP/QUIC dla szybkości).
> **Materiały:**
> * [Tokio Tutorial: UDP Echo](https://tokio.rs/tokio/tutorial/io)
> * [Beej's Guide to Network Programming (UDP section)](https://beej.us/guide/bgnet/)

### 🟡 Poziom Średni

#### 1. Budowanie Pakietu Cebulowego (Onion Encryption)
* **Opis:** Zaszyfruj wiadomość warstwowo dla 3 węzłów.
    * **Rozwinięcie:**
        1. Weź wiadomość `M`.
        2. Zaszyfruj dla Węzła 3: `E3(M)`.
        3. Zaszyfruj dla Węzła 2: `E2(E3(M))`.
        4. Zaszyfruj dla Węzła 1: `E1(E2(E3(M)))`.
    * Każda warstwa musi zawierać instrukcję "Gdzie wysłać dalej?".
* **Czego to uczy (Teoria & Praktyka):**
    * **Algorytmy:** Onion Routing (jak w Tor/Oxen).
    * **Rust:** Praca na surowych bajtach (`Vec<u8>`).
> **Materiały:**
> * [Wideo: How Tor Works (Computerphile)](https://www.youtube.com/watch?v=QRYzre4bf7I)
> * [Oxen Whitepaper (sekcja Onion Requests)](https://oxen.io/whitepaper.pdf)

#### 2. Service Node Selection (Algorytm Wyboru)
* **Opis:** Napisz algorytm losujący ścieżkę.
    * **Rozwinięcie:** Masz listę 1000 węzłów. Musisz wylosować 3, ale nie mogą to być byle jakie węzły (muszą być aktywne). Zaimplementuj logikę wyboru losowej ścieżki.
* **Czego to uczy (Teoria & Praktyka):**
    * **Algorytmy:** Random Sampling.
    * **Oxen:** Jak klient wybiera trasę, żeby zachować anonimowość.
> **Materiały:**
> * [Teoria: Reservoir Sampling](https://en.wikipedia.org/wiki/Reservoir_sampling)

### 🔴 Poziom Trudny

#### 1. Symulacja Routingu (Mini-Oxen)
* **Opis:** Pełna symulacja przesyłu pakietu przez 3 procesy (Węzły).
    * **Rozwinięcie:**
        * Uruchom 3 instancje swojego programu (Węzły A, B, C).
        * Klient wysyła pakiet do A.
        * A zdejmuje warstwę szyfrowania i widzi "Wyślij do B". Wysyła do B.
        * B zdejmuje warstwę, widzi "Wyślij do C". Wysyła do C.
        * C zdejmuje ostatnią warstwę i zapisuje wiadomość.
* **Czego to uczy (Teoria & Praktyka):**
    * **Architektura:** Systemy rozproszone.
    * **Krypto + Sieci:** Łączenie wszystkiego w całość (Decryption + Forwarding).
> **Materiały:**
> * [Oxen Docs: Service Nodes](https://docs.oxen.io/service-nodes/about-service-nodes)
> * [Kod Źródłowy: Session Desktop (Onion Requests)](https://github.com/oxen-io/session-desktop/blob/master/ts/session/onion_requests.ts)

```