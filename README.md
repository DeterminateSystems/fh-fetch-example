# `fh fetch` example

Load a Docker image built with Nix no matter which system you're on:

```shell
SYSTEM=$(nix eval --impure --expr 'builtins.currentSystem' --raw)
fh fetch "DeterminateSystems/fh-fetch-example/*#dockerImages.${SYSTEM}.server" ./server-img
docker load < ./server-img
```
