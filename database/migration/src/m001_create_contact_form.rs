use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Create the Goal enum type (PostgreSQL only)
        manager
            .get_connection()
            .execute_unprepared(
                r#"DO $$
                    BEGIN
                        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'goal') THEN
                            CREATE TYPE goal AS ENUM (
                                'BuyFirstProperty',
                                'MaxCashFlow',
                                'Appreciation',
                                'Scale',
                                'Exchange'
                            );
                        END IF;
                    END$$;"#,
            )
            .await?;

        // Create the contacts table
        manager
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contacts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")) // Add default UUID generation
                    )
                    .col(ColumnDef::new(Contacts::FullName).string().not_null())
                    .col(ColumnDef::new(Contacts::Email).string().not_null())
                    .col(ColumnDef::new(Contacts::Phone).string().not_null())
                    .col(ColumnDef::new(Contacts::Market).string().not_null())
                    // Use custom_type instead of enumeration for PostgreSQL enum
                    .col(ColumnDef::new(Contacts::Goal)
                        .custom(Alias::new("goal"))
                        .not_null()
                    )
                    .col(ColumnDef::new(Contacts::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Contacts::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;


        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the table first
        manager
            .drop_table(
                Table::drop()
                    .table(Contacts::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        // Drop the enum type
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE IF EXISTS goal;")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
    FullName,
    Email,
    Phone,
    Market,
    Goal,
    CreatedAt,
    UpdatedAt,
}
