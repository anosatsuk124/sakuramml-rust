/// Sutoton Mode Converter
use super::cursor::TokenCursor;
use super::token::zen2han;

#[derive(Debug)]
struct SutotonItem {
    name: String,
    value: String,
    length: usize,
}
impl SutotonItem {
    fn from(name: &str, value: &str) -> Self {
        Self {
            name: String::from(name),
            value: String::from(value),
            length: name.chars().count(),
        }
    }
}

fn init_items() -> Vec<SutotonItem> {
    let mut items: Vec<SutotonItem> = vec![];
    items.push(SutotonItem::from("トラック", "TR="));
    items.push(SutotonItem::from("音長", "l"));
    items.push(SutotonItem::from("音階", "o"));
    items.push(SutotonItem::from("音色", "@"));
    items.push(SutotonItem::from("音量", "v"));
    items.push(SutotonItem::from("ド", "c"));
    items.push(SutotonItem::from("レ", "d"));
    items.push(SutotonItem::from("ミ", "e"));
    items.push(SutotonItem::from("ファ", "f"));
    items.push(SutotonItem::from("フ", "f"));
    items.push(SutotonItem::from("ソ", "g"));
    items.push(SutotonItem::from("ラ", "a"));
    items.push(SutotonItem::from("シ", "b"));
    sort_items(&mut items);
    items
}
fn sort_items(items: &mut Vec<SutotonItem>) {
    items.sort_by(|a, b| b.name.len().cmp(&a.name.len()));
}

pub fn convert(src: &str) -> String {
    let mut items = init_items();
    let mut res = String::new();
    let mut cur = TokenCursor::from(src);
    while !cur.is_eos() {
        let ch = zen2han(cur.peek_n(0));
        // string ?
        match ch {
            '{' => {
                let s = cur.get_token_nest('{', '}');
                res.push_str(&s);
                continue;
            },
            '\u{0020}'..='\u{007D}' => {
                res.push(ch);
                cur.index += 1;
                continue;
            },
            // add item
            '~' => {
                cur.index += 1;
                cur.skip_space();
                if cur.peek_n(0) != '{' { continue; }
                let name = cur.get_token_nest('{', '}');
                cur.skip_space();
                let ch = cur.next().unwrap_or('\0');
                if ch != '=' { continue; }
                cur.skip_space();
                if cur.peek_n(0) != '{' { continue; }
                let value = cur.get_token_nest('{', '}');
                items.push(SutotonItem::from(&name, &value));
                sort_items(&mut items);
                continue;
            }
            _ => {
            }
        }
        // check sutoton
        let mut found = false;
        for cmd in items.iter() {
            if cur.eq(&cmd.name) {
                res.push_str(&cmd.value);
                cur.index += cmd.length;
                found = true;
                break;
            }
        }
        if !found {
            res.push(ch);
            cur.index += 1;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(convert("ドレミ"), String::from("cde"));
        assert_eq!(convert("トラック3ドレミ"), String::from("TR=3cde"));
    }
    #[test]
    fn test_ex() {
        assert_eq!(convert("~{ど}={c}ドレミどレミ"), String::from("cdecde"));
        assert_eq!(convert("~{じゅー}={c}ドレミじゅーレミ"), String::from("cdecde"));
    }
}

