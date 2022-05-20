## Roco
A simple tool to maintain(retry) connections. 

## Usage
```sh
/bin/roco <action that needs to be retried>
```

## Development
After making your changes, run `make release` to create a new release.

## Some Notes
- It's dead simple.
- Built to maintain kubernetes portforwarding operations but it's feeding off unix apis, so it will probably work for any unix command / unixy program.
- As you can see, i'm learning rust, don't judge my code.
- Open to code reviews.