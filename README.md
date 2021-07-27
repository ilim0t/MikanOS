# Mikan-OS

[ゼロからの OS 自作入門](http://zero.osdev.jp/)を実装したリポジトリ

## ファイル構成

```txt=
MikanOS
├── README.md           : 今見ているファイル
├── bootloader/         : bootloader (C)
├── ckernel/            : kernel (C++)
├── dev/                : 開発者用ツール
├── docker-compose.yml  : ビルド&起動 コマンド用
├── rkernel/            : kernel (Rust)
└── tools/              : コードに組み込む外部データ作成用
```

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
