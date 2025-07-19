@echo off
REM Pidgin Compiler Update Script for Windows
REM This script helps you update the Pidgin compiler to the latest version

echo Pidgin Compiler Update Script
echo ================================

REM Detect architecture
for /f "tokens=*" %%i in ('wmic os get osarchitecture /value ^| find "="') do set %%i
if "%OSArchitecture%"=="64-bit" (
    set PLATFORM=windows-x86_64
) else (
    echo Error: Unsupported architecture
    exit /b 1
)

echo Detected platform: %PLATFORM%

REM Find current installation
set CURRENT_PATH=
where pidgin-compiler.exe >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('where pidgin-compiler.exe') do set CURRENT_PATH=%%i
    echo Current installation found at: %CURRENT_PATH%
) else (
    echo No current installation found
)

REM Get latest version (using PowerShell)
echo Checking for latest version...
for /f "tokens=*" %%i in ('powershell -Command "(Invoke-RestMethod -Uri 'https://api.github.com/repos/ojutalayomi/pidgin/releases/latest').tag_name"') do set LATEST_VERSION=%%i

if "%LATEST_VERSION%"=="" (
    echo Failed to get latest version
    exit /b 1
)

echo Latest version: %LATEST_VERSION%

REM Check if we need to update
if not "%CURRENT_PATH%"=="" (
    for /f "tokens=*" %%i in ('pidgin-compiler.exe --version 2^>nul ^| findstr /r "v[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*"') do set CURRENT_VERSION=%%i
    echo Current version: %CURRENT_VERSION%
    
    if "%CURRENT_VERSION%"=="%LATEST_VERSION%" (
        echo You already have the latest version!
        exit /b 0
    )
)

REM Create temporary directory
set TEMP_DIR=%TEMP%\pidgin-update-%RANDOM%
mkdir "%TEMP_DIR%"

REM Download latest release
set DOWNLOAD_URL=https://github.com/ojutalayomi/pidgin/releases/download/%LATEST_VERSION%/pidgin-compiler-%PLATFORM%.zip
echo Downloading latest release...

powershell -Command "& {[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP_DIR%\pidgin-compiler-%PLATFORM%.zip'}"

if not exist "%TEMP_DIR%\pidgin-compiler-%PLATFORM%.zip" (
    echo Failed to download latest release
    rmdir /s /q "%TEMP_DIR%"
    exit /b 1
)

REM Extract the release
echo Extracting release...
cd /d "%TEMP_DIR%"
powershell -Command "Expand-Archive -Path 'pidgin-compiler-%PLATFORM%.zip' -DestinationPath '.' -Force"
cd "pidgin-compiler-%PLATFORM%"

REM Install the update
echo Installing update...
if exist "install.bat" (
    REM Check if we can write to the current installation directory
    if not "%CURRENT_PATH%"=="" (
        for %%i in ("%CURRENT_PATH%") do set INSTALL_DIR=%%~dpi
        echo Updating existing installation...
        copy "pidgin-compiler.exe" "%CURRENT_PATH%" >nul
        echo Update completed successfully!
    ) else (
        echo Installing new version...
        call install.bat
    )
) else (
    echo Installation script not found in release
    cd /d /
    rmdir /s /q "%TEMP_DIR%"
    exit /b 1
)

REM Clean up
cd /d /
rmdir /s /q "%TEMP_DIR%"

REM Verify the update
where pidgin-compiler.exe >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('pidgin-compiler.exe --version 2^>nul ^| findstr /r "v[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*"') do set NEW_VERSION=%%i
    echo Update verified! New version: %NEW_VERSION%
) else (
    echo Warning: Could not verify the update
)

echo.
echo Update complete!
echo You can now use: pidgin-compiler.exe --version
pause 