# 🔍 Findoor

_Findoor_ is a simple project aiming to help you mine for addresses with special criteria. Unlike
other address miners who's main purpose is to find a vanity address with a specific starting
sequence this miner lets you easily define a custom "match" condition by simply replacing the
`matches` function in the [`criteria.rs`](src/criteria.rs) file.

For vanity address miners it's recommended to use one of:
- [profanity2](https://github.com/1inch/profanity2)
- [create2crunch](https://github.com/0age/create2crunch/)
