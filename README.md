# Project Mango

- [Project mango](#project-mango)
  - [Description](#description)
  - [Getting Started](#getting-started)
    - [Dependencies](#dependencies)
    - [Installing](#installing)
    - [Executing program](#executing-program)
  - [Help](#help)

## Description

Eckko is a platform for converting documents to audio books

## Getting Started

### Dependencies

- [GNU make](https://gnu.org/software/make)
- [Cmake](https://cmake.org)
- [Rust](https://rust-lang.org)
- [NodeJs v24 or greater](https://nodejs.org)
- [Clang/g++](https://)
- [just](https://just.systems)

### Installing

```sh
git clone https://github.com/opeolluwa/mango
cd mango
```

### Executing program

- Frontend

```sh
just watch frontend 
```

- Backend

```sh
just server-watch
```

- Application (android,desktop or ios)

```sh
  just watch {target}
```

## Help

Run `just` to see all available commands
