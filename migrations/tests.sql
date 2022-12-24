DROP TABLE IF EXISTS tests;
CREATE TABLE tests(  
    id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    author_id int NOT NULL,
    deleted BOOL DEFAULT FALSE,
    created DATE DEFAULT NOW()
);
-- COMMENT ON TABLE  IS '';
-- COMMENT ON COLUMN .name IS '';