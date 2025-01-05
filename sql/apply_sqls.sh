#!/bin/bash

set -e

echo "[INFO] Starting pgrx embedded process..."

DB_NAME="morpheme_funcs"
PORT="28813"
HOST="/home/vscode/.pgrx"

# カレントディレクトリ
CURRENT_DIR=$(pwd)
SQL_PATH="$CURRENT_DIR/"
INIT_FILE_PATH="${SQL_PATH}init.sql"
INSERTS_FILES_PATH="${SQL_PATH}inserts/"
UPDATE_FILE_PATH="${SQL_PATH}update_morpheme_array.sql"

echo "[INFO] inserts_files_path: $INSERTS_FILES_PATH"

# init.sql が存在するかチェック
if [[ ! -f "$INIT_FILE_PATH" ]]; then
    echo "[ERROR] init.sql not found: $INIT_FILE_PATH"
    exit 1
fi

# init.sql を最初に実行
psql -d "$DB_NAME" -f "$INIT_FILE_PATH" -p "$PORT" --host "$HOST"
echo "[INFO] init.sql を実行しました"

# inserts ディレクトリ内の SQL ファイルを取得してソート
SQL_FILES=$(find "$INSERTS_FILES_PATH" -type f -name "*.sql" | sort)

# その他の SQL ファイルを実行
echo "[INFO] Starting execution of SQL scripts in $INSERTS_FILES_PATH"
for SQL_FILE in $SQL_FILES; do
    echo "[INFO] Executing $SQL_FILE..."
    if psql -d "$DB_NAME" -p "$PORT" --host "$HOST" -f "$SQL_FILE"; then
        echo "[SUCCESS] $SQL_FILE executed successfully"
    else
        echo "[FAILED] $SQL_FILE failed to execute"
    fi
done

# update_morpehem_array.sql を実行
psql -d "$DB_NAME" -f "$UPDATE_FILE_PATH" -p "$PORT" --host "$HOST"

echo "[INFO] All scripts executed. Process completed."
