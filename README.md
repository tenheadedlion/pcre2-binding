# [Pcre2](https://github.com/PCRE2Project/pcre2) Binding Demo

## Build

```shell
git submodule update --init
make
cargo build
```

## Start nc server

```shell
nc -ul localhost 34254
```

## Run the pcre2 client

```shell
cargo run
```

## Manifest

- /src/search.c is compiled as a static library
- `int search(const char *pattern_in, const char *subject_in, char *out)`
    - `char *out` is the out parameter that receives a newline-separated output
    - if the return value is `0`, then there are pattern matches in the subject string.
