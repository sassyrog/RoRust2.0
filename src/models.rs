use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json::Value as JsonValue;

use diesel_derive_newtype::DieselNewType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::game::game_types::*;

#[derive(Debug, Clone, PartialEq, Eq, DieselNewType, Serialize, Deserialize)]
pub struct DbGameType(GameType);

impl From<GameType> for DbGameType {
    fn from(game_type: GameType) -> Self {
        DbGameType(game_type)
    }
}

impl From<DbGameType> for GameType {
    fn from(db_game_type: DbGameType) -> Self {
        db_game_type.0
    }
}

impl DbGameType {
    pub fn inner(&self) -> &GameType {
        &self.0
    }
}

#[derive(Debug, DieselNewType, Serialize, Deserialize)]
pub struct DbDecimal(Decimal);

impl From<Decimal> for DbDecimal {
    fn from(d: Decimal) -> Self {
        DbDecimal(d)
    }
}

impl From<DbDecimal> for Decimal {
    fn from(d: DbDecimal) -> Self {
        d.0
    }
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::account_roles)]
pub struct AccountRole {
    pub id: i32,
    pub account_id: Option<i32>,
    pub role_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::account_roles)]
pub struct NewAccountRole {
    pub account_id: Option<i32>,
    pub role_id: Option<i32>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::account_roles)]
pub struct UpdateAccountRole {
    pub account_id: Option<i32>,
    pub role_id: Option<i32>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::accounts)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::accounts)]
pub struct NewAccount {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub is_active: Option<bool>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::accounts)]
pub struct UpdateAccount {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::player_bets)]
pub struct PlayerBet {
    pub id: i32,
    pub player_id: Option<i32>,
    pub room_id: Option<i32>,
    pub bet_amount: DbDecimal,
    pub currency_code: String,
    pub bet_type: String,
    pub bet_details: Option<JsonValue>,
    pub outcome: Option<String>,
    pub payout_amount: Option<DbDecimal>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::player_bets)]
pub struct NewPlayerBet {
    pub player_id: Option<i32>,
    pub room_id: Option<i32>,
    pub bet_amount: DbDecimal,
    pub currency_code: String,
    pub bet_type: String,
    pub bet_details: Option<JsonValue>,
    pub status: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::player_bets)]
pub struct UpdatePlayerBet {
    pub outcome: Option<String>,
    pub payout_amount: Option<DbDecimal>,
    pub status: Option<String>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::players)]
pub struct Player {
    pub id: i32,
    pub room_id: Option<i32>,
    pub account_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::players)]
pub struct NewPlayer {
    pub room_id: Option<i32>,
    pub account_id: Option<i32>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::players)]
pub struct UpdatePlayer {
    pub room_id: Option<i32>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
pub struct Role {
    pub id: i32,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
pub struct UpdateRole {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::rooms)]
pub struct Room {
    pub id: i32,
    pub session_id: Option<String>,
    pub active: Option<bool>,
    pub no_of_active_players: Option<i32>,
    pub no_of_players: Option<i32>,
    pub game_type: DbGameType,
    pub game_capacity: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rooms)]
pub struct NewRoom {
    pub session_id: Option<String>,
    pub active: Option<bool>,
    pub game_type: DbGameType,
    pub game_capacity: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::rooms)]
pub struct UpdateRoom {
    pub session_id: Option<String>,
    pub active: Option<bool>,
    pub no_of_active_players: Option<i32>,
    pub no_of_players: Option<i32>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub citizen_id: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub citizen_id: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub citizen_id: Option<String>,
}
