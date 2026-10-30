---
title: "Run 2: macOS-App-Scaffold im Unterordner macos/"
status: pending
---

Erstelle das App-Scaffold für die native macOS-App. Orientiere dich strikt an `MACOS_PLAN.md` (Abschnitt „Zielarchitektur" und Empfehlung aus dem Optionen-Vergleich).

Anforderungen:

1. **Ort:** Der gesamte App-Code kommt in einen neuen Unterordner `macos/` im Repo-Root. Der bestehende Code außerhalb von `macos/` bleibt unverändert, soweit nicht unten anders angegeben.
2. **Scaffold:** Lege das Projekt nach der empfohlenen Technologie an (Tauri 2 oder Electron gemäß `MACOS_PLAN.md`). Enthalten sein müssen:
   - Fenster-Konfiguration (Titel, Mindestgröße, Dark/Light Mode folgt System)
   - Platzhalter-App-Icon und Fenster-Icon (klar als Platzhalter markiert)
   - Menüleiste-Eintrag mit Quit-Handhabung (Cmd+Q)
   - Verbindung zur bestehenden Funktionalität gemäß Zielarchitektur (z. B. CLI als Sidecar einbinden oder bestehende UI laden)
3. **Integration:** Ergänze `.gitignore` um Einträge für den gewählten Stack (z. B. `macos/target/`, `macos/node_modules/`, `macos/dist/`).
4. **Dokumentation:**
   - `macos/README.md` mit Schritt-für-Schritt-Build- und Dev-Anleitung für macOS (inkl. Voraussetzungen: Xcode Command Line Tools, Rust/Node-Versionen).
   - Kurzer Abschnitt im Haupt-`README.md`, der auf `macos/` verweist.
5. **Qualität:** Der Scaffold muss mit den in `macos/README.md` dokumentierten Befehlen starten, ohne dass Code außerhalb von `macos/` benötigt wird. Alles, was nicht in der Linux-VM testbar ist, klar im PR kommentieren.
