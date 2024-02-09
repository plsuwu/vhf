# vhost-enum

simple multithreaded cli tool to enumerate for virtual hosts/subdomains.

## usage

build:

```
$ cargo build
   Compiling vhfuzz v0.0.1 (/home/please/git/plsuwu-git/vhost-enum)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s

$ la target/release
# ...
-rwxr-xr-x 2 please users 4.6M Feb 10 04:18 target/release/vhfuzz
```

**run**

requires
- `--url`/`-u`
    - second-level domain to prepend subdomains to (e.g `google.com`)
        - passed to the `Host` header in an HTTP request (`Host: <subdomain>.google.com`)
- `--ip`/`-i`
    - IP to pass in as the host URL
        - this is how the `Host` header is resolved; kind of acts as the DNS to resolve the `Host` header.
- `--wordlist`/`-w`
    - a list of subdomains
- `--concurrency`/`-c` (OPTIONAL)
    - threads to run with (defaults to `50`).

```
$ vhfuzz --help
Usage: vhfuzz [OPTIONS] --url <URL> --ip <IP> --wordlist <WORDLIST>

Options:
  -u, --url <URL>
  -i, --ip <IP>
  -w, --wordlist <WORDLIST>
  -c, --concurrency <CONCURRENCY>  [default: 50]
  -h, --help                       Print help
  -V, --version                    Print version

# example:
$ vhfuzz --url devvortex.htb --ip 10.10.11.242 -w SecLists/Discovery/DNS/subdomains-top1million-5000.txt
```
