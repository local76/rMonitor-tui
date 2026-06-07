@echo off
echo =========================================
echo Building rMonitor (Rust System Monitor)
echo =========================================
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cargo build failed!
    exit /b %ERRORLEVEL%
)
copy /y target\release\rmonitor.exe .\rmonitor.exe
echo =========================================
echo Build successful! Run .\rmonitor.exe to start.
echo =========================================
