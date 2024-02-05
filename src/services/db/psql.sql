CREATE TABLE users (
    id UUID PRIMARY KEY, 
    email TEXT UNIQUE NOT NULL, 
    password TEXT NOT NULL
);

CREATE TABLE sessions (
    userid UUID NOT NULL, 
    token UUID PRIMARY KEY, 
    valid_until TIMESTAMPTZ NOT NULL
);

INSERT INTO users VALUES 
    (gen_random_uuid(), 'ada7024@gmail.com', 'tfcytcvvb'),
    (gen_random_uuid(), 'bob212d3@gmail.com', 'oijoguyvbj'),
    (gen_random_uuid(), 'catherine23113nds@outlook.com', 'ligy7985tsx9cyt'),
    (gen_random_uuid(), 'dorisdeng23196@qq.com', 'ibh7659765yftd'),
    (gen_random_uuid(), 'ellenkimbereal@gmail.com', 'hg8676fvyb9');

