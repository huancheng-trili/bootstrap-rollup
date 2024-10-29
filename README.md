```sh
$ docker run -it --rm --volume $(pwd):/home/tezos/hello-world-kernel --entrypoint /bin/sh --name octez-container tezos/tezos:master
```

Inside the container:
```sh
$ cd /home/tezos/hello-world-kernel
$ ./sandbox_node.sh &
$ sleep 15
$ ./init_rollup.sh
```