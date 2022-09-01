use super::db::{Result, Table};
use crate::model::task::{EditTask, NewTask, Task};
use sqlx::query_as;

impl Table<Task> {
    pub async fn find_all(&self) -> Result<Vec<Task>> {
        query_as!(
            Task,
            r#"
SELECT *
FROM tasks
ORDER BY id
        "#
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn find_one(&self, id: i64) -> Result<Task> {
        query_as!(
            Task,
            r#"
SELECT *
FROM tasks
where id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn insert(&self, new_task: NewTask) -> Result<Task> {
        query_as!(
            Task,
            r#"
INSERT INTO tasks (description)
VALUES ( $1 )
RETURNING *
            "#,
            new_task.description
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update(&self, id: i64, edit_task: EditTask) -> Result<Task> {
        query_as!(
            Task,
            r#"
UPDATE tasks
SET description = $1, completed = $2
WHERE id = $3
RETURNING *
            "#,
            edit_task.description,
            edit_task.completed,
            id
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn delete(&self, id: i64) -> Result<Task> {
        query_as!(
            Task,
            r#"
DELETE FROM tasks
WHERE id = $1
RETURNING *
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await
    }
}
