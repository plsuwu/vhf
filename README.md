# virtual host fuzzer

```
[please@ruby]:[~] $ vhf --help
Usage: vhf [OPTIONS] --url <URL> --ip <IP>

Options:
  -u, --url <URL>
  -i, --ip <IP>
  -w, --wordlist <WORDLIST>  [default: 1]
  -a, --agent <AGENT>        [default: 0]
  -t, --threads <THREADS>    [default: 25]
  -f, --filter
      --no-tls
  -h, --help                 Print help
  -V, --version              Print version
```

vaguely working but its a bit unattractive. 

## usage & testing

can be tested via the docker image in the `testing-server` directory. 
add your docker IP to your `/etc/hosts` file and then run the following commands:

```bash
# if uncloned
git clone https://github.com/plsuwu/vhost-enum
cd vhost-enum

# build and run the image with `compose.yaml`
docker compose up --detach

# the $DOCKER_IP here may or may not work, do a manual `ip a` if not lmao
DOCKER_IP="$(ip a | grep -ie 'docker*' | awk 'NR==2 { print $2 }' | awk -F/ '{ print $1 }')"
cargo run -- -i "$DOCKER_IP" -u vhf-test.vhf --no-tls
```

the output needs some work, but the above should indicate something like the following:

```bash
cargo run -- -i "$DOCKER_IP" -u vhf-test.vhf --no-tls

# ...

[*] Auto-transforming IP to target the server at 'http://172.17.0.1/'
heuristic content_length result: 19
{19965] => filtered response [200 OK] for 'driss.vhf-test.vhf'hf'vhf'vhf''f'hf'f''.vhf-test.vhf'
    "dev.vhf-test.vhf": 200,
}
```

... with the server reporting `200`s for everything:
```bash
# ...

vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         706ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         676ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         683ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         706ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         467ns |       127.0.0.1 | GET      "/test"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         697ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         745ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         594ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         686ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         596ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         712ns |       127.0.0.1 | GET      "/"
vhf-testing-1  | [GIN] 2024/08/28 - 20:44:03 | 200 |         474ns |       127.0.0.1 | GET      "/"

# ...
```

the `/test` endpoint is served via a separate NGINX server - requests for `dev.vhf-test.vhf` would resolve to this endpoint if it
were in the `/etc/hosts` file; we can get the same result from setting the `Host: dev.vhf-test.vhf` header here.
