DROP TABLE IF EXISTS tests;
CREATE TABLE tests(  
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    author_id int NOT NULL,
    deleted BOOLEAN DEFAULT FALSE,
    created DATE DEFAULT CURRENT_DATE
);
-- COMMENT ON TABLE  IS '';
-- COMMENT ON COLUMN .name IS '';