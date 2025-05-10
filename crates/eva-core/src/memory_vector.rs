use crate::memory::MemoryRecord;
use blake3::Hasher;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::OnceLock;
use chrono::{DateTime, Utc};

static POOL: OnceLock<Option<SqlitePool>> = OnceLock::new();

async fn pool() -> anyhow::Result<Option<&'static SqlitePool>> {
    if POOL.get().is_none() {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://memory_vec.db".into());
        let conn = match SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
        {
            Ok(p) => Some(p),
            Err(e) => {
                tracing::warn!("sqlite init failed: {e}");
                None
            }
        };
        if let Some(p) = &conn {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS memory_vec(
                    trace_id TEXT PRIMARY KEY,
                    role TEXT,
                    text TEXT,
                    ts TEXT,
                    embedding BLOB
                )",
            )
            .execute(p)
            .await?;
        }
        POOL.set(conn).ok();
    }
    Ok(POOL.get().unwrap().as_ref())
}

/// 產生 8 維假向量（blake3 哈希 + PRNG）；將來可替換真正 Embedding
fn fake_embed(text: &str) -> [f32; 8] {
    let mut hasher = Hasher::new();
    hasher.update(text.as_bytes());
    let hash = hasher.finalize();
    let seed = u64::from_le_bytes(hash.as_bytes()[0..8].try_into().unwrap());
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vec = [0f32; 8];
    for v in &mut vec {
        *v = rng.gen();
    }
    vec
}

pub async fn insert(rec: MemoryRecord) -> anyhow::Result<()> {
    if let Some(p) = pool().await? {
        let emb = fake_embed(&rec.text);
        let blob = bincode::serialize(&emb)?;
        sqlx::query(
            "INSERT OR IGNORE INTO memory_vec(trace_id,role,text,ts,embedding)
             VALUES (?1,?2,?3,?4,?5)",
        )
        .bind(&rec.trace_id)
        .bind(&rec.role)
        .bind(&rec.text)
        .bind(rec.ts.to_rfc3339())
        .bind(blob)
        .execute(p)
        .await?;
    }
    Ok(())
}

/// 回傳前 k 條最相似記憶（cosine → 簡化為歐氏距離）
pub async fn similar(query: &str, k: i64) -> anyhow::Result<Vec<MemoryRecord>> {
    if let Some(p) = pool().await? {
        let q_emb = fake_embed(query);
        let rows = sqlx::query_as::<_, (String, String, String, String, Vec<u8>)>(
            "SELECT trace_id, role, text, ts, embedding FROM memory_vec",
        )
        .fetch_all(p)
        .await?;
        let mut scored: Vec<(f32, MemoryRecord)> = rows
            .into_iter()
            .filter_map(|(id, role, text, ts, blob)| {
                let emb: [f32; 8] = bincode::deserialize(&blob).ok()?;
                let dist = emb
                    .iter()
                    .zip(q_emb.iter())
                    .map(|(a, b)| (*a - *b).powi(2))
                    .sum::<f32>()
                    .sqrt();
                let ts_parsed = DateTime::parse_from_rfc3339(&ts).ok()?.with_timezone(&Utc);
                Some((
                    dist,
                    MemoryRecord {
                        trace_id: id,
                        role,
                        text,
                        ts: ts_parsed,
                    },
                ))
            })
            .collect();
        scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(scored.into_iter().take(k as usize).map(|(_, r)| r).collect())
    } else {
        Ok(Vec::new())
    }
}
