## 1. ŚCIEŻKA EDUKACYJNA (ROADMAP)

### Fundamenty Rust (Backend & Core Logic)
**Czas:** 4-6 tygodni
1.  **Podstawy:** Zmienne, własność (ownership), pożyczanie (borrowing), cykl życia (lifetimes).
2.  **Struktury danych:** Structs, Enums, Pattern Matching, Result/Option.
3.  **Współbieżność:** Wątki, Async/Await (Tokio runtime).
* *Materiały:* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)

### Moduł 1.5: Bazy Danych Key-Value (Sled & Serde)
**Czas:** 2-3 tygodnie
**Cel:** Nauczyć się trwałego zapisu danych w nowoczesnej, wbudowanej bazie Key-Value, gdzie każda wartość jest szyfrowana przed zapisem (Application Level Encryption).

1.  **Serializacja Danych (Serde + Bincode):**
    * Baza `sled` przechowuje surowe bajty. Musisz umieć zamienić swoje struktury (np. `Message`) na ciąg bajtów i z powrotem.
    * *Kluczowe pojęcia:* `Serialize`, `Deserialize`, format `bincode` (jest znacznie szybszy i lżejszy niż JSON, idealny do krypto).
    * **Materiały do nauki:**
        * [Serde.rs - Oficjalna dokumentacja](https://serde.rs/)
        * [Bincode Docs - Format binarny](https://docs.rs/bincode/latest/bincode/)
        * [Rust Cookbook: Serializing with Bincode](https://rust-lang-nursery.github.io/rust-cookbook/encoding/complex.html)

2.  **Sled - Baza Danych (The Metal Store):**
    * Obsługa `sled`: wbudowanej bazy napisanej w 100% w Rust. Działa jak wielka `HashMap`, która zapisuje się na dysku.
    * *Kluczowe pojęcia:* `Db::open`, `insert`, `get`, `remove`, `IVec` (wewnętrzny typ bufora w sled).
    * **Materiały do nauki:**
        * [Sled Crate Documentation](https://docs.rs/sled/latest/sled/)
        * [Sled Examples (GitHub)](https://github.com/spacejam/sled/tree/master/examples)
        * [Tutorial: Using Sled in Rust](https://blog.logrocket.com/using-sled-rust-embedded-database/)

3.  **Strategia Szyfrowania (Encrypt-then-Store):**
    * `sled` nie szyfruje automagicznie jak SQLCipher. Musisz to zrobić sam.
    * *Kluczowe pojęcia:* Szyfrowanie wartości (`ChaCha20Poly1305`) **przed** wywołaniem `db.insert()`. Klucz bazy może pozostać jawny (np. ID wiadomości), ale Wartość musi być blobem nieczytelnym dla atakującego.
    * **Materiały do nauki:**
        * [RustCrypto: AEAD Traits](https://docs.rs/aead/latest/aead/)
        * [Pattern: Application-Level Encryption](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)

4.  **Drzewa i Skanowanie (Trees & Scanning):**
    * Jak pobrać "wszystkie wiadomości" bez SQL-owego `SELECT *`?
    * *Kluczowe pojęcia:* Prefiksy kluczy (np. `chat_01:msg_001`), funkcja `scan_prefix` (iterator po zakresie), `Batch` (atomowy zapis wielu operacji naraz – transakcje).
    * **Materiały do nauki:**
        * [Sled Docs: Trees (Namespaces)](https://docs.rs/sled/latest/sled/struct.Db.html#method.open_tree)
        * [Sled Docs: Batch (Atomicity)](https://docs.rs/sled/latest/sled/struct.Batch.html)


### Moduł 2: Kryptografia Stosowana (Nie matematyka, a inżynieria)
**Czas:** 4-5 tygodni
**Cel:** Zrozumienie, jak bezpiecznie używać gotowych algorytmów. Twoim celem nie jest napisanie szyfrowania, ale jego poprawne zaimplementowanie (żeby nie pomylić klucza prywatnego z publicznym lub nonce).

1.  **Fundamenty (Teoria):**
    * **Szyfrowanie Symetryczne (AES/ChaCha20):** Jeden klucz do szyfrowania i deszyfrowania.
    * **Szyfrowanie Asymetryczne (ECC - Krzywe Eliptyczne):** Klucz prywatny (Twój sekret) i publiczny (Twój identyfikator).
    * **Funkcje Skrótu (Hashing):** Dlaczego SHA-256 jest jednokierunkowe.
    * **Podpisy Cyfrowe (Ed25519):** Jak udowodnić, że wiadomość wysłałeś Ty, a nie ktoś, kto przechwycił Twój pakiet.

2.  **Protokół Signal (Double Ratchet):**
    * To serce Session (i Signala, i WhatsAppa). Pozwala na to, że nawet jeśli ktoś ukradnie Twój klucz dzisiaj, nie odczyta wiadomości z wczoraj (Forward Secrecy).
    * *Kluczowe pojęcia:* Root Key, Chain Key, Message Key, Diffie-Hellman Ratchet.

3.  **Biblioteki Rust (Praktyka):**
    * Nauczysz się używać `x25519-dalek` (wymiana kluczy), `ed25519-dalek` (podpisy) i `ring`.

* **Materiały Edukacyjne (Darmowe):**
    * **Kurs:** [Dan Boneh - Cryptography I (Coursera)](https://www.coursera.org/learn/crypto) – Absolutny klasyk. Możesz obejrzeć za darmo (opcja "Audit"). Obejrzyj pierwsze tygodnie o szyfrowaniu symetrycznym i asymetrycznym.
    * **Dokumentacja:** [Signal Protocol Technical Documentation](https://signal.org/docs/) – To jest "Biblia". Przeczytaj o "X3DH" i "Double Ratchet". Session bazuje na modyfikacji tego protokołu.
    * **Książka (Online):** [A Graduate Course in Applied Cryptography](https://toc.cryptobook.us/) – Darmowa wersja online książki Boneha i Shoupa. Rozdziały o Public Key Encryption.
    * **Rust Specific:** [Rust Crypto Guidelines](https://github.com/RustCrypto) – Przejrzyj repozytoria i przykłady użycia ("examples" w folderach).

---

### Moduł 3: Sieć Oxen i Onion Routing (Infrastruktura)
**Czas:** 4-6 tygodni
**Cel:** Zrozumienie, jak wysłać pakiet, żeby nikt nie wiedział, kto go wysłał i do kogo.

1.  **Onion Routing (Trasowanie Cebulowe):**
    * Pakiet jest szyfrowany warstwowo. Każdy węzeł w sieci zdejmuje jedną warstwę i wie tylko, gdzie przesłać pakiet dalej. Nie wie, skąd przyszedł ani co jest w środku.
    * Różnica względem Tora: Oxen używa "Onion Requests" opartych na pakietach, a nie długotrwałych tuneli (jak Tor), co jest lepsze dla wiadomości asynchronicznych.

2.  **Service Nodes (Węzły Usługowe):**
    * To serwery, które przechowują wiadomości. Nie ma jednego centralnego serwera.
    * **Swarms (Rojowiska):** Grupy węzłów, które przechowują wiadomości dla konkretnego zakresu kluczy publicznych. Musisz umieć obliczyć, do którego "roju" wysłać wiadomość dla użytkownika X.

3.  **Session Protocol:**
    * Jak wygląda pakiet Session? Jak klient pyta sieć: "Czy są dla mnie wiadomości?".

* **Materiały Edukacyjne (Specyficzne dla Oxen):**
    * **Źródło Prawdy:** [Oxen Documentation](https://docs.oxen.io/) – Tu znajdziesz opis API, jak działają Service Nodes i jak budować zapytania.
    * **Whitepaper:** [Oxen/Loki Whitepaper](https://oxen.io/whitepaper) – Dokument techniczny opisujący architekturę. Przeczytaj sekcje o "Service Nodes" i "Storage Server".
    * **Kod Źródłowy (Najważniejsze!):**
        * [Oxen Core (C++)](https://github.com/oxen-io/oxen-core) – Backend sieci. Trudny, ale warto rzucić okiem.
        * [Session Desktop (JS/Electron)](https://github.com/oxen-io/session-desktop) – Łatwiej zrozumieć logikę w JavaScript niż w C++. Zobacz jak budują "onion requests".
        * [Session Android (Kotlin/C++)](https://github.com/oxen-io/session-android) – Zobacz folder `app/src/main/java/network/loki/messenger`.








