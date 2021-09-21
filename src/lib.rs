#![allow(clippy::tabs_in_doc_comments)]

pub mod error;

use crate::error::Error;
pub use cookie::{Cookie, CookieJar};
use error::ParseError;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub use time::OffsetDateTime;

/// A netscape cookie parser
/// allowing generating a new [`CookieJar`](cookie::CookieJar) or writing to an exist one.
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
        for c in s.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let (http_only, mut fileds) = if c.starts_with('#') {
                if c.starts_with("#HttpOnly_") {
                    (true, c.trim_start_matches("#HttpOnly_").split('\t'))
                } else {
                    continue;
                }
            } else {
                (false, c.split('\t'))
            };
            let domain = fileds.next().ok_or(ParseError::TooFewFileds)?;
            let _ = fileds.next(); // ignore subdomain
            let path = fileds.next().ok_or(ParseError::TooFewFileds)?;
            let secure = match fileds.next().ok_or(ParseError::TooFewFileds)? {
                "TRUE" => true,
                "FALSE" => false,
                value => return Err(ParseError::InvaildValue(value.to_owned()).into()),
            };
            let expiration: i64 = match fileds.next() {
                Some(value) => match value.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ParseError::InvaildValue(value.to_owned()).into()),
                },
                _ => return Err(ParseError::TooFewFileds.into()),
            };
            let name = fileds.next().ok_or(ParseError::TooFewFileds)?;
            let value = fileds.next().ok_or(ParseError::TooFewFileds)?;
            let cookie = Cookie::build(name, value)
                .domain(domain)
                .path(path)
                .secure(secure)
                .expires(match expiration {
                    0 => None,
                    exp => Some(OffsetDateTime::from_unix_timestamp(exp)),
                });
            let cookie = if http_only {
                cookie.http_only(true).finish()
            } else {
                cookie.finish()
            };
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
