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
                    .col(text(Music::Name).unique_key())
                    .col(integer(Music::Duration))
                    .col(text(Music::Directory).unique_key())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .if_not_exists()
                    .col(pk_auto(Score::Id))
                    .col(integer(Score::Total))
                    .col(timestamp(Score::Date))
                    .col(boolean(Score::Completed))
                    .col(integer(Score::HighestStreak))
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
        manager
            .create_index(
                Index::create()
                    .table(Music::Table)
                    .col(Music::Name)
                    .name("idx-music-name")
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Music::Table)
                    .col(Music::Directory)
                    .name("idx-music-directory")
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
    Duration,
    Directory,
}

#[derive(DeriveIden)]
enum Score {
    Table,
    Id,
    Total,
    Date,
    HighestStreak,
    Completed,
    MusicId,
}
