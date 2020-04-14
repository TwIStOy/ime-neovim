use ime::data::trie::Trie;

fn main() {
    let mut tr = Trie::<u8>::create();
    tr.insert("abc".to_string(), 1);
    print!("{:#?}", tr);
}
