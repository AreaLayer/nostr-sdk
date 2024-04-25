// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! NIP98
//!
//! This NIP defines an ephemerial event used to authorize requests to HTTP servers using nostr events.
//! This is useful for HTTP services which are build for Nostr and deal with Nostr user accounts.
//!
//! <https://github.com/nostr-protocol/nips/blob/master/98.md>

use alloc::vec::Vec;
use core::fmt;

use bitcoin::hashes::sha256::Hash as Sha256Hash;

use crate::{HttpMethod, Tag, TagStandard, UncheckedUrl};

/// [`HttpData`] required tags
#[derive(Debug)]
pub enum RequiredTags {
    /// [`Tag::AbsoluteURL`]
    AbsoluteURL,
    /// [`Tag::Method`]
    Method,
}

impl fmt::Display for RequiredTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbsoluteURL => write!(f, "url"),
            Self::Method => write!(f, "method"),
        }
    }
}

/// [`HttpData`] error
#[derive(Debug)]
pub enum Error {
    /// Hex decoding error
    Hex(bitcoin::hashes::hex::HexToBytesError),
    /// Tag missing when parsing
    MissingTag(RequiredTags),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(e) => write!(f, "{e}"),
            Self::MissingTag(tag) => write!(f, r#"missing tag "{tag}""#),
        }
    }
}

impl From<bitcoin::hashes::hex::HexToBytesError> for Error {
    fn from(e: bitcoin::hashes::hex::HexToBytesError) -> Self {
        Self::Hex(e)
    }
}

/// HTTP Data
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HttpData {
    /// Absolute request URL
    pub url: UncheckedUrl,
    /// HTTP method
    pub method: HttpMethod,
    /// SHA256 hash of the request body
    pub payload: Option<Sha256Hash>,
}

impl HttpData {
    /// New [`HttpData`]
    #[inline]
    pub fn new(url: UncheckedUrl, method: HttpMethod) -> Self {
        Self {
            url,
            method,
            payload: None,
        }
    }

    /// Add hex-encoded SHA256 hash of the request body
    #[inline]
    pub fn payload(mut self, payload: Sha256Hash) -> Self {
        self.payload = Some(payload);
        self
    }
}

impl From<HttpData> for Vec<Tag> {
    fn from(data: HttpData) -> Self {
        let HttpData {
            url,
            method,
            payload,
        } = data;

        let mut tags: Vec<Tag> = vec![
            Tag::from_standardized_without_cell(TagStandard::AbsoluteURL(url)),
            Tag::from_standardized_without_cell(TagStandard::Method(method)),
        ];
        if let Some(payload) = payload {
            tags.push(Tag::from_standardized_without_cell(TagStandard::Payload(
                payload,
            )));
        }

        tags
    }
}

impl TryFrom<Vec<Tag>> for HttpData {
    type Error = Error;

    fn try_from(value: Vec<Tag>) -> Result<Self, Self::Error> {
        let url = value
            .iter()
            .find_map(|t| match t.as_standardized() {
                Some(TagStandard::AbsoluteURL(u)) => Some(u),
                _ => None,
            })
            .cloned()
            .ok_or(Error::MissingTag(RequiredTags::AbsoluteURL))?;
        let method = value
            .iter()
            .find_map(|t| match t.as_standardized() {
                Some(TagStandard::Method(m)) => Some(m),
                _ => None,
            })
            .cloned()
            .ok_or(Error::MissingTag(RequiredTags::Method))?;
        let payload = value
            .iter()
            .find_map(|t| match t.as_standardized() {
                Some(TagStandard::Payload(p)) => Some(p),
                _ => None,
            })
            .cloned();

        Ok(Self {
            url,
            method,
            payload,
        })
    }
}
