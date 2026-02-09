# Instrukcja konfiguracji ochrony gałęzi main (Branch Protection Setup Guide)

## Automatyczna konfiguracja (Automated Setup)

Jeśli repozytorium ma zainstalowaną aplikację [GitHub Settings App](https://github.com/apps/settings), ochrona gałęzi zostanie automatycznie skonfigurowana na podstawie pliku `.github/settings.yml`.

**Instalacja GitHub Settings App:**
1. Przejdź do https://github.com/apps/settings
2. Kliknij "Install" lub "Configure"
3. Wybierz repozytorium Taran
4. Aplikacja automatycznie zastosuje ustawienia z pliku `settings.yml`

## Ręczna konfiguracja (Manual Setup)

Jeśli GitHub Settings App nie jest zainstalowana, wykonaj następujące kroki:

### Krok 1: Przejdź do ustawień repozytorium
1. Otwórz repozytorium na GitHub: https://github.com/Shaqal7/Taran
2. Kliknij na zakładkę **Settings** (Ustawienia)
3. W menu bocznym wybierz **Branches** (Gałęzie)

### Krok 2: Dodaj regułę ochrony gałęzi
1. W sekcji "Branch protection rules" kliknij **Add rule** (Dodaj regułę)
2. W polu "Branch name pattern" wpisz: `main`

### Krok 3: Skonfiguruj zasady ochrony

Zaznacz następujące opcje:

#### ✅ Require a pull request before merging
**Wymagaj pull requesta przed mergowaniem**

W tej sekcji zaznacz:
- ✅ **Require approvals** (Wymagaj zatwierdzeń)
  - Ustaw liczbę wymaganych zatwierdzeń: **1** (lub więcej)
- ✅ **Dismiss stale pull request approvals when new commits are pushed**
  - Odrzuć stare zatwierdzenia gdy nowe commity zostaną dodane
- ✅ **Require review from Code Owners** (zalecane)
  - Wymagaj recenzji od właścicieli kodu (zdefiniowanych w pliku CODEOWNERS)

#### ✅ Require status checks to pass before merging (opcjonalne)
**Wymagaj przejścia testów przed mergowaniem**

Jeśli masz skonfigurowane CI/CD:
- ✅ **Require branches to be up to date before merging**
  - Wymagaj aby gałęzie były aktualne przed mergowaniem

#### ✅ Require conversation resolution before merging (opcjonalne)
**Wymagaj rozwiązania wszystkich komentarzy przed mergowaniem**

#### ⬜ Require signed commits (opcjonalne)
**Wymagaj podpisanych commitów**

#### ⬜ Require linear history (opcjonalne)
**Wymagaj liniowej historii**

#### ⬜ Include administrators (opcjonalne)
**Stosuj zasady również do administratorów**
- Zaznacz to, jeśli chcesz aby zasady obowiązywały również administratorów repozytorium

#### ✅ Do not allow bypassing the above settings
**Nie pozwalaj na omijanie powyższych ustawień**

#### Restrictions (opcjonalne)
**Ograniczenia**
- Możesz określić konkretnych użytkowników lub zespoły, którzy mogą pushować do gałęzi

### Krok 4: Zapisz zmiany
Kliknij **Create** lub **Save changes** na dole strony.

## Weryfikacja (Verification)

Po skonfigurowaniu ochrony gałęzi, sprawdź czy działa:

1. Spróbuj wykonać bezpośredni push do main:
   ```bash
   git checkout main
   git pull
   echo "test" >> test.txt
   git add test.txt
   git commit -m "Test push"
   git push origin main
   ```
   
   **Oczekiwany rezultat:** Push powinien zostać odrzucony z komunikatem o ochronie gałęzi.

2. Prawidłowy workflow:
   ```bash
   # Utwórz nową gałąź
   git checkout -b feature/test-branch
   
   # Wprowadź zmiany
   echo "test" >> test.txt
   git add test.txt
   git commit -m "Test changes"
   
   # Wypchnij gałąź
   git push origin feature/test-branch
   
   # Utwórz Pull Request na GitHub
   # Poczekaj na review i zatwierdzenie
   # Zmerguj PR przez interfejs GitHub
   ```

## Rozwiązywanie problemów (Troubleshooting)

### Problem: Nadal mogę pushować bezpośrednio do main
- Sprawdź czy reguła ochrony jest aktywna w ustawieniach
- Sprawdź czy jesteś administratorem i czy opcja "Include administrators" jest zaznaczona
- Upewnij się, że wzorzec nazwy gałęzi to dokładnie `main`

### Problem: Nie mogę zmergować PR mimo zatwierdzenia
- Sprawdź czy wszystkie wymagane testy przeszły
- Sprawdź czy wszystkie komentarze zostały rozwiązane (jeśli ta opcja jest włączona)
- Sprawdź czy gałąź jest aktualna z main (jeśli ta opcja jest włączona)

## Dodatkowe informacje

- Szczegółowa dokumentacja: [.github/BRANCH_PROTECTION.md](.github/BRANCH_PROTECTION.md)
- Właściciele kodu: [.github/CODEOWNERS](.github/CODEOWNERS)
- Konfiguracja: [.github/settings.yml](.github/settings.yml)

## Linki

- [GitHub Documentation - Branch Protection Rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [GitHub Settings App](https://github.com/apps/settings)
