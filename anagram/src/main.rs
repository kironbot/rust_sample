use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn sorted_string(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    s.into_iter().collect::<String>()
}

struct Anagram(HashMap<String, Vec<String>>);

impl Anagram {
    fn new<P: AsRef<Path>>(dictfile: P) -> Result<Self, io::Error> {
        let file = File::open(dictfile)?;
        let file = io::BufReader::new(file);

        // ハッシュマップを準備しておく
        let mut anagram = Anagram(HashMap::new());
        for line in file.lines() {
            let word = line?;
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    fn add_word(&mut self, word: String) {
        // 単語をアルファベット順にソートしたものを作ってキーにする
        let sorted = sorted_string(&word);

        // キーに対応する値があればそれを，なければ新たにデフォルト値を入れる
        // 返り値はキーに対応する値
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    // 検索はRead Onlyなので`&self`を使う
    // キーはRead Onlyなので`word`も参照で受け取る
    fn find(&self, word: &str) -> Option<&Vec<String>> {
        let word = sorted_string(word);
        self.0.get(&word)
    }
}

fn main() {
    // 実行時にコマンドライン引数として単語を受け取る
    let word = std::env::args().nth(1).expect("USAGE: word");
    //辞書からAnagram構造体を作る
    // 多くのUnix環境ではこのパスに辞書がある
    let table = Anagram::new("/usr/share/dict/words").expect("failed to make table");

    println!("{:?}", table.find(&word));
}
