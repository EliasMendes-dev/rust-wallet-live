-- Convert financial columns from floating point to exact numeric values.
ALTER TABLE assets
    ALTER COLUMN unit_value TYPE NUMERIC
    USING unit_value::numeric;

ALTER TABLE owned_assets
    ALTER COLUMN bought_for TYPE NUMERIC
    USING bought_for::numeric,
    ALTER COLUMN quantity_owned TYPE NUMERIC
    USING quantity_owned::numeric;
