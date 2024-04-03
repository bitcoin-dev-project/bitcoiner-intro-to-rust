# Background

Let's start by outlining what our program is going to do at a high level. 

### Project Overview: Decoding Raw Transactions
If you have already set up Bitcoin Core locally and are familiar with the Bitcoin command line interface, what we are going to build is something very similar to `bitcoin-cli decoderawtransaction [raw transaction hex]`. What this does is it takes a raw transaction in hexadecimal format as a command line argument, decodes it and prints it out in a human readable format. For example, consider the following terminal command:

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

Our Rust program will do something very similar. Instead of running `bitcoin-cli decoderawtransaction [raw transaction hex]`, we will call our program using the Rust package manager, Cargo:

`
$ cargo run -- 0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000
`

### Refresher

As a quick refresher, transactions are one of the primary components of Bitcoin. They are a data collection that represent the transfer of money and provide proof to full nodes that a transfer is valid and that the node database and network should be updated to reflect the new ownership. Transaction data is relayed to nodes as a collection of bytes. Nodes and miners, using Bitcoin software, know how to deconstruct and analyze that byte data according to the consensus protocol specification. Chapter 6 of Mastering Bitcoin goes into detail breaking down a transaction. I highly recommend reading this chapter before starting to work on this program. It also offers very useful byte map diagrams, which we'll reference as a visual aid. For example,

![Alt text](https://raw.githubusercontent.com/bitcoinbook/bitcoinbook/develop/images/mbc3_0601.png)
Source: https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#alice_tx_byte_map

### Other Helpful Resources

* If you have not set up Bitcoin Core and are not familiar with `bitcoin-cli`, I highly recommend taking some time to play around with Bitcoin from the command line. A great educational resource can be found here: https://github.com/BlockchainCommons/Learning-Bitcoin-from-the-Command-Line?tab=readme-ov-file.

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="02_setup.md">>>> Next Lesson: Setup</a></p>
</div>
