#!/bin/bash -ev

target/debug/rust-kr &
SERVER_PID=$!
trap "kill $SERVER_PID" EXIT
sleep 5
phantomjs tests/capture_screen.js

IMAGE_URL=$(
    curl -X POST -H "Authorization: Client-ID ${IMGUR_CLIENT_ID}" \
        -F "title=Build #${TRAVIS_BUILD_NUMBER}" \
        -F "album=${IMGUR_ALBUM_DELETEHASH}" \
        -F "image=@index.png" \
        https://api.imgur.com/3/image \
    | node -pe 'JSON.parse(require("fs").readFileSync("/dev/stdin").toString()).data.link')
echo "Screenshot upload to ${IMAGE_URL}"
