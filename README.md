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

## Dostępne flagi

- `-r`, `--recursive`: Jeśli ustawione, program będzie uwzględniał każdy katalog w ścieżce.
- `-d`, `--depth [GŁĘBOKOŚĆ]`: Użyj tej opcji, aby ograniczyć głębokość przeszukiwania katalogów. Domyślnie: 999.
- `-q`, `--quiet`: Jeśli ustawione, program będzie wyświetlał tylko podsumowanie informacji.
- `-i`, `--ignore-extension [ROZSZERZENIE]`: Jeśli ustawione, program będzie ignorował podane rozszerzenia plików.
- `-o`, `--only-extension [ROZSZERZENIE]`: Jeśli ustawione, program będzie uwzględniał tylko podane rozszerzenia plików,
  ignorując wszystkie inne.
- `--human-unit`: Jeśli ustawione, program będzie wyświetlał rozmiar w formacie czytelnym dla człowieka.

## Opis funkcji

1. **main()** - Funkcja główna programu, inicjuje parsowanie argumentów wiersza poleceń, sprawdza istnienie i typ
   ścieżki podanej przez użytkownika, a następnie rozpoczyna proces przeszukiwania.

2. **print_summary()** - Funkcja do wyświetlania podsumowania wyników przeszukiwania, w tym liczby katalogów, plików,
   całkowitego rozmiaru oraz największego pliku.

3. **traverse_entry()** - Główna funkcja przeszukiwania, rekursywnie przeszukuje katalogi i zbiera informacje o ich
   zawartości.

4. **print_entry_data()** - Funkcja do wyświetlania danych pojedynczego elementu (pliku lub katalogu) w zależności od
   jego głębokości w drzewie katalogów.

5. **convert_size_to_human_unit()** - Funkcja konwertująca rozmiar pliku z bajtów na czytelne jednostki zgodnie z
   konwencją binarną (np. KB, MB, GB).
