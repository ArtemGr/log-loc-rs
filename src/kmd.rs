use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};

/// Matches the $tag in `Log.println('$tag',`
fn tagⁿ (i: &[u8]) -> IResult<&[u8], &[u8]> {
  for ix in 0 .. i.len() {
    let ch = i[ix];
    let pass =
      (ch >= b'a' && ch <= b'z') ||
      (ch >= b'A' && ch <= b'Z') ||
      (ch >= b'0' && ch <= b'9') ||
      ch == b'_' ||
      ch == b'.' ||
      ch == b':' ||
      ch == b' ' ||
      ch == b'-';
    if !pass {return Ok ((&i[ix..], &i[0..ix]))}}
  return Ok ((i, &i[0..0]))}

#[derive(Debug)]
pub struct Tag<'a> {
  source: &'a [u8],
  start: usize,
  end: usize}
impl<'a> Tag<'a> {
  pub fn head (&self) -> &[u8] {&self.source[..self.start]}
  pub fn tail (&self) -> &[u8] {&self.source[self.end..]}}

/// Matches the `Log.println('$tag',`, returning the matched slice and the tag offsets.  
/// Also the shorter `Log('$tag',` version.
fn log_printlnⁿ (iₒ: &[u8]) -> IResult<&[u8], Tag> {
  let (i, _) = alt ((tag ("Log.println"), tag ("Log"))) (iₒ) ?;
  let (i, _) = multispace0 (i) ?;
  let (i, _) = char ('(') (i) ?;
  let (i, _) = multispace0 (i) ?;
  let (i, _) = char ('\'') (i) ?;
  let start = i.as_ptr() as usize - iₒ.as_ptr() as usize;
  let (i, _) = tagⁿ (i) ?;
  let end = i.as_ptr() as usize - iₒ.as_ptr() as usize;
  let (i, _) = char ('\'') (i) ?;
  let (i, _) = multispace0 (i) ?;
  let (i, _) = char (',') (i) ?;
  let endₒ = i.as_ptr() as usize - iₒ.as_ptr() as usize;
  Ok ((i, Tag {source: &iₒ[0..endₒ], start, end}))}

#[derive(Debug)]
pub enum El<'a> {Source (&'a [u8]), Tag (Tag<'a>)}

/// Recongnize the tags in `Log.println('$tag',`, splitting the `input` into a list of source code and tags.
pub fn find_tags (mut input: &[u8]) -> Result<Vec<El>, String> {
  let mut els = Vec::new();
  let mut ix = 0;
  loop {
    if let Ok ((i, tag)) = log_printlnⁿ (&input[ix..]) {
      els.push (El::Source (&input[0..ix]));
      els.push (El::Tag (tag));
      ix = 0; input = i;
    } else if ix == input.len() {
      els.push (El::Source (input));
      break
    } else {
      ix += 1}}

  Ok (els)}
