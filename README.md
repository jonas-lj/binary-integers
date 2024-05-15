Binary integers
---

This repo contains a PoC of Binary Integers which is a representation of aribitrarily large integers by their odd and even part, e.g. represent an 
integer <i>n</i> by <i>(x,y)</i> where <i>n = 2<sup>x</sup> y</i> and where <i>y</i> is odd. This speeds up shifts a bit, which gives a speed up
to some algorithms. Here we give an example of how it speeds up Stein's algorithm, aka Binary GCD.
