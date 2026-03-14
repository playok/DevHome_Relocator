# DevHome Relocator

**開発ツールのキャッシュをシステムドライブから別のドライブに移動し、ディスク容量を確保するWindows GUIユーティリティ**

[한국어](README.ko.md) | [English](README.md) | [中文](README.zh.md)

## なぜ必要？

Cargo、Rustup、Gradle、npm、NuGet、Maven、JetBrains、VS Code拡張機能など、多くの開発ツールはデフォルトでC:ドライブのユーザープロファイル配下にキャッシュやデータを保存します。時間が経つにつれ、これらのキャッシュは大幅に増加し（10GBを超えることも珍しくありません）、システムドライブの容量不足の主な原因となります。

DevHome Relocatorは、これらのディレクトリを別のドライブ（D:、E:など）に安全に移動し、システムドライブの空き容量を確保します。

## スクリーンショット

<!-- スクリーンショットを追加してください -->
![DevHome Relocator Screenshot](screenshot.png)

## 仕組み

DevHome Relocatorは2つの方式でディレクトリを移動します。

### 1. 環境変数方式（EnvVar）

環境変数による設定をサポートするツールの場合、該当する環境変数を新しいパスに設定します。ツールは次回起動時に自動的に新しい場所を認識します。

例：`CARGO_HOME=D:\DevCache\.cargo`と設定すると、CargoはD:ドライブを使用します。

### 2. NTFS Junction方式

環境変数オプションを提供しないツールの場合、NTFSディレクトリJunction（透過的なファイルシステムリンク）を作成します。元のパスがJunctionに置き換わるため、ツールは従来のパスをそのまま使用しながら、実際のデータは別のドライブに保存されます。

例：`C:\Users\ユーザー\.jdks` → `D:\DevCache\.jdks`（元のパスはJunctionに置換）

## 対応ターゲット

### 環境変数方式

| 対象 | 環境変数 | デフォルトの場所 |
|------|---------|----------------|
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

### Junction方式

| 対象 | デフォルトの場所 |
|------|----------------|
| JetBrains JDK | `%USERPROFILE%\.jdks` |
| Codeium/Windsurf | `%USERPROFILE%\.codeium` |
| Cursor | `%USERPROFILE%\.cursor` |
| Windsurf | `%USERPROFILE%\.windsurf` |
| Claude Code | `%USERPROFILE%\.claude` |
| Antigravity | `%USERPROFILE%\.antigravity` |
| Maven Local Repo | `%USERPROFILE%\.m2` |
| VS Code | `%USERPROFILE%\.vscode` |
| Theia IDE | `%USERPROFILE%\.theia-ide` |

## 主な機能

- **自動検出**: インストール済みの開発ツールディレクトリを自動スキャンし、サイズを計算
- **移動済み検出**: 既に移動されたディレクトリ（AlreadyMoved）を自動認識
- **再移動対応**: Junctionで移動済みのディレクトリを別の場所に再移動可能
- **プロセス競合解決**: ファイルロック（OSエラー32）発生時、競合プロセスを検出して終了するダイアログを提供
- **リアルタイム進捗バー**: バイトレベルの進捗追跡によるリアルタイムProgressBar表示
- **サイズ検証**: コピー完了後にソースとターゲットのサイズを比較してデータの整合性を検証
- **ロールバック**: 移動を取り消し、元の状態に復元可能
- **Dry Run**: 実際のファイル移動なしにシミュレーション実行
- **PATH自動更新**: 移動時にUser PATHのbinパスを自動更新
- **ツール環境変数パネル**: JAVA_HOME、MAVEN_HOMEなどの関連環境変数の現在値を表示
- **多言語対応**: 한국어、English、日本語、中文（システムロケール自動検出）

## ビルドと実行

```bash
cargo build
cargo run
```

リリースビルド：

```bash
cargo build --release
```

## システム要件

- **OS**: Windows 10/11
- **ファイルシステム**: NTFS（Junction機能に必要）
- **ビルドツール**: Rust 2021 Edition
- **最小解像度**: 1300 x 630

## 技術スタック

- **言語**: Rust（2021 Edition）
- **GUI**: egui / eframe
- **レジストリ**: winreg
- **システム情報**: sysinfo
- **ファイル走査**: walkdir
- **ファイルダイアログ**: rfd
- **シリアライズ**: serde / serde_json
- **ロギング**: tracing / tracing-subscriber / tracing-appender

## ログ

ログファイルは`%LOCALAPPDATA%\DevHomeRelocator\logs`ディレクトリに日別で保存されます。

## ライセンス

MIT License
