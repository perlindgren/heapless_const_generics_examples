# Generic Array B Gone

``` shell
rustup override set nightly

cargo run
```

## Turned into a Memory Lane POC

examples/target_side.rs

Serializes two heapless Vec<Point, 8> into buf, and writes that to a binary file.

examples/host_side.rs

Deserializes the two heapless vectors into Vec<Point, 8>.

## Caveats

The host "knows" what types to read.
