CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name varchar(255) NOT NULL,
    last_name varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    email_verified TIMESTAMPTZ,
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE accounts (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    provider varchar(255) NOT NULL,
    google_id varchar(255),
    password varchar(255),
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, provider)
);
CREATE TABLE sessions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id uuid NOT NULL REFERENCES accounts(id),
    token varchar(255) NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expire_date TIMESTAMPTZ NOT NULL,
    device varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    UNIQUE (email, token)
);
CREATE TABLE email_verify_codes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    code varchar(255) NOT NULL,
    expire_date TIMESTAMPTZ NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE password_reset_codes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    code varchar(255) NOT NULL,
    expire_date TIMESTAMPTZ NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE ingredient_items (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar(255) NOT NULL,
    creator_id uuid NOT NULL REFERENCES users(id),
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE meal_items (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar(255) NOT NULL,
    ingredient_items uuid [] NOT NULL,
    instructions text NOT NULL,
    creator_id uuid NOT NULL REFERENCES users(id),
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE calendar_items (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    shared_with uuid [] NOT NULL,
    meal_item_id uuid REFERENCES meal_items(id),
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);