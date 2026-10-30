# Jules macOS-Workflow – Setup

Orchestriert Jules-Runs linear: Nach jedem Merge auf `main` startet automatisch der nächste Run aus `tasks/`.

## Struktur

```
tasks/01-analyse-plan.md          # Run 1: Analyse + MACOS_PLAN.md
tasks/02-macos-app-scaffold.md    # Run 2: App-Scaffold in macos/
tasks/99-template.md              # Vorlage für eigene Runs
AGENTS.md                         # Langzeitgedächtnis für Jules
.github/workflows/jules-orchestrator.yml
```

## Einmalig einrichten

1. **Jules API-Key:** In Jules (jules.google.com) API-Key erzeugen, dann im Repo unter
   `Settings → Secrets and variables → Actions` als `JULES_API_KEY` hinterlegen.
2. **Auto-Merge aktivieren:** `Settings → General → Pull Requests → Allow auto-merge`.
3. **Branch Protection auf `main`:** PR erforderlich, Status-Checks erforderlich,
   „Require branches to be up to date", Auto-Delete-Head-Branches an.
4. **Ersten Commit pushen**, damit `tasks/` auf `main` existiert.

## Ablauf

1. Push auf `main` → Workflow „Jules Orchestrator" findet die erste Task-Datei mit `status: pending`
   (numerisch sortiert), startet eine Jules-Session per API (`startingBranch: main`)
   und markiert den Task als `in-progress`.
2. Jules erstellt einen PR. Nach deinem Review und Merge auf `main` läuft der Job
   „Nach Merge Task abschließen": Der `in-progress`-Task wird auf `status: done` gesetzt.
3. Dieser Tick-Commit (`[task-tick]` in der Message) triggert den Orchestrator erneut → der nächste Run startet.
4. Sind keine `pending`-Tasks mehr vorhanden, beendet sich der Workflow still.

## Queue erweitern

`tasks/99-template.md` kopieren → `tasks/03-dein-run.md` → Titel und Prompt anpassen,
`status: pending` sicherstellen. Keine Lücken in der Nummerierung, Reihenfolge = Zahlenfolge.

## Fehlerfälle

- **Jules-Lauf schlägt fehl / kein PR:** Task steht auf `in-progress`. Manuell per
  `git` auf `status: pending` zurücksetzen und pushen – der Run startet erneut.
- **Falscher Task geschlossen:** In der Git-History nach `[task-tick]` suchen und den
  Status manuell korrigieren.
- **Endlosschleife:** Schutz ist die Prüfung auf `[task-tick]` in der Commit-Message.
  Keine eigenen Commits mit diesem Marker verwenden.

## Integration in DeepTutor

Workflow und Tasks liegen im Repository-Root unter `.github/workflows/` bzw.
`tasks/`. Die vorhandenen Root-Dateien `AGENTS.md` und `README.md` wurden nicht
überschrieben. Die mitgelieferte Agent-Vorlage liegt hier als
`AGENTS.template.md`; sie ist keine automatisch geltende Agent-Anweisung.

### Bekannte Einschränkungen des unveränderten Archiv-Workflows

- `JULES_API_KEY` muss als GitHub-Actions-Secret eingerichtet und das Repository
  in Jules verbunden sein. Es wurde keine Jules-Session gestartet.
- Pushes mit dem standardmäßigen `GITHUB_TOKEN` lösen keine weiteren
  Push-Workflows aus. Zusätzlich schließt `start-next` Commits mit
  `[task-tick]` ausdrücklich aus. Der dokumentierte automatische Folgestart
  funktioniert daher in dieser Fassung nicht.
- Jeder gemergte PR kann den ersten laufenden Task abschließen; eine Zuordnung
  zur Jules-Session fehlt.
- `tasks/99-template.md` hat im Original `status: pending` und wird deshalb
  ebenfalls als ausführbarer Task erkannt. Vor produktiver Nutzung aus der
  Queue entfernen oder den Status ändern.
- Die API-Quellenkennung und PR-Erstellung wurden nicht gegen Jules geprüft.
- Direkte Status-Pushes auf `main` können an Branch-Protection scheitern.

Vor unbeaufsichtigter Nutzung müssen diese Punkte im Workflow korrigiert werden.
