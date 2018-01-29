Loom
====

An implementation in C.  This effort was abandonded in favor of Rust!

src/loom.c
loom takes tranascations, schedules them in the most optimial to execute way

    1. read packets from the network
        * src/sock.c
        * src/reader.c
    2. verify the signatures
        * if its a PoRep, check all of the sigs
    3. fetch all the data from the big table
        * src/fetcher.c
    3.5 sorter
        * create an ops table, and then sort it by reads first, then writes
            * read from address
            * read to address
            * write to address
        * src/reader.c
    4. do all the account transfers
        * src/executor.c
    5. write all the results
        * src/writer.c
    6. compute the merkle of the big table
    7. sequence all the transactions
    8. add the merkle result to the sequence 

src/tbd.c
proof of replication nodes

    1. encrypt the table with your key
    2. update it
    3. compute the merkle of the big table
    4. post it to the loom

    5. after N posts, ask everyone to verify them all at once
    6. post the verification packet into the loom if you have more then 51% of the verifications
        * might need to make these packets space effecient, so first have a global route of previous ProRep verifiers, and stack the signatures
    7. loom will credit you with PoRep which count as votes
    8. you can cash them out as coins with another packet
