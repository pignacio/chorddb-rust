@ECHO OFF

SET RUST_BACKTRACE=1

:start
cargo build
IF %ERRORLEVEL% NEQ 0 goto long_wait

cargo run
IF %ERRORLEVEL% NEQ 0 goto long_wait
goto start

:long_wait
timeout 100
goto start
