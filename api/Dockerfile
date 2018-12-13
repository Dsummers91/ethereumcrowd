FROM rustlang/rust:nightly

COPY . /home/api
WORKDIR /home/api

RUN rustup install nightly
RUN rustup default nightly
RUN cargo build

WORKDIR ./target/debug/
CMD ["api"]
