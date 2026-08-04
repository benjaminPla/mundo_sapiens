CREATE TABLE sellers (
    company_name VARCHAR(50) NOT NULL,
    contact_name VARCHAR(50),
    email        VARCHAR(254),
    id           BIGSERIAL PRIMARY KEY,
    phone        VARCHAR(20)
);

CREATE TABLE designs (
    id        BIGSERIAL PRIMARY KEY,
    name      TEXT   NOT NULL,
    seller_id BIGINT NOT NULL REFERENCES sellers(id)
);

CREATE TABLE purchases (
    cost      DOUBLE PRECISION NOT NULL,
    date      DATE   NOT NULL,
    design_id BIGINT NOT NULL REFERENCES designs(id),
    id        BIGSERIAL PRIMARY KEY,
    seller_id BIGINT NOT NULL REFERENCES sellers(id)
);

CREATE TABLE batches (
    date         DATE   NOT NULL,
    design_id    BIGINT NOT NULL REFERENCES designs(id),
    id           BIGSERIAL PRIMARY KEY,
    purchase_id  BIGINT NOT NULL REFERENCES purchases(id),
    qty_produced BIGINT NOT NULL
);

CREATE TABLE stock_movements (
    batch_id   BIGINT NOT NULL REFERENCES batches(id),
    date       DATE   NOT NULL,
    from_state TEXT,
    id         BIGSERIAL PRIMARY KEY,
    note       TEXT,
    qty        BIGINT NOT NULL CHECK (qty > 0),
    to_state   TEXT   NOT NULL,
    CHECK (from_state IS NULL OR from_state IN ('Purchased', 'Magnetized', 'Cut', 'Ready', 'Sold')),
    CHECK (to_state IN ('Purchased', 'Magnetized', 'Cut', 'Ready', 'Sold'))
);

CREATE TABLE sales (
    date DATE NOT NULL,
    id   BIGSERIAL PRIMARY KEY,
    note TEXT
);

CREATE TABLE sale_lines (
    batch_id BIGINT NOT NULL REFERENCES batches(id),
    id       BIGSERIAL PRIMARY KEY,
    qty      BIGINT NOT NULL CHECK (qty > 0),
    sale_id  BIGINT NOT NULL REFERENCES sales(id)
);

CREATE INDEX stock_movements_batch_id_idx ON stock_movements(batch_id);
CREATE INDEX batches_design_id_idx ON batches(design_id);
