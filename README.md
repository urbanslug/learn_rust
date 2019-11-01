Experiments with Rust packaging

## Tests

```
cargo test
```

## Benchmarks

```
cargo +nightly bench
```

## Profiling

Uncomment `debug = true` in Cargo.toml

### OSX
Source: https://carol-nichols.com/2017/04/20/rust-profiling-with-dtrace-on-osx/

```
cargo build --release
```


```
sudo dtrace -c './target/release/learn_rust' \
-o out.stacks \
-n 'profile-997 \
/execname == "zopfli"/ \
{ @[ustack(100)] = count(); }'
```


```
./stackcollapse.pl out.stacks | ./flamegraph.pl > pretty-graph.svg
```

### Linux
