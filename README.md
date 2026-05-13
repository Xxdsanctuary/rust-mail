# rust-mail

A Rust starter template for a production-oriented, standalone desktop (non-web) multi-account email client targeting Linux first (with Windows and optional macOS support).

## What this template includes

- OAuth-first account model for **Gmail** and **Microsoft 365 Exchange**
- SMTP / IMAP protocol capability mapping for each provider
- Multi-account setup with per-account color identity (calendar-friendly)
- Calendar overlap detection primitive for Outlook/Apple-style overlapping events
- Security and desktop integration stubs:
  - Windows Defender integration flag
  - System notifications
  - Startup on Linux and Windows
  - Tray icon support
- Validation rules that enforce baseline requirements for the starter project

## Project layout

- `src/lib.rs`
  - Core template domain model (`EmailClientTemplate`)
  - Provider/protocol definitions
  - Calendar overlap support (`CalendarBoard`)
  - Requirement validation (`validate`)
  - Focused unit tests
- `src/main.rs`
  - Minimal executable entrypoint that validates the starter template at runtime

## Requirement mapping

- OAuth + Gmail/M365 + SMTP/IMAP: `Provider`, `MailProtocol`, `EmailClientTemplate::protocols_for`
- Outlook/Apple-like UX target: modeled as template architecture baseline for a native desktop app (for example: egui/iced/.NET desktop host + Rust backend)
- Security verification/auth + Defender integration: `SecurityTemplate`
- Calendar with account colors + overlap handling: `AccountTemplate::color_hex`, `CalendarBoard::overlapping_pairs`
- Endpoint synchronization readiness: provider-aware account model and validation hooks
- Multi-account and mixed providers: starter includes Gmail + M365 accounts
- Linux compatibility required, macOS optional: `supported_platforms`
- Reliable notifications, startup options, tray icon: `DesktopIntegrationTemplate`

## Run

```bash
cargo test
cargo run
```

## Next implementation steps

1. Add real OAuth token flows (Gmail + M365) with secure storage.
2. Implement IMAP/SMTP clients and provider-specific sync workers.
3. Build a native desktop GUI shell (egui, iced, or .NET desktop host) that follows Outlook/Apple layouts.
4. Add calendar sync adapters (Google Calendar + Microsoft Graph).
5. Add endpoint hardening (certificate pinning, malware scan hooks, audit logging).
