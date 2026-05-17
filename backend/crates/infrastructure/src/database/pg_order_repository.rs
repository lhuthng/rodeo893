use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::order::{Order, OrderDeliveryInfo, OrderItem, OrderStatus, DeliveryMethod},
    repositories::order::{OrderDeliveryInfoRepository, OrderFilters, OrderItemRepository, OrderRepository},
    DomainError,
};

pub struct PgOrderRepository(pub PgPool);

#[async_trait]
impl OrderRepository for PgOrderRepository {
    async fn create(&self, o: &Order) -> Result<Order, DomainError> {
        sqlx::query_as!(Order,
            r#"INSERT INTO orders (id,reference_code,user_id,status,total_amount,currency,note,created_at,updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
               RETURNING id,reference_code,user_id,status as "status: OrderStatus",total_amount,currency,note,created_at,updated_at"#,
            o.id, o.reference_code, o.user_id, o.status as _,
            o.total_amount, o.currency, o.note, o.created_at, o.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Order, DomainError> {
        sqlx::query_as!(Order,
            r#"SELECT id,reference_code,user_id,status as "status: OrderStatus",total_amount,currency,note,created_at,updated_at
               FROM orders WHERE id=$1"#, id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Order {}", id)))
    }

    async fn find_by_reference(&self, reference: &str) -> Result<Order, DomainError> {
        sqlx::query_as!(Order,
            r#"SELECT id,reference_code,user_id,status as "status: OrderStatus",total_amount,currency,note,created_at,updated_at
               FROM orders WHERE reference_code=$1"#, reference
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Order ref {}", reference)))
    }

    async fn update_status(&self, id: Uuid, status: OrderStatus) -> Result<Order, DomainError> {
        sqlx::query_as!(Order,
            r#"UPDATE orders SET status=$2, updated_at=NOW() WHERE id=$1
               RETURNING id,reference_code,user_id,status as "status: OrderStatus",total_amount,currency,note,created_at,updated_at"#,
            id, status as _
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn list(&self, filters: OrderFilters) -> Result<Vec<Order>, DomainError> {
        sqlx::query_as!(Order,
            r#"SELECT id,reference_code,user_id,status as "status: OrderStatus",total_amount,currency,note,created_at,updated_at
               FROM orders
               WHERE ($1::uuid IS NULL OR user_id = $1)
               AND   ($2::order_status IS NULL OR status = $2)
               ORDER BY created_at DESC LIMIT $3 OFFSET $4"#,
            filters.user_id,
            filters.status as _,
            filters.limit,
            filters.offset
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }

    async fn count(&self, filters: &OrderFilters) -> Result<i64, DomainError> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) as cnt FROM orders
               WHERE ($1::uuid IS NULL OR user_id = $1)
               AND   ($2::order_status IS NULL OR status = $2)"#,
            filters.user_id,
            filters.status as _,
        )
        .fetch_one(&self.0).await.map_err(db_err)?;
        Ok(row.cnt.unwrap_or(0))
    }
}

pub struct PgOrderItemRepository(pub PgPool);

#[async_trait]
impl OrderItemRepository for PgOrderItemRepository {
    async fn create_many(&self, items: &[OrderItem]) -> Result<Vec<OrderItem>, DomainError> {
        let mut result = Vec::with_capacity(items.len());
        for item in items {
            let created = sqlx::query_as!(OrderItem,
                r#"INSERT INTO order_items (id,order_id,product_id,batch_id,occasion_product_id,quantity,unit_price)
                   VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *"#,
                item.id, item.order_id, item.product_id, item.batch_id,
                item.occasion_product_id, item.quantity, item.unit_price
            )
            .fetch_one(&self.0).await.map_err(db_err)?;
            result.push(created);
        }
        Ok(result)
    }

    async fn list_for_order(&self, order_id: Uuid) -> Result<Vec<OrderItem>, DomainError> {
        sqlx::query_as!(OrderItem,
            "SELECT * FROM order_items WHERE order_id=$1", order_id
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }
}

pub struct PgOrderDeliveryInfoRepository(pub PgPool);

#[async_trait]
impl OrderDeliveryInfoRepository for PgOrderDeliveryInfoRepository {
    async fn create(&self, d: &OrderDeliveryInfo) -> Result<OrderDeliveryInfo, DomainError> {
        sqlx::query_as!(OrderDeliveryInfo,
            r#"INSERT INTO order_delivery_info
                   (id,order_id,recipient_name,phone,street,ward,district,city,country,delivery_note,method,preferred_date,preferred_time_slot)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
               RETURNING id,order_id,recipient_name,phone,street,ward,district,city,country,delivery_note,
                         method as "method: DeliveryMethod",preferred_date,preferred_time_slot"#,
            d.id, d.order_id, d.recipient_name, d.phone, d.street, d.ward,
            d.district, d.city, d.country, d.delivery_note,
            d.method as _, d.preferred_date, d.preferred_time_slot
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_order(&self, order_id: Uuid) -> Result<OrderDeliveryInfo, DomainError> {
        sqlx::query_as!(OrderDeliveryInfo,
            r#"SELECT id,order_id,recipient_name,phone,street,ward,district,city,country,delivery_note,
                      method as "method: DeliveryMethod",preferred_date,preferred_time_slot
               FROM order_delivery_info WHERE order_id=$1"#,
            order_id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("DeliveryInfo for order {}", order_id)))
    }

    async fn update(&self, d: &OrderDeliveryInfo) -> Result<OrderDeliveryInfo, DomainError> {
        sqlx::query_as!(OrderDeliveryInfo,
            r#"UPDATE order_delivery_info
               SET recipient_name=$2,phone=$3,street=$4,ward=$5,district=$6,city=$7,
                   country=$8,delivery_note=$9,method=$10,preferred_date=$11,preferred_time_slot=$12
               WHERE id=$1
               RETURNING id,order_id,recipient_name,phone,street,ward,district,city,country,delivery_note,
                         method as "method: DeliveryMethod",preferred_date,preferred_time_slot"#,
            d.id, d.recipient_name, d.phone, d.street, d.ward,
            d.district, d.city, d.country, d.delivery_note,
            d.method as _, d.preferred_date, d.preferred_time_slot
        )
        .fetch_one(&self.0).await.map_err(db_err)
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
