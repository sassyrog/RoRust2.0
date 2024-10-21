-- Drop indexes first
DROP INDEX IF EXISTS idx_player_bets_room_id;
DROP INDEX IF EXISTS idx_player_bets_player_id;
DROP INDEX IF EXISTS idx_players_account_id;
DROP INDEX IF EXISTS idx_players_room_id;
DROP INDEX IF EXISTS idx_account_roles_role_id;
DROP INDEX IF EXISTS idx_account_roles_account_id;
DROP INDEX IF EXISTS idx_accounts_user_id;

DROP TABLE IF EXISTS player_bets;
DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS account_roles;
DROP TABLE IF EXISTS accounts;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS CASINO_GAME;