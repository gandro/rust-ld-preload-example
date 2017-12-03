# Example: `LD_PRELOAD` with Rust 

    # build a Rust cdylib which hijacks call to `puts`
    $ cargo build --release

    # a example C program
    $ gcc hello.c -o hello
    $ ./hello 
    Hello there, C is awesome!

    # use LD_PRELOAD
    $ LD_PRELOAD=./target/release/libpreload.so ./hello
    Hello there, Rust is awesome!

