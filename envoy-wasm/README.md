# KMamiz WASM EnvoyFilter

> KMamiz WASM Envoy filter plugin written in Go

## Deployment

1. First, run the `build.sh` script then build KMamiz, the `KMamizEnvoyFilter.wasm` file will be exposed on `/wasm/KMamizEnvoyFilter.wasm`.

## Development

1. (Optional) Install TinyGo
2. Download the [proxy-wasm-go-sdk](https://github.com/tetratelabs/proxy-wasm-go-sdk)

```bash
go mod edit -require=github.com/tetratelabs/proxy-wasm-go-sdk@main
go mod download github.com/tetratelabs/proxy-wasm-go-sdk
```

3. Watch out for unimplemented functionalities
4. To try out locally, run:

```bash
docker run -it --rm -v "$PWD"/envoy.yaml:/etc/envoy/envoy.yaml -v "$PWD"/KMamizEnvoyFilter.wasm:/etc/envoy/optimized.wasm -p 9901:9901 -p 10000:10000 envoyproxy/envoy:v1.17.0
```

5. Poke the endpoint at `http://localhost:10000` to see actions in the terminal.

```
curl -X POST localhost:10000 -H 'Content-Type: application/json' --data '{"id": "xxx", "token": "xxx"}'
```

```
curl -X POST localhost:10000 -H 'Content-Type: application/json' --data '{"fruits":{"a":"apple","b":"banana"},"colors":["red","green"],"obj":{"enabled":true,"count":123},"objects":[{"name":"a","num":1},{"name":"b","num":2},{"name":"c","num":3}],"weird":{"b":null}}'
```

```
curl -X POST localhost:10000 \
    -H "Content-Type: application/json" \
    -H "x-request-id: 669084db-e52d-9825-8d03-aab35afa6f4a" \
    -H "x-b3-traceid: dad62e0cb93a980cc6bba3d0762fefc8" \
    -H "x-b3-spanid: d40b8bb597882141" \
    -H "x-b3-parentspanid: c6bba3d0762fefc8" \
    --data '{"fruits":{"a":"apple","b":"banana"},"colors":["red","green"],"obj":{"enabled":true,"count":123},"objects":[{"name":"a","num":1},{"name":"b","num":2},{"name":"c","num":3}],"weird":{"b":null}}'
```
