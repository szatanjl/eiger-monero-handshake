Monero P2P Handshake
====================

This repo contains basic implementation of Monero P2P Handshake protocol
in Rust.


Building and running
--------------------

In order to build and run the code do the following:

    cargo build
    cargo run -- [<URL>]

URL is an optional parameter that tells to which P2P node connect to
(by default it connects to node: `node.moneroworld.com:18080`).

If everything works correctly then you should see messages received from
the P2P node being written to stdout.


Building and testing on local machine
-------------------------------------

- Clone monero code from <https://github.com/monero-project/monero>
- Checkout tag `release-v0.18`
- Build project following instructions from monero GitHub page
- Once it is built run `./monerod [--prune-blockchain]`
- Run `cargo run -- 127.0.0.1:18080`
- Profit!


Notes
-----

Monero P2P protocol == Levin protocol

`LEVIN_PROTOCOL.md` describes header.
`p2p/p2p_protocol_defs.h` describes payloads.

Monero (Levin) uses Little Endian for numbers in protocol.

Levin defines commands 1004-1006 (StatInfo, NetworkState, PeerId) but
monero actually doesnt handle them from what it looks like in code.


Files worth noting in monero implementation
-------------------------------------------

    p2p/p2p_protocol_defs.h
        structs COMMAND_*::request_t/response_t
    p2p/net_node.h:315
        handle commands mapping (HANDLE_INVOKE_T2)
    p2p/net_node.inl
        handle commands implementation

    cryptonote_protocol/cryptonote_protocol_handler.h
        cryptonote protocol


References
----------

- <https://github.com/monero-project/monero/blob/master/docs/LEVIN_PROTOCOL.md>
- <https://github.com/xmr-rs/xmr/blob/master/doc/levin.md>
- <https://github.com/sanderfoobar/py-levin>
- <https://github.com/jeffro256/levin-rs/tree/main>
