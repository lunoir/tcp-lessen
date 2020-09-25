# 簡単な独自プロトコルを設計し、rustでtcpサーバを作成して、Wiresharkにluaスクリプトを読み込ませて独自プロトコルを解析する

## 簡単な独自プロトコル

| 0 - 7 | コマンドID 1 |  
| 8 - 15| コマンドID 2 |  
|16 - ..| データ       |  

### サーバ側動作

#### コマンドID 1

`0x30`: 固定文字列を返す。コマンドID 2の値によって返す文字列が変わる  
`0x30`以外:  受信データをechoする  

#### コマンドID 2

`0x30`: `30 30 6f 72 65 67 61 20 67 61 6e 64 61 6d 75 20 64 61 21 OD OA` を返す（`orega gundam da!`）  
`0x30`以外: `30 2D 68 65 6c 6c 6f 20 77 6f 72 6c 64 21 0D 0A` を返す（`hello world!`）  

### クライアント側動作

入力データをそのまま送信し、受信データをそのまま表示する。  
コマンドIDによって動作が変更になることはない。  
telnetで動作確認可能。  

## rustでtcpサーバ

`tcpserver`ディレクトリの下にコード一式がある。  

### サーバ実行方法

```sh
$ cargo run
```

### telnet接続方法

```sh
$ telnet 127.0.0.1 37564
```

## Wiresharkにluaスクリプトを読み込ませて独自プロトコルを解析する

`dissector`ディレクトリの下にluaスクリプトがある。  

### インストール方法

1. Wiresharkを起動し、メニューから`Wireshark > About Wireshark`を選択してAbout画面を表示する
1. About画面の`Folders`を選択し、`Personal Lua Plugins`をダブルクリックすると、ディレクトリが存在していれば開き、存在していなければ作成するかどうか聞いてくるので作成する
1. `Personal Lua Plugins`ディレクトリの中にluaスクリプトをコピーする
1. Wiresharkのメニューから`Analyze > Reload Lua Plugins`を実行

### 解析する

Wiresharkを起動しておき、port `37564` をキャプチャする。  
tcpサーバを起動し、telnetで通信する。  

```telnet
01hoge
0-hello world!
00hoge
00orega gundam da!
11hoge
11hoge
```

すると、`Protocol`欄に`LESSEN`と出てくる通信がある。  
それを選択すると、`Lessen Protocol`という項目が解析欄に表示され、開くと解析結果が表示されている。  
