---
title: "Run 1: Analyse & Machbarkeitsplan"
status: pending
---

Analysiere dieses Repository gründlich und erstelle einen Machbarkeitsplan für die Umwandlung in eine native macOS-App. Arbeite exakt nach dieser Spezifikation:

1. Erstelle eine neue Datei `MACOS_PLAN.md` im Repo-Root mit folgenden Abschnitten:
   - **Bestandsaufnahme:** Technologie-Stack, Einstiegspunkte (CLI/Web/Desktop?), Abhängigkeiten, Test-Setup (anhand der vorhandenen Build-/Test-Konfiguration).
   - **Optionen-Vergleich:** Tauri 2 vs. Electron vs. weitere sinnvolle Optionen für genau dieses Projekt. Tabelle mit: Bundle-Größe, RAM-Verbrauch, Aufwand, Rust-Anteil, Fazit mit Empfehlung und Begründung.
   - **Zielarchitektur:** Skizziere, wie die bestehende Funktionalität in die App eingebettet wird (z. B. CLI als Sidecar-Binary, Web-UI im WebView, direkter Port).
   - **Meilensteinplan:** nummerierte Meilensteine, einer davon ist „Scaffold im Unterordner `macos/`".
   - **Risiken:** mindestens 3 konkrete Risiken mit Mitigation.
   - **Nicht testbar in Linux-VM:** Liste der Schritte, die lokal auf macOS ausgeführt werden müssen (z. B. Codesigning, Notarisierung, DMG-Build).
2. Ändere **keinen anderen Code** und erstelle **keine anderen Dateien** außer `MACOS_PLAN.md`.
3. Schreibe den Plan auf Deutsch, präzise und so, dass ein Coding-Agent die Meilensteine nacheinander abarbeiten kann.
