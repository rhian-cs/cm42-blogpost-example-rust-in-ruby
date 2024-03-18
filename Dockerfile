# docker build -t adder .
# docker run --rm -it adder

#### Stage 1: Rust ####
FROM rust:1-bookworm as rust

WORKDIR /adder
COPY adder .

RUN cargo build -r

#### Stage 2: Ruby ####
FROM ruby:3-bookworm

# Copy the built artifact from the other container
COPY --from=rust /adder/target/release /opt/adder

WORKDIR /app
COPY ruby .

# Install dependencies using Bundler
RUN bundle install

ENV LIBADDER_SO_PATH=/opt/adder/libadder.so

# Start your Ruby/Rails application
CMD ["irb", "-r", "/app/main"]
