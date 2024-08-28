FROM golang:latest
WORKDIR /app

COPY . .

RUN chmod +x /app/entry.sh

RUN apt-get update && \
    apt-get install -y nginx curl git

# RUN rm /etc/nginx/nginx.conf /etc/nginx/conf.d/default.confV
COPY nginx.conf /etc/nginx
RUN nginx
RUN go mod download

EXPOSE 80/tcp
EXPOSE 8080/tcp

# ENTRYPOINT [ "go", "run", "server.go" ]
ENTRYPOINT [ "/app/entry.sh" ]
