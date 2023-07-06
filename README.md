#  Meghin Cypher
A single-file Rust program that implements a weird symmetric encryption algorithm my wife thought of. The algorithm is based around the palm of a hand, or more specifically the "fingers" of a cat's paw.

This repository builds a program that can encrypt sentences and words using this algorithm.

# The Algorithm

Each letter of the alphabet has an index. A=1, B=2, Z=26, etc. Now, imagine that you were to build
a table where there are special "bindings", mapping letters to numbers. To make it easy, let's say that

`A=1`

So, all of a sudden, `AA` is equal to `2`. Which, if you look up index `2` in the `alphabet`, it's `B`.

So now you've encrypted one letter. You may wind up with \[`AAA`, `A`, `AA`\] being `CAB`.

You can create the bindings any way which you want. Z could actually be 1. In the end, that part doesn't
matter.

**The heart of the alogorithm is that letters become additive, with regard to the alphabet index**. You must use the highest-valued letter possible for any given letter, similar to giving change at a cash register.

# License
Obviously there are no unit tests, or extensibility baked into this repository. So if you find this
useful somehow, absolutely go ahead and do anything you'd like with this. Fair game.
