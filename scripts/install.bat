@echo off
setlocal enabledelayedexpansion

set BINARY=hdisk
set INSTALL_DIR=%USERPROFILE%\.cargo\bin

if "%1"=="--uninstall" (
    echo Removing %BINARY% from %INSTALL_DIR%...
    del /f /q "%INSTALL_DIR%\%BINARY%.exe" 2>nul
    echo Done.
    exit /b 0
)

echo Building %BINARY%...
cargo build --release

echo Installing %BINARY% to %INSTALL_DIR%...
copy /y "target\release\%BINARY%.exe" "%INSTALL_DIR%\%BINARY%.exe"
echo Installed. Run '%BINARY%' to start.
