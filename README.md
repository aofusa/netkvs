NetKVS
=====


以下の機能を持つネットワークKeyValueストア
- 1024バイト以下の文字列の格納と取得


使い方
-----
```sh
# no tls
cargo run

# with tls
cargo run --features tls
```

サーバ証明書の作成
```sh
openssl req -new -newkey ed25519 -nodes -text -out server.csr -keyout server.key -subj /CN=localhost
openssl x509 -req -in server.csr -text -days 365 -extfile /dev/stdin -extensions v3_ca -signkey server.key -out server.crt <<EOF
[ v3_ca ]
subjectAltName = DNS:localhost, DNS:*.local
EOF
```

別タブでtelnetを実行
```sh
# TLSなしの場合
telnet localhost 11211

# TLSありの場合
openssl s_client -connect localhost:11211
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

