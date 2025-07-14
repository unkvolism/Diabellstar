# Diabellstar — ETW Bypass via NtTraceEvent Hooking (Rust)

<p align="center">
  <img src="[https://upload.wikimedia.org/wikipedia/en/f/f5/Kuriboh.jpg](https://i.imgur.com/M0QtPDW.png)" width="200" alt="Diabellstar"/>
</p>

**Diabellstar** is a lightweight and effective tool written in Rust to bypass Event Tracing for Windows (ETW) by patching the `NtTraceEvent` API at runtime.

By corrupting the implementation of `NtTraceEvent` in `ntdll.dll`, this loader disables ETW-based telemetry and logging mechanisms, which are commonly used by EDRs and Windows internals for monitoring process execution.

---

## Disclaimer

This project is intended for educational and research purposes only.  
Use responsibly and only in controlled environments.

---

## How It Works

- Loads `ntdll.dll` dynamically at runtime.
- Resolves the address of `NtTraceEvent` via `GetProcAddress`.
- Uses `VirtualProtect` to change memory permissions to `PAGE_EXECUTE_READWRITE`.
- Overwrites the 4th byte of `NtTraceEvent` with a `RET` instruction (`0xC3`), effectively short-circuiting its execution.
- Restores original memory protection.

This approach ensures that any call to `NtTraceEvent` will return immediately, disabling most ETW-based telemetry from functioning.

---

## Building and Running

### Prerequisites

- Rust (stable)
- Windows target

### Build

```bash
cargo build --release
