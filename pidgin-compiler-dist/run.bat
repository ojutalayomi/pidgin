@echo off
REM Pidgin Compiler Runner Script for Windows
REM Usage: run.bat <file.pg>

if "%~1"=="" (
    echo Usage: %0 ^<file.pg^>
    echo Example: %0 examples\hello.pg
    exit /b 1
)

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0
set EXECUTABLE=%SCRIPT_DIR%pidgin-compiler.exe

REM Check if the executable exists
if not exist "%EXECUTABLE%" (
    echo Error: pidgin-compiler.exe not found in %SCRIPT_DIR%
    exit /b 1
)

REM Run the compiler
"%EXECUTABLE%" %*
