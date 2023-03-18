# arpc
arpc reverse polish calculator reads math in [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) from stdin
and calculates!

## Example
```console
git clone https://github.com/sanj0/arpc.git
cd arpc
cargo r <<< 2 0.5 ^ 2 0.5 ^ *
2.0000000000000004 # whaaaaaaat? f64 is not suited for calculators???
```

