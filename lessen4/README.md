# rustでechoサーバを動かしてみる

rustでechoサーバを動かしてみます。
echoサーバは下記2つを使用します。

## スレッドを使用したechoサーバ

https://github.com/teru01/socket-programming

cloneしたrepositoryで下記コマンドを実行してください。

```
$ cargo run tcp server 127.0.0.1:33333
```

このサーバに接続するには下記コマンドを実行してください。

```
$ telnet 127.0.0.1 33333
```

## epollを使用したechoサーバ

https://github.com/mmisono/aa_echo
http://mmi.hatenablog.com/entry/2019/09/29/203156

こっちはlinux上でビルドして動かす必要があるので、lessen4ディレクトリにある `Dockerfile` と `docker-compose.yml` を利用してください。

実行するにはcloneしたrepositoryに `Dockerfile` と `docker-compose.yml` をコピーして下記コマンドを実行してください。

```
$ docker-compose up --build
```

コードをいじらない場合はローカルからしか接続できないので、下記コマンドでコンテナに入ってください。

```
$ docker exec -it aa_echo_app_1 /bin/sh
```

その後にtelnetコマンドを実行してください。

```
# telnet 127.0.0.1 8080
```

