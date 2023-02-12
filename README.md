# badcat

It's `cat`, but multithreaded. That is to say, it prints each input line using a different thread.

And each line-thread prints each character using a different character-thread.

This approach has the following advantages:

* 

This approach has the following tradeoffs:

* Out of sequence
* Slower
* Uses more memory
* Makes programmers' eyes bleed

[@plredmond](https://github.com/plredmond) and [@nebkor](https://github.com/nebkor) made me do it.
