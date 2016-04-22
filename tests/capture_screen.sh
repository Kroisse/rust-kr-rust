#!/bin/bash -ev

target/debug/rust-kr &
SERVER_PID=$!
trap "kill $SERVER_PID" EXIT
sleep 5
phantomjs tests/capture_screen.js
