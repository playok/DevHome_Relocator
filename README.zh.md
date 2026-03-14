# DevHome Relocator

**将开发工具缓存从系统盘迁移到其他驱动器，释放磁盘空间的 Windows GUI 工具**

[한국어](README.ko.md) | [English](README.md) | [日本語](README.ja.md)

## 为什么需要这个工具？

Cargo、Rustup、Gradle、npm、NuGet、Maven、JetBrains、VS Code 扩展等众多开发工具，默认将缓存和数据存储在 C: 盘的用户配置文件目录下。随着时间推移，这些缓存会显著增长（往往超过 10GB），成为系统盘空间不足的主要原因。

DevHome Relocator 通过将这些目录安全地迁移到其他驱动器（D:、E: 等），帮助您释放系统盘空间。

## 截图

<!-- 请添加截图 -->
![DevHome Relocator Screenshot](screenshot.png)

## 工作原理

DevHome Relocator 使用两种方式迁移目录：

### 1. 环境变量方式（EnvVar）

对于支持环境变量配置的工具，将对应的环境变量设置为新路径。工具在下次启动时会自动识别新位置。

示例：设置 `CARGO_HOME=D:\DevCache\.cargo` 后，Cargo 将使用 D: 盘。

### 2. NTFS Junction 方式

对于不提供环境变量选项的工具，创建 NTFS 目录 Junction（一种透明的文件系统链接）。原始路径被替换为指向新位置的 Junction，因此工具继续使用原始路径，而实际数据存储在其他驱动器上。

示例：`C:\Users\用户\.jdks` → `D:\DevCache\.jdks`（原始路径变为 Junction）

## 支持的目标

### 环境变量方式

| 目标 | 环境变量 | 默认位置 |
|------|---------|---------|
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

### Junction 方式

| 目标 | 默认位置 |
|------|---------|
| JetBrains JDK | `%USERPROFILE%\.jdks` |
| Codeium/Windsurf | `%USERPROFILE%\.codeium` |
| Cursor | `%USERPROFILE%\.cursor` |
| Windsurf | `%USERPROFILE%\.windsurf` |
| Claude Code | `%USERPROFILE%\.claude` |
| Antigravity | `%USERPROFILE%\.antigravity` |
| Maven Local Repo | `%USERPROFILE%\.m2` |
| VS Code | `%USERPROFILE%\.vscode` |
| Theia IDE | `%USERPROFILE%\.theia-ide` |

## 主要功能

- **自动检测**：自动扫描已安装的开发工具目录并计算大小
- **已迁移检测**：自动识别已经迁移过的目录（AlreadyMoved 状态）
- **重新迁移**：支持将已通过 Junction 迁移的目录移动到新位置
- **进程冲突解决**：文件锁定（OS error 32）时，检测并提供终止冲突进程的对话框
- **实时进度条**：基于字节级别的实时 ProgressBar 显示复制进度
- **大小验证**：复制完成后比较源目录和目标目录的大小，确保数据完整性
- **回滚**：撤销迁移操作，恢复到原始状态
- **Dry Run**：不实际移动文件，仅模拟运行
- **PATH 自动更新**：迁移时自动更新用户 PATH 中的 bin 路径
- **工具环境变量面板**：显示 JAVA_HOME、MAVEN_HOME 等相关环境变量的当前值
- **多语言支持**：한국어、English、日本語、中文（根据系统区域设置自动检测）

## 构建与运行

```bash
cargo build
cargo run
```

发布构建：

```bash
cargo build --release
```

## 系统要求

- **操作系统**：Windows 10/11
- **文件系统**：NTFS（Junction 功能所需）
- **构建工具**：Rust 2021 Edition
- **最低分辨率**：1300 x 630

## 技术栈

- **语言**：Rust（2021 Edition）
- **GUI**：egui / eframe
- **注册表**：winreg
- **系统信息**：sysinfo
- **文件遍历**：walkdir
- **文件对话框**：rfd
- **序列化**：serde / serde_json
- **日志**：tracing / tracing-subscriber / tracing-appender

## 日志

日志文件按天存储在 `%LOCALAPPDATA%\DevHomeRelocator\logs` 目录中。

## 许可证

MIT License
