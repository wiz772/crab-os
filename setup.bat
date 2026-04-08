@echo off
echo [INIT] Trying to set up: crab-os.

echo [INFO] Current folder: %CD%

IF EXIST Cargo.toml (
    echo [OK] Rust project detected.
) ELSE (
    echo [ERROR]  !
    pause
    exit /b
)

set /p confirm=Is this the correct project? (y/n): 
if /I "%confirm%" NEQ "y" (
    echo [ABORT] Operation cancelled.
    exit /b
)

echo [CMD] Overriding to rust nightly
rustup override set nightly

echo [CMD] Installation of bootimage...
cargo install bootimage

echo [CMD] Adding llvms components...
rustup component add llvm-tools-preview

echo [CMD] Building the cargo project...
cargo build

echo [END] Done, it could have failed tho. Good luck!
pause