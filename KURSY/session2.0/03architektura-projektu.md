## 2. ARCHITEKTURA PROJEKTU

Projekt podzielony na warstwy ("Clean Architecture").

### Struktura folderów:
```text
my_secure_chat/
├── android/ & ios/      # Pliki natywne platform
├── lib/                 # KOD DART (FLUTTER) - UI
│   ├── main.dart        
│   ├── screens/         
│   ├── providers/       # Riverpod (Stan aplikacji)
│   ├── models/          
│   └── ffi.dart         # Most do Rusta (wygenerowany automat)
├── native/              # KOD RUST - LOGIKA
│   ├── src/
│   │   ├── api.rs       # Funkcje wystawione dla Fluttera
│   │   ├── crypto.rs    # Szyfrowanie (X25519, AES/Chacha)
│   │   ├── network.rs   # Obsługa Oxen/Onion routing
│   │   └── storage.rs   # Szyfrowana baza danych (lokalna)
│   └── Cargo.toml       
└── pubspec.yaml

---