# 10 Solution

### Quiz
*How do nodes know whether the transaction is a legacy or a segwit transaction as they read it? How do they know whether to view the next field after the version as an input length encoded as compactSize or as the marker and flag for a Segwit transaction?*

### Solution
From [BIP-144](https://github.com/bitcoin/bips/blob/master/bip-0144.mediawiki#serialization): 
> Parsers supporting this BIP will be able to distinguish between the old serialization format (without the witness) and this one. The marker byte is set to zero so that this structure will never parse as a valid transaction in a parser that does not support this BIP. If parsing were to succeed, such a transaction would contain no inputs and a single output.

In other words, if the next byte is `0x00` followed by a `0x01`, then the parser will interpret the transaction as having witness data and be a segwit transaction. If instead, the next byte after the version is non-zero, then this is a legacy transaction with the byte indicating the compactSize length of inputs. This scheme takes advantage of the fact that a valid transaction cannot have zero inputs. 