FROM --platform=linux/amd64 rust:1-buster

WORKDIR /app

# install solana since `cargo build-bpf` custom cargo command (cargo-build-bpf) is generated here
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.10.26/install)"

# export additioan path
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# execute what `npm run build:program-rust` does
CMD cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program

# After build, run
# docker run --platform linux/amd64 --mount type=bind,src=$(pwd),dst=/app builder