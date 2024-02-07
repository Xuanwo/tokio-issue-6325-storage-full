# Tokio issue 6325 storage full

## Setup

```shell
fallocate -l 512K disk.img
mkfs disk.img

mkdir /tmp/test_dir
sudo mount -o loop disk.img /tmp/test_dir

sudo chmod a+wr /tmp/test_dir
```

## Run

```shell
cargo run
```
