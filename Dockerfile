# syntax=docker/dockerfile:1
FROM rustlang/rust:nightly as build
WORKDIR /hangzone
ADD . /hangzone
RUN cargo build --release
CMD ["./target/debug/hangzone_backend"]


# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /hangzone/target/release/hangzone_backend .

# set the startup command to run your binary
CMD ["./hangzone_backend"]