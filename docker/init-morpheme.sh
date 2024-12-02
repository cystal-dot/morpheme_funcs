#!/bin/bash

# exec gosu postgres bash -c "
# cd /morpheme_funcs && \
# cargo pgrx package && \
# cp target/release/morpheme_funcs-pg14/usr/lib/postgresql/14/lib/morpheme_funcs.so /usr/lib/postgresql/14/lib/ && \
# cp target/release/morpheme_funcs-pg14/usr/share/postgresql/14/extension/morpheme_funcs* /usr/share/postgresql/14/extension/ && \
# psql -d product -c \"drop extension if exists morpheme_funcs cascade; create extension morpheme_funcs;\" && \
# psql -d product -c \"UPDATE product SET morpheme_array = (SELECT to_morpheme_array(name));\""]

cd /morpheme_funcs && \
cargo pgrx package && \
cp target/release/morpheme_funcs-pg14/usr/lib/postgresql/14/lib/morpheme_funcs.so /usr/lib/postgresql/14/lib/ && \
cp target/release/morpheme_funcs-pg14/usr/share/postgresql/14/extension/morpheme_funcs* /usr/share/postgresql/14/extension/ && \
psql -d product -c "drop extension if exists morpheme_funcs cascade; create extension morpheme_funcs;" && \
psql -d product -c "ALTER TABLE product ADD COLUMN IF NOT EXISTS morpheme_array TEXT[];" && \
psql -d product -c "UPDATE product SET morpheme_array = (SELECT to_morpheme_array(name));"