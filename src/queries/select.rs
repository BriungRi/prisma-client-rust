use std::marker::PhantomData;

use query_core::{Operation, Selection};
use serde::de::DeserializeOwned;

use crate::{PrismaClientInternals, Query, QueryConvert};

pub trait SelectType {
    // TODO: ModelActions
    type Data: DeserializeOwned;
    type ModelData;

    fn to_selections(self) -> Vec<Selection>;
}

pub struct Select<'a, Data: DeserializeOwned> {
    operation: Operation,
    client: &'a PrismaClientInternals,
    _data: PhantomData<Data>,
}

impl<'a, Data: DeserializeOwned> Select<'a, Data> {
    pub fn new(client: &'a PrismaClientInternals, operation: Operation) -> Self {
        Self {
            client,
            operation,
            _data: PhantomData {},
        }
    }

    pub async fn exec(self) -> super::Result<Data> {
        self.client.execute(self.operation).await
    }
}

impl<'a, Data: DeserializeOwned + 'static> QueryConvert for Select<'a, Data> {
    type RawType = Data;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> Self::ReturnValue {
        raw
    }
}

impl<'a, Data: DeserializeOwned + 'static> Query<'a> for Select<'a, Data> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        (self.operation, self.client)
    }
}
