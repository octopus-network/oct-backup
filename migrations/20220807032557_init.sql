-- Add migration script here

CREATE TABLE "public"."validator_set"
(
    "appchain_id"                varchar(20)    NOT NULL,
    "era_number"                 numeric(20, 0) NOT NULL,
    "total_stake"                numeric(40, 0) NOT NULL,
    "start_block_height"         numeric(20, 0) NOT NULL,
    "start_timestamp"            numeric(20, 0) NOT NULL,
    "staking_history_index"      numeric(20, 0) NOT NULL,
    "unprofitable_validator_ids" text           NOT NULL,
    "valid_total_stake"          numeric(40, 0) NOT NULL,
    "processing_status"          jsonb          NOT NULL,
    "start_timestamp_date"       timestamp      NOT NULL,
    "update_time"                timestamp      NOT NULL,
    PRIMARY KEY ("appchain_id", "era_number")
);



CREATE TABLE public.validator_infos
(
    appchain_id               varchar(25) NOT NULL,
    era_number                decimal(20) NOT NULL,
    validator_id              varchar(64) NOT NULL,
    validator_id_in_appchain  varchar(66) NOT NULL,
    deposit_amount            decimal(40) NOT NULL,
    can_be_delegated_to       boolean     NOT NULL,
    is_unbonding              boolean     NOT NULL,
    total_reward              decimal(40) NOT NULL,
    unwithdrawn_reward        decimal(40) NOT NULL,
    update_date               timestamp   NOT NULL,
    PRIMARY KEY (appchain_id, era_number, validator_id)
);


CREATE TABLE public.delegator_infos
(
    appchain_id               varchar(25) NOT NULL,
    era_number                decimal(20) NOT NULL,
    validator_id              varchar(64) NOT NULL,
    delegator_id              varchar(64) NOT NULL,
    deposit_amount            decimal(20) NOT NULL,
    total_reward              decimal(40) NOT NULL,
    unwithdrawn_reward        decimal(40) NOT NULL,
    update_date               timestamp   NOT NULL,
    PRIMARY KEY (appchain_id, era_number, validator_id, delegator_id)
);


CREATE TABLE public.staking_histories
(
    staking_fact   jsonb       NOT NULL,
    block_height   decimal(20) NOT NULL,
    timestamp    decimal(20) NOT NULL,
    index        decimal(20) NOT NULL,
    timestamp_date timestamp   NOT NULL,
    update_date    timestamp   NOT NULL,
    appchain_id    varchar(25) NOT NULL,
    PRIMARY KEY (appchain_id)
);

