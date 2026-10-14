# Build, install and execute the integer path

## Build independently

Use Linux, Rust 1.90, Python 3, a C compiler, the Vulkan loader, SPIRV-Tools and a Vulkan device with the four required features. Mesa llvmpipe can run the tests without physical GPU hardware. Enable the Khronos validation layer for qualification runs.

```sh
rustup toolchain install 1.90.0 --profile minimal --component rustfmt --component clippy
rustup default 1.90.0
make -j2 portable-check
make -j2 portable-test
make -j2 portable-release
python3 -m unittest discover -s validation -p test_installer.py -v
```

Each package has its own manifest and lock file. The build script discovers packages under `portable`, `backends`, `bindings` and `frontend`; it does not enumerate backend identities. The portable targets do not initialize F*/Karamel or require CUDA. Do not use the legacy default `make all` when testing this independent path.

## Install workers additively

```sh
python3 scripts/install-portable-worker.py backends/spirt/target/release/kuiper-spirt-worker --root .portable-install
python3 scripts/install-portable-worker.py backends/vulkan/target/release/kuiper-vulkan-worker --root .portable-install
portable/core/target/release/kuiper-core inspect .portable-install
```

The installer measures the binary, sends a bounded `describe` request over `kuiper.worker/1`, validates the response and copies the binary into an immutable identity/digest directory with a canonical manifest. Installations require an operator-controlled root. A worker description declares compatible contracts; it does not supply trusted proof evidence.

For a compatible additional backend, add its independent package, lock file and measured worker. Declare the compiler's KIR/profile inputs and artifact/format/word-ABI outputs. Declare the runtime's artifact/format/word-ABI inputs and execution-schema/word-ABI outputs. Install both workers. No existing source, backend registry or package list needs editing. A different source operation or incompatible semantic profile requires its own contract and admission work.

## Execute a fixture

Use [vector-add.json](../../validation/fixtures/vector-add.json) as the package. Save this invocation as `invocation.json`:

```json
{"entry":"vector_add","workgroups":[1,1,1],"parameters":[],"buffers":[{"resource":0,"words":[1,2,3,4],"offset":0,"length":4},{"resource":1,"words":[10,20,30,40],"offset":0,"length":4},{"resource":2,"words":[0,0,0,0],"offset":0,"length":4}]}
```

```sh
portable/core/target/release/kuiper-core check validation/fixtures/vector-add.json
portable/core/target/release/kuiper-core reference validation/fixtures/vector-add.json invocation.json
portable/core/target/release/kuiper-core run validation/fixtures/vector-add.json invocation.json .portable-install --allow-experimental
```

The output resource must contain `[11,22,33,44]`. Reads and allocation padding must remain unchanged. The explicit experimental flag prevents accidentally presenting this path as qualified verified execution.

## Replay the independent test path

```sh
make -j2 portable-integration
```

This installs both measured workers into a fresh directory, compiles a C driver and executes the fixtures through the core and C ABI. It checks literal expected values and the independent reference evaluator. The result records source hashes, installed executable hashes, validator identity, actual device and each executed case. A failure stops the run; it is not written as a passing report.

Vulkan 1.2, `robustBufferAccess`, `vulkanMemoryModel` and `vulkanMemoryModelDeviceScope` must all be supported and enabled. The compiler does not use availability/visibility chains, shared memory or barriers in this profile. The official [Vulkan SPIR-V environment](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html) defines the target requirements.
