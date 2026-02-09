# Plan: Utworzenie docs/FAZA_0.md

## Kontekst
Użytkownik chce szczegółowego rozbicia Fazy 0 z PLAN.md na plik `docs/FAZA_0.md`, który będzie służył jako przewodnik implementacji. Projekt jest na etapie czystej dokumentacji — zero kodu Rust.

## Co zostanie zrobione
Utworzenie pliku `docs/FAZA_0.md` zawierającego:

### 14 zadań (0.1–0.14) w kolejności zależności:

| # | Zadanie | Milestone | Zależy od |
|---|---------|-----------|-----------|
| 0.1 | Inicjalizacja Cargo workspace (7 crate'ów) | — | — |
| 0.2 | Konfiguracja rustfmt/clippy/.editorconfig | — | 0.1 |
| 0.3 | Definicje Error enum per crate (thiserror) | — | 0.1 |
| 0.4 | Modele domenowe i traity w taran-core | — | 0.3 |
| 0.5 | Parser scenariuszy TOML (taran-config) | — | 0.4 |
| 0.6 | Szkielet taran-metrics (SimpleCollector) | — | 0.4 |
| 0.7 | HttpClient w taran-protocols (reqwest) | — | 0.4 |
| 0.8 | Stub taran-script | — | 0.3 |
| 0.9 | Stub taran-report + ConsoleReporter | — | 0.4 |
| 0.10 | CLI z clap (taran-cli) — `taran --help` | **M0** | 0.5 |
| 0.11 | CI/CD GitHub Actions | — | 0.2 |
| 0.12 | TestRunner w taran-core (1 VU, sekwencyjny) | — | 0.6, 0.7 |
| 0.13 | Integracja e2e — `taran run test.toml` | **M1** | 0.10, 0.12, 0.9 |
| 0.14 | Testy integracyjne (wiremock, E2E) | — | 0.13 |

### Ścieżka krytyczna
```
0.1 → 0.3 → 0.4 → {0.5, 0.6, 0.7} → 0.10 → 0.12 → 0.13 → 0.14
```

### Kluczowe decyzje architektoniczne
- Generics (statyczny dispatch) zamiast trait objects — YAGNI, 1 protokół
- `HumanDuration` wrapper w taran-core do deserializacji "60s"/"500ms"
- `SimpleCollector` z `Mutex<HashMap>` — lock-free HDR w Fazie 3
- `#[tokio::main]` TYLKO w taran-cli/src/main.rs

### Pliki do utworzenia
~48 plików (źródła, manifesty, testy, CI/CD, przykłady)

## Weryfikacja
Po utworzeniu FAZA_0.md:
- Plik jest spójny z PLAN.md i CLAUDE.md
- Każde zadanie ma: opis, pliki do utworzenia, decyzje, kryterium ukończenia
- Graf zależności jest poprawny
