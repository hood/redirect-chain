# redirect-chain

A simple command-line tool to trace HTTP redirect chains for a given URL.

## Usage

```sh
redirect-chain <url>
```

## Example

```sh
$ redirect-chain http://example.com
Tracing redirects for: http://1.com
1: 301 Moved Permanently -> https://1.com
2: 302 Found -> https://2.com
3: 200 OK -> https://3.com
```
