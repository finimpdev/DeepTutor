# Plan: Native macOS-App für DeepTutor

Dieses Dokument analysiert die Optionen und beschreibt einen Plan, um aus dem bestehenden DeepTutor-Projekt (Python-Backend + Next.js-Web-Frontend) eine native macOS-App zu erstellen.

## 1. Analyse der Ausgangslage
Das Projekt besteht aus:
- **Backend:** Python (FastAPI/uvicorn-artig) mit Abhängigkeiten wie `openai`, `pydantic`, `aiohttp`, etc.
- **Frontend:** Next.js (React) im Ordner `web/`.
Aktuell wird es oft über Docker oder manuell als zwei getrennte Prozesse gestartet. Ziel ist eine verpackte, native macOS-Erfahrung.

## 2. Vergleich der Technologien (Electron vs. Tauri vs. Andere)

| Kriterium | Electron | Tauri | Andere (Mac Catalyst / Swift) |
| :--- | :--- | :--- | :--- |
| **Technologie** | Chromium + Node.js | System-Webview (WebKit) + Rust | Swift, SwiftUI |
| **Größe der App** | Sehr groß (~100-150 MB+) | Sehr klein (~10-20 MB) | Sehr klein |
| **Speicherverbrauch** | Hoch (Chromium-Instanzen) | Gering (nutzt macOS WebKit) | Gering (Nativ) |
| **Integration von Python** | Relativ einfach (z.B. per `child_process`) | Sehr gut möglich (Sidecar) | Komplexer (PyObjC oder eigener Prozess) |
| **Lernkurve (Web-Devs)** | Sehr gering (JavaScript/TypeScript) | Mittel (TypeScript + Rust für Backend-Logik/Sidecar) | Hoch (Swift) |
| **Sicherheit** | Mittel (Node-Integration kann kritisch sein) | Hoch (Rust + isolierter Webview) | Hoch |

### Empfohlener Ansatz: Tauri (mit Python Sidecar)
Aufgrund der modernen Architektur, des geringen Ressourcenverbrauchs (auf macOS besonders wichtig) und der exzellenten Unterstützung für das Bündeln von externen Binärdateien (Sidecar-Muster), ist **Tauri** die beste Wahl für eine Next.js + Python Anwendung. Das Next.js-Frontend kann als SSG (Static Site Generation) oder über einen lokalen Server exportiert werden, während das Python-Backend über PyInstaller zu einer ausführbaren Datei gebündelt und als Tauri-Sidecar gestartet wird.

*Alternativ:* Electron ist machbar und evtl. etwas schneller initial aufzusetzen, wenn man Rust vermeiden will, aber die Bloatware-Nachteile überwiegen bei einer macOS-Desktop-App oft.

## 3. Architektur des empfohlenen Ansatzes (Tauri + Python Sidecar)
1. **Frontend (Tauri App):** Ein Tauri-Projekt, das das Next.js-Frontend (als statischen Export) lädt.
2. **Backend (Python Sidecar):** Das Python-Projekt wird mit `pyinstaller` (oder `py2app`) in ein Standalone-Binary für macOS (ARM64 & x64) kompiliert.
3. **Lebenszyklus:** Tauri startet das Python-Binary im Hintergrund beim App-Start und beendet es beim Schließen.
4. **Kommunikation:** Das Frontend spricht über HTTP (localhost) mit dem Python-Sidecar.

## 4. Meilensteine

### Meilenstein 1: Machbarkeitsprüfung & Vorbereitung
- [ ] Python-Backend so anpassen, dass es lokal (ohne Docker) sauber läuft und Ports dynamisch oder fest zugewiesen werden können.
- [ ] Next.js Frontend für den `out`-Export (`output: 'export'`) konfigurieren, da Tauri standardmäßig statische Dateien benötigt. (Fallback: Tauri kann auch einen lokalen Next.js-Server wrappen, aber statisch ist robuster).
- [ ] Python-App mit `pyinstaller` testweise in ein macOS-Binary kompilieren.

### Meilenstein 2: Tauri Setup
- [ ] Initialisieren eines Tauri-Projekts im Repo (z.B. in `macos-app/` oder als Erweiterung des `web/`-Ordners).
- [ ] Konfigurieren der `tauri.conf.json`, um den Build-Output von Next.js als Frontend zu nutzen.
- [ ] Testlauf der Tauri-App (ohne Backend) um zu prüfen, ob die UI lädt.

### Meilenstein 3: Sidecar-Integration
- [ ] Konfigurieren von Tauri zur Nutzung des kompilierten Python-Binaries als "Sidecar".
- [ ] Implementieren des Start/Stop-Lifecycles in Tauri (Rust), damit das Backend synchron zur App läuft.
- [ ] Anpassen der API-Endpunkte im Next.js-Frontend, sodass sie auf den vom Sidecar geöffneten Port (localhost) zeigen.

### Meilenstein 4: Polish, Build & Distribution
- [ ] Einrichten von GitHub Actions für den automatisierten Build (macOS Runner).
- [ ] Erstellen von Universal Binaries (Apple Silicon + Intel).
- [ ] Code-Signing und Notarization (Apple Developer Account erforderlich).
- [ ] Erstellen des `.dmg` (Disk Image) oder `.pkg` Installers.

## 5. Risiken
1. **Python-Abhängigkeiten:** Einige Libraries (z.B. C-Extensions, lokale Datenbanken, RAG/VectorDBs) könnten sich schwer mit PyInstaller kompilieren lassen oder erfordern spezifische Workarounds.
2. **Next.js Features:** Next.js Server Actions, API Routes oder Image-Optimization funktionieren beim statischen Export (`output: 'export'`) nicht. Diese müssten umgeschrieben werden (entweder client-side oder ins Python-Backend verschoben). Falls ein statischer Export unmöglich ist, muss Tauri einen lokalen Node-Server mitbringen (komplexer).
3. **Apple Notarization:** Strikte Regeln von macOS bzgl. Berechtigungen. Das verpackte Python-Binary muss ordnungsgemäß signiert werden, sonst blockiert Gatekeeper die App.
4. **Modell-Gewichte / Daten:** Wenn das Projekt lokale LLMs oder Embedding-Modelle (RAG) nutzt, werden diese die App-Größe massiv erhöhen. Hier sollte überlegt werden, ob Modelle erst bei der Laufzeit heruntergeladen werden.
