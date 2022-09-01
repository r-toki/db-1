use sqlx::PgPool;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::model::task::Task;

pub type Result<T> = anyhow::Result<T, sqlx::Error>;

pub struct Table<T> {
    pub pool: Arc<PgPool>,
    _row: PhantomData<T>,
}

impl<T> Table<T> {
    fn new(pool: Arc<PgPool>) -> Self {
        Table {
            pool,
            _row: PhantomData,
        }
    }
}

pub struct Database {
    pub tasks: Arc<Table<Task>>,
}

impl Database {
    pub async fn new(database_url: &str) -> Database {
        let pool = PgPool::connect(database_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            tasks: Arc::from(Table::new(pool.clone())),
        }
    }
}
