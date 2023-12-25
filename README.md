# Household_Account_Book
家計簿アプリ

# 使い方
docker-compose up -d //docker起動（DB立ち上げ）

<br>

・DB準備
1. migrate配下のV0.0.0__create_db.sqlを実行し、DBを作成する。
2. migrate配下で、
```zsh
DATABASE_URL="mysql://<ユーザ名>:<パスワード>@localhost:3306/household_account" sea-orm-cli migrate refresh
```
を実行し、マイグレーションを実行する。（sea-orm-cliがインストールされている前提）



# 構成
DDD採用している。<br>

以下ディレクトリ構成 <br>

**・app **  <br>

mainアプリを格納（ルーティング含む）

・presentation <br>

APIのI/F定義<br>

機能別にフォルダ切っている。

・domain <br>

ドメインオブジェクトやドメインサービスを格納 <br>
ドメイン別にフォルダ切っている。<br>

・usecase <br>

機能の振る舞いを定義する。 <br>

ドメインオブジェクトを直接扱わず、DTOを経由して公開可能なフィールドのみを扱う。 <br>
サービス別にフォルダを切っている。 <br>

・infra <br>

DBとのやり取りを定義 <br>
サービス別にフォルダを切っている。 <br>

・migration
Migration用フォルダ
m始まりのファイルでテーブル定義を書く。
同ディレクトリのlib.rsにファイル名を追加することで、Migration対象に入れることができる。

SQL上でバージョン確認するには、seaql_migrationsテーブルを確認すればOK！
