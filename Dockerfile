FROM rust:1.71 as build
WORKDIR /usr/src/sql_wrapper
COPY . .
RUN cargo install --path .

FROM rust:1.71
COPY --from=build /usr/src/sql_wrapper/target/release/sql_wrapper /usr/bin/sql_wrapper
EXPOSE 9000
CMD ["sql_wrapper"]
