[![Build Status](https://travis-ci.org/loomprotocol/loom.svg?branch=master)](https://travis-ci.org/loomprotocol/loom)
Loom
=====
Loom is a new architecture for blockchain. It can achieve up to 710,000 transactions per second on a 1 gigabit network without data partitioning. Loom can eventually recover from a fault of any size, and it provides a way to cheaply verify a distributed ledger.

Bitcoin and Proof of Work is a phenomenal technical achievement. It’s a program that runs without interruption and is resilient to attacks and partitions. Scaling it has been a difficult challenge. 

Not only is scaling throughput a hard problem, but any high performance blockchain has to deal with an ever increasing mountain of data. Scaling bitcoin to just 25,000 transactions per second would create a petabyte of data each year.

We solve both of the problems by solving for time. The key innovation proposed by Loom is Proof of History — encoding passage of time as data into the ledger. Loom uses strong assumption of time to construct simpler and more efficient consensus and storage protocols.

whitepaper
-----------
https://loomprotocol.com

slack
------
* auto inviter

https://loomprotocol-slack.herokuapp.com/

* slack

https://loomprotocol.slack.com

trello
------

https://trello.com/b/RdNE8vbC/engineering

src/data.rs
-----------

data structures for the protocol, data types must have C layout, and no gaps

src/net.rs
-----------

network code, encoding is little endian C layout, not network effecient, but fast to read and write

src/state.rs
-----------

state machine for transactions

