# Blue

## Run Genesis Core Node

```bash
./blue {port}
```

```bash
$ cargo run 7878
Bootstrapping as a GENESIS Core Node...
Initializing core node...
Initializing connection manager...
Listening on 127.0.0.1:7878
```

## Run Core Node

```bash
./blue {port} {port num of genesis core node}
```

```bash
$ cargo run 17879 7878
Bootstrapping as a Core Node...
Initializing core node...
Sending message: Message { type: Add, source_port: "17879" }
Successfully connected to the node: Node("127.0.0.1", "7878")
```
