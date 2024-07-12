#![allow(non_snake_case)]

use dbsp::declare_tuples;
use sqlvalue::*;

declare_tuples! {
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
}

pipeline_types::deserialize_without_context!(
    Tup21, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20
);
pipeline_types::deserialize_without_context!(
    Tup22, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20, T21
);
pipeline_types::deserialize_without_context!(
    Tup14, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13
);

sqlvalue::to_sql_row_impl! {
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
}
