FROM nginx:latest
COPY ./index.html /usr/share/nginx/html/index.html
COPY ./pkg/* /usr/share/nginx/html/pkg/
