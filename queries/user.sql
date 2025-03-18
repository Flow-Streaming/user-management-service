
CREATE TABLE users (
    id uuid primary key,
    email varchar(255) not null unique,
    username varchar(100),
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now(),
    subscription_plan varchar(20) CHECK (subscription_plan IN ('basic', 'standard', 'premium')),
    profile_picture_url text,
);

-- Create an index on auth_id for faster lookups
CREATE INDEX idx_users_auth_id ON users(auth_id);

-- Create an index on email for faster lookups
CREATE INDEX idx_users_email ON users(email);
