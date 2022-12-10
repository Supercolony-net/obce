# OpenBrush Chain Extension library

The library provides tools and primitives to simplify the development of chain 
extensions for ink! and for the substrate.

The library provides macros that allow implementing the same trait on ink! and 
substrate. Macros generate all logic related to encoding and decoding arguments, 
calculation of functions id and extension id, matching function ids, and returning 
errors.  On the substrate side, you need to implement the trait and write the logic 
of each method. The ink! side will already be prepared to pass all data to you.

## TODO:
- [ ] Add unit tests
- [ ] Add examples and documentation
- [ ] Setup CI
- [ ] Add support of the `obce::weight` attributes for method 
to simplify benchmarking of the chain extension.
- [ ] Maybe ore features based on the use cases=)

## Chain extension examples
- [`pallet-assets`](https://github.com/727-Ventures/pallet-assets-chain-extension)