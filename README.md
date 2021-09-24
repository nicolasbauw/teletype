# Teletype Emulator

[![Current Crates.io Version](https://img.shields.io/crates/v/teletype.svg)](https://crates.io/crates/teletype)
[![Downloads badge](https://img.shields.io/crates/d/teletype.svg)](https://crates.io/crates/teletype)

This is a teletype emulator, written for my [8080 emulator](https://crates.io/crates/intel8080).
It emulates a teletype interfaced on a 88-SIO board (MITS/Altair)

To install:
```text
cargo install teletype
```

You can run an altair binary, for example Basic 3.2 :
```
teletype 4kbas32.bin
```

It has been tested with the echo test routine and the Altair basic 3.2:
```
❯ teletype ~/Dev/4kbas32.bin 

MEMORY SIZE? 8192
TERMINAL WIDTH? 
WANT SIN? N
WANT RND? N
WANT SQR? N

5068 BYTES FREE

BASIC VERSION 3.2
[4K VERSION]

OK
```
