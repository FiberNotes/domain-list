# Domain List

## Использование

1. Скачать нужный конфиг.
```shell
FILE=alabama.dat && wget -O "/opt/etc/xray/dat/$FILE" "https://github.com/FiberNotes/domain-list/releases/latest/download/$FILE"  
```
2. Добавить правило для роутинга в Xray:

   ```json
   {
     "inboundTag": ["redirect", "tproxy"],
     "outboundTag": "vless-reality",
     "type": "field",
     "domain": ["ext:alabama.dat:alabama"]
   }
   ```

   Домен указывается в формате: \["ext:&lt;config_name&gt;.dat:&lt;config_name&gt;"\]

   config_name можно найти в [релизах](https://github.com/FiberNotes/domain-list/releases).
   

   Также в релизе есть `domains.dat` в котором собраны все домены из `domains`. Его можно использовать для точечной настройки уже имеющегося конфига:
   ```json
   {
     "inboundTag": ["redirect", "tproxy"],
     "outboundTag": "direct",
     "type": "field",
     "domain": ["ext:domains.dat:telegram"]
   },
   {
     "inboundTag": ["redirect", "tproxy"],
     "outboundTag": "vless-reality",
     "type": "field",
     "domain": ["ext:alabama.dat:alabama"]
   },
   ```

## Контрибьют

В папке \`aggregate_configs\` находятся конфиги, которые будут присутствовать в релизах.

В папке \`domains\` собранны домены по категориям и сервисам, которые можно использовать в своих конфигах.

Для расширения конфигов из \`aggregate_configs\` нужен апрув от владельца конфига. Для расширения или создания новых категорий и сервисов в \`domains\` нужно оталкиваться потребностей.

В \`domains\` запрещено делать импорт конфига из \`aggregate_configs\`.

### Добавление новых правил

Для импорта доменов или конфигов используется ключевое слово include: \`include:ai\`.

Для строго сравнения домена используется ключевое слово full: \`full:[x.com](http://x.com)\`.

Если ключевое слово full не используется, то в правила попадут все поддомены. Например, в \`[x.com](http://x.com)\` попадут все поддомены [x.com](http://x.com), [www.x.com](http://www.x.com), [cdn.x.com](http://cdn.x.com) и тд.

Комментарии пишутся через #: \`# This is comment\`

### Создание релиза

1. Для того чтобы создать релиз, нужно создать ветку с названием по паттерну: "v" + version. Например: "v0.0.0"
2. Описать свои изменения. Описание будет включено в релиз
3. Получить апрув и замержить ветку в \`main\`
