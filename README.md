## Build image

```
$ cd docker-calculator
$ docker build -t docker-calculator -f docker/Dockerfile .
```

## Run

```
$ docker run -dp 127.0.0.1:8000:8080 docker-calculator
```