CREATE TABLE sellers (
    id      BIGSERIAL PRIMARY KEY,
    name    TEXT NOT NULL,
    contact TEXT
);

CREATE TABLE designs (
    id         BIGSERIAL PRIMARY KEY,
    name       TEXT NOT NULL,
    seller_id  BIGINT NOT NULL REFERENCES sellers(id),
    image_path TEXT
);

CREATE TABLE purchases (
    id        BIGSERIAL PRIMARY KEY,
    design_id BIGINT NOT NULL REFERENCES designs(id),
    seller_id BIGINT NOT NULL REFERENCES sellers(id),
    cost      DOUBLE PRECISION NOT NULL,
    date      DATE NOT NULL
);

CREATE TABLE batches (
    id           BIGSERIAL PRIMARY KEY,
    design_id    BIGINT NOT NULL REFERENCES designs(id),
    purchase_id  BIGINT NOT NULL REFERENCES purchases(id),
    qty_produced BIGINT NOT NULL,
    date         DATE NOT NULL
);

CREATE TABLE stock_movements (
    id         BIGSERIAL PRIMARY KEY,
    batch_id   BIGINT NOT NULL REFERENCES batches(id),
    from_state TEXT,
    to_state   TEXT NOT NULL,
    qty        BIGINT NOT NULL CHECK (qty > 0),
    date       DATE NOT NULL,
    note       TEXT,
    CHECK (from_state IS NULL OR from_state IN ('Purchased', 'Magnetized', 'Cut', 'Ready', 'Sold')),
    CHECK (to_state IN ('Purchased', 'Magnetized', 'Cut', 'Ready', 'Sold'))
);

CREATE TABLE sales (
    id   BIGSERIAL PRIMARY KEY,
    date DATE NOT NULL,
    note TEXT
);

CREATE TABLE sale_lines (
    id       BIGSERIAL PRIMARY KEY,
    sale_id  BIGINT NOT NULL REFERENCES sales(id),
    batch_id BIGINT NOT NULL REFERENCES batches(id),
    qty      BIGINT NOT NULL CHECK (qty > 0)
);

CREATE INDEX stock_movements_batch_id_idx ON stock_movements(batch_id);
CREATE INDEX batches_design_id_idx ON batches(design_id);
