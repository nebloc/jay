FROM rust:1.67 as builder
RUN cargo install wasm-pack
COPY . .
RUN cargo test
RUN wasm-pack build --release --target web


FROM nginx:latest
COPY index.html /usr/share/nginx/html/index.html
COPY --from=builder pkg/* /usr/share/nginx/html/pkg/
COPY nginx.conf /etc/nginx/conf.d/default.conf
