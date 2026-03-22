## CZESC 5: ANALIZA TECHNICZNA: - PLIK CARGO.TOML
# Ten plik definiuje "klocki", z których zbudowany będzie backend Twojej aplikacji.
# Znajduje się w folderze: native/Cargo.toml

[package]
name = "native"
version = "0.1.0"
edition = "2021"

# To jest kluczowe dla mobile. Mówi Rustowi, żeby budował bibliotekę dynamiczną (.so/.dylib)
# którą Android/iOS mogą załadować, oraz statyczną.
[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
# --- MOST MIĘDZY JĘZYKAMI ---
# To narzędzie generuje kod w Dart i Rust, który pozwala im rozmawiać.
flutter_rust_bridge = "1.80"

# --- ASYNCHRONICZNOŚĆ ---
# Rust domyślnie jest synchroniczny. Tokio to "silnik", który pozwala robić wiele rzeczy naraz
# (np. nasłuchiwać wiadomości z sieci i jednocześnie obsługiwać kliknięcia w UI).
tokio = { version = "1", features = ["full"] }
futures = "0.3" # Narzędzia do obsługi zadań, które zakończą się w przyszłości.

# --- KRYPTOGRAFIA (SERCE SESSION) ---
# X25519: Do wymiany kluczy (Diffie-Hellman). Pozwala ustalić wspólny sekret.
x25519-dalek = "2.0"
# Ed25519: Do podpisywania wiadomości. Dowodzi, że Ty to Ty.
ed25519-dalek = "2.0"
# ChaCha20-Poly1305: Szybkie i bezpieczne szyfrowanie treści wiadomości.
chacha20poly1305 = "0.10"
# Rand: Generator liczb losowych. Musi być kryptograficznie bezpieczny (CSPRNG).
rand = "0.8"
# Haszowanie (np. do tworzenia identyfikatorów z kluczy).
sha2 = "0.10"
# Kodowanie binarne do tekstu (np. klucze publiczne często przesyła się jako base64).
base64 = "0.21"

# --- SERIALIZACJA DANYCH ---
# Serde to standard w Rust. Pozwala zamienić strukturę `Wiadomosc` na bajty (i odwrotnie).
serde = { version = "1.0", features = ["derive"] }
# Format binarny, lżejszy i szybszy od JSON. Idealny do sieci P2P.
bincode = "1.3"

# --- BAZA DANYCH (LOKALNA) ---
# SQLite jest prosty na start. W wersji produkcyjnej użyjesz SQLCipher do szyfrowania bazy.
rusqlite = { version = "0.29", features = ["bundled"] }

# --- OBSŁUGA BŁĘDÓW ---
# Anyhow pozwala łatwo zwracać błędy w funkcjach (używaj w aplikacji).
anyhow = "1.0"
# Thiserror pozwala definiować własne typy błędów (używaj w bibliotekach/modułach).
thiserror = "1.0"

# --- LOGOWANIE ---
# Pozwala widzieć co robi Rust w konsoli Android Studio (Logcat) lub Xcode.
log = "0.4"
android_logger = "0.13"
oslog = "0.1"



