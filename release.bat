@echo off
REM Manual release trigger script for Fast Image (Windows)

echo 🚀 Fast Image Release Script for Windows

REM Check if we're in a git repository
if not exist ".git" (
    echo ❌ Not a git repository. Please run this script from the project root.
    pause
    exit /b 1
)

REM Get current version from Cargo.toml
for /f "tokens=3 delims= " %%a in ('findstr "^version = " Cargo.toml') do set CURRENT_VERSION=%%a
set CURRENT_VERSION=%CURRENT_VERSION:"=%
echo 📦 Current version: %CURRENT_VERSION%

REM Ask for new version
set /p NEW_VERSION="🔢 Enter new version (or press Enter to use current): "
if "%NEW_VERSION%"=="" set NEW_VERSION=%CURRENT_VERSION%

echo 🏷️  Preparing release for version: %NEW_VERSION%

REM Confirm
set /p CONFIRM="❓ Continue with release %NEW_VERSION%? (y/N): "
if /i not "%CONFIRM%"=="y" (
    echo ❌ Release cancelled
    pause
    exit /b 1
)

REM Update version in Cargo.toml if different
if not "%NEW_VERSION%"=="%CURRENT_VERSION%" (
    echo 📝 Updating version in Cargo.toml...
    powershell -Command "(Get-Content Cargo.toml) -replace '^version = .*', 'version = \"%NEW_VERSION%\"' | Set-Content Cargo.toml"
)

REM Update Cargo.lock
echo 🔒 Updating Cargo.lock...
cargo update

REM Create release commit
echo 💾 Creating release commit...
git add Cargo.toml Cargo.lock
git commit -m "Release v%NEW_VERSION%" 2>nul || echo No changes to commit

REM Create and push tag
echo 🏷️  Creating and pushing tag...
git tag -a "v%NEW_VERSION%" -m "Release v%NEW_VERSION%"
git push origin "v%NEW_VERSION%"
git push origin master

echo 🎉 Release v%NEW_VERSION% has been triggered!
echo 📋 Check GitHub Actions for build progress:
echo    https://github.com/lihongjie0209/fast-image/actions
echo.
echo 📦 Release will be available at:
echo    https://github.com/lihongjie0209/fast-image/releases/tag/v%NEW_VERSION%

pause
