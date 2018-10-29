# bak, bakt, baktm

[![Travis CI build status](https://travis-ci.org/PhilboBaggins/bak.svg?branch=master)](https://travis-ci.org/PhilboBaggins/bak)

.....

..... [???](./Original bash implementations.md) ......

## bak

.....

    $ cargo run --bin bak test-file.txt
    Copied test-file.txt -> test-file.txt.bak

## bakt

.....

    $ cargo run --bin bakt test-file.txt
    Copied test-file.txt -> test-file.txt_2018-07-12_02-28-28.bak

## baktm

.....

    $ cargo run --bin baktm test-file.txt
    Copied test-file.txt -> test-file.txt_2018-07-12_02-28-15.bak

## License

Licensed under either of the following:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
