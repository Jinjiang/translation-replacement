use std::{cell::RefCell, rc::Rc, usize::MAX};

use crate::{
  char_type::CharType,
  token_type::{
    CommonToken,
    GroupToken,
    Mark,
    MarkType,
    MutGroupToken,
    MutableMark,
    TokenType,
    Pair,
    Token
  },
};

pub struct ParseStatus {
  pub last_token: Option<Rc<RefCell<Token>>>,
  pub last_group: Option<Rc<RefCell<GroupToken>>>,
  pub last_mark: Option<Rc<RefCell<Mark>>>,

  pub tokens: Rc<RefCell<GroupToken>>,
  pub marks: Vec<Rc<RefCell<Mark>>>,
  pub groups: Vec<Rc<RefCell<GroupToken>>>,

  pub mark_stack: Vec<Rc<RefCell<Mark>>>,
  pub group_stack: Vec<Rc<RefCell<GroupToken>>>,

  pub errors: Vec<Rc<RefCell<String>>>, // TODO: Validation
}

pub struct ParseResult {
  pub tokens: Rc<RefCell<GroupToken>>,
  pub groups: Vec<Rc<RefCell<GroupToken>>>,
  pub marks: Vec<Rc<RefCell<Mark>>>,
  pub errors: Vec<Rc<RefCell<String>>>, // TODO: Validation
}

pub struct MutableParseResult {
  pub tokens: Rc<RefCell<MutGroupToken>>,
  pub groups: Vec<Rc<RefCell<MutGroupToken>>>,
  pub marks: Vec<Rc<RefCell<MutableMark>>>,
  pub errors: Vec<Rc<RefCell<String>>>, // TODO: Validation
}

pub fn create_status(str: &str) -> ParseStatus {
  let tokens: GroupToken = GroupToken {
    token: CommonToken {
      index: 0,
      length: str.len(),
      value: str.to_string(),
      space_after: String::from(""),
      mark: None,
      mark_side: None,
    },
    pair: Pair {
      start_index: 0,
      start_value: String::from(""),
      end_index: str.len(),
      end_value: String::from(""),
    },
    token_type: TokenType::Group,
    inner_space_before: String::from(""),
    children: vec![],
  };
  let tokens_rc = Rc::new(RefCell::new(tokens));
  ParseStatus {
    last_token: None,
    last_group: Some(Rc::clone(&tokens_rc)),
    last_mark: None,
  
    tokens: Rc::clone(&tokens_rc),
    marks: vec![],
    groups: vec![],
  
    mark_stack: vec![],
    group_stack: vec![],
  
    errors: vec![],
  }
}

pub fn create_mark(
  mut status: ParseStatus,
  index: usize,
  c: char,
  t: MarkType
) -> Rc<RefCell<Mark>> {
  if status.last_mark.is_some() {
    status.mark_stack.push(
      status.last_mark.unwrap()
    );
    status.last_mark = None;
  }
  let mark: Mark = Mark {
    pair: Pair {
      start_index: index,
      start_value: c.to_string(),
      end_index: MAX,
      end_value: String::from(""),
    },
    mark_type: t,
    meta: None,
  };
  let mark_rc = Rc::new(RefCell::new(mark));
  status.marks.push(mark_rc.clone());
  status.last_mark = Some(mark_rc.clone());
  return mark_rc;
}

pub fn add_bracket_token() {}

#[allow(unused_variables)]
pub fn finalize_last_token(
  status: &ParseStatus,
  index: usize
) {}

pub fn finalize_current_token() {}

pub fn finalize_current_mark() {}

#[allow(unused_variables)]
pub fn handle_letter(
  index: usize,
  c: char,
  char_type: CharType,
  status: &ParseStatus
) {}

#[allow(unused_variables)]
pub fn handle_punctuation(
  index: usize,
  c: char,
  char_type: CharType,
  status: &ParseStatus
) {}

// TODO:
pub fn mark_to_token() {}

// TODO:
pub fn add_hyper_token() {}

// TODO:
pub fn add_raw_content() {}

pub fn add_sinple_punctuation_token() {}

pub fn add_unmatched_token() {}

pub fn create_group() {}

pub fn finalize_group() {}

pub fn create_content() {}

#[allow(unused_variables)]
pub fn append_value(
  status: &ParseStatus,
  c: char
) {}

#[allow(unused_variables, dead_code)]
pub fn get_space_length(str: &str, i: usize) -> usize {
  1
}

#[allow(unused_variables)]
pub fn get_prev_token(status: &ParseStatus) -> Option<&Rc<RefCell<Token>>> {
  None
}

#[allow(unused_variables)]
pub fn is_shorthand(
  str: &str,
  status: &ParseStatus,
  i: usize,
  c: char
) -> bool {
  return false;
}

pub fn get_hyper_content_type() {}

pub fn temp_add_spaces(
  token: &Rc<RefCell<Token>>,
  spaces: &str
) {
  match &mut *token.borrow_mut() {
    Token::SingleToken(single_token) => {
      single_token.token.space_after = String::from(spaces);
    },
    Token::GroupToken(group_token) => {
      group_token.token.space_after = String::from(spaces);
    },
  }
}