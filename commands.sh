# Some components of the bundle installed by rustup might be broken/missing on the latest
# nightly release: --allow-downgrade tells rustup to find and install the latest nightly
# where all the needed components are available
rustup toolchain install nightly --allow-downgrade
cargo +nightly expand 
cargo expand
cargo expand --test health_check # <- name of the test file
