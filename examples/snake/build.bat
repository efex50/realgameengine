@echo off
setlocal enabledelayedexpansion

echo === Building SDL3 + wgpu Example ===

set "windows_target=x86_64-pc-windows-msvc"
set "wasm_target=wasm32-unknown-unknown"
set "release="

if "%~1"=="native" goto native
if "%~1"=="web" goto web
if "%~1"=="target" goto target
if "%~1"=="dev" goto dev

:help
echo Usage: build.bat [native^|web]
echo.
echo   native     - Build desktop version with SDL3
echo   native static - statically link sdl
echo   native run - Build and run
echo   web        - Build WebAssembly version with wasm-pack
echo   web run    - Build and run
echo   target (target)     - Build for custom rust target SDL3
echo   target (target) static - statically link sdl
echo   target (target) run - Build and run
echo.
echo   change dev environment
echo   dev (target)      example: dev windows, dev web
echo   or custom target  example: dev x86_64-pc-windows-gnu
exit /b 1

:native
echo Building native...
if "%~3"=="static" (
    set "release=,native-bin-release"
) else if "%~2"=="static" (
    set "release=,native-bin-release"
)

if "%~2"=="run" (
    cargo run -r --features="!release!"  --target !windows_target!
) else if "%~3"=="run" (
    cargo run -r --features="!release!"  --target !windows_target!
) else (
    cargo build --release --bin native --features="!release!" --target !windows_target!
    echo Run with: cargo run --release --bin native
)
exit /b 0

:web
echo Building for web (wasm32)...

where wasm-pack >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing wasm-pack...
    cargo install wasm-pack
)

wasm-pack build --target web --out-dir web\pkg

echo.
if "%~2"=="run" (
    echo Build complete!
    echo starting...
    python -m http.server 8080 -d web
) else (
    echo Build complete! To run:
    echo   cd web ^&^& python -m http.server 8080
    echo   Open http://localhost:8080 in a WebGPU-enabled browser
)
exit /b 0

:target
echo todo!
if "%~2"=="run" (
    if not "%~3"=="" (
        set "custom_target=%~3"
        cargo run -r --features="native-bin" --bin native --target !custom_target!
        exit /b 0
    )
    exit /b 1
) else if not "%~2"=="" (
    set "custom_target=%~2"
    echo !custom_target! %~2
    cargo build --release --bin native --features="native-bin" --target !custom_target!
)
exit /b 0

:dev
if "%~2"=="" (
    goto help
)

if not exist ".cargo" mkdir .cargo

if "%~2"=="windows" (
    echo [build] > .cargo\config.toml
    echo target = "!windows_target!" >> .cargo\config.toml
) else if "%~2"=="web" (
    echo [build] > .cargo\config.toml
    echo target = "!wasm_target!" >> .cargo\config.toml
) else (
    echo [build] > .cargo\config.toml
    echo target = "%~2" >> .cargo\config.toml
)
exit /b 0
