#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate yoin;
use yoin::{ipadic, tokenizer};
use std::collections::HashMap;



use tauri::{CustomMenuItem, Menu, Submenu, MenuItem};

fn main() {
  let context = tauri::generate_context!();

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![jpkana])
    .invoke_handler(tauri::generate_handler![jproma])
    .run(context)
    .expect("Error While Running");
}


#[tauri::command]
fn greet(name: &str) -> String {
  println!("Hello, {}!", name);
  format!("Hello, {}!", name)
}

#[tauri::command]
fn jpkana(text: &str) -> String {
  let mut s = JSentence::new(text);
  s.kana_word_ruby()
}

#[tauri::command]
fn jproma(text: &str) -> String {
  if text.is_empty() {
    "".to_string()
  } else {
    let mut s = JSentence::new(text);
    s.roma_word_ruby()
  }

}

trait Sentence {
  fn new(text: &str) -> Self;
  fn is_ruby_empty(&self) -> bool;
  fn show_ori(&self) -> String;
  fn show_ruby(&self) -> String;
}

struct JSentence<'a> {
  ori_text: String,
  ruby_text: String,
  kata_to_hira: HashMap<char, char>,
  kata_to_roma: HashMap<char, &'a str>,
  hira_to_roma: HashMap<char, &'a str>,
  tokenizer: tokenizer::Tokenizer<'a>,
}

struct KSenetnce {
  ori_text: String,
  ruby_text: String,
}

struct CSentence {
  ori_text: String,
  ruby_text: String,
}

impl Sentence for JSentence<'_> {
  fn new(text: &str) -> Self {
      let mut s = JSentence {
          ori_text: String::new(),
          ruby_text: String::new(),
          kata_to_hira: HashMap::from([
              ('ア', 'あ'), ('イ', 'い'), ('ウ', 'う'), ('エ', 'え'), ('オ', 'お'),
              ('カ', 'か'), ('キ', 'き'), ('ク', 'く'), ('ケ', 'け'), ('コ', 'こ'),
              ('サ', 'さ'), ('シ', 'し'), ('ス', 'す'), ('セ', 'せ'), ('ソ', 'そ'),
              ('タ', 'た'), ('チ', 'ち'), ('ツ', 'つ'), ('テ', 'て'), ('ト', 'と'),
              ('ナ', 'な'), ('ニ', 'に'), ('ヌ', 'ぬ'), ('ネ', 'ね'), ('ノ', 'の'),
              ('ハ', 'は'), ('ヒ', 'ひ'), ('フ', 'ふ'), ('ヘ', 'へ'), ('ホ', 'ほ'),
              ('マ', 'ま'), ('ミ', 'み'), ('ム', 'む'), ('メ', 'め'), ('モ', 'も'),
              ('ラ', 'ら'), ('リ', 'り'), ('ル', 'る'), ('レ', 'れ'), ('ロ', 'ろ'),
              ('ワ', 'わ'), ('ヰ', 'ゐ'), ('ヲ', 'を'), ('ン', 'ん'),
              ('ヤ', 'や'), ('ユ', 'ゆ'), ('ヨ', 'よ'),
              ('ガ', 'が'), ('ギ', 'ぎ'), ('グ', 'ぐ'), ('ゲ', 'げ'), ('ゴ', 'ご'),
              ('ザ', 'ざ'), ('ジ', 'じ'), ('ズ', 'ず'), ('ゼ', 'ぜ'), ('ゾ', 'ぞ'),
              ('ダ', 'だ'), ('ヂ', 'ぢ'), ('ヅ', 'づ'), ('デ', 'で'), ('ド', 'ど'),
              ('バ', 'ば'), ('ビ', 'び'), ('ブ', 'ぶ'), ('ベ', 'べ'), ('ボ', 'ぼ'),
              ('パ', 'ぱ'), ('ピ', 'ぴ'), ('プ', 'ぷ'), ('ペ', 'ぺ'), ('ポ', 'ぽ'),
              ('ァ', 'ぁ'), ('ィ', 'ぃ'), ('ゥ', 'ぅ'), ('ェ', 'ぇ'), ('ォ', 'ぉ'),
              ('ャ', 'ゃ'), ('ュ', 'ゅ'), ('ョ', 'ょ'),
              ('ッ', 'っ'),
          ]),
          kata_to_roma: HashMap::from([
              ('ア', "a"), ('イ', "i"), ('ウ', "u"), ('エ', "e"), ('オ', "o"),
              ('カ', "ka"), ('キ', "ki"), ('ク', "ku"), ('ケ', "ke"),('コ', "ko"),
              ('サ', "sa"), ('シ', "shi"), ('ス', "su"), ('セ', "se"), ('ソ', "so"),
              ('タ', "ta"), ('チ', "chi"), ('ツ', "tsu"), ('テ', "te"), ('ト', "to"),
              ('ナ', "na"), ('ニ', "ni"), ('ヌ', "nu"), ('ネ', "ne"), ('ノ', "no"),
              ('ハ', "ha"), ('ヒ', "hi"), ('フ', "fu"), ('ヘ', "he"), ('ホ', "ho"),
              ('マ', "ma"), ('ミ', "mi"), ('ム', "mu"), ('メ', "me"), ('モ', "mo"),
              ('ラ', "ra"), ('リ', "ri"), ('ル', "ru"), ('レ', "re"), ('ロ', "ro"),
              ('ワ', "wa"), ('ヰ', "wi"), ('ヲ', "wo"), ('ン', "n"),
              ('ヤ', "ya"), ('ユ', "yu"), ('ヨ', "yo"),
              ('ガ', "ga"), ('ギ', "gi"), ('グ', "gu"), ('ゲ', "ge"), ('ゴ', "go"),
              ('ザ', "za"), ('ジ', "ji"), ('ズ', "zu"), ('ゼ', "ze"), ('ゾ', "zo"),
              ('ダ', "da"), ('ヂ', "ji"), ('ヅ', "zu"), ('デ', "de"), ('ド', "do"),
              ('バ', "ba"), ('ビ', "bi"), ('ブ', "bu"), ('ベ', "be"), ('ボ', "bo"),
              ('パ', "pa"), ('ピ', "pi"), ('プ', "pu"), ('ペ', "pe"), ('ポ', "po"),
              ('ァ', "wa"), ('ィ', "wi"), ('ゥ', "wu"), ('ェ', "we"), ('ォ', "wo"),
              ('ャ', "ya"), ('ュ', "yu"), ('ョ', "yo"),
              ('ッ', "-"),
          ]),
          hira_to_roma: HashMap::from([
              ('あ', "a"), ('い', "i"), ('う', "u"), ('え', "e"), ('お', "o"),
              ('か', "ka"), ('き', "ki"), ('く', "ku"), ('け', "ke"),('こ', "ko"),
              ('さ', "sa"), ('し', "shi"), ('す', "su"), ('せ', "se"), ('そ', "so"),
              ('た', "ta"), ('ち', "chi"), ('つ', "tsu"), ('て', "te"), ('と', "to"),
              ('な', "na"), ('に', "ni"), ('ぬ', "nu"), ('ね', "ne"), ('の', "no"),
              ('は', "ha"), ('ひ', "hi"), ('ふ', "fu"), ('へ', "he"), ('ほ', "ho"),
              ('ま', "ma"), ('み', "mi"), ('む', "mu"), ('め', "me"), ('も', "mo"),
              ('ら', "ra"), ('り', "ri"), ('る', "ru"), ('れ', "re"), ('ろ', "ro"),
              ('わ', "wa"), ('ゐ', "wi"), ('を', "wo"), ('ん', "n"),
              ('や', "ya"), ('ゆ', "yu"), ('よ', "yo"),
              ('が', "ga"), ('ぎ', "gi"), ('ぐ', "gu"), ('げ', "ge"), ('ご', "go"),
              ('ざ', "za"), ('じ', "ji"), ('ず', "zu"), ('ぜ', "ze"), ('ぞ', "zo"),
              ('だ', "da"), ('ぢ', "ji"), ('づ', "zu"), ('で', "de"), ('ど', "do"),
              ('ば', "ba"), ('び', "bi"), ('ぶ', "bu"), ('べ', "be"), ('ぼ', "bo"),
              ('ぱ', "pa"), ('ぴ', "pi"), ('ぷ', "pu"), ('ぺ', "pe"), ('ぽ', "po"),
              ('ぁ', "wa"), ('ぃ', "wi"), ('ぅ', "wu"), ('ぇ', "we"), ('ぉ', "wo"),
              ('ゃ', "ya"), ('ゅ', "yu"), ('ょ', "yo"),
              ('っ', "-"),
          ]),
          tokenizer: ipadic::tokenizer(),
      };
      s.ori_text = text.to_string();
      s.ruby_text = "".to_string();
      s.ruby_text.push('\n');
      s
  }

  fn is_ruby_empty(&self) -> bool {
      self.ruby_text.is_empty()
  }

  fn show_ori(&self) -> String {
      self.ori_text.clone()
  }

  fn show_ruby(&self) -> String {
      self.ruby_text.clone()
  }
  
}

