create schema rinha;

create table pessoa (
    id uuid NOT NULL,
    nome character varying NOT NULL,
    cpfcnpj character varying NOT NULL,
    nascimento date,
    seguros text[],
    UNIQUE(cpfcnpj)
);

CREATE INDEX nome_idx ON pessoa (nome) INCLUDE (nome, seguros);