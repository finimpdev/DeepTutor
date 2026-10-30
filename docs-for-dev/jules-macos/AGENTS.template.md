# AGENTS.md

Diese Datei wird von Jules (und anderen Coding-Agenten) automatisch gelesen.
Halte sie aktuell – sie ist das Langzeitgedächtnis über alle Runs hinweg.

## Projekt

- Bestehender Code: [KURZBESCHREIBUNG DES REPOS EINTRAGEN]
- Ziel: Umwandlung in eine native macOS-App.
- Plan: siehe `MACOS_PLAN.md` (wird in Run 1 erstellt).
- macOS-App-Code: Unterordner `macos/` (wird ab Run 2 befüllt).

## Build & Test

- Bestehendes Projekt: [BUILD- UND TEST-BEFEHLE EINTRAGEN, z. B. `npm test`, `cargo test`]
- macOS-App: siehe `macos/README.md`.
- Nicht testbar in der Jules-Linux-VM: Codesigning, Notarisierung, DMG-Build → lokal auf macOS prüfen.

## Konventionen

- Jeder Run ändert nur das, was in seiner Task-Datei unter `tasks/` erlaubt ist.
- Keine Änderungen außerhalb von `macos/` ohne explizite Freigabe im Task-Prompt.
- PRs auf Deutsch kommentieren, Code-Kommentare auf Englisch.
- Fehler, die beim lokalen macOS-Build auftauchen, als neue Task-Datei unter `tasks/` festhalten.
