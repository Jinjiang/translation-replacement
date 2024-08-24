use std::collections::HashMap;

use regex::Regex;

use crate::{hyper::markdown::context::{InlineMark, InlineType}, token::{char_type::{get_char_type, CharType}, token_type::HyperTokenType}};

pub fn get_space_length(
  str: &str,
  i: usize
) -> usize {
  let mut space_length = 0;
  let mut j = i;
  while j < str.len() {
    let c = str.chars().nth(j).unwrap();
    if get_char_type(c) == CharType::Space {
      space_length += 1;
    } else {
      break;
    }
    j += 1;
  }
  space_length
}

pub fn get_hyper_content_type(
  str: &str
) -> HyperTokenType {
  if Regex::new("\n").unwrap().is_match(str) {
    return HyperTokenType::HyperContent;
  }
  if Regex::new("^<code.*>.*<\\/code.*>$").unwrap().is_match(str) {
    return HyperTokenType::CodeContent;
  }
  if Regex::new("^<.+>$").unwrap().is_match(str) {
    return HyperTokenType::HyperContent;
  }
  return HyperTokenType::CodeContent;
}

pub fn get_hyper_mark_map(hyper_marks: &mut Vec<InlineMark>) -> HashMap<usize, InlineMark> {
  let mut hyper_mark_map: HashMap<usize, InlineMark> = HashMap::new();
  hyper_marks.iter().for_each(|mark| {
    match mark.meta {
      InlineType::MarkPair | InlineType::MarkPairWithCode => {
        hyper_mark_map.insert(mark.pair.start_range.start, mark.clone());
        hyper_mark_map.insert(mark.pair.end_range.start, mark.clone());
      },
      InlineType::SingleMark | InlineType::SingleMarkConnect => {
        hyper_mark_map.insert(mark.pair.start_range.start, mark.clone());
      },
      _ => {}
    }
  });
  hyper_mark_map
}