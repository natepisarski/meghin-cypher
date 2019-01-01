#  Meghin Cypher
This repository is the end result of what happens when both members of a romantic relationship are some
kind of scientist. An entomologist / computer programmer, somehow gives you this single-file codebase.

When "doing something with the cat", as I heard, Meghin came up with a symmetric cypher to encrypt and
decrypt text. The heart of it revolves around fingers on your hand, or claws on a cat's paw. In the end,
it can exist outside of anything physical.

This repository is a rust "script" that can encrypt sentences using this algorithm.

# The Algorithm

Each letter of the alphabet has an index. A=1, B=2, Z=26, etc. Now, imagine that you were to build
a table where there are special bindings, mapping letters to numbers. To make it easy, let's say that

`A=1`

So, all of a sudden, `AA` is equal to `2`. Which, if you look up index `2` in the `alphabet`, it's `B`.

So now you've encrypted one letter. You may wind up with \[`AAA`, `A`, `AA`\] being `CAB`.

You can create the bindings any way which you want. Z could actually be 1. In the end, that part doesn't
matter.

**The heart of the alogorithm is that letters become additive, with regard to the alphabet index**

# License
Obviously there are no unit tests, or extensibility baked into this repository. So if you find this
useful somehow, absolutely go ahead and do anything you'd like with this. Fair game.