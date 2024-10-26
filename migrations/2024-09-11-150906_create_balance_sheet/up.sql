-- Your SQL goes here
CREATE TABLE assets (
                        id BIGINT AUTO_INCREMENT PRIMARY KEY,
                        name VARCHAR(255) NOT NULL,
                        debit DOUBLE NOT NULL,
                        credit DOUBLE NOT NULL
);

CREATE TABLE liabilities (
                             id BIGINT AUTO_INCREMENT PRIMARY KEY,
                             name VARCHAR(255) NOT NULL,
                             debit DOUBLE NOT NULL,
                             credit DOUBLE NOT NULL
);

CREATE TABLE equity (
                        id BIGINT AUTO_INCREMENT PRIMARY KEY,
                        name VARCHAR(255) NOT NULL,
                        debit DOUBLE NOT NULL,
                        credit DOUBLE NOT NULL
);
