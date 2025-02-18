FROM rust:1.84.1-bullseye AS build

# create a new empty shell project
RUN USER=root cargo new --bin overseerr-sonarr
WORKDIR /overseerr-sonarr

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.rhack ./.rhack

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/overseerr_sonarr*
RUN cargo build --release

# our final base
FROM rust:1.84.1-slim-bullseye

# copy the build artifact from the build stage
COPY --from=build /overseerr-sonarr/target/release/overseerr-sonarr .

# set the startup command to run your binary
CMD ["./overseerr-sonarr"]
