## Roco
Some software that automatically heals broken tcp connections. 

## Inspiration
In 2022, I used to work for a company running over 100 microservices. With that kind of deployment, you had to do alot of k8s port-forwarding in order to test services locally.  

Thing is, the internet I had at home wasn't very good and would break from time to time, in-turn breaking my connections. So I wrote roco to automatically retry commands, so I could deal with other pains of building microservices.


## Usage
```sh
/bin/roco <action that needs to be retried>
```

## Development
After making your changes, run `make release` to create a new release.

## Some Notes
- It's dead simple.
- It's feeding off unix apis, so it will probably work for any unix command / unixy program.