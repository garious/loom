[![Build Status](https://travis-ci.org/loomprotocol/loom.svg?branch=master)](https://travis-ci.org/loomprotocol/loom)

Loom &copy;
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

src/bin/client.rs
-----------------

user client binary that implements a basic wallet

TBD, conform to a full wallet spec instead of rolling our own

src/bin/spool.rs
---------------

spool daemon for verifiers


src/bin/loom.rs
---------------

loom daemon


src/data.rs
-----------

data structures for the protocol, data types must have little endian C99 layout, no gaps, and same layout on LP64 and LLP64 and other variants.

TBD a lightweight serealization format.

src/net.rs
-----------

network code, assuming all endpoints are reading and writing little endian C99 LP64 layout.

src/state.rs
-----------

state machine for transactions

src/gossip.rs
-------------

track gossip subscribers

src/wallet.rs
-------------

wallet library

