// ★使っていないファイル★
// buildしてDBの起動前にrsファイルを実行する手段が分からないため

use std::env;
use std::fs;
use std::path;
use std::process::Command;

fn main() {
    println!("[INFO] Starting pgrx embedded process...");

    // PostgreSQL 拡張の初期化
    pgrx_initialize();

    // SQL スクリプトをロードして適用
    apply_sql_scripts();
}

/// PostgreSQL の初期化処理
fn pgrx_initialize() {
    println!("[INFO] Initializing pgrx extension...");
    // 必要に応じて初期化処理を追加できます
}

/// SQL スクリプトを適用する
fn apply_sql_scripts() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let sql_path = format!("{}{}", current_dir.display(), "/");
    let init_file_path = format!("{}{}", sql_path, "init.sql");
    let inserts_files_path = format!("{}{}", sql_path, "inserts/");

    println!("[INFO] inserts_files_path: {}", inserts_files_path);

    let dir = fs::read_dir(&inserts_files_path).expect("Failed to read directory");
    let mut other_sqls: Vec<path::PathBuf> = Vec::new();
    for item in dir {
        other_sqls.push(item.unwrap().path());
    }
    other_sqls.sort(); // ファイルをソートして実行順を保証

    println!("[INFO] init_file_path: {}", init_file_path);
    if !path::Path::new(&init_file_path).exists() {
        panic!("init.sql not found: {}", init_file_path);
    }

    // init.sql を最初に実行
    execute_sql(&init_file_path);
    println!("[INFO] init.sql を実行しました");

    // その他の SQL ファイルを実行
    let mut results = Vec::new();
    for insert_sql in other_sqls {
        let result = execute_sql(insert_sql.to_str().unwrap());
        results.push((insert_sql.to_str().unwrap().to_string(), result));
    }

    // 実行結果を出力
    println!("\nExecution Summary:");
    for (file, success) in results {
        if success {
            println!("[SUCCESS] {}", file);
        } else {
            println!("[FAILED] {}", file);
        }
    }
}

/// SQL スクリプトを実行する
fn execute_sql(file_path: &str) -> bool {
    let output = Command::new("psql")
        .arg("-d")
        .arg("morpheme_funcs") // 必要に応じて変更
        .arg("-f")
        .arg(file_path)
        .output()
        .expect("Failed to execute SQL script");

    if !output.status.success() {
        eprintln!("[ERROR] Failed to execute SQL script: {}", file_path);
        eprintln!("[DETAILS] {}", String::from_utf8_lossy(&output.stderr));
        false
    } else {
        println!("[INFO] Successfully executed SQL script: {}", file_path);
        true
    }
}
