CREATE TABLE event
(
    id         VARCHAR(64) NOT NULL PRIMARY KEY,
    created_at INT         NOT NULL,
    kind       INT         NOT NULL,
    tags       jsonb       NOT NULL,
    content    TEXT        NOT NULL,
);