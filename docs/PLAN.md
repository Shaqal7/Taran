# Taran — Plan realizacji projektu

> Taran — wysokowydajne narzędzie do testów obciążeniowych napisane w Rust.
> Alternatywa dla JMeter, Gatling i K6.

---

## 1. Ocena sensowności projektu

### Dlaczego TAK

- **Realna luka rynkowa:** Goose wymaga kompilacji Rusta (bariera dla testerów), Drill jest zbyt prosty (brak skryptowania, korelacji, asercji), K6 jest w Go (gorszy profil pamięci i brak zero-cost abstractions). Żadne narzędzie nie łączy jednocześnie: binarka Rust + skryptowanie bez rekompilacji + real-time metryki.
- **Trend rynkowy:** Świat DevOps/SRE odchodzi od GUI na rzecz narzędzi CI/CD-friendly. Taran idealnie wpisuje się w ten trend.
- **Przewaga techniczna Rusta:** Brak GC = deterministyczne pomiary latencji. Tokio async = dziesiątki tysięcy VU przy minimalnym zużyciu RAM. To nie jest marketingowy slogan — to mierzalna różnica.
- **Problem Coordinated Omission:** Większość narzędzi go ignoruje. Poprawna implementacja open-loop modelu w Rust będzie USP (Unique Selling Point).
- **Dystrybucja:** Pojedyncza binarka bez zależności (statyczny link z musl) — łatwa do wdrożenia w kontenerach i pipeline'ach CI/CD.

### Ryzyka do zarządzania

- Ograniczony ekosystem crate'ów dla protokołów niszowych (JDBC, JMS, SOAP).
- Bariera wejścia dla testerów przyzwyczajonych do GUI JMeter'a.
- Konkurencja z K6, które ma silną społeczność i backing Grafana Labs.
- Złożoność async Rust (borrow checker + współdzielony stan metryk).

### Strategia mitygacji ryzyk

- Skupić się na HTTP/HTTPS/gRPC/WebSocket jako MVP. Niszowe protokoły dodawać przez system pluginów.
- Oferować doskonałą dokumentację i przykłady. Rozważyć TUI (terminal UI) jako kompromis między CLI a GUI.
- Wyróżnić się wydajnością i poprawnością pomiarów (HDR Histogram, brak coordinated omission).
- Intensywnie korzystać z `Arc<AtomicU64>` i lock-free struktur zamiast Mutex dla metryk.

---

## 2. Architektura wysokopoziomowa

```
┌─────────────────────────────────────────────────────────┐
│                        CLI (clap)                       │
│              Parsowanie argumentów i komend              │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                  Config Loader (TOML/YAML)               │
│         Wczytywanie scenariuszy i konfiguracji           │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                   Script Engine (Rhai)                    │
│        Skryptowanie scenariuszy bez rekompilacji         │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│               Execution Engine (Tokio)                   │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐      │
│  │  VU #1  │ │  VU #2  │ │  VU #3  │ │  VU #N  │      │
│  │ (task)  │ │ (task)  │ │ (task)  │ │ (task)  │      │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘      │
│       └────────────┴──────────┴────────────┘            │
│                        │                                 │
│  ┌─────────────────────▼───────────────────────────┐    │
│  │          Protocol Clients (reqwest/hyper)         │    │
│  │     HTTP · gRPC · WebSocket · TCP raw            │    │
│  └─────────────────────┬───────────────────────────┘    │
└────────────────────────┼────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│              Metrics Collector (lock-free)                │
│  HDR Histogram · Throughput · Error rates · Percentyle   │
└──────────┬─────────────────────────────┬────────────────┘
           │                             │
┌──────────▼──────────┐    ┌─────────────▼───────────────┐
│  Real-time Reporter │    │     Export / Sink            │
│  (TUI: ratatui)     │    │  InfluxDB · Prometheus      │
│  Live dashboard      │    │  JSON · CSV · HTML          │
└─────────────────────┘    └─────────────────────────────┘
```

---

## 3. Fazy realizacji

