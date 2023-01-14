# api-delay-simulator

![current build status](https://github.com/don41382/api-delay-simulator/actions/workflows/build.yml/badge.svg)

A http server, which accepts http requests with a given delay to simulate long lasting http requests.

For example a request to the server:
```shell
$ curl -X GET http://localhost:3000/1500
```
will response after 1.5 seconds.

## Installation

There is no installation required. Just download the [latest version](https://github.com/don41382/api-delay-simulator/releases) from the current releases for your operating system.

## how to run

Just execute the downloaded binary, e.g. for Windows

```shell
$ api-delay-simulator.exe --help
```

```shell
Starts an api server, which will response to requests with a given delay.

In example, a GET request to http://localhost:3000/1000 will response after 1 second.

Usage: api-delay-simulator [PORT]

Arguments:
  [PORT]  [default: 3000]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## development

Checkout the git project and run cargo.

```shell
$ cargo run
```

In order to build the `api-delay-simulator` for windows on mac, you'll have to do the following:

```shell
 $ brew install mingw-w64
 $ rustup target add x86_64-pc-windows-gnu
 $ cargo build --target=x86_64-pc-windows-gnu --release
```