# ApexManager

A Windows desktop GUI for managing **Assetto Corsa Competizione** dedicated server configuration files — no more hand-editing JSON in Notepad.

---

## What is it?

Running an ACC dedicated server means maintaining a handful of JSON config files with dozens of fields, cryptic defaults, and UTF-16LE encoding that most text editors mangle silently. ApexManager gives you a clean visual interface for all of them, validates your input, and saves files in exactly the format ACC expects.

---

## Features

### Config editing
- **Server Config** — UDP/TCP ports, connection limits, public IP, LAN discovery, lobby registration
- **Settings** — server name, passwords, car class, slot count, driver rating requirements, formation lap type, race options
- **Event** — track selection, ambient temperature, cloud level, rain, weather randomness, weather presets (Clear → Storm), session builder (Practice / Qualifying / Race), time-of-day and multiplier per session
- **Event Rules** — qualifying standing type, pit window, driver stint limits, mandatory pitstops and their requirements (refuel / tyre change / driver swap), max drivers, tyre set count
- **Assist Rules** — stability control ceiling, per-assist disable flags (auto steer, lights, wipers, engine start, pit limiter, gear, clutch, ideal line)
- **Entry List** — reserved race numbers, forced car models, grid positions, per-entry ballast/restrictor, driver profiles with Steam ID validation, server admin flag
- **Balance of Performance** — per-track per-car ballast (kg) and restrictor (%) table

### Server control
- **Start / Stop / Restart** server directly from the top bar — no terminal needed
- Live status indicator: Stopped / Running / Crashed with exit code
- Correctly identifies a closed console window as a clean stop (not a crash)

### Quality of life
- **Fresh config detection** — if you point ApexManager at a stock ACC install it detects the untouched defaults and applies sensible starting values automatically
- **Preset system** — save your entire configuration (all 7 files) as a named preset, load or delete it any time; presets are stored in `%APPDATA%\ApexManager\presets\`
- **Quick Reference panels** — every config section has a collapsible "ℹ Quick Reference" that explains each field in plain English without leaving the app
- **Unsaved changes tracking** — nav buttons highlight in amber when a file has unsaved changes; "Save All" shows the count
- **Resizable / collapsible sidebar** — drag the sidebar edge to resize, or hide it entirely with the toggle button for more editing space
- **Scrollable panels** — works at any window size without clipping
- **Admin command helper** — step-by-step guide for authenticating as admin in-game and issuing kick/ban commands with the correct race number

### Results & logs
- Browse and read session result JSON files
- Browse and read server log files with auto-scroll

---

## Installation

Download the latest `ApexManager-vX.X.X-windows-x64.exe` from the [Releases](../../releases) page and run it — no installer, no dependencies.

> **Windows SmartScreen** may warn about an unknown publisher the first time. Click **More info → Run anyway**. The app does not connect to the internet and does not modify anything outside your server folder and `%APPDATA%\ApexManager\`.

---

## Getting started

1. Launch `ApexManager.exe`
2. Click **Open Folder…** in the sidebar and select your server's `server\` subfolder  
   *(the one that contains `accServer.exe` directly inside it — not the parent)*  
   Default Steam path: `…\steamapps\common\Assetto Corsa Competizione Dedicated Server\server\`
3. Edit your config using the navigation on the left
4. Click **Save All** when you're done — files are written in UTF-16LE as ACC requires
5. Hit **▶ Start Server** in the top bar

### First run on a fresh install

If ApexManager detects that you've never configured the server (ACC ships with placeholder values like `udpPort: 1`), it automatically fills in sensible defaults. You'll see a "Fresh config detected" message. Review the settings and hit **Save All**.

### Port forwarding

Open these ports on your router and firewall for the server to be reachable:

| Purpose | Protocol | Default port |
|---------|----------|-------------|
| Car telemetry | UDP | 9231 |
| Client connections | TCP | 9323 |

---

## Building from source

Requires [Rust](https://rustup.rs/) (stable).

```bash
git clone https://github.com/YOUR_USERNAME/ApexManager.git
cd ApexManager
cargo build --release
```

The binary will be at `target/release/apex_manager.exe`.

---

## Releases

Releases are published automatically via GitHub Actions when a version tag is pushed. To install a specific version, go to the [Releases](../../releases) page and download the `.exe`.

---

## Contributing

ApexManager is a young project and there's plenty of room to grow. All feedback is welcome — whether you race competitively, run a league, or just want to try hosting a server for friends.

### Ways to get involved

- **Found a bug?** [Open an issue](../../issues/new?template=bug_report.md) — describe what happened, what you expected, and include your OS version and server log if relevant
- **Have a feature idea?** [Start a discussion](../../discussions/new?category=ideas) — explain your use case; even rough ideas are worth sharing
- **Something unclear?** [Ask a question](../../discussions/new?category=q-a) in Discussions — if you're confused, the docs or UI probably need improving
- **Want to contribute code?** Fork the repo, make your changes, and open a pull request — all skill levels welcome

### Good first areas to explore

- ACC has a lot of edge-case behaviour — if you discover something the app handles wrong, please report it with your server log
- Translation / localization
- Dark/light theme toggle
- Validation improvements (e.g. warn when session order is invalid for ACC)
- Support for ACC's `configuration.json` and `bop.json` global files

### Running in development

```bash
cargo run
```

Logs print to the terminal. The app reloads config from disk each time you open a folder.

---

## License

MIT — see [LICENSE](LICENSE) for details.
