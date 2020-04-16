use ime::data::trie::Trie;
use ime::engine::keymap::KeyMap;

fn main() {
  {
    let mut tr = Trie::<char, String>::new();
    tr.insert("aaa".chars().collect::<Vec<char>>().iter(), "1".to_string());
    tr.insert("aab".chars().collect::<Vec<char>>().iter(), "1".to_string());
    tr.insert("aac".chars().collect::<Vec<char>>().iter(), "1".to_string());
    tr.insert("aad".chars().collect::<Vec<char>>().iter(), "1".to_string());
    tr.insert("aad".chars().collect::<Vec<char>>().iter(), "2".to_string());
    tr.insert(
      "aad".chars().collect::<Vec<char>>().iter(),
      "啊".to_string(),
    );

    for line in tr.print_tree() {
      println!("{}", line);
    }
  }

  {
    let mut tr = Trie::<String, String>::new();
    let mut tmp: Vec<String> = Vec::new();
    tmp.push("fuck".to_string());
    tmp.push("shit".to_string());
    tr.insert(tmp.iter(), "1".to_string());

    for line in tr.print_tree() {
      println!("{}", line);
    }
  }

  println!("{:?}", KeyMap::available_keymaps());
  println!("{:?}", KeyMap::load("小鹤双拼.plist"));
}
