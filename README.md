# DevHome Relocator

**A Windows GUI utility that frees up disk space by relocating developer tool caches from the system drive to another drive**

[한국어](README.ko.md) | [日本語](README.ja.md) | [中文](README.zh.md)

## The Problem

Many developer tools -- Cargo, Rustup, Gradle, npm, NuGet, Maven, JetBrains, VS Code extensions, and more -- store their caches and data under the user profile on the C: drive by default. Over time, these caches grow significantly (often exceeding 10GB), and they become a major cause of running out of disk space on the system drive.

DevHome Relocator solves this by safely moving these directories to another drive (D:, E:, etc.), reclaiming valuable space on your system drive.

## Screenshot

<!-- Add a screenshot here -->
![DevHome Relocator Screenshot](docs/screenshot.png)

## How It Works

DevHome Relocator uses two methods to relocate directories:

### 1. Environment Variables (EnvVar)

For tools that support configuration via environment variables, the tool sets the corresponding variable to point to the new location. The tool will automatically use the new path on its next run.

Example: Setting `CARGO_HOME=D:\DevCache\.cargo` makes Cargo use the D: drive.

### 2. NTFS Junction

For tools that don't provide an environment variable option, the tool creates an NTFS directory junction -- a transparent filesystem link. The original path is replaced with a junction pointing to the new location, so the tool continues to see the original path while the actual data lives on another drive.

Example: `C:\Users\You\.jdks` -> `D:\DevCache\.jdks` (original path becomes a junction)

## Supported Targets

### Environment Variable Method

| Target | Environment Variable | Default Location |
|--------|---------------------|-----------------|
| Cargo | `CARGO_HOME` | `%USERPROFILE%\.cargo` |
| Rustup | `RUSTUP_HOME` | `%USERPROFILE%\.rustup` |
| Gradle | `GRADLE_USER_HOME` | `%USERPROFILE%\.gradle` |
| Go Path | `GOPATH` | `%USERPROFILE%\go` |
| Go Build Cache | `GOCACHE` | `%LOCALAPPDATA%\go-build` |
| npm Cache | `NPM_CONFIG_CACHE` | `%APPDATA%\npm-cache` |
| pip Cache | `PIP_CACHE_DIR` | `%LOCALAPPDATA%\pip` |
| NuGet Packages | `NUGET_PACKAGES` | `%USERPROFILE%\.nuget\packages` |
| Deno | `DENO_DIR` | `%LOCALAPPDATA%\deno` |
| pnpm Store | `PNPM_STORE_DIR` | `%LOCALAPPDATA%\pnpm\store` |

### Junction Method

| Target | Default Location |
|--------|-----------------|
| JetBrains JDK | `%USERPROFILE%\.jdks` |
| Codeium/Windsurf | `%USERPROFILE%\.codeium` |
| Cursor | `%USERPROFILE%\.cursor` |
| Windsurf | `%USERPROFILE%\.windsurf` |
| Claude Code | `%USERPROFILE%\.claude` |
| Antigravity | `%USERPROFILE%\.antigravity` |
| Maven Local Repo | `%USERPROFILE%\.m2` |
| VS Code | `%USERPROFILE%\.vscode` |
| Theia IDE | `%USERPROFILE%\.theia-ide` |

## Key Features

- **Auto-detection**: Automatically scans for installed developer tool directories and calculates their sizes
- **Already-moved detection**: Recognizes directories that have already been relocated (AlreadyMoved status)
- **Re-relocation support**: Move previously junctioned directories to a different location
- **Process conflict resolution**: Detects and offers to kill processes that are locking files (OS error 32) during relocation
- **Real-time progress bar**: Byte-level progress tracking during file copy operations
- **Size verification**: Compares source and destination sizes after copy to ensure data integrity
- **Rollback**: Undo a relocation and restore the original state
- **Dry Run**: Simulate the relocation without actually moving any files
- **Automatic PATH update**: Updates User PATH entries (e.g., bin directories) when relocating
- **Tool environment panel**: Displays related environment variables like JAVA_HOME, MAVEN_HOME, etc.
- **Multi-language support**: Korean, English, Japanese, Chinese (auto-detected from system locale)

## Build & Run

```bash
cargo build
cargo run
```

Release build:

```bash
cargo build --release
```

## System Requirements

- **OS**: Windows 10/11
- **File System**: NTFS (required for Junction support)
- **Build Tool**: Rust 2021 Edition
- **Minimum Resolution**: 1300 x 630

## Tech Stack

- **Language**: Rust (2021 Edition)
- **GUI**: egui / eframe
- **Registry**: winreg
- **System Info**: sysinfo
- **File Traversal**: walkdir
- **File Dialog**: rfd
- **Serialization**: serde / serde_json
- **Logging**: tracing / tracing-subscriber / tracing-appender

## Logs

Log files are stored daily in the `%LOCALAPPDATA%\DevHomeRelocator\logs` directory.

## License

MIT License
