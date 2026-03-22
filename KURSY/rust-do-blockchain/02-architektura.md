Jasne, oto kompletny, jednolity plik `ARCHITECTURE_GUIDE.md` zawierający **wszystkie 6 punktów** (od modelu warstwowego po strategię implementacji).

Możesz skopiować całą zawartość poniżej i zapisać w pliku.

```markdown
# 🏗️ Architecture & Engineering Guide: WhisperVault

> **Dokument Projektowy**
> **Projekt:** WhisperVault (Secure Mobile Dead Drop)
> **Stack:** Flutter (UI) + Rust (Core Logic) + Solana (Blockchain)
> **Status:** Draft Architektury v1.0

---

## 1. Model Warstwowy (The Big Picture)

Projekt nie jest monolitem. To system złożony z trzech niezależnych warstw, które komunikują się w ściśle określony sposób.

### 📱 Warstwa 1: Prezentacja (Flutter)
* **Rola:** "Pilot do telewizora".
* **Odpowiedzialność:** Wyświetlanie pikseli, obsługa gestów, animacje, routing, walidacja formularzy.
* **Zasada:** Flutter jest "głupi". Nie zna kluczy prywatnych, nie szyfruje danych, nie łączy się z bazą SQL. Tylko prosi o dane i je wyświetla.

### ⚙️ Warstwa 2: Core & Logika (Rust)
* **Rola:** "Silnik".
* **Odpowiedzialność:**
    * **Kryptografia:** Haszowanie (SHA256), Szyfrowanie (XChaCha20Poly1305), Podpisywanie (Ed25519).
    * **Key Management:** Generowanie i bezpieczne usuwanie kluczy z pamięci (`zeroize`).
    * **Pamięć trwała:** Obsługa bazy danych (SQLite/SQLCipher).
    * **Sieć:** Klient RPC Solany, budowanie i wysyłanie transakcji.
* **Zasada:** To tutaj żyje "bezpieczeństwo". Dane wrażliwe nigdy nie opuszczają tej warstwy w formie jawnej.

### 🏛️ Warstwa 3: Źródło Prawdy (Data Layer)
* **Prawda Prywatna (SQLite):** Lokalna historia transakcji, zaszyfrowane klucze, metadane (np. nazwy kontaktów). Baza znajduje się na urządzeniu.
* **Prawda Publiczna (Solana Blockchain):** Stan kont, salda, dowody (Hashes/Proofs). To jest ostateczny arbiter stanu finansowego.

---

## 2. Przepływ Danych (Data Flow)

Dane wędrują przez **Most FFI** (`flutter_rust_bridge`).



### Scenariusz: "Wyślij Depozyt" (Deposit Flow)
1.  **Flutter:** Pobiera od użytkownika input (np. `amount: 1.5 SOL`).
2.  **Most:** Przekazuje `1.5` do funkcji Rusta `create_deposit()`.
3.  **Rust (Core):**
    * Generuje losowy `Secret` (32 bajty).
    * Oblicza `Hash(Secret)`.
    * Buduje instrukcję Anchor `deposit` + podpisuje transakcję kluczem prywatnym (z secure storage).
    * Wysyła transakcję do węzła RPC Solany.
4.  **Solana:** Waliduje podpis i zmienia stan konta on-chain (przelewa środki do PDA).
5.  **Rust (Core):**
    * Odbiera potwierdzenie (Signature).
    * Szyfruje `Secret` i zapisuje go w lokalnym SQLite dla historii transakcji.
6.  **Most:** Zwraca do Fluttera obiekt sukcesu (zawierający `Secret` do wyświetlenia w QR kodzie).

> **Złota Zasada:** Flutter nigdy nie dotyka bazy danych ani klucza prywatnego bezpośrednio. Zawsze robi to "rękami" Rusta.

---

## 3. Struktura Katalogów (Organizacja Kodu)

Architektura musi być widoczna w strukturze plików. Odseparuj UI od Logiki.

```text
whisper_vault/
├── app/                  <-- WARSTWA PREZENTACJI (FLUTTER)
│   ├── pubspec.yaml
│   ├── lib/
│   │   ├── main.dart
│   │   ├── bridge_generated.dart  <-- Automatyczny kod FFI (nie dotykać!)
│   │   ├── screens/               <-- Ekrany (Deposit, Claim, History)
│   │   ├── providers/             <-- State Management (Riverpod) - tu wołasz Rusta
│   │   └── models/                <-- Modele danych UI
│
├── native/               <-- WARSTWA LOGIKI (RUST)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── api.rs        <-- KONTRAKT DLA FLUTTERA (Public Interface)
│   │   ├── lib.rs        <-- Entry point dla FFI
│   │   ├── crypto.rs     <-- Logika szyfrowania (Argon2, ChaCha20)
│   │   ├── db.rs         <-- Obsługa SQLite/SQLCipher (Schema, Queries)
│   │   └── solana.rs     <-- Klient RPC, instrukcje Anchor
│
└── contracts/            <-- WARSTWA DANYCH ON-CHAIN (ANCHOR)
    ├── Anchor.toml
    ├── programs/
    │   └── vault/src/lib.rs <-- Twój Smart Contract
    └── tests/            <-- Testy kontraktu w TypeScript

