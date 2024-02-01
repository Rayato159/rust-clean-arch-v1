use async_trait::async_trait;
use std::{result::Result, error::Error};
use crate::cockroach::entities::cockroach::Cockroach;

#[async_trait]
pub trait CockroachRepository {
    async fn insert_cockroach_data(&self, cockroach_data: &Cockroach) -> Result<Cockroach, Box<dyn Error>>;
}