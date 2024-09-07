use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Music::Table)
                    .if_not_exists()
                    .col(pk_auto(Music::Id))
                    .col(text(Music::Name))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .if_not_exists()
                    .col(integer(Score::Total))
                    .col(timestamp(Score::Date))
                    .col(boolean(Score::Completed))
                    .col(integer(Score::MusicId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-score-music_id")
                            .from(Score::Table, Score::MusicId)
                            .to(Music::Table, Music::Id),
                    )
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Music::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Score::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Music {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Score {
    Table,
    Total,
    Date,
    Completed,
    MusicId,
}
