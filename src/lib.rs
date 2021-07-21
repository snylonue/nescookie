#![allow(clippy::tabs_in_doc_comments)]

pub use cookie::{Cookie, CookieJar};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub use time::OffsetDateTime;

#[derive(Debug, Parser)]
#[grammar = "cookie.pest"]
struct CookieParser {}

#[derive(Debug)]
pub enum Error {
    ParseError(pest::error::Error<Rule>),
    IoError(std::io::Error),
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::ParseError(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
            Self::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseError(e) => Some(e),
            Self::IoError(e) => Some(e),
        }
    }
}

/// Parses a [`CookieJar`](cookie::CookieJar) from a path
///
/// ```
/// let jar = nescookie::open("tests/cookies.txt").unwrap();
/// ```
#[inline]
pub fn open(path: impl AsRef<Path>) -> Result<CookieJar, Error> {
    parse_buffer(&mut BufReader::new(File::open(path)?))
}
/// Parses a [`CookieJar`](cookie::CookieJar) from something that implements [`BufRead`](std::io::BufRead)
///
/// ```
/// use std::io::Cursor;
///
/// let buf = Cursor::new(b".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n");
/// let jar = nescookie::parse_buffer(buf).unwrap();
/// ```
pub fn parse_buffer(mut buf: impl BufRead) -> Result<CookieJar, Error> {
    let mut s = String::new();
    buf.read_to_string(&mut s)?;
    parse(&s)
}
/// Parses a [`CookieJar`](cookie::CookieJar) from an str
///
/// ```
/// let content = ".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n";
/// let jar = nescookie::parse(content).unwrap();
/// ```
pub fn parse(s: &str) -> Result<CookieJar, Error> {
    let file = CookieParser::parse(Rule::file, s)?.next().unwrap();
    let mut jar = CookieJar::new();
    for c in file
        .into_inner()
        .take_while(|r: &Pair<Rule>| !matches!(r.as_rule(), Rule::EOI))
    {
        let mut fileds: Pairs<Rule> = c.into_inner();
        let domain = fileds.next().unwrap().as_str();
        let path = fileds.next().unwrap().as_str();
        let secure = fileds.next().unwrap().as_str() == "TRUE"; // this value is either "TRUE" or "FALSE"
        let expiration: i64 = fileds.next().unwrap().as_str().parse().unwrap();
        let name = fileds.next().unwrap().as_str();
        let value = fileds.next().unwrap().as_str();
        let cookie = Cookie::build(name, value)
            .domain(domain)
            .path(path)
            .secure(secure)
            .expires(match expiration {
                0 => None,
                exp => Some(OffsetDateTime::from_unix_timestamp(exp)),
            })
            .finish();
        jar.add(cookie.into_owned());
    }
    Ok(jar)
}
