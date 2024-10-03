use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SCORE_MUSIC_ID_INDEX_NAME: &str = "idx-score-music-id";

async fn create_column<T: IntoIden>(schema_manager: &SchemaManager<'_>, column: T) -> Result<(), DbErr> {
    schema_manager
        .alter_table(
            Table::alter()
                .table(Score::Table)
                .add_column_if_not_exists(
                    ColumnDef::new(column)
                        .integer()
                        .default(0)
                )
                .to_owned()
        ).await
}

async fn drop_column<T: IntoIden>(schema_manager: &SchemaManager<'_>, column: T) -> Result<(), DbErr> {
    schema_manager
        .alter_table(
            Table::alter()
                .table(Score::Table)
                .drop_column(column)
                .to_owned()
        ).await
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_column(manager, Score::BreathAverageStrength).await?;
        create_column(manager, Score::TotalCloseHits).await?;
        create_column(manager, Score::TotalGreatHits).await?;
        create_column(manager, Score::TotalOkHits).await?;
        create_column(manager, Score::HighestBreathingDuration).await?;
        create_column(manager, Score::TotalBreathingDuration).await?;
        manager
            .create_index(
                Index::create()
                    .table(Score::Table)
                    .col(Score::MusicId)
                    .name(SCORE_MUSIC_ID_INDEX_NAME)
                    .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_column(manager, Score::BreathAverageStrength).await?;
        drop_column(manager, Score::TotalCloseHits).await?;
        drop_column(manager, Score::TotalGreatHits).await?;
        drop_column(manager, Score::TotalOkHits).await?;
        drop_column(manager, Score::HighestBreathingDuration).await?;
        drop_column(manager, Score::TotalBreathingDuration).await?;
        manager
            .drop_index(
                Index::drop()
                    .table(Score::Table)
                    .name(SCORE_MUSIC_ID_INDEX_NAME)
                    .to_owned()
            ).await?;
        
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Score {
    Table,
    MusicId,
    BreathAverageStrength,
    TotalBreathingDuration,
    HighestBreathingDuration,
    TotalGreatHits,
    TotalOkHits,
    TotalCloseHits
}
