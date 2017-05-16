Blockchain Protocol
===================

For now, this is just a _Proof of Stake_ (PoS) system. _Proof of Work_ (PoW) will come later.


What we need:
--------

- *Blocks*
    - hash of predecessor
    - timestamp
    - minter (identity + signature)
    - list of artifacts


- *Artifacts*
    - timestamp
    - author
    - data (kind / information)



Notes:
--------

- Repeating information (e.g. an artifact storing its own unsigned hash)
is always bad because you have to validate it anyways, therefore it provides no benefit.

- How many artifacts in each block?

- Blocks need to be minted within a time window. There needs to be a condition when the chosen
minter fails to create the block, in which case it gets shouldered to another block. This is completely
based on timestamps.


Network Protocol
================

What we need:
-------

- Message types
    - locating peers
    - sending artifacts (old / new)
    - asking for blocks / sending blocks


Notes:
--------

- Does every node need a full picture of the current network graph?

- How do you know who you want to connect to?

- How are conflicts resolved?
      - e.g. ties, when two different blocks are equally long

- I think bloom filters may be helpful, since we have a lot of
hashed information that we need to prevent duplicates of.
