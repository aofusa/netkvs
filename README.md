NetKVS
=====


以下の機能を持つネットワークKeyValueストア
- 1024バイト以下の文字列の格納と取得


使い方
-----
```sh
cargo run
```

別タブでtelnetを実行
```sh
telnet localhost 11211
```

コマンド
- set <key\> <value\>

  <key\>に<value\>を設定。既に存在する場合は上書き

- get <key\>

  <key\>の値を取得。存在しない場合は空文字列が返る

- quit
  接続の終了


例
```sh
set key 123
STORED

get key
123
END

quit
QUIT
```

