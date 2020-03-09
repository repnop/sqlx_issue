use sqlx::MySqlPool;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = MySqlPool::new("mysql://root:supersecret@mysql/testdb").await?;
    let mut connection = pool.begin().await?;

    sqlx::query(
        "
            INSERT INTO test_table
                (a, b, c, d, e, f, g, h, i, j, k, l)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                a = VALUES(a),
                b = VALUES(b),
                c = VALUES(c),
                d = VALUES(d),
                e = VALUES(e),
                f = VALUES(f),
                g = VALUES(g),
                h = VALUES(h),
                i = VALUES(i),
                j = VALUES(j),
                k = VALUES(k),
                l = VALUES(l)
        ",
    )
    .bind(&5i32)
    .bind(&"1234")
    .bind(&None::<i32>)
    .bind(&Some(10u8))
    .bind(&"test2")
    .bind(&"aaaa")
    .bind(&1i32)
    .bind(&2i32)
    .bind(&3i32)
    .bind(&4i32)
    .bind(&5i32)
    .bind(&6i32)
    .execute(&mut connection)
    .await?;

    println!("OK");

    Ok(())
}