```

---

## 4. Definiowanie API (Kontrakt Rust-Flutter)

Plik `native/src/api.rs` definiuje, co Flutter może zrobić. Traktuj to jak menu w restauracji – Flutter (Klient) zamawia, Rust (Kuchnia) wykonuje.

To tutaj używasz typów, które rozumie `flutter_rust_bridge`.

```rust
// native/src/api.rs

// --- STRUKTURY DANYCH (OUTPUT) ---
// Muszą być proste i serializowalne.

pub struct DepositSummary {
    pub signature: String,     // Hash transakcji
    pub secret_qr: String,     // Sekret do wyświetlenia
    pub timestamp: i64,
}

pub struct WalletBalance {
    pub sol: f64,
    pub usdc: f64,
}

// --- FUNKCJE (ACTIONS) ---

// Inicjalizacja bazy danych i generowanie kluczy.
// Przyjmuje hasło, zwraca Public Key (jako String) lub Błąd.
pub fn init_wallet(password: String) -> Result<String> {
    // 1. KDF (Argon2) na haśle -> Key
    // 2. Otwarcie SQLCipher
    // 3. Sprawdzenie czy klucze (Ed25519) istnieją? Jak nie -> generuj.
    // 4. Zwróć PubKey.
}

// Logika biznesowa depozytu.
// Przyjmuje kwotę, zwraca dane do wyświetlenia (QR).
pub fn create_deposit(amount: f64) -> Result<DepositSummary> {
    // 1. Generuj Secret (32 bajty).
    // 2. Zbuduj Tx do Solany (Anchor Instruction: Deposit).
    // 3. Wyślij Tx do RPC.
    // 4. Po sukcesie: Zaszyfruj Secret i zapisz w DB.
}

// Pobranie historii (tylko odczyt z lokalnej bazy).
pub fn fetch_history() -> Result<Vec<DepositSummary>> {
    // Wykonaj: SELECT * FROM deposits ORDER BY timestamp DESC
}

// Odbieranie depozytu (Claim).
pub fn claim_deposit(secret: String) -> Result<String> {
    // 1. Hash(secret).
    // 2. Zbuduj Tx do Solany (Anchor Instruction: Withdraw).
    // 3. Wyślij.
}

```

---

## 5. Zarządzanie Stanem (State Management)

Kluczem do stabilnej aplikacji jest wiedza, gdzie żyją dane i jak długo tam przebywają.

| Typ Stanu | Przykłady | Gdzie przechowywać? | Trwałość | Zasada bezpieczeństwa |
| --- | --- | --- | --- | --- |
| **Stan UI (Ulotny)** | Tekst w polu input, stan animacji, "Is Loading", błędy walidacji | **Flutter** (Riverpod `StateProvider` / `Hook`) | Ginie po zmianie ekranu | Dane jawne, nietrwałe. |
| **Stan Aplikacji (Cache)** | Lista transakcji pobrana z Rusta, Public Key, Saldo | **Flutter** (Riverpod Cache) | Ginie po restarcie apki | Tylko dane publiczne. |
| **Stan Wrażliwy (Runtime)** | Klucz do bazy (po Argon2), Seed Phrase (podczas podpisywania) | **Rust** (Heap / `Secrecy` crate / `LazyStatic`) | Ginie po restarcie (Musi zginąć!) | Użyj `Zeroize` (nadpisywanie pamięci zerami) po użyciu. |
| **Stan Trwały (Local)** | Historia, Zaszyfrowane Klucze, Ustawienia | **Rust** (SQLite / SQLCipher) | Przetrwa restart telefonu | Szyfrowanie "At-Rest" (klucz zna tylko użytkownik). |
| **Stan Globalny (Chain)** | Saldo konta, Stan Smart Kontraktu | **Solana** (Accounts) | Wieczna (On-Chain) | Ostateczne źródło prawdy finansowej. |

---

## 6. Strategia Implementacji (Vertical Slices)

Nie buduj warstwami (najpierw cały UI, potem cały backend). Buduj funkcjonalnościami (Vertical Slices).

1. **Slice 1: Hello World & Bridge**
* **Cel:** Przycisk we Flutterze wywołuje `fn hello() -> String` w Ruście.
* **Weryfikacja:** `flutter_rust_bridge` jest poprawnie skonfigurowany, aplikacja kompiluje się na emulatorze.


2. **Slice 2: Secure Storage (Baza)**
* **Cel:** Flutter wysyła tekst -> Rust szyfruje go i zapisuje w SQLite (SQLCipher).
* **Weryfikacja:** Możesz odczytać notatkę w aplikacji, ale plik bazy jest nieczytelny w notatniku.


3. **Slice 3: Network Read (Solana)**
* **Cel:** Flutter prosi o saldo -> Rust pyta RPC Solany -> Wyświetlasz wynik.
* **Weryfikacja:** Widzisz poprawne saldo z Devnetu (wyślij sobie airdrop).


4. **Slice 4: On-Chain Write (Kontrakt)**
* **Cel:** Flutter zleca depozyt -> Rust podpisuje i wysyła transakcję do Anchor (Localhost/Devnet).
* **Weryfikacja:** Transakcja jest widoczna na Solscan, a stan konta PDA się zmienia.


5. **Slice 5: Pełna Pętla (WhisperVault MVP)**
* **Cel:** Generowanie sekretu, zapis do bazy, wysyłka on-chain, odbiór sekretu, wypłata.
* **Weryfikacja:** Jeden telefon wpłaca, drugi wypłaca (znając sekret).



```

```