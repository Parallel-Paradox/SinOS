# Rust Server

This project is a little demo that builds a game server with rust, using [axum](https://github.com/tokio-rs/axum)
as web framework.

[Project Document >>](https://mint-side-571.notion.site/Action-Code-Development-log-3d8c13b9353140a9ac9e7228c39bfc8e)

## Dependency

- Redis 7.0.4 [>>Install>>](https://redis.io/docs/getting-started/installation/)
- MongoDB Community 6.0 [>>Install>>](https://www.mongodb.com/docs/manual/administration/install-community/)

See more in Cargo.toml

## Build

### Database service

Start database service. If you have started before, just ignore.

```shell
# on macOS
brew services start mongodb-community@6.0
brew services start redis
```

MongoDB will listen on `127.0.0.1:27017` by default.  
Redis will listen on `127.0.0.1:6379` by default.  
If you change the ports that these services listen on, remember to change the configuration in
constant.rs.

If you want to close service, run:

```shell
# on macOS
brew services stop mongodb-community@6.0
brew services stop redis
```

- `brew services info ${SERVICE_NAME}` : check the status of the given service.
- `redis-cli` : Open a redis session.
- `mongosh` : Open a mongodb session.

Set the authorization information `MONGO_CREDENTIAL_${SUFFIX}` in `/constant/secret.rs`.  
Go to mongodb with `mongosh` and create an administrator in admin database.

```sql
use admin
db.createUser({
    user: '<USER-NAME>',
    pwd: '<PASSWORD>',
    roles: [{role:'userAdminAnyDatabase', db: 'admin'}]
})
```

To enable authorization for mongodb, set `/etc/mongod.conf`. If there's no such file, create one.

```yaml
security:
  authorization: "enabled"
```

Set the connect uri in `/sh/create_word_list.js`, do `load(${Path_to_create_word_list.js})` in 
mongosh, the word list should be inserted into db automatically.

### Debug build

```shell
cargo run
```

### Release build

```shell
cargo run --release
```