### Faza 0 — Fundament (2-3 tygodnie)

**Cel:** Działający szkielet projektu z minimalnym przepływem end-to-end.

#### Zadania:

- [ ] Inicjalizacja projektu Cargo (workspace z wieloma crate'ami)
  ```
  taran/
  ├── Cargo.toml          (workspace)
  ├── taran-cli/          (binarka — punkt wejścia)
  ├── taran-core/         (silnik wykonawczy, VU, scheduler)
  ├── taran-config/       (parsowanie TOML/YAML)
  ├── taran-metrics/      (zbieranie i agregacja metryk)
  ├── taran-report/       (generowanie raportów)
  ├── taran-script/       (silnik skryptowy Rhai)
  └── taran-protocols/    (klienty protokołów)
  ```
- [ ] Konfiguracja CI/CD (GitHub Actions)
  - Linting (`clippy`), formatowanie (`rustfmt`), testy (`cargo test`)
  - Budowanie binarek na Linux (musl), macOS, Windows
- [ ] Podstawowa struktura CLI z `clap`
  - `taran run <scenario.toml>` — uruchomienie testu
  - `taran validate <scenario.toml>` — walidacja scenariusza
  - `taran --version`, `taran --help`
- [ ] Definicja formatu scenariusza testowego (TOML)
  ```toml
  [scenario]
  name = "Basic HTTP Test"
  
  [load_profile]
  type = "constant"      # constant | ramp | stepped | spike
  users = 100
  duration = "60s"
  ramp_up = "10s"
  
  [[steps]]
  name = "GET Homepage"
  protocol = "http"
  method = "GET"
  url = "https://example.com/"
  
  [steps.assertions]
  status = 200
  max_response_time = "500ms"
  
  [[steps]]
  name = "POST Login"
  protocol = "http"
  method = "POST"
  url = "https://example.com/api/login"
  headers = { "Content-Type" = "application/json" }
  body = '{"username": "test", "password": "test123"}'
  
  [steps.extract]
  token = { from = "body", type = "jsonpath", expr = "$.token" }
  
  [steps.assertions]
  status = 200
  ```
- [ ] Minimalna implementacja Tokio runtime z jednym VU wykonującym HTTP GET

### Faza 1 — Silnik wykonawczy (3-4 tygodnie)

**Cel:** Pełny silnik obciążeniowy z poprawnym modelem generowania ruchu.

#### Zadania:

- [ ] **Scheduler (load profile):**
  - Constant load (stała liczba VU)
  - Ramp-up / ramp-down (liniowe zwiększanie/zmniejszanie)
  - Stepped load (schodkowy wzrost)
  - Spike test (nagły skok)
  - Custom profile (definiowany w skrypcie)
- [ ] **Virtual User (VU) lifecycle:**
  - `init()` → `execute()` → `teardown()` per iteration
  - Współdzielone dane między VU (np. pool tokenów)
  - Pacing (stały interwał między iteracjami, nie zależny od czasu odpowiedzi — **anty coordinated omission**)
- [ ] **Open-loop vs Closed-loop model:**
  - Open-loop: żądania wysyłane wg harmonogramu niezależnie od odpowiedzi (poprawne mierzenie)
  - Closed-loop: nowe żądanie po otrzymaniu odpowiedzi (symulacja realnego użytkownika)
  - Konfigurowalny wybór w scenariuszu
- [ ] **Obsługa błędów i retry:**
  - Konfigurowalny timeout per request
  - Retry z exponential backoff
  - Circuit breaker (opcjonalny)
- [ ] **Korelacja danych między krokami:**
  - Ekstrakcja z odpowiedzi (JSONPath, regex, XPath, header)
  - Przekazywanie zmiennych między krokami
  - Data feeder (CSV, JSON) do parametryzacji

### Faza 2 — Protokoły (3-4 tygodnie)

**Cel:** Obsługa najważniejszych protokołów.

#### Zadania:

- [ ] **HTTP/1.1 i HTTP/2** (reqwest + hyper)
  - Pełna obsługa metod (GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD)
  - Cookies (jar per VU)
  - Automatyczne podążanie za redirectami (konfigurowalne)
  - Multipart form-data, file upload
  - TLS/mTLS (client certificates)
  - HTTP/2 multiplexing
  - Connection pooling per VU
- [ ] **gRPC** (tonic)
  - Unary, server-streaming, client-streaming, bidirectional
  - Ładowanie `.proto` w runtime (reflection)
- [ ] **WebSocket** (tokio-tungstenite)
  - Połączenie, wysyłanie, odbieranie, zamknięcie
  - Pomiary per-message latency
- [ ] **Raw TCP/UDP** (tokio::net)
  - Surowe połączenia dla niestandardowych protokołów
- [ ] **GraphQL** (nadbudówka nad HTTP)
  - Obsługa query, mutation, subscription

### Faza 3 — Metryki i raportowanie (2-3 tygodnie)

**Cel:** Precyzyjne zbieranie metryk i elastyczne raportowanie.

#### Zadania:

- [ ] **Zbieranie metryk (lock-free):**
  - HDR Histogram (crate `hdrhistogram`) dla latencji — dokładność do mikrosekund
  - Atomowe liczniki: total requests, successes, failures, bytes sent/received
  - Percentyle: p50, p75, p90, p95, p99, p99.9
  - Throughput (requests/sec, bytes/sec)
  - Error rate per step i globalny
  - Metryki per VU (opcjonalnie)
- [ ] **Real-time TUI dashboard** (ratatui):
  - Wykres throughput w czasie
  - Tabela z metrykami per step
  - Histogram latencji
  - Pasek postępu testu
  - Error log
- [ ] **Eksport metryk:**
  - JSON (pełny dump po teście)
  - CSV (do analizy w Excelu/Pandas)
  - HTML report (wbudowany template, brak zewnętrznych zależności)
  - InfluxDB (real-time push via line protocol)
  - Prometheus (endpoint `/metrics` podczas testu)
  - OpenTelemetry (OTLP export)

### Faza 4 — Silnik skryptowy (2-3 tygodnie)

**Cel:** Możliwość pisania złożonych scenariuszy bez rekompilacji.

#### Zadania:

- [ ] **Integracja Rhai** jako głównego silnika skryptowego:
  ```rhai
  // scenario.rhai
  fn setup() {
    set_base_url("https://api.example.com");
    set_header("Content-Type", "application/json");
  }
  
  fn default() {
    let resp = http_get("/users");
    check(resp.status == 200, "status is 200");
    
    let user_id = resp.json("$.users[0].id");
    
    let resp2 = http_post("/users/" + user_id + "/action", #{
      action: "activate"
    });
    check(resp2.status == 204, "activation successful");
    
    sleep(think_time(1.0, 3.0));  // losowy czas "myślenia"
  }
  
  fn teardown() {
    log("Test completed for VU " + vu_id());
  }
  ```
- [ ] **API dostępne w skryptach:**
  - `http_get`, `http_post`, `http_put`, `http_delete`
  - `ws_connect`, `ws_send`, `ws_receive`
  - `grpc_call`
  - `check(condition, name)` — asercje
  - `sleep(duration)`, `think_time(min, max)`
  - `vu_id()`, `iteration()`, `timestamp()`
  - `group(name, fn)` — grupowanie kroków
  - `set_variable(key, val)`, `get_variable(key)`
  - `csv_feeder(path)` — dane z pliku
- [ ] **Tryb hybrydowy:** Scenariusze TOML mogą osadzać bloki Rhai inline:
  ```toml
  [[steps]]
  name = "Custom logic"
  script = """
    let token = get_variable("auth_token");
    http_get("/api/data", #{ "Authorization": "Bearer " + token });
  """
  ```
- [ ] Rozważenie opcjonalnego wsparcia **Lua** (mlua) jako alternatywy

### Faza 5 — Tryb rozproszony (3-4 tygodnie)

**Cel:** Skalowanie testów na wiele maszyn.

#### Zadania:

- [ ] **Architektura Controller ↔ Worker:**
  ```
  taran controller --workers 5 --scenario test.toml
  taran worker --controller 192.168.1.1:9090
  ```
- [ ] Komunikacja gRPC między controller a workers
- [ ] Controller rozdziela konfigurację VU między workerów
- [ ] Agregacja metryk z wielu workerów w real-time
- [ ] Auto-discovery workerów w sieci lokalnej (opcjonalnie)
- [ ] Wsparcie dla Kubernetes (Helm chart, Job-based workers)

### Faza 6 — Ekosystem i polish (2-4 tygodnie)

**Cel:** Gotowość produkcyjna.

#### Zadania:

- [ ] **System pluginów** (dynamiczne `.so`/`.dll` lub WASM):
  - Interfejs trait dla custom protokołów
  - Rejestr pluginów (opcjonalnie online)
- [ ] **Konwerter z JMeter (`.jmx`) → Taran (`.toml`)**
  - Parsowanie XML JMeter'a
  - Mapowanie komponentów na odpowiedniki Taran
  - Partial support (ostrzeżenia dla nieobsługiwanych elementów)
- [ ] **Konwerter z K6 (`.js`) → Taran (`.rhai`)**
- [ ] **Integracja z CI/CD:**
  - GitHub Actions action
  - GitLab CI template
  - Exit code na podstawie thresholds (np. p95 > 500ms → exit 1)
- [ ] **Dokumentacja:**
  - mdBook lub Docusaurus
  - Quickstart guide
  - Cookbook (typowe scenariusze)
  - API reference dla skryptów Rhai
- [ ] **Homebrew formula, cargo install, Docker image, .deb/.rpm packages**

---

## 4. Stack technologiczny

| Komponent | Crate / Technologia | Uzasadnienie |
|---|---|---|
| Async runtime | `tokio` | Standard przemysłowy, najlepsze wsparcie ekosystemu |
| HTTP client | `reqwest` + `hyper` | reqwest dla wygody, hyper dla low-level kontroli |
| gRPC | `tonic` | Natywny async gRPC dla Tokio |
| WebSocket | `tokio-tungstenite` | Async WebSocket na Tokio |
| CLI | `clap` (derive) | Najlepszy crate CLI w Rust |
| Config | `serde` + `toml` + `serde_yaml` | De/serializacja konfiguracji |
| Skryptowanie | `rhai` | Embeddable, sandboxed, Rust-native |
| Metryki | `hdrhistogram` | Precyzyjne histogramy latencji |
| TUI | `ratatui` | Nowoczesny terminal UI |
| JSON extraction | `jsonpath-rust` | JSONPath queries |
| Regex | `regex` | Ekstrakcja z odpowiedzi |
| Logging | `tracing` + `tracing-subscriber` | Strukturalne logowanie, async-aware |
| Error handling | `thiserror` + `anyhow` | Ergonomiczne zarządzanie błędami |
| TLS | `rustls` | Pure Rust TLS (brak zależności od OpenSSL) |
| Serialization | `serde_json` | JSON handling |
| Template (reports) | `minijinja` | Szablony HTML raportów |
| Benchmarking | `criterion` | Benchmarki wewnętrzne narzędzia |

---

## 5. Kluczowe decyzje architektoniczne

### 5.1 Model pamięci metryk

```
Każdy VU posiada lokalny bufor metryk (thread-local HDR Histogram).
Co 1 sekundę metryki są mergowane do globalnego collectora.
Brak Mutex na hot path — zero contention.
```

### 5.2 Coordinated Omission — poprawna implementacja

```
Open-loop scheduler:
  - Timer tikuje co T = 1/target_rps
  - Jeśli poprzednie żądanie nie zakończyło się — i tak wysyłamy nowe
  - Mierzymy CZAS OD ZAPLANOWANIA, nie od wysłania
  - To daje prawdziwy obraz latencji widzianej przez użytkownika
```

### 5.3 Podejście do pluginów

```
Faza MVP: wbudowane protokoły (HTTP, gRPC, WS)
Faza 1.x: trait Protocol + dynamiczne ładowanie .so/.dll
Faza 2.x: WASM sandbox dla bezpiecznych pluginów
```

---

## 6. Definicja MVP (Minimum Viable Product)

**MVP = Faza 0 + Faza 1 + HTTP z Fazy 2 + podstawowe metryki z Fazy 3**

Funkcjonalność MVP:
- ✅ Scenariusze definiowane w TOML
- ✅ HTTP/1.1 + HTTP/2 (GET, POST, PUT, DELETE)
- ✅ Constant load + ramp-up
- ✅ Ekstrakcja z JSON (JSONPath) i przekazywanie między krokami
- ✅ Asercje (status code, response time, body contains)
- ✅ HDR Histogram metryk z percentylami
- ✅ Wynik w konsoli (tabela podsumowująca)
- ✅ JSON export
- ✅ Exit code na podstawie thresholds
- ✅ Binarka na Linux/macOS/Windows

**Szacowany czas do MVP: 8-10 tygodni** (1 osoba, pełny etat)

---

## 7. Kamienie milowe

| Milestone | Termin (od startu) | Deliverable |
|---|---|---|
| M0: Skeleton | Tydzień 2 | Kompilujący się workspace, CI/CD, `taran --help` |
| M1: First Request | Tydzień 4 | `taran run test.toml` wysyła 1 HTTP request |
| M2: Load Engine | Tydzień 6 | 1000 VU, ramp-up, constant load |
| M3: MVP | Tydzień 10 | Pełne MVP (patrz sekcja 6) |
| M4: Scripting | Tydzień 13 | Rhai scripting engine |
| M5: Protocols | Tydzień 16 | gRPC + WebSocket |
| M6: Dashboard | Tydzień 18 | Real-time TUI + HTML report |
| M7: Distributed | Tydzień 22 | Controller/Worker mode |
| M8: v1.0 | Tydzień 26 | Produkcyjna wersja 1.0 |

---

## 8. Metryki sukcesu projektu

- **Wydajność:** 50,000+ RPS z jednego rdzenia przy prostym HTTP GET
- **Pamięć:** < 100 MB RAM dla 10,000 VU
- **Precyzja:** Odchylenie pomiarów latencji < 1% vs baseline (bez coordinated omission)
- **Binarka:** < 20 MB (statycznie zlinkowana)
- **Startup:** < 100ms do pierwszego żądania
- **GitHub Stars:** 500+ w pierwszym roku (cel społecznościowy)

---

## 9. Nazwa i branding

**Taran** (тара́н) — taran oblężniczy. Idealnie oddaje charakter narzędzia:
- Krótka, łatwa do zapamiętania
- Unikalna w ekosystemie (brak konfliktu nazw na crates.io)
- Sugeruje siłę i przebijanie / testowanie wytrzymałości

Sugerowane CLI:
```bash
taran run scenario.toml          # uruchom test
taran run scenario.rhai          # uruchom test ze skryptem
taran validate scenario.toml     # waliduj scenariusz
taran report results.json        # wygeneruj HTML raport
taran convert jmeter test.jmx    # konwertuj z JMeter
taran controller --port 9090     # tryb rozproszony
taran worker --connect host:9090 # worker
```

---

## 10. Następne kroki (Tydzień 1)

1. `cargo init` workspace z crate'ami
2. Setup CI/CD (GitHub Actions)
3. Zdefiniować publiczne API `taran-core` (traity: `Protocol`, `LoadProfile`, `MetricsCollector`)
4. Zaimplementować parser TOML scenariuszy
5. Pierwszy HTTP GET przez `reqwest` wewnątrz Tokio runtime
6. Pierwszy test integracyjny z mockowanym serwerem (`wiremock`)

---

*Dokument tworzony: 2026-02-09*
*Autor: Plan wygenerowany na podstawie analizy koncepcji projektu Taran*
