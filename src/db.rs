use anyhow::{Context, Result};
use futures::{future::BoxFuture, stream, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Acquire, FromRow, PgExecutor, Pool, Postgres, Transaction};
use std::{cell::Cell, pin::Pin, sync::Arc};
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct CnnvdCollect {
    pub id: i64,
    pub cnnvd_id: Option<String>,
    pub cnnvd_code: Option<String>,
    pub vul_type: Option<String>,
    pub cnnvd_source_json: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]

pub struct CnnvdCollectUpdate {
    pub last_counts: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CnnvdProviderToken {
    pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]

pub struct CnnvdProviderUpdates {
    pub token: String,
    pub cnnvd_collect_id: i64,
}

impl CnnvdCollect {
    pub async fn upsert<'a, E>(
        cnnvd_id: &'a str,
        cnnvd_code: &'a str,
        vul_type: &'a str,
        cnnvd_source_json: &'a str,
        db_pool: E,
    ) -> Result<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"INSERT INTO "CnnvdCollect" (cnnvd_id, cnnvd_code,vul_type,cnnvd_source_json) VALUES ($1, $2,$3,$4) ON CONFLICT (cnnvd_id,cnnvd_code,vul_type) DO UPDATE SET Cnnvd_source_json = $4"#,cnnvd_id,cnnvd_code,vul_type,cnnvd_source_json
        )
            .execute(db_pool)
            .await.with_context(|| format!("insert or update CnnvdCollect {} failed", cnnvd_id))?;
        Ok(())
    }
    pub async fn get_mmmmany_Cnnvd(
        start_id: u64,
        max_count: u64,
        db_pool: &Pool<Postgres>,
    ) -> Result<Vec<CnnvdCollect>> {
        let result = sqlx::query_as!(
            CnnvdCollect,
            r#"SELECT * FROM "CnnvdCollect" WHERE id > $1 ORDER BY ID ASC  LIMIT $2"#,
            start_id as i64,
            max_count as i64
        )
        .fetch_all(db_pool)
        .await
        .with_context(|| format!("select CnnvdCollect failed"))?;
        Ok(result)
    }
    pub async fn get_by_ids(ids: Vec<u64>, db_pool: &Pool<Postgres>) -> Result<Vec<CnnvdCollect>> {
        let sql = format!(
            "SELECT * FROM \"CnnvdCollect\" WHERE id IN ({})",
            ids.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        return sqlx::query_as::<_, CnnvdCollect>(&sql)
            .fetch_all(db_pool)
            .await
            .with_context(|| format!("select get_by_ids failed"));
    }
    pub async fn get_one<'a>(
        cnnvd_id: &'a str,
        cnnvd_code: &'a str,
        vul_type: &'a str,
        db_pool: &Pool<Postgres>,
    ) -> Result<Option<Self>> {
        let result = sqlx::query_as!(
            CnnvdCollect,
            r#"SELECT * FROM "CnnvdCollect" WHERE cnnvd_id=$1 and cnnvd_code=$2 and vul_type=$3"#,
            cnnvd_id,
            cnnvd_code,
            vul_type
        )
        .fetch_optional(db_pool)
        .await
        .with_context(|| format!("select CnnvdCollect failed"));
        result
    }
    pub async fn get_empty_cnnvd(
        db_pool: &'static Pool<Postgres>,
    ) -> Pin<
        Box<
            dyn futures::Stream<
                    Item = Result<sqlx::Either<PgQueryResult, CnnvdCollect>, sqlx::Error>,
                > + std::marker::Send,
        >,
    > {
        let result = sqlx::query_as!(
            CnnvdCollect,
            r#"SELECT * FROM "CnnvdCollect" WHERE cnnvd_source_json=''"#
        )
        .fetch_many(db_pool);
        return result;
    }
}

