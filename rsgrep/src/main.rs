use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
extern crate regex;
use regex::Regex;

fn usage() {
    println!("rsgrep PATTERN FILENAME")
}


fn main() {
    // 引数からパターンを取り出す
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };
    // 取り出したパターンから`Regex`をあらためて作る
    // 無効な正規表現だった場合などにはエラーが返る
    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("invalid regexp {}: {}", pattern, e);
            return;
        }
    };

    // envモジュールのargs関数でプログラムの引数を取得．
    // そのうち2番目を`nth`で取得（0番目はプログラム名，1番目はパターン）
    // 引数があるかわからないのでOptionで返す
    let filename = match env::args().nth(2) {
        // あれば取り出す
        Some(filename) => filename,
        // なければヘルプを表示して終了
        None => {
            usage();
            return;
        }
    };

    // `File`構造体の`open`関連関数でファイルを開く．
    // 失敗する可能性があるので結果は`Result`で返す．
    // 下のほうでもう一度`filename`を使うので，`&filename`と参照渡しする．
    let file = match File::open(&filename) {
        // 成功すれば取り出す．
        Ok(file) => file,
        // エラーの場合はプログラム終了
        Err(e) => {
            println!("An error occured while opening file {}: {}",filename, e);
            return;
        }
    };

    // Fileをそのまま使うと遅いのと`lines`メソッドを使うために`BufReader`に包む．
    // この`new`もただの関連関数．
    let input = BufReader::new(file);
    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error occured while reading a line {}", e);
                return;
            }
        };
        if reg.is_match(&line) {
            println!("{}", line);
        }
    }
}

