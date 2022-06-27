# Procedure

```bash
docker build -t elasticsearch-demo .

# Run container
docker run -p 9200:9200 -p 9300:9300 --mount type=bind,src=$(pwd),destination=/app -e "discovery.type=single-node"  elasticsearch-demo

# Login container
docker exec -it <CONTAINER_NAME> bash
```

Inside your container,

```bash
# insert documents
curl -X POST localhost:9200/movies/_doc -H 'Content-Type: application/json' -d @movies.json

# search
bash query.sh

# performance
time bash query.sh
```

# Miscelleneous

``` bash

# show fields
curl localhost:9200/movies/_mapping/field/*
```


# Result

```
[root@46ae7036594d app]# time bash query.sh
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 15.4M  100 15.4M  100    68   214M    944 --:--:-- --:--:-- --:--:--  214M

real	0m0.087s
user	0m0.006s
sys	0m0.007s
```