# Disk Analyzer

Projekt zaliczeniowy na przedmiot *Systemy Operacyjne* na [Akademii Górnośląskiej](https://www.gwsh.pl) w Katowicach.

Projekt jest prostym CLI umożliwiającym przeanalizowanie plików w podanym katalogu pod względem ich rozmiaru.

## Uruchomienie

Aby uruchomić program, należy sklonować repozytorium, a następnie wykonać polecenie:

### Wersja rozwojowa

```bash
cargo run
```

### Wersja produkcyjna

```bash
cargo build --release
./target/release/disk-analyzer <flagi> <ścieżka do katalogu>
```

Opis dostępnych flag można uzyskać wykonując polecenie:

```bash
./target/release/disk-analyzer --help
```
