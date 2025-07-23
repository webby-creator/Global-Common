use serde::{Deserialize, Serialize};

pub mod filter;
pub mod id;
pub mod object_id;
pub mod request;
pub mod response;
pub mod schema;
pub mod tz;
pub mod upload;
pub mod uuid;
pub mod value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn unwrap_left(self) -> L {
        match self {
            Either::Left(v) => v,
            Either::Right(_) => panic!("Attempted to unwrap Left but found Right"),
        }
    }

    pub fn unwrap_right(self) -> R {
        match self {
            Either::Left(_) => panic!("Attempted to unwrap Right but found Left"),
            Either::Right(v) => v,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMulti<V> {
    Single(V),
    Multiple(Vec<V>),
}

impl<V> SingleOrMulti<V> {
    pub fn try_map<R, E, F: Fn(V) -> Result<R, E>>(self, func: F) -> Result<SingleOrMulti<R>, E> {
        Ok(match self {
            SingleOrMulti::Single(v) => SingleOrMulti::Single(func(v)?),
            SingleOrMulti::Multiple(i) => {
                SingleOrMulti::Multiple(i.into_iter().map(func).collect::<Result<Vec<_>, E>>()?)
            }
        })
    }
}

impl<V> From<V> for SingleOrMulti<V> {
    fn from(value: V) -> Self {
        SingleOrMulti::Single(value)
    }
}

impl<V> From<Vec<V>> for SingleOrMulti<V> {
    fn from(value: Vec<V>) -> Self {
        SingleOrMulti::Multiple(value)
    }
}
