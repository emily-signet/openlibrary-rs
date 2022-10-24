const BASE_URL: &str = "https://openlibrary.org/api";

use std::{borrow::Cow, marker::PhantomData};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BorrowCow};
use smallvec::SmallVec;
use smallvec_map::VecMap;

mod serde_crimes;
use serde_crimes::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "'a: 'de"))]
pub struct Book<'a> {
    #[serde_as(deserialize_as = "BorrowCow")]
    pub url: Cow<'a, str>,
    #[serde_as(deserialize_as = "BorrowCow")]
    pub key: Cow<'a, str>,
    #[serde_as(deserialize_as = "BorrowCow")]
    pub title: Cow<'a, str>,
    #[serde_as(deserialize_as = "Option<BorrowCow>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub authors: SmallVec<[NamedUrl<'a>; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<usize>,
    #[serde(default)]
    #[serde_as(deserialize_as = "VecMap<BorrowCow, SmallVecShim<BorrowCow, 1>, 4>")]
    pub identifiers: VecMap<Cow<'a, str>, SmallVec<[Cow<'a, str>; 1]>, 4>,
    #[serde(default)]
    #[serde_as(deserialize_as = "VecMap<BorrowCow, SmallVecShim<BorrowCow, 1>, 4>")]
    pub classifications: VecMap<Cow<'a, str>, SmallVec<[Cow<'a, str>; 1]>, 4>,
    #[serde(borrow, default)]
    pub publishers: SmallVec<[NamedUrl<'a>; 1]>,
    #[serde(borrow, default)]
    pub publish_places: SmallVec<[NamedUrl<'a>; 1]>,
    #[serde_as(deserialize_as = "Option<BorrowCow>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub subjects: SmallVec<[NamedUrl<'a>; 16]>,
    #[serde(borrow, default)]
    pub subject_places: SmallVec<[NamedUrl<'a>; 8]>,
    #[serde(borrow, default)]
    pub subject_people: SmallVec<[NamedUrl<'a>; 8]>,
    #[serde_as(deserialize_as = "Option<BorrowCow>")]
    pub notes: Option<Cow<'a, str>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub cover: Option<Cover<'a>>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Cover<'a> {
    #[serde_as(deserialize_as = "BorrowCow")]
    pub small: Cow<'a, str>,
    #[serde_as(deserialize_as = "BorrowCow")]
    pub medium: Cow<'a, str>,
    #[serde_as(deserialize_as = "BorrowCow")]
    pub large: Cow<'a, str>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct NamedUrl<'a> {
    #[serde_as(deserialize_as = "BorrowCow")]
    pub name: Cow<'a, str>,
    #[serde_as(deserialize_as = "Option<BorrowCow>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
}

pub struct DeserializableResponse<'a, T> {
    inner: Bytes,
    target: PhantomData<&'a T>,
}

impl<'de, T: Deserialize<'de>> DeserializableResponse<'de, T> {
    pub fn get(&'de self) -> serde_json::Result<T> {
        serde_json::from_slice(&self.inner)
    }
}

pub struct OpenLibraryClient {
    client: reqwest::Client,
}

impl OpenLibraryClient {
    pub fn new() -> OpenLibraryClient {
        OpenLibraryClient {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> OpenLibraryClient {
        OpenLibraryClient {
            client,
        }
    }
}

pub type BooksResponse<'a> = DeserializableResponse<'a, VecMap<&'a str, Book<'a>, 1>>;

impl OpenLibraryClient {
    pub async fn by_bibkey(
        &self,
        bibkey: &str,
    ) -> reqwest::Result<BooksResponse<'_>> {
        Ok(DeserializableResponse {
            inner: self
                .client
                .get(const_format::concatcp!(BASE_URL, "/books"))
                .query(&[
                    ("bibkeys", bibkey),
                    ("jscmd", "data"),
                    ("format", "json"),
                ])
                .send()
                .await?
                .bytes()
                .await?,
            target: PhantomData,
        })
    }
}
