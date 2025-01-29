# Iterator Helper - iter_help
This library provides helpful structures, traits, and functionality for iterators with the goal of encapsulating complex concepts and making iterating over multiple iterators efficient.

## Features

- ping_pong
  - This module allows for users to map a tuple of Vec or Iter into a single Vec of some other type while ensuring that each collection is only traveled through once. This optimizes the iteration into an efficiency of O(SUM(Ni)) where Ni is the length of each collection.
  - A common usage of this module is to merge sorted collections into one final collection.

## Examples

TODO