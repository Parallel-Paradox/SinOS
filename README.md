# Rust Server

This project is a little demo that builds a game server with rust, using 
[axum](https://github.com/tokio-rs/axum) as web framework.

[Project Document >>](https://mint-side-571.notion.site/Action-Code-Development-log-3d8c13b9353140a9ac9e7228c39bfc8e)

## Dependency

- Redis 7.0.4 [>>Install>>](https://redis.io/docs/getting-started/installation/)
- MongoDB Community 6.0 [>>Install>>](https://www.mongodb.com/docs/manual/administration/install-community/)

See more in Cargo.toml

## Build on macOS

### Database service

**Start database service.** If you have started before, just ignore.

```shell
brew services start mongodb-community@6.0
brew services start redis
```

MongoDB will listen on `127.0.0.1:27017` by default.  
Redis will listen on `127.0.0.1:6379` by default.  
If you change the ports that these services listen on, remember to change the
configuration in constant.rs.

If you want to close service, run:

```shell
brew services stop mongodb-community@6.0
brew services stop redis
```

- `brew services info ${SERVICE_NAME}` : check the status of the given service.
- `redis-cli` : Open a redis session.
- `mongosh` : Open a mongodb session.

### Debug build

```shell
cargo run
```

### Release build

```shell
cargo run --release
```
