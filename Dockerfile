FROM rust:1.56-bullseye
WORKDIR app
COPY . .

RUN apt-get update

RUN apt-get install -y clang

CMD ["cargo", "build", "--release", "--target=x86_64-unknown-linux-gnu"]
