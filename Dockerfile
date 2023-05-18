FROM rust:1.69
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
EXPOSE 8080
CMD ["http-async-publisher"]