// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "casino_game"))]
    pub struct CasinoGame;
}

diesel::table! {
    account_roles (id) {
        id -> Int4,
        account_id -> Nullable<Int4>,
        role_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    accounts (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    player_bets (id) {
        id -> Int4,
        player_id -> Nullable<Int4>,
        room_id -> Nullable<Int4>,
        bet_amount -> Numeric,
        #[max_length = 3]
        currency_code -> Bpchar,
        #[max_length = 50]
        bet_type -> Varchar,
        bet_details -> Nullable<Jsonb>,
        #[max_length = 20]
        outcome -> Nullable<Varchar>,
        payout_amount -> Nullable<Numeric>,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        room_id -> Nullable<Int4>,
        account_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        code -> Nullable<Varchar>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CasinoGame;

    rooms (id) {
        id -> Int4,
        #[max_length = 500]
        session_id -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        no_of_active_players -> Nullable<Int4>,
        no_of_players -> Nullable<Int4>,
        game_type -> CasinoGame,
        game_capacity -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 50]
        citizen_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(account_roles -> accounts (account_id));
diesel::joinable!(account_roles -> roles (role_id));
diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(player_bets -> players (player_id));
diesel::joinable!(player_bets -> rooms (room_id));
diesel::joinable!(players -> accounts (account_id));
diesel::joinable!(players -> rooms (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_roles,
    accounts,
    player_bets,
    players,
    roles,
    rooms,
    users,
);
