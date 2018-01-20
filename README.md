loom
=====

A new architecture for a high performance blockchain.

whitepaper
-----------
https://loomprotocol.com

slack
------
* auto inviter

https://loomprotocol-slack.herokuapp.com/

* slack

https://loomprotocol.slack.com

src/data.rs
-----------

data structures for the protocol, data types must have C layout, and no gaps

src/net.rs
-----------

network code, encoding is little endian C layout, not network effecient, but fast to read and write

src/state.rs
-----------

state machine for transactions

