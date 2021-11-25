-- Your SQL goes here

Create Table streams(
   pda_account Varchar PRIMARY KEY,
    start_time BIGINT NOT NUll,
    end_time BIGINT NOT NUll,
    receiver Varchar Not NUll,
    lamports_withdrawn BIGINT NOT NUll,
    amount_second BIGINT NOT NUll,
    sender Varchar Not NUll, 
    total_amount BIGINT NOT NUll
)