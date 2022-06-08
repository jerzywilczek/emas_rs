# Specyfikacja wymagań
## Wymagania funkcjonalne
### Implementacja algorytmu EMAS
Należy zaimplementować klasyczną wersję algorytmu EMAS
#### Populacja
   * agenty są porozrzucane na wyspach
   * wyspy układają się zawsze w topologię grafu pełnego
   * energia wśród populacji jest stała w każdym kroku czasowym
#### Agenty
   * rodzą się z zadaną energią
   * mają w genotypie zakodowane rozwiązanie problemu
   * w zależności od energii podejmują z zadanym prawdopodobieństwem decyzję o walce z innym agentem bądź reprodukcji
#### Sposób walki 
   * agenty porównują swoje przystosowanie (wartość funkcji celu) - im mniejsza, tym lepiej
   * agent lepiej przystosowany ma większą szansę na zwycięstwo (zgodnie z zadanym prawdopodobieństwem)
   * wygrany przekazuje zdefiniowaną ilość lub część energii
#### Reprodukcja 
   * agenty mogą się reprodukować pod warunkiem, że ich energia jest wyższa od progu reprodukcji
   * jej wynikiem jest powstanie dwóch nowych osobników o cechach zbliżonych do rodziców
   * genotyp dzieci określamy na podstawie:
       * funkcji rekombinacji – skomponowania nowego genotypu na podstawie genotypu rodziców
       * funkcji mutacji – zmutowania genotypu powstałego w wyniku rekombinacji
   * agenty będące rodzicami tracą pewną część bądź wartość swej energii na rzecz dzieci, które dzielą się tą energią po równo
#### Migracja
   * odbywa się co ustaloną liczbę kroków czasowych
   * wybieramy grupę X osobników o najlepszym przystosowaniu na wyspie, a następnie wybieramy spośród nich losowo Y <= X, które zostają po równo rozdzielone pomiędzy pozostałe wyspy
#### Śmierć 
   * umierają agenty, których energia nie przekracza progu śmierci
   * powyższy warunek sprawdzamy po każdym kroku czasowym związanym z reprodukcją i walką
#### Kroki czasowe
   * klasyczny – odbywa się walka i reprodukcja, a następnie czyszczenie wyspy ze zmarłych osobników
   * migracja – wykonujemy operację migracji
   * wyznaczamy co ile kroków ma się odbywać migracja, w pozostałych wykonujemy krok klasyczny
#### Funkcja celu
   * funkcja, której argumentem jest wektor
   * problemy ciągłe i dyskretne
#### Wynik
   * genotyp agenta, którego przystosowanie było historycznie najlepsze

### API
#### Możliwości konfiguracji (interface)
Użytkownik ma mieć możliwość konfiguracji następujących elementów algorytmu:
 * funkcji celu, którą algorytm będzie optymalizował
 * ilości wysp
 * początkowej ilości agentów
 * energii startowej agenta
 * funkcji wybierającej na podstawie ilości energii agenta, jaką akcję ma on podjąć w danym kroku
 * liczbę kroków czasowych, po której mamy zalogować dotychczasowy postęp

 * funkcji dokonującej rekombinacji genetycznej
 * funkcji wprowadzającej losowe mutacje w kodzie genetycznym nowo narodzonych
 * ilości energii przekazywanej dzieciom

 * funkcji wykonującej _walkę_ dwóch agentów

Konfigurowanie parametrów algorytmu powinno być jak najbardziej idiomatyczne 
### Wizualizacja
Biblioteka powinna wspierać wypisywanie _logów_ do formatu, który będzie łatwo przetworzyć na wykresy ilustrujące przebieg algorytmu (np. csv). Możliwe, że będzie też potrafiła je narysować, ale na obecnym etapie prac ciężko nam ocenić, czy damy radę zaimplementować taką funkcjonalność jako jej element. Logować będziemy nastepujące wartości:

#### Logowanie wyników
Zbieramy następujące wartości:
 * najlepszy osobnik historycznie
 * najlepszy osobnik w tym momencie
 * liczba osobników na danej wyspie
 * sumaryczna ilość energii na poszczególnych wyspach
 * średni fitness wyspy
 * średnia energia osobnika na wyspie
 * timestamp

Logowanie powinno odbywać się niezależnie od przebiegu algorytmu (co zadany interwał czasowy).

## Wymagania niefunkcjonalne
### Rust
Wybór języka jest nieprzypadkowy:
 * jego szybkość jest porównywalna z C
 * zapewnia niemal całkowite bezpieczeństwo pamięciowe
 * wymaga na programiście obsługę wszystkich możliwych do uzyskania podczas wykonania programu sytuacji i wyjątków
 * pozwala na bezproblemową współbieżność

Wszystko to sprawia, że rust jest idealny do realizacji szybkich obliczeń.
### Wydajność
Współbieżne wykonywanie algorytmu (tj. osobne działanie wysp) zostanie zaimplementowane w kolejnym semestrze, po pełnym zrealizowaniu wszystkich funkcjonalności
### Użyteczność
Biblioteka powinna dostarczać gotowy do użycia przez użytkownika interfejs z domyślnymi ustawieniami oraz przykładowym wywołanie. Chcemy, by początkowy użytkownik nie musiał być zaznajomiony ze szczegółami algorytmu i implementacji, a raczej by mógł szybko uruchomić bibliotekę i później wdrażać się w szczegóły i niuanse wszystkich ustawień.
