# guccicci

## 概要

グループワークなどのチーム分けを行うCUIアプリケーションです。

## 使い方

```bash
guccicci ${PATH_TO_SETTING_TOML}
```

## 設定値

`example.setting.toml`を参照

|設定値|型|サンプル値|説明|
|--|--|--|--|
|num_of_teams|u8|4|チーム数(必須)|
|flat|bool|false|trueに設定するとattendees.leaderの値を無視して全員がリーダー候補となる(任意・デフォルトはfalse)|
|[[attendees]]|Vec<attendee>|-|出席者のリスト(必須)|
|attendess.leader|bool|false|出席者がリーダーになるかどうか(任意・デフォルトfalse) リーダーの数は最低限num_of_teamsの数だけ必要|
|[attendees.parson]|parson|-|出席者情報(必須)|
|attendees.parson.name|string|Taro|出席者名(必須)|

## 出力値

*サンプル*
```toml
[[team]]
[team.leader]
name = 'Lisa'

[[team.member]]
name = 'Beth'

[[team.member]]
name = 'Yoko'

[[team]]
[team.leader]
name = 'Mike'

[[team.member]]
name = 'John'

[[team.member]]
name = 'Takashi'
```


