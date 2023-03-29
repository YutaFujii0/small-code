# About

A demo of OAuth2 client server.

The purpose is to learn OAuth2, rather than how to take advantage of spring libraries to implement it.
Therefore, this is not something you can use for production. 


### Dependency locking

By default, gradle does not lock dependencies.
To make lockfile, follow:
https://docs.gradle.org/current/userguide/dependency_locking.html

* strict mode, or lenient

```shell
# update lockfile
./gradlew build --write-locks
```
