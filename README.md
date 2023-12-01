# Household_Account_Book
家計簿アプリ

# 使い方
docker-compose up -d //docker起動（DB立ち上げ）

<br>

migrate配下のSQL実行（マイグレーションが上手くいかないので、一旦手動）

# 構成
DDD採用している。<br>

以下ディレクトリ構成 <br>

・app <br>

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
