use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

async fn add_col<T: IntoIden>(schema_manager: &SchemaManager<'_>, col: T) -> Result<(), DbErr> {
    schema_manager
        .alter_table(
            Table::alter()
                .table(Score::Table)
                .add_column_if_not_exists(
                    ColumnDef::new(col)
                        .integer()
                        .default(0)
                )
                .to_owned()
        ).await
}

async fn drop_col<T: IntoIden>(schema_manager: &SchemaManager<'_>, col: T) -> Result<(), DbErr> {
    schema_manager
        .alter_table(
            Table::alter()
                .table(Score::Table)
                .drop_column(col)
                .to_owned()
        ).await
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        add_col(manager, Score::TotalEarlyMisses).await?;
        add_col(manager, Score::TotalMisses).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_col(manager, Score::TotalEarlyMisses).await?;
        drop_col(manager, Score::TotalMisses).await
    }
}

#[derive(DeriveIden)]
enum Score {
    Table,
    TotalMisses,
    TotalEarlyMisses
}
