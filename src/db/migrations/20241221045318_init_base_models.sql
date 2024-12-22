-- Add migration script here
CREATE TABLE users (
    addr VARCHAR(42) PRIMARY KEY,
    total_lp BIGINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

-- 20240220000002_create_sessions.sql
CREATE TABLE sessions (
    user_addr VARCHAR(42) NOT NULL REFERENCES users(addr),
    start_time BIGINT NOT NULL,
    end_time BIGINT,
    earned_lp BIGINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    PRIMARY KEY (user_addr, start_time)
);
