CREATE SCHEMA core;

CREATE TYPE core.container_type_enum AS ENUM (
    'Fish',
    'Vegetable',
    'Other'
);

CREATE TABLE core.user
(
    id       SERIAL PRIMARY KEY,
    username character varying NOT NULL,
    email    character varying NOT NULL,
    password character varying NOT NULL
);

CREATE TABLE core.container
(
    id       SERIAL PRIMARY KEY,
    name     character varying                    NOT NULL,
    container_type     core.container_type_enum DEFAULT 'Other'::core.container_type_enum NOT NULL,
    volume   BIGINT                              NOT NULL,
    red      integer                  DEFAULT 125 NOT NULL,
    green    integer                  DEFAULT 125 NOT NULL,
    blue     integer                  DEFAULT 125 NOT NULL,
    user_id integer NOT NULL,
    FOREIGN KEY (user_id) REFERENCES core.user (id)
);

CREATE TABLE core.probe
(
    id            SERIAL PRIMARY KEY,
    name          character varying NOT NULL,
    unit          character varying DEFAULT 'celsius':: character varying NOT NULL,
    probe_type          character varying DEFAULT 'heat':: character varying NOT NULL,
    min           double precision  NOT NULL,
    max           double precision  NOT NULL,
    container_id integer NOT NULL,
    FOREIGN KEY (container_id) REFERENCES core.container (id)
);

CREATE TABLE core."probingEntry"
(
    id          SERIAL PRIMARY KEY,
    value       double precision                       NOT NULL,
    "timestamp" timestamp with time zone DEFAULT now() NOT NULL,
    probe_id   integer NOT NULL,
    FOREIGN KEY (probe_id) REFERENCES core.probe (id)
);