# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src
RUN cd fuzzing && cargo +nightly afl build --release

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /src/fuzzing/target/release/fuzz /
