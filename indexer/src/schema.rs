table! {
    streams (pda_account) {
        pda_account -> Varchar,
        start_time -> Int8,
        end_time -> Int8,
        receiver -> Varchar,
        lamports_withdrawn -> Int8,
        amount_second -> Int8,
        sender -> Varchar,
        total_amount -> Int8,
    }
}
