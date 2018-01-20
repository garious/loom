loom
=====

Loom is a new architecture for blockchain that can achieve up to 710,000 transactions per second on a 1 gigabit network without data partitioning. 

The key innovation is Proof of History - encoding passage of time as data into the ledger. Loom uses the strong assumption of time to construct simpler and more efficient protocols and proofs.

Learn more about Loom and read the whitepaper, or join us on slack at https://loomprotocol.com

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

