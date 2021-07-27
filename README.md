# Mikan-OS

## Requirements

- X Server([XQuartz](https://www.xquartz.org/)など)
- [Docker Compose](https://docs.docker.com/compose/install/#install-compose)

> X Server に XQuartz を使う場合は、環境設定の「ネットワーク・クライアントからの接続を許可」のチェックボックスをオンにしてください。

## The kernel written in C

### Build

```sh=
docker-compose up build_efi build_ckernel
```

### Run

```sh=
docker-compose up run_qemu
```

If you are using XQuartz, you will need to set the DISPLAY variable as follows.

```sh=
DISPLAY=$(hostname):0 docker-compose up run_cqemu
```

## The kernel written in Rust

## Build

```sh=
docker-compose up build_efi build_rkernel
```

## Run

```sh=
docker-compose up run_rqemu
```

If you are using XQuartz, you will need to set the DISPLAY variable as follows.

```sh=
DISPLAY=$(hostname):0 docker-compose up run_qemu
```
