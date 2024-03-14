# SCPI Library
This is a general library for general SCPI communications. This library can be
built individually in the top level workspace with 

```bash
cargo build --release -p scpi
```

both the <code>--release</code> and <code>-p scpi</code> arguments are
ultimately optional. If ommitted, every library in this repo will be built
