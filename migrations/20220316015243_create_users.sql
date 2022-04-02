CREATE TABLE Users (
    user_id UUID NOT NULL,
    password VARCHAR NOT NULL,
    ra VARCHAR(11) NOT NULL,
    email VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,

    CONSTRAINT pk_users PRIMARY KEY (user_id)
);
