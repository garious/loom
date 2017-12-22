loom
=====

whitepaper
https://www.overleaf.com/read/hffssmprmsgs

src/data.rs
-----------

data structures for the protocol, data types must have C layout, and no gaps

src/net.rs
-----------

network code, encoding is little endian C layout, not network effecient, but fast to read and write

