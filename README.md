# CSV の指定した列だけにフィルターする

## 注意

Rust 勉強中の自習課題です。

クローン、使用は自己責任でお願い致します。

## 仕様

まずは素朴な実装から考える。

- コマンドライン引数は全部条件指定に使ってしまう。
- 入力は標準入力からのみ
- 出力は標準出力のみ

sample : tests/members.csv
```csv
id,name,age
10-1,John,30
10-2,Ken,33
15-0,Bob,18
n-09,Jen,48
```

引数にてフィールド名(カラム名)(完全一致)を指定できる
```sh
<tests/members.csv cargo run -- id name >tests/expect-id-name.csv
```
このコマンドラインの出力は下記のようになる。

sample : tests/expect-id-name.csv
```csv
id,name
10-1,John
10-2,Ken
15-0,Bob
n-09,Jen
```

引数で指定した順番を守る。
