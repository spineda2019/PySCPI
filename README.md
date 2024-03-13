# PySCPI
PySCPI is a python module written in rust to enable fast communication to
instruments using the SCPI protocol <em>fast</em>. No need to sacrifice
python's readable semantics for speed.

## Building the wheel from source
This project uses <a href = "https://github.com/PyO3/maturin">maturin</a> to
build the python wheel. The libraries are written and tested in native rust in
the <code>"scpi"</code> folder (and could be used in any other projects using
the DLLs/SOs built from it) and glued to python using PyO3's macros. To build
the wheel, run the following in the top level workspace folder:

```bash
maturin build --release -m python_bindings/cargo.toml
```

Optionally for debug builds, you may omit <code>--release</code>. Then your
wheel will be built in <code>target/wheels</code> and can be pip installed!
Currently, there are a few public APIs available from the module, but I have
not documented them quite yet

## Public APIs
Coming soon...
