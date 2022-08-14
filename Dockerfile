#Start with a rust docker image
FROM rust:1.63.0

# switch to the app folder
WORKDIR /app

# install
RUN apt update && apt install lld clang -y

#copy all files from our work environment to our Docker image
COPY . .

#sqlx offline flag
ENV SQLX_OFFLINE true

#build he binary
RUN cargo build --release

ENV APP_ENV production

#entrypoint
ENTRYPOINT ["./target/release/zero2prod"]
