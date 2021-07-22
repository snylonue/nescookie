#![allow(clippy::tabs_in_doc_comments)]

pub mod error;

use crate::error::Error;
pub use cookie::{Cookie, CookieJar};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub use time::OffsetDateTime;

#[derive(Debug, Parser)]
#[grammar = "cookie.pest"]
struct CookieParser {}

#[derive(Debug, Default)]
pub struct CookieJarBuilder {
    jar: CookieJar,
}

impl CookieJarBuilder {
    /// Creates a new `CookieJarBuilder`
    /// ```
    /// use nescookie::CookieJarBuilder;
    ///
    /// let jar = CookieJarBuilder::new().finish();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a new `CookieJarBuilder` from a [`CookieJar`](cookie::CookieJar)
    /// parsed cookies will be added to it
    pub fn with_jar(jar: CookieJar) -> Self {
        Self { jar }
    }
    /// Opens a file with `path` and parses it as cookies
    ///
    /// ```
    /// use nescookie::CookieJarBuilder;
    ///
    /// let jar = CookieJarBuilder::new().open("tests/cookies.txt").unwrap().finish();
    /// ```
    pub fn open(self, path: impl AsRef<Path>) -> Result<Self, Error> {
        self.parse_buffer(BufReader::new(File::open(path)?))
    }
    /// Parses cookies from something that implements [`BufRead`](std::io::BufRead)
    ///
    /// ```
    /// use nescookie::CookieJarBuilder;
    /// use std::io::Cursor;
    ///
    /// let buf = Cursor::new(b".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n");
    /// let jar = CookieJarBuilder::new().parse_buffer(buf).unwrap().finish();
    /// ```
    pub fn parse_buffer(self, mut buf: impl BufRead) -> Result<Self, Error> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        self.parse(&s)
    }
    /// Parses cookies from an str
    ///
    /// ```
    /// use nescookie::CookieJarBuilder;
    ///
    /// let content = ".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n";
    /// let jar = CookieJarBuilder::new().parse(content).unwrap().finish();
    /// ```
    pub fn parse(mut self, s: &str) -> Result<Self, Error> {
        let file = CookieParser::parse(Rule::file, s)?.next().unwrap();
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
            self.jar.add(cookie.into_owned());
        }
        Ok(self)
    }
    /// Returns the built `CookieJar`
    pub fn finish(self) -> CookieJar {
        self.jar
    }
}

/// Opens a file with `path` and parses it as [`CookieJar`](cookie::CookieJar)
///
/// ```
/// let jar = nescookie::open("tests/cookies.txt").unwrap();
/// ```
#[inline]
pub fn open(path: impl AsRef<Path>) -> Result<CookieJar, Error> {
    CookieJarBuilder::new().open(path).map(|jar| jar.finish())
}
/// Parses a [`CookieJar`](cookie::CookieJar) from something that implements [`BufRead`](std::io::BufRead)
///
/// ```
/// use std::io::Cursor;
///
/// let buf = Cursor::new(b".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n");
/// let jar = nescookie::parse_buffer(buf).unwrap();
/// ```
pub fn parse_buffer(buf: impl BufRead) -> Result<CookieJar, Error> {
    CookieJarBuilder::new()
        .parse_buffer(buf)
        .map(|jar| jar.finish())
}
/// Parses a [`CookieJar`](cookie::CookieJar) from an str
///
/// ```
/// let content = ".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n";
/// let jar = nescookie::parse(content).unwrap();
/// ```
pub fn parse(s: &str) -> Result<CookieJar, Error> {
    CookieJarBuilder::new().parse(s).map(|jar| jar.finish())
}
