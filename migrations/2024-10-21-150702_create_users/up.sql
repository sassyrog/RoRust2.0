CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    citizen_id VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS accounts (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    last_login TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_accounts_users FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    code VARCHAR(50) UNIQUE,
    name VARCHAR(100),
    description VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS account_roles (
    id SERIAL PRIMARY KEY,
    account_id INT REFERENCES accounts(id),
    role_id INT REFERENCES roles(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT unique_account_role UNIQUE (account_id, role_id)
);

CREATE TYPE CASINO_GAME AS ENUM (
    'AMERICAN_ROULETTE',
    'EUROPEAN_ROULETTE',
    'FRENCH_ROULETTE',
    'BLACKJACK',
    'BACCARAT',
    'POKER_TEXAS_HOLDEM',
    'POKER_OMAHA',
    'POKER_SEVEN_CARD_STUD',
    'POKER_CARIBBEAN_STUD',
    'VIDEO_POKER',
    'CRAPS',
    'SICIWIN_LOTTERY',
    'KENO',
    'BINGO',
    'SLOT_MACHINES',
    'VIDEO_SLOTS',
    'PROGRESSIVE_SLOTS',
    'PAI_GOW_POKER',
    'PAI_GOW_TILES',
    'SPORTSWIN_BETTING',
    'HORSE_RACING',
    'DOG_RACING',
    'WHEEL_OF_FORTUNE',
    'PACHINKO',
    'MAHJONG',
    'FANTAN',
    'SICIWIN_GAMES',
    'THREE_CARD_POKER',
    'FOUR_CARD_POKER',
    'LET_IT_RIDE',
    'CASINO_WAR',
    'RED_DOG',
    'PONTOON',
    'SPANISH_21',
    'CHUCK_A_LUCK',
    'BIG_SIX_WHEEL',
    'SICIWIN_DICE',
    'BACKGAMMON',
    'BINGO_MATCH',
    'FAST_LOTTERY'
);

CREATE TABLE IF NOT EXISTS rooms (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(500) UNIQUE,
    active BOOLEAN DEFAULT TRUE,
    no_of_active_players INT DEFAULT 0,
    no_of_players INT DEFAULT 0,
    game_type CASINO_GAME NOT NULL,
    game_capacity INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT check_player_counts CHECK (no_of_active_players <= no_of_players AND no_of_players <= game_capacity)
);

CREATE TABLE IF NOT EXISTS players (
    id SERIAL PRIMARY KEY,
    room_id INT REFERENCES rooms(id),
    account_id INT REFERENCES accounts(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS player_bets (
    id SERIAL PRIMARY KEY,
    player_id INT REFERENCES players(id),
    room_id INT REFERENCES rooms(id),
    bet_amount DECIMAL(15, 2) NOT NULL,
    currency_code CHAR(3) NOT NULL,
    bet_type VARCHAR(50) NOT NULL,
    bet_details TEXT,
    outcome VARCHAR(20),
    payout_amount DECIMAL(15, 2),
    status VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_bet_amount_positive CHECK (bet_amount > 0),
    CONSTRAINT check_payout_amount_non_negative CHECK (payout_amount >= 0)
);

-- Add indexes
CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_account_roles_account_id ON account_roles(account_id);
CREATE INDEX idx_account_roles_role_id ON account_roles(role_id);
CREATE INDEX idx_players_room_id ON players(room_id);
CREATE INDEX idx_players_account_id ON players(account_id);
CREATE INDEX idx_player_bets_player_id ON player_bets(player_id);
CREATE INDEX idx_player_bets_room_id ON player_bets(room_id);