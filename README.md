# thesamo
This is improved version if [thesamo]() rewritten in Rust.

## About
This tool allows to syncronize parts of the config files across remote servers. Enclose the part of config that needs to be syncronised in a "tag" and the tool will ensure that the parts of the files are similar across the machines.

## Instalation

TBD


## Configuration

``` toml

master = true
minion = false

open_tag = "%%>"
close_tag = "<%%"

[network]
bind_port = 12345
bind_address = "127.0.0.1"

[[files]]
path = "./tests/file_one.txt"
minion_address = "127.0.0.1"
minion_port = 12345

[[files]]
path = "./tests/file_two.txt"
minion_address = "127.0.0.1"
minion_port = 12345

```

Then in the file one wishes to syncronise just mark the block with the specified tags:

``` ini

some part of config specific to this machine

%%>
Part of config to be syncronised with other servers
<%%

Rest of the file 
```
## Usage

On master:

``` shell
# master -c <path/to/config/file>
```

On minion:

``` shell
# minion -c <path/to/config/file>
```


