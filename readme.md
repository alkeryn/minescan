# Minescan, a minecraft server scanner cli tool and library written for extensibility

You can look at src/main.rs on how to add your own commands.

you can change how the results are handled (for example storing them in a database
instead of just printing them) by implementing the Writer trait and passing it to one of
the public functions in src/cli.rs.

You can change where the list of ip is coming from by implementing the Reader trait for your type
or running run_stream with your custom stream directly.

You should use a tool like zmap to get a list of all ip with the default port open.

You may want to use `ulimit -n` if you want a good amount of concurrency, or you might get the error :

```
Too many open files (os error 24)
```

I suggest doing a first run with -v to be sure everything is going correctly as it won't print errors by default.

## Cli example :

```bash
ulimit -n 32000 # to be able to open enough connections at once
zmap -p 25565 2>/dev/null | ./minescan -c 10000 -t 1000 > results

./minescan <minecraft server IP>
./minescan <minecraft server IP>:<custom port>

./minescan -c 10000 -t 500 < ./list_of_minecraft_servers_ip
```

```
minescan 0.9.4
@alkeryn
Minescan, a minecraft server scanner cli tool and library written for extensibility

USAGE:
    minescan [OPTIONS] [ADDRESS]

ARGS:
    <ADDRESS>    Address

OPTIONS:
    -c, --concurency <CONCURENCY>    concurency [default: 100]
    -h, --help                       Print help information
    -t, --timeout <TIMEOUT>          timeout in ms [default: 500]
    -v, --verbose                    verbose
    -V, --version                    Print version information
```

This software is used by the for now private minescan-prod package, which adds and update the results to a database and
expose it through a web frontend with an api, comming soon.
