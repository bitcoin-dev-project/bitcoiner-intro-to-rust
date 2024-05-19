# Project Overview: Decoding Raw Transactions

What we are going to build is something similar to `bitcoin-cli decoderawtransaction [raw transaction hex]`.
What it does is to take a raw transaction in hexadecimal format as a command line argument, decode and print it out in a human readable json format.
For example, consider the following terminal command:

`
$ bitcoin-cli decoderawtransaction 0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000
`

This will print the following json response to the terminal:
```
{
  "txid": "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16",
  "hash": "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16",
  "version": 1,
  "size": 275,
  "vsize": 275,
  "weight": 1100,
  "locktime": 0,
  "vin": [
    {
      "txid": "0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9",
      "vout": 0,
      "scriptSig": {
        "asm": "304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d09[ALL]",
        "hex": "47304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901"
      },
      "sequence": 4294967295
    }
  ],
  "vout": [
    {
      "value": 10.00000000,
      "n": 0,
      "scriptPubKey": {
        "asm": "04ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84c OP_CHECKSIG",
        "desc": "pk(04ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84c)#hsw9ejus",
        "hex": "4104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac",
        "type": "pubkey"
      }
    },
    {
      "value": 40.00000000,
      "n": 1,
      "scriptPubKey": {
        "asm": "0411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3 OP_CHECKSIG",
        "desc": "pk(0411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3)#u7qfa49l",
        "hex": "410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac",
        "type": "pubkey"
      }
    }
  ]
}
```

Our Rust program will do something similar.
Instead of running `bitcoin-cli decoderawtransaction [raw transaction hex]`, we will call our program using the Rust package manager, Cargo:

`
$ cargo run -- 0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000
`

### Refresher

Transactions are one of the primary components of Bitcoin.
A transaction is a data structure that represent the transfer of value in the network.
It provides proof to full nodes that a transfer is valid and that the node database should be updated to reflect the new ownership.
Transaction data is relayed between nodes as a sequence of bytes (also called a bitstream).
Nodes and miners, using Bitcoin software, know how to deconstruct and analyze that byte sequence according to the consensus protocol specification.
Chapter 6 of Mastering Bitcoin goes into detail breaking down a transaction.
I highly recommend reading this chapter before starting to work on this program.
It also offers a useful byte map diagram to visualize the different components:

<img src="https://raw.githubusercontent.com/bitcoinbook/bitcoinbook/develop/images/mbc3_0601.png" width=500>

### Other Helpful Resources

* If you have not set up Bitcoin Core and are not familiar with `bitcoin-cli`, please take some time to play around with Bitcoin from the command line.
A great educational resource can be found [here](https://github.com/BlockchainCommons/Learning-Bitcoin-from-the-Command-Line?tab=readme-ov-file).
* [learnmeabitcoin.com](https://learnmeabitcoin.com/) also has some great educational content and tutorials explaining complex aspects of the Bitcoin protocol in a very accessible way.

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="02_setup.md">>>> Next Lesson: Setup</a></p>
</div>
