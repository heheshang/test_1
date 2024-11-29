RUST_LOG="debug"
[registries.crates-io]
# protocol = "sparse"

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'myres'
[source.myres]
registry = "https://mirror.ghproxy.com/https://github.com/rust-lang/crates.io-index"
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
[build]
rustc-wrapper = "sccache"
incremental = true
target-dir = "temp/target"
rustflags = ["-C", "link-arg=-B=mold"]
#rustflags = ["--cfg", "tokio_unstable"]
#rustflags = ["--cfg", "tracing_unstable"]
[profile.release]
lto="thin"
codegen-units=1
opt-level = 1
[profile.dev]
lto="thin"
codegen-units=1
opt-level = 3
[profile.dev.package."*"]
opt-level = 3
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-B=mold"]
