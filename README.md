# Household_Account_Book
家計簿アプリ

# 使い方
docker-compose up -d //docker起動（DB立ち上げ）

<br>

migrate配下のSQL実行（マイグレーションが上手くいかないので、一旦手動）

# 構成
DDD採用している。

以下ディレクトリ構成

・app
mainアプリを格納（ルーティング含む）

・presentation
APIのI/F定義
機能別にフォルダ切っている。

・domain
ドメインオブジェクトやドメインサービスを格納
ドメイン別にフォルダ切っている。

・usecase
機能の振る舞いを定義する。
ドメインオブジェクトを直接扱わず、DTOを経由して公開可能なフィールドのみを扱う。
サービス別にフォルダを切っている。

・infra
DBとのやり取りを定義
サービス別にフォルダを切っている。
