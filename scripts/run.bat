@ECHO OFF
:start
cargo build
IF %ERRORLEVEL% NEQ 0 goto long_wait

cargo run
goto start

:long_wait
timeout 100
goto start
