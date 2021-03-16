FROM node:15-slim AS builder
WORKDIR /tmp

ADD ./ ./
RUN npm install && API_URL=/api npm run build

FROM donatowolfisberg/spa-server:0.1.2 as spa-builder

COPY --from=builder /tmp/public/ ./public
RUN ./build.sh

FROM scratch

COPY --from=spa-builder /app/server /server
CMD ["/server"]
