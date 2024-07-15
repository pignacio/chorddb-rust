# bookworm images are more secure compared to alpine
FROM oven/bun:1.1 AS build
# FROM node:20-bookworm AS build

ENV NODE_ENV=production

WORKDIR /app

COPY package.json ./
COPY bun.lockb ./
RUN bun install

COPY . ./

RUN bun run build

# FROM oven/bun:1.1
FROM node:20-bookworm
WORKDIR /app
COPY --from=build /app .
# ENV HOST is not though you can uncomment if if needed
# ENV HOST=0.0.0.0
EXPOSE 3000

CMD ["node", "build"]
