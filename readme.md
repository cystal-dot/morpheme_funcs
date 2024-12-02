```
cd docker
docker compose up -d
docker compose exec -u postgres postgres bash
```

テスト
```
cargo pgrx run pg14
drop extension if exists morpheme_funcs cascade;
create extension morpheme_funcs;
```

DBクラスタを読み込んでの実行
```
psql -d product
select calculate_morpheme_score('大ねじ小ねじ','ねじ');
```

