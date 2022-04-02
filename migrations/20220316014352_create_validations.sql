CREATE TABLE Validations (
  validation_id uuid NOT NULL,
	email VARCHAR NOT NULL,
	hashed_code VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL,
	CONSTRAINT pk_validations PRIMARY KEY (validation_id)
);
