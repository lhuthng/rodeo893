use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::product::{Product, ProductCategory, ProductOrderingDay},
    repositories::product::{ProductCategoryRepository, ProductOrderingDayRepository, ProductRepository},
    DomainError,
};

pub struct PgProductCategoryRepository(pub PgPool);

#[async_trait]
impl ProductCategoryRepository for PgProductCategoryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ProductCategory, DomainError> {
        sqlx::query_as!(ProductCategory, "SELECT * FROM product_categories WHERE id=$1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Category {}", id)))
    }

    async fn list(&self) -> Result<Vec<ProductCategory>, DomainError> {
        sqlx::query_as!(ProductCategory, "SELECT * FROM product_categories ORDER BY sort_order")
            .fetch_all(&self.0).await.map_err(db_err)
    }
}

pub struct PgProductRepository(pub PgPool);

#[async_trait]
impl ProductRepository for PgProductRepository {
    async fn create(&self, p: &Product) -> Result<Product, DomainError> {
        sqlx::query_as!(Product,
            r#"INSERT INTO products (id,category_id,name,slug,description,image_url,base_price,currency,is_active,created_at,updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) RETURNING *"#,
            p.id, p.category_id, p.name, p.slug, p.description, p.image_url,
            p.base_price, p.currency, p.is_active, p.created_at, p.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Product, DomainError> {
        sqlx::query_as!(Product, "SELECT * FROM products WHERE id=$1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Product {}", id)))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Product, DomainError> {
        sqlx::query_as!(Product, "SELECT * FROM products WHERE slug=$1", slug)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Product slug {}", slug)))
    }

    async fn list(&self, active_only: bool) -> Result<Vec<Product>, DomainError> {
        if active_only {
            sqlx::query_as!(Product, "SELECT * FROM products WHERE is_active=true ORDER BY name")
                .fetch_all(&self.0).await.map_err(db_err)
        } else {
            sqlx::query_as!(Product, "SELECT * FROM products ORDER BY name")
                .fetch_all(&self.0).await.map_err(db_err)
        }
    }

    async fn update(&self, p: &Product) -> Result<Product, DomainError> {
        sqlx::query_as!(Product,
            r#"UPDATE products SET name=$2,description=$3,image_url=$4,base_price=$5,
               is_active=$6,updated_at=$7 WHERE id=$1 RETURNING *"#,
            p.id, p.name, p.description, p.image_url, p.base_price, p.is_active, p.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn deactivate(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("UPDATE products SET is_active=false WHERE id=$1", id)
            .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }

    async fn has_open_orders(&self, product_id: Uuid) -> Result<bool, DomainError> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) as cnt FROM order_items oi
               JOIN orders o ON o.id = oi.order_id
               WHERE oi.product_id = $1 AND o.status NOT IN ('delivered','cancelled','refunded')"#,
            product_id
        )
        .fetch_one(&self.0).await.map_err(db_err)?;
        Ok(row.cnt.unwrap_or(0) > 0)
    }
}

pub struct PgProductOrderingDayRepository(pub PgPool);

#[async_trait]
impl ProductOrderingDayRepository for PgProductOrderingDayRepository {
    async fn set_for_product(&self, product_id: Uuid, days: &[i16]) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM product_ordering_days WHERE product_id=$1", product_id)
            .execute(&self.0).await.map_err(db_err)?;
        for &day in days {
            sqlx::query!(
                "INSERT INTO product_ordering_days (product_id, day_of_week) VALUES ($1,$2)",
                product_id, day
            )
            .execute(&self.0).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn list_for_product(&self, product_id: Uuid) -> Result<Vec<i16>, DomainError> {
        let rows = sqlx::query!(
            "SELECT day_of_week FROM product_ordering_days WHERE product_id=$1 ORDER BY day_of_week",
            product_id
        )
        .fetch_all(&self.0).await.map_err(db_err)?;
        Ok(rows.into_iter().map(|r| r.day_of_week).collect())
    }
}

fn db_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound("Record not found".into()),
        sqlx::Error::Database(ref dbe) if dbe.code().as_deref() == Some("23505") =>
            DomainError::Conflict(dbe.message().to_string()),
        other => DomainError::Internal(other.to_string()),
    }
}
