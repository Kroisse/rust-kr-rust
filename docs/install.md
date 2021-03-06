# Rust 설치하기

Rust는 현재 언어와 표준 라이브러리가 계속하여 발전하고 있습니다.
따라서 Rust의 현재 상태를 경험하기 위해서는 릴리즈 버전이 아닌 최신 버전 사용을 추천합니다.

## 나이틀리 사용하기

공식 나이틀리 버전이 시험적으로 제공되고 있습니다.
아래에서 알맞은 바이너리를 받으세요.

-   linux64: http://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
-   linux32: http://static.rust-lang.org/dist/rust-nightly-i686-unknown-linux-gnu.tar.gz
-   win32: http://static.rust-lang.org/dist/rust-nightly-install.exe
-   mac64: http://static.rust-lang.org/dist/rust-nightly-x86_64-apple-darwin.pkg
-   mac32: http://static.rust-lang.org/dist/rust-nightly-i686-apple-darwin.pkg

자세한 내용은 [메일링 리스트 글](https://mail.mozilla.org/pipermail/rust-dev/2014-March/009223.html)을
참고해주세요.

## 직접 빌드하기

1. 먼저 [github 저장소][rust-github]에서 소스를 다운 받으세요.

    ```bash
    $ git clone https://github.com/mozilla/rust.git
    ```

2. 해당 디렉토리로 이동해 `./configure` 를 수행하세요.

    ```bash
    $ cd rust/
    $ ./configure
    ```

3. `./configure` 가 문제 없이 실행되었다면 `make` 를 수행하여 빌드하세요.

    ```bash
    $ make
    ```

4. 빌드가 끝났다면 두가지의 선택지가 있습니다.

    1. `make install DESTDIR=<경로>` 를 이용해 특정한 경로에 rust 를 설치하기.

        이 경우에는 원하는 장소에 rust 를 설치하여 사용할 수 있습니다.

        ```bash
        $ make install DESTDIR="/Users/Jeyraof/bin/"
        ```

    2. 빌드된 현재의 저장소를 그대로 사용하기. <추천>

        이 경우는 rust 가 저장소 밖을 벗어나지 않아 다른곳을 더럽히지 않는다는 장점이 있습니다. Build 가 완료되면 생성되는 디렉토리속의 'stage2/bin/' 를 그냥 사용하시면 됩니다.

5. 위에서 결정한 바이너리의 디렉토리를 PATH 에 등록해 줍니다.

    ```bash
    $ export PATH=$PATH:<경로>
    ```

    매번 수행하기 귀찮다면 자신의 profile 에 등록해 놓으셔도 됩니다.

    ~/.profile 또는 ~/.bash_profile 의 내용:

    ```.profile
    export PATH=$PATH:<경로>
    ```

    이제 새로운 세션부터는 그냥 rustc, rustpkg, rustdoc 을 사용할 수 있습니다.


    4.2의 "빌드된 현재의 저장소를 그대로 사용하기" 를 사용하시려면 다음과 같이 하시면 됩니다. (<경로>는 보통 '<소스디렉토리>/x86_64-unknown-linux-gnu' 이런 형식으로 생성됩니다)

    ```bash
    $ export PATH=$PATH:<경로>/stage2/bin
    $ export LD_LIBRARY_PATH=$:<경로>/stage2/lib
    ```

    ~/.profile 또는 ~/.bash_profile 의 내용:

    ```.profile
    export PATH=$PATH:<경로>/stage2/bin
    export LD_LIBRARY_PATH=$PATH:<경로>/stage2/lib
    ```

6. 추가 정보

    처음으로 빌드할 때에는 [LLVM][llvm]이 먼저 빌드됩니다. 여기에서 상당한 시간이 소모되는데, 일단 한번 빌드가 되고나면 llvm 자체가 업그레이드 되지 않는 한 **기존 빌드결과물이 계속해서 사용**됩니다. 그러므로 같은 저장소에서 지속적으로 빌드하는것이 좋습니다.

### 윈도

윈도에서 Rust를 빌드하려면 먼저 [mingw][]를 준비해야 합니다.

-   [mingw 다운로드 페이지][mingw-sf-files]에서 인스톨러를 받아서 실행해주세요.
    몇 가지 기본 패키지가 받아진 다음 MinGW Installation Manager가 나타납니다.
-   "mingw-developer-toolkit", "mingw32-base", "mingw32-gcc-g++"를 선택해 설치해주세요.
    mingw 디렉토리에서 `msys/1.0/msys.bat`을 실행하면 셸이 뜹니다.
-   `/postinstall/pi.sh`를 실행해주세요.
    몇 가지 세팅이 끝나면 Rust를 빌드할 수 있는 환경이 준비됩니다.

[rust-github]: http://github.com/mozilla/rust
[llvm]: http://llvm.org/
[mingw]: http://mingw.org/
[mingw-sf-files]: http://sourceforge.net/projects/mingw/files/
