# nvml-sys

[nvml-sys] is a rust crate that provides a low-level FFI wrapper around the Persistent Memory Devleopment Kit, [PMDK](https://pmem.io) (formerly NVML) and its libraries, including `libpmem` and `libpmemobj` amongst others.

This library currently tracks version 1.3.1.

Bindings are created for the following libraries:-

* `libpmem`
* `libpmemblk`
* `libpmemlog`
* `libpmemobj` (although it is not possible to support transactions as they using `setjmp`).
* `libpmempool`

Bindings are not created for:-

* `librpmem`, as this pulls in the monstrosity which is `libfabric`. This library, as of May 2017, is not production ready;
* `libvmem`
* `libvmmalloc`, an alternative malloc using persistent memory, which is deprecated (and almost impossible to use from Rust in any event). PMDK recommend the use of the `memkind` library instead.

## Licensing

The license for this project is MIT.

[nvml-sys]: https://github.com/lemonrock/nvml-sys "nvml-sys GitHub page"
