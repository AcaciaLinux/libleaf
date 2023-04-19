# libleaf

The core library for the leaf package manager

# What?

Leaf consists of a frontend and this library. The fontend's job ist to take the user input and instructions for leaf and converts them to the appropriate action for this library to take. This library then manages all the hairy bits of managing packages on a system. This allows the leaf package manager to be implemented once and have different frontends. One such frontend is `branch`. It uses the leaf library to install all the necessary packages for building new packages. The cli `leaf` is a frontend too and this modularity allows for easy integration into a GUI app for the desktop user.

# Building and installing

First of all, you will need a `rust` toolchain and internet access to build this library.

After that, you can aqcuire the source code by using git:

```bash
git clone https://github.com/AcaciaLinux/libleaf
```

After changing to the source code directory by using `cd libleaf`, you can proceed with building `libleaf` by using the `Makefile`:

```bash
make
```

alternatively, you can invoke `cargo` directly:

```bash
cargo build --release
```

Once you have built `libleaf`, you can proceed to installing it by using the following `make` command (as `root`):

```bash
make install
```

The environment variable `DESTDIR` is respected here, normally the package gets installed into `/`, but by setting this variable, you can change the behaviour:

```bash
make DESTDIR=/my/custom/install/dir install
```
