-- "mutable" user data, which includes display names, e-mails, etc
CREATE TABLE userdata (
    id UUID UNIQUE PRIMARY KEY NOT NULL REFERENCES users,
    -- Profile
    user_display VARCHAR NOT NULL,
    user_url VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    dob DATE NOT NULL,
    location VARCHAR NOT NULL,
    socials JSONB NOT NULL,
    profile_image JSONB NOT NULL,
    cover_image JSONB NOT NULL,
    bio TEXT NOT NULL,

    -- Statistics
    join_date TIMESTAMP NOT NULL,
    last_login TIMESTAMP NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    followers_count INT NOT NULL DEFAULT 0,
    following_count INT NOT NULL DEFAULT 0,
    post_count INT NOT NULL DEFAULT 0,

    -- User Settings
    prefs JSONB NOT NULL
)