impl JSentence<'_> {

  fn kana_word_ruby(&mut self) -> String {
      self.ruby_text.push_str("<ruby>");
      
      for token in self.tokenizer.tokenize(&self.ori_text) {
          let surface = token.surface();
          let pronouncation = token.features().last().unwrap();

          let mut hira = String::new();
          
          for c in pronouncation.chars() {
              if self.kata_to_hira.contains_key(&c) {
                  hira.push(self.kata_to_hira[&c]);
              } else {
                  hira.push(c);
              }
          }
          self.ruby_text.push_str(&format!("<rb>{}</rb>", surface));
          self.ruby_text.push_str(&format!("<rt>{}</rt>", hira));
      }
      self.ruby_text.push_str("</ruby>");
      self.ruby_text.clone()
  }


  fn roma_word_ruby(&mut self) -> String {
      self.ruby_text.push_str("<ruby>");

      for token in self.tokenizer.tokenize(&self.ori_text) {
          let surface = token.surface();
          let pronouncation = token.features().last().unwrap();
          let mut kata = String::new();
          let mut roma = String::new();
          for c in pronouncation.chars() {
              if self.kata_to_roma.contains_key(&c) {
                  kata.push(c);
                  roma.push_str(self.kata_to_roma[&c]);
              } else {
                  kata.push(c);
                  roma.push(c);
              }
          }
          self.ruby_text.push_str(&format!("<rb>{}</rb>", surface));
          self.ruby_text.push_str(&format!("<rt>{}</rt>", roma));
      }
      self.ruby_text.push_str("</ruby>");
      self.ruby_text.clone()
  }
  
}

