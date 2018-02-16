[![Build Status](https://travis-ci.org/loomprotocol/loom.svg?branch=master)](https://travis-ci.org/loomprotocol/loom)
[![codecov](https://codecov.io/gh/loomprotocol/loom/branch/master/graph/badge.svg)](https://codecov.io/gh/loomprotocol/loom)

Disclaimer
==========
All claims, content, designs, algorithms, estimates, roadmaps, specifications, and performance measurements described in this project are done with the author's best effort.  It is up to the reader to check and validate their accuracy and truthfulness.  Furthermore nothing in this project constitutes a solicitation for investment.

Loom &trade;
=====

Loom is a new architecture for blockchain based on the idea of encoding passage of time as data. It can achieve up to **710,000 transactions per second on a 1 gigabit network without data partitioning**. Loom can eventually recover from a fault of any size, and it provides a way to cheaply verify a distributed ledger.

Scaling blockchain has been a difficult challenge.  Not only is scaling throughput a hard problem, but any high performance blockchain has to deal with an ever increasing mountain of data. Scaling bitcoin to just 25,000 transactions per second would create a petabyte of data each year.

We solve both of the problems by solving for time. The key innovation proposed by Loom is Proof of History — encoding passage of time as data into the ledger. Loom uses strong assumption of time to construct simpler and more efficient consensus and storage protocols.

whitepaper
-----------
https://loomprotocol.com

telegram
--------

* https://t.me/loomprotocol
* https://web.telegram.org/#/im?p=@loomprotocol

Roadmap
-------

https://github.com/loomprotocol/loom/milestones

Usage
=====

The build produces a handle of command-line executables in the "target/release/" directory, `loom` and `loomd`.

loomd
-----

Loom daemon

```
Usage: loomd [options]

Options:
    -h, --help          print this help menu
    -s ADDRESS          Run as a Spool node with the Loom address
    -l PORT             Run as a Loom with a listen port

```


loom
----

user client that implements a basic wallet

```
Usage: loom FILE [options]

Options:
    -c                  create a new address
    -x                  transfer
    -b                  check the balance of destination address
    -l, --list          list your addresses and balances
    -h, --help          print this help menu
    -t ADDRESS          destination address
    -f ADDRESS          source address
    -a AMOUNT           amount
```


Build instructions
==================

For development:

```bash
$ cargo +beta build
```


Optimized for performance:

```bash
$ cargo +beta build --release
```

