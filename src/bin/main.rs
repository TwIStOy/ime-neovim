use ime::data::trie::Trie;

fn main() {
  let mut tr = Trie::<String>::create();
  tr.insert("aaa".to_string(), "1".to_string());
  tr.insert("aab".to_string(), "1".to_string());
  tr.insert("aac".to_string(), "1".to_string());
  tr.insert("aad".to_string(), "1".to_string());
  tr.insert("aad".to_string(), "2".to_string());
  tr.insert("aad".to_string(), "啊".to_string());

  for line in tr.print_tree() {
    println!("{}", line);
  }
}
