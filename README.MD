# Request Reflector

A quick http request reflector built in Rust to help troubleshoot reverse proxies.

## Use from source

```
cargo run
```

Then curl the server on port `3000`:

```
$ curl localhost:3000
<HTML>
<BODY>
  <P>Path: /</P>
  <P>Method: GET</P>
  <P>Request Headers</P>
  <P>host: localhost:3000</P>
  <P>user-agent: curl/8.5.0</P>
  <P>accept: */*</P>
  <P>Request body</P>
  <P></P>
</BODY>
</HTML>
```
