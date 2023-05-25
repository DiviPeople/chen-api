use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::FullName).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string())
                    .col(ColumnDef::new(Users::Salt).string())
                    .col(ColumnDef::new(Users::IsSuperuser).boolean().not_null())
                    .col(ColumnDef::new(Users::IsStaff).boolean().not_null())
                    .col(ColumnDef::new(Users::ImgUrl).string())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp())
                    .col(ColumnDef::new(Users::Integrations).json())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    FullName,
    Email,
    PasswordHash,
    Salt,
    IsSuperuser,
    IsStaff,
    ImgUrl,
    CreatedAt,
    UpdatedAt,
    Integrations,
}