impl CnnvdCollectUpdate {
    pub async fn get_last_counts(db_pool: &Pool<Postgres>) -> Result<Option<i64>> {
        let result = sqlx::query_as!(CnnvdCollectUpdate, r#"SELECT * FROM "CnnvdCollectUpdate" "#)
            .fetch_optional(db_pool)
            .await
            .with_context(|| format!("select last update tag failed"))?;
        Ok(result.map(|x| x.last_counts))
    }
    pub async fn update_counts(update_counts: &i64, db_pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"UPDATE "CnnvdCollectUpdate" SET last_counts=$1 "#,
            update_counts
        )
        .execute(db_pool)
        .await
        .with_context(|| format!("update last update tag {} failed", update_counts))?;
        Ok(())
    }
}
impl CnnvdProviderUpdates {
    pub async fn upsert<'a, 'b, E: sqlx::Executor<'b, Database = Postgres>>(
        token: &'a str,
        cnnvd_collect_id: u64,
        db_pool: E,
    ) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO "CnnvdProviderUpdates" (token, cnnvd_collect_id) VALUES ($1, $2) ON CONFLICT (token, Cnnvd_collect_id) DO NOTHING"#,token,cnnvd_collect_id as i64
        )
            .execute(db_pool)
            .await.with_context(|| format!("insert or update CnnvdProviderUpdate {} failed", token))?;
        Ok(())
    }
    pub async fn get_update_cnnvd_id_by_token(
        token: &str,
        db_pool: &Pool<Postgres>,
    ) -> Result<Vec<i64>> {
        let result = sqlx::query_as!(
            CnnvdProviderUpdates,
            r#"SELECT * FROM "CnnvdProviderUpdates" WHERE  token=$1"#,
            token
        )
        .fetch_all(db_pool)
        .await
        .with_context(|| format!("select update Cnnvd by token {} failed", token))?;

        let result = result
            .into_iter()
            .map(|x| x.cnnvd_collect_id)
            .collect::<Vec<i64>>();
        Ok(result)
    }
    pub async fn delete_confirmed_Cnnvd_id_by_token(
        token: &str,
        Cnnvd_collect_id: Vec<u64>,
        db_pool: &Pool<Postgres>,
    ) -> Result<()> {
        stream::iter(Cnnvd_collect_id.into_iter())
            .for_each(|Cnnvd_collect_id| async move {
                let _ = sqlx::query!(
                    r#"DELETE FROM "CnnvdProviderUpdates" WHERE token=$1 AND Cnnvd_collect_id=$2"#,
                    token,
                    Cnnvd_collect_id as i64
                )
                .execute(db_pool)
                .await
                .with_context(|| format!("delete confirmed Cnnvd by token {} failed", token))
                .map(|_| ())
                .map_err(|e| {
                    error!(
                        "delete confirmed Cnnvd by token {} failed, error: {}",
                        token, e
                    );
                });
            })
            .await;
        Ok(())
    }
}

impl CnnvdProviderToken {
    pub async fn select_all<'a>(db_pool: impl PgExecutor<'a>) -> Result<Vec<CnnvdProviderToken>> {
        return sqlx::query_as!(CnnvdProviderToken, r#"SELECT * FROM "CnnvdProviderToken" "#)
            .fetch_all(db_pool)
            .await
            .with_context(|| format!("select all CnnvdProviderToken  failed"));
    }
}

pub async fn add_new_provider_update<'a>(
    cnnvd_collect_id: i64,
    tx: impl Acquire<'a, Database = Postgres>,
) -> Result<()> {
    let mut conn = tx.acquire().await?;
    let ts = CnnvdProviderToken::select_all(&mut *conn).await?;

    for i in ts.iter() {
        CnnvdProviderUpdates::upsert(i.token.as_str(), cnnvd_collect_id as u64, &mut *conn).await?;
    }

    Ok(())
}

/*
.await
            .map(|_| ())
            .map_err(|e| {
                error!(
                    "upsert CnnvdProviderUpdates {} failed, error: {}",
                    cnnvd_collect_id, e
                );
            });
             */
