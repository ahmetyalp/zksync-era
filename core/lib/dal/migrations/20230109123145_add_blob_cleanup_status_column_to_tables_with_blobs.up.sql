ALTER TABLE prover_jobs
    ADD COLUMN IF NOT EXISTS is_blob_cleaned BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE witness_inputs
    ADD COLUMN IF NOT EXISTS is_blob_cleaned BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE leaf_aggregation_witness_jobs
    ADD COLUMN IF NOT EXISTS is_blob_cleaned BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE node_aggregation_witness_jobs
    ADD COLUMN IF NOT EXISTS is_blob_cleaned BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE scheduler_witness_jobs
    ADD COLUMN IF NOT EXISTS is_blob_cleaned BOOLEAN NOT NULL DEFAULT FALSE;
