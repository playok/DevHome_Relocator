# DevHome Relocator

Windows GUI 유틸리티 (Rust + egui). 시스템 드라이브의 개발 도구 캐시 디렉토리를 다른 드라이브로 이동하여 디스크 공간을 확보합니다.

## 빌드 & 실행

```bash
cargo build
cargo run
```

- Rust Edition: 2021
- 최소 해상도: 1300x630
- Windows 전용 (NTFS Junction, Windows Registry 사용)

## 프로젝트 구조

```
src/
├── main.rs                  # 엔트리포인트, eframe 윈도우 설정
├── config/
│   ├── mod.rs
│   └── env_manager.rs       # Windows 레지스트리 환경변수 관리 (User/Machine)
├── core/
│   ├── mod.rs
│   └── relocation_target.rs # 핵심 데이터 모델 (RelocationTarget, TargetStatus)
├── i18n/
│   └── mod.rs               # 한/영 i18n (Locale 자동 감지)
├── mover/
│   ├── mod.rs
│   ├── file_mover.rs        # 파일 복사, 진행률, 프로세스 충돌 감지/kill
│   └── junction.rs          # NTFS Junction 생성/삭제/감지
├── scanner/
│   ├── mod.rs
│   ├── detector.rs          # 도구 디렉토리 감지, AlreadyMoved 판별
│   └── size_analyzer.rs     # 비동기 디렉토리 크기 계산
├── ui/
│   ├── mod.rs
│   ├── main_window.rs       # 메인 UI (헤더, 버튼, 다이얼로그, 로그)
│   └── table.rs             # 타겟 테이블 렌더링 (ProgressBar 포함)
└── utils/
    ├── mod.rs
    ├── disk_usage.rs         # 드라이브 정보
    └── logger.rs             # 일별 로그 파일 (%LOCALAPPDATA%\DevHomeRelocator\logs)
```

## 이동 방식

- **EnvVar**: 환경변수를 새 경로로 설정 (CARGO_HOME, RUSTUP_HOME, GRADLE_USER_HOME 등)
- **Junction**: NTFS 디렉토리 연결 (원래 경로에 가상 링크 생성, 설정 변경 불필요)

## 지원 대상 디렉토리

### 환경변수 방식
| 대상 | 환경변수 |
|------|---------|
| Cargo | CARGO_HOME |
| Rustup | RUSTUP_HOME |
| Gradle | GRADLE_USER_HOME |
| Go Path | GOPATH |
| Go Build Cache | GOCACHE |
| npm Cache | NPM_CONFIG_CACHE |
| pip Cache | PIP_CACHE_DIR |
| NuGet Packages | NUGET_PACKAGES |
| Deno | DENO_DIR |
| pnpm Store | PNPM_STORE_DIR |

### Junction 방식
JetBrains JDK (.jdks), Codeium/Windsurf (.codeium), Cursor (.cursor), Windsurf (.windsurf), Claude Code (.claude), Antigravity (.antigravity), Maven Local Repo (.m2), VS Code (.vscode), Theia IDE (.theia-ide)

## 주요 기능

- 이미 이동된 디렉토리 자동 감지 (AlreadyMoved)
- Junction으로 이동된 디렉토리를 새 위치로 재이동 지원
- 파일 잠금 (os error 32) 시 프로세스 kill 다이얼로그
- 복사 후 사이즈 검증, 원본 삭제 (백업 없음 - 디스크 절약 목적)
- 실시간 ProgressBar (바이트 기반 진행률)
- User PATH 자동 업데이트 (bin 경로 포함)
- 도구 환경변수 표시 패널 (JAVA_HOME, MAVEN_HOME 등)
- Dry Run 모드
- 롤백 지원
- 한/영 i18n (시스템 로케일 자동 감지)

## 코딩 컨벤션

- i18n 텍스트는 `src/i18n/mod.rs`의 `Texts` 구조체에 메서드로 추가
- 새 대상 추가 시: `detector.rs`의 `TOOL_DEFINITIONS` 배열 + `env_var_for_tool()` (EnvVar인 경우)
- 환경변수 표시 추가 시: `env_manager.rs`의 `collect_tool_env_vars()` 배열에 추가
- egui 한글 폰트: `C:\Windows\Fonts\malgun.ttf` (맑은 고딕) 로딩
- 프로세스 감시 목록: `file_mover.rs`의 `check_conflicting_processes()` 내 `watch_list`
