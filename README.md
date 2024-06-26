# container-status

Simple dashboard that displays the status of all docker containers running on a system.

## Usage

### Native

1. Install Rust
2. Install Docker
3. Clone repository
4. Run `cargo run --release`

### Docker

#### Run

1. Install Docker
2. Pull image from: `<tbd>`
3. Create a `docker-compose.yml`
4. Run `docker compose up -d`

```yml
services:
  container-status:
    image: container-status:latest
    container_name: container-status
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    ports:
      - 42069:42069
```

#### Build

1. Install Docker
2. Build the docker image locally using the `Dockerfile` or use the `docker-compose.yml` file (`docker compose up -d --force-recreate --build container-status`)
