-- Revert numeric columns back to floating point values.
ALTER TABLE assets
    ALTER COLUMN unit_value TYPE DOUBLE PRECISION
    USING unit_value::double precision;

ALTER TABLE owned_assets
    ALTER COLUMN bought_for TYPE DOUBLE PRECISION
    USING bought_for::double precision,
    ALTER COLUMN quantity_owned TYPE DOUBLE PRECISION
    USING quantity_owned::double precision;
