// Automatically-generated file
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unconditional_panic)]

#![allow(non_camel_case_types)]

#[cfg(test)]
use hashing::*;
use dbsp::{
    algebra::{ZSet, MulByRef, F32, F64, Semigroup, SemigroupValue, ZRingValue,
         UnimplementedSemigroup, DefaultSemigroup, HasZero, AddByRef, NegByRef,
         AddAssignByRef,
    },
    circuit::{checkpointer::Checkpoint, Circuit, CircuitConfig, Stream},
    operator::{
        Generator,
        FilterMap,
        Fold,
        time_series::{RelRange, RelOffset, OrdPartitionedIndexedZSet},
        MaxSemigroup,
        MinSemigroup,
        CmpFunc,
    },
    OrdIndexedZSet, OrdZSet,
    TypedBox,
    utils::*,
    zset,
    indexed_zset,
    DBWeight,
    DBData,
    DBSPHandle,
    Error,
    Runtime,
    NumEntries,
    MapHandle, ZSetHandle, OutputHandle,
    dynamic::{DynData,DynDataTyped},
};
use dbsp_adapters::Catalog;
use pipeline_types::{deserialize_table_record, serialize_table_record};
use size_of::*;
use ::serde::{Deserialize,Serialize};
use compare::{Compare, Extract};
use std::{
    collections::BTreeMap,
    convert::identity,
    ops::Neg,
    fmt::{Debug, Formatter, Result as FmtResult},
    path::Path,
    marker::PhantomData,
};
use core::cmp::Ordering;
use rust_decimal::Decimal;
use dbsp::declare_tuples;
use json::*;
use sqllib::{
    *,
    array::*,
    casts::*,
    binary::*,
    geopoint::*,
    timestamp::*,
    interval::*,
    string::*,
    operators::*,
    aggregates::*,
};
use sqlvalue::*;
#[cfg(test)]
use readers::*;
#[cfg(test)]
use sqlx::{AnyConnection, any::AnyRow, Row};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[allow(non_upper_case_globals)]
#[export_name = "malloc_conf"]
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0";

declare_tuples! {
    Tup16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>,
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    Tup15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>,
}

pipeline_types::deserialize_without_context!(Tup16, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
pipeline_types::deserialize_without_context!(Tup21, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
pipeline_types::deserialize_without_context!(Tup14, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
pipeline_types::deserialize_without_context!(Tup15, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);

sqlvalue::to_sql_row_impl! {
    Tup16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>,
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    Tup15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>,
}


pub fn circuit(cconf: CircuitConfig) -> Result<(DBSPHandle, (ZSetHandle<Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup2<Option<i32>, Option<Decimal>>>, ZSetHandle<Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<i32>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>, ZSetHandle<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>, ZSetHandle<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>, OutputHandle<WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>>, OutputHandle<WSet<Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>>, OutputHandle<WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>>, OutputHandle<WSet<Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>>, OutputHandle<WSet<Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>>>, OutputHandle<WSet<Tup14<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>, OutputHandle<WSet<Tup8<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Timestamp>, Option<Decimal>, Option<String>>>>, )), Error> {

    let (circuit, streams) = Runtime::init_circuit(cconf, |circuit| {
        // CREATE TABLE `WAREHOUSE_STATIC` (`W_ID` INTEGER PRIMARY KEY, `W_NAME` VARCHAR(10), `W_STREET_1` VARCHAR(20), `W_STREET_2` VARCHAR(20), `W_CITY` VARCHAR(20), `W_STATE` CHAR(2), `W_ZIP` CHAR(9), `W_TAX` DECIMAL(4, 4))
        // DBSPSourceMultisetOperator 49
        // CREATE TABLE `WAREHOUSE_STATIC` (`W_ID` INTEGER PRIMARY KEY, `W_NAME` VARCHAR(10), `W_STREET_1` VARCHAR(20), `W_STREET_2` VARCHAR(20), `W_CITY` VARCHAR(20), `W_STATE` CHAR(2), `W_ZIP` CHAR(9), `W_TAX` DECIMAL(4, 4))
        let (stream49, handle49) = circuit.add_input_zset::<Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>();

        // CREATE TABLE `WAREHOUSE` (`W_ID` INTEGER PRIMARY KEY, `W_YTD` DECIMAL(12, 2))
        // DBSPSourceMultisetOperator 67
        // CREATE TABLE `WAREHOUSE` (`W_ID` INTEGER PRIMARY KEY, `W_YTD` DECIMAL(12, 2))
        let (stream67, handle67) = circuit.add_input_zset::<Tup2<Option<i32>, Option<Decimal>>>();

        // CREATE TABLE `DISTRICT_STATIC` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NAME` VARCHAR(10), `D_STREET_1` VARCHAR(20), `D_STREET_2` VARCHAR(20), `D_CITY` VARCHAR(20), `D_STATE` CHAR(2), `D_ZIP` CHAR(9), `D_TAX` DECIMAL(4, 4), PRIMARY KEY (`D_W_ID`, `D_ID`))
        // DBSPSourceMultisetOperator 120
        // CREATE TABLE `DISTRICT_STATIC` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NAME` VARCHAR(10), `D_STREET_1` VARCHAR(20), `D_STREET_2` VARCHAR(20), `D_CITY` VARCHAR(20), `D_STATE` CHAR(2), `D_ZIP` CHAR(9), `D_TAX` DECIMAL(4, 4), PRIMARY KEY (`D_W_ID`, `D_ID`))
        let (stream120, handle120) = circuit.add_input_zset::<Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>();

        // CREATE TABLE `DISTRICT_NEXT_ID` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NEXT_O_ID` INTEGER, PRIMARY KEY (`D_W_ID`, `D_ID`))
        // DBSPSourceMultisetOperator 143
        // CREATE TABLE `DISTRICT_NEXT_ID` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NEXT_O_ID` INTEGER, PRIMARY KEY (`D_W_ID`, `D_ID`))
        let (stream143, handle143) = circuit.add_input_zset::<Tup3<Option<i32>, Option<i32>, Option<i32>>>();

        // CREATE TABLE `DISTRICT_YTD` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_YTD` DECIMAL(12, 2), PRIMARY KEY (`D_W_ID`, `D_ID`))
        // DBSPSourceMultisetOperator 166
        // CREATE TABLE `DISTRICT_YTD` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_YTD` DECIMAL(12, 2), PRIMARY KEY (`D_W_ID`, `D_ID`))
        let (stream166, handle166) = circuit.add_input_zset::<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>();

        // CREATE TABLE `CUSTOMER` (`C_ID` INTEGER, `C_D_ID` INTEGER, `C_W_ID` INTEGER, `C_FIRST` VARCHAR(16), `C_MIDDLE` CHAR(2), `C_LAST` VARCHAR(16), `C_STREET_1` VARCHAR(20), `C_STREET_2` VARCHAR(20), `C_CITY` VARCHAR(20), `C_STATE` CHAR(2), `C_ZIP` CHAR(9), `C_PHONE` CHAR(16), `C_SINCE` TIMESTAMP, `C_CREDIT` CHAR(2), `C_CREDIT_LIM` DECIMAL(12, 2), `C_DISCOUNT` DECIMAL(4, 4), `C_BALANCE` DECIMAL(12, 2), `C_YTD_PAYMENT` DECIMAL(12, 2), `C_PAYMENT_CNT` INTEGER, `C_DELIVERY_CNT` INTEGER, `C_DATA` VARCHAR(500), PRIMARY KEY (`C_W_ID`, `C_D_ID`, `C_ID`))
        // DBSPSourceMultisetOperator 279
        // CREATE TABLE `CUSTOMER` (`C_ID` INTEGER, `C_D_ID` INTEGER, `C_W_ID` INTEGER, `C_FIRST` VARCHAR(16), `C_MIDDLE` CHAR(2), `C_LAST` VARCHAR(16), `C_STREET_1` VARCHAR(20), `C_STREET_2` VARCHAR(20), `C_CITY` VARCHAR(20), `C_STATE` CHAR(2), `C_ZIP` CHAR(9), `C_PHONE` CHAR(16), `C_SINCE` TIMESTAMP, `C_CREDIT` CHAR(2), `C_CREDIT_LIM` DECIMAL(12, 2), `C_DISCOUNT` DECIMAL(4, 4), `C_BALANCE` DECIMAL(12, 2), `C_YTD_PAYMENT` DECIMAL(12, 2), `C_PAYMENT_CNT` INTEGER, `C_DELIVERY_CNT` INTEGER, `C_DATA` VARCHAR(500), PRIMARY KEY (`C_W_ID`, `C_D_ID`, `C_ID`))
        let (stream279, handle279) = circuit.add_input_zset::<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>();

        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        // DBSPSourceMultisetOperator 332
        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        let (stream332, handle332) = circuit.add_input_zset::<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>();

        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPFilterOperator 360
        let stream360: Stream<_, WSet<Tup2<Option<i32>, Option<Decimal>>>> = stream67.filter(move |t_1: &Tup2<Option<i32>, Option<Decimal>>, | -> 
        bool {
            (!(*t_1).0.is_none())
        });
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPFilterOperator 373
        let stream373: Stream<_, WSet<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream332.filter(move |t_2: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        bool {
            (!(*t_2).1.is_none())
        });
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPMapIndexOperator 451
        let stream451: Stream<_, IndexedWSet<Tup1<i32>, Tup2<Option<i32>, Option<Decimal>>>> = stream360.map_index(move |t_3: &Tup2<Option<i32>, Option<Decimal>>, | -> 
        (Tup1<i32>, Tup2<Option<i32>, Option<Decimal>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_3).0)), Tup2::new((*t_3).0, (*t_3).1.clone()), )
        });
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPMapIndexOperator 492
        let stream492: Stream<_, IndexedWSet<Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream373.map_index(move |t_4: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_4).1)), Tup9::new((*t_4).0, (*t_4).1, (*t_4).2, (*t_4).3, (*t_4).4, (*t_4).5, (*t_4).6.clone(), (*t_4).7, (*t_4).8), )
        });
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPStreamJoinOperator 3610(555)
        let stream3610: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> = stream451.stream_join(&stream492, move |t_5: &Tup1<i32>, t_3: &Tup2<Option<i32>, Option<Decimal>>, t_4: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup3<Option<i32>, Option<i32>, Option<Decimal>> {
            Tup3::new((*t_4).0, (*t_3).0, plus_decimalN_decimalN(cast_to_decimalN_decimalN((*t_3).1.clone().clone(), 13, 2), cast_to_decimalN_decimalN((*t_4).6.clone().clone(), 13, 2)))
        });
        // CREATE VIEW `WAREHOUSE_UPDATES` AS
        // SELECT `T`.`TXN_ID`, `W`.`W_ID`, `W`.`W_YTD` + `T`.`H_AMOUNT` AS `W_YTD`
        // FROM `schema`.`WAREHOUSE` AS `W`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `W`.`W_ID` = `T`.`W_ID`
        // DBSPSinkOperator 19234(573)
        let handle19234 = stream3610.output();

        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPFilterOperator 607
        let stream607: Stream<_, WSet<Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>> = stream49.filter(move |t_8: &Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, | -> 
        bool {
            (!(*t_8).0.is_none())
        });
        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPMapIndexOperator 746
        let stream746: Stream<_, IndexedWSet<Tup1<i32>, Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>> = stream607.map_index(move |t_10: &Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, | -> 
        (Tup1<i32>, Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_10).0)), Tup8::new((*t_10).0, (*t_10).1.clone(), (*t_10).2.clone(), (*t_10).3.clone(), (*t_10).4.clone(), (*t_10).5.clone(), (*t_10).6.clone(), (*t_10).7.clone()), )
        });
        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPMapIndexOperator 26082(787)
        let stream26082: Stream<_, IndexedWSet<Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream373.map_index(move |t_11: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_11).1)), Tup9::new((*t_11).0, (*t_11).1, (*t_11).2, (*t_11).3, (*t_11).4, (*t_11).5, (*t_11).6.clone(), (*t_11).7, (*t_11).8), )
        });
        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPStreamJoinOperator 26086(862)
        let stream26086: Stream<_, WSet<Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream746.stream_join(&stream26082, move |t_12: &Tup1<i32>, t_10: &Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, t_11: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>> {
            Tup7::new((*t_10).2.clone().clone(), (*t_10).3.clone().clone(), (*t_10).4.clone().clone(), (*t_10).5.clone().clone(), (*t_10).6.clone().clone(), (*t_10).1.clone().clone(), (*t_11).0)
        });
        // CREATE VIEW `WAREHOUSE_SELECT` AS
        // SELECT `W`.`W_STREET_1`, `W`.`W_STREET_2`, `W`.`W_CITY`, `W`.`W_STATE`, `W`.`W_ZIP`, `W`.`W_NAME`, `T`.`TXN_ID`
        // FROM `schema`.`WAREHOUSE_STATIC` AS `W`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `W`.`W_ID` = `T`.`W_ID`
        // DBSPSinkOperator 26088(888)
        let handle26088 = stream26086.output();

        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPFilterOperator 927
        let stream927: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> = stream166.filter(move |t_15: &Tup3<Option<i32>, Option<i32>, Option<Decimal>>, | -> 
        bool {
            (!or_b_b((*t_15).0.is_none(), (*t_15).1.is_none()))
        });
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPFilterOperator 946
        let stream946: Stream<_, WSet<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream332.filter(move |t_16: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        bool {
            (!or_b_b((*t_16).1.is_none(), (*t_16).2.is_none()))
        });
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPMapIndexOperator 1038
        let stream1038: Stream<_, IndexedWSet<Tup2<i32, i32>, Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> = stream927.map_index(move |t_17: &Tup3<Option<i32>, Option<i32>, Option<Decimal>>, | -> 
        (Tup2<i32, i32>, Tup3<Option<i32>, Option<i32>, Option<Decimal>>, ) {
            (Tup2::new(cast_to_i32_i32N((*t_17).1), cast_to_i32_i32N((*t_17).0)), Tup3::new((*t_17).0, (*t_17).1, (*t_17).2.clone()), )
        });
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPMapIndexOperator 1079
        let stream1079: Stream<_, IndexedWSet<Tup2<i32, i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream946.map_index(move |t_18: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup2<i32, i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup2::new(cast_to_i32_i32N((*t_18).1), cast_to_i32_i32N((*t_18).2)), Tup9::new((*t_18).0, (*t_18).1, (*t_18).2, (*t_18).3, (*t_18).4, (*t_18).5, (*t_18).6.clone(), (*t_18).7, (*t_18).8), )
        });
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPStreamJoinOperator 5388(1143)
        let stream5388: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> = stream1038.stream_join(&stream1079, move |t_19: &Tup2<i32, i32>, t_17: &Tup3<Option<i32>, Option<i32>, Option<Decimal>>, t_18: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup3<Option<i32>, Option<i32>, Option<Decimal>> {
            Tup3::new((*t_17).1, (*t_17).0, plus_decimalN_decimalN(cast_to_decimalN_decimalN((*t_17).2.clone().clone(), 13, 2), cast_to_decimalN_decimalN((*t_18).6.clone().clone(), 13, 2)))
        });
        // CREATE VIEW `DISTRICT_UPDATES` AS
        // SELECT `D`.`D_W_ID`, `D`.`D_ID`, `D`.`D_YTD` + `T`.`H_AMOUNT` AS `D_YTD`
        // FROM `schema`.`DISTRICT_YTD` AS `D`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `D`.`D_W_ID` = `T`.`W_ID` AND `D`.`D_ID` = `T`.`D_ID`
        // DBSPSinkOperator 19238(1161)
        let handle19238 = stream5388.output();

        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPFilterOperator 1206
        let stream1206: Stream<_, WSet<Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>> = stream120.filter(move |t_22: &Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, | -> 
        bool {
            (!or_b_b((*t_22).0.is_none(), (*t_22).1.is_none()))
        });
        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPMapIndexOperator 1365
        let stream1365: Stream<_, IndexedWSet<Tup2<i32, i32>, Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>> = stream1206.map_index(move |t_24: &Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, | -> 
        (Tup2<i32, i32>, Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, ) {
            (Tup2::new(cast_to_i32_i32N((*t_24).1), cast_to_i32_i32N((*t_24).0)), Tup9::new((*t_24).0, (*t_24).1, (*t_24).2.clone(), (*t_24).3.clone(), (*t_24).4.clone(), (*t_24).5.clone(), (*t_24).6.clone(), (*t_24).7.clone(), (*t_24).8.clone()), )
        });
        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPMapIndexOperator 26090(1406)
        let stream26090: Stream<_, IndexedWSet<Tup2<i32, i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream946.map_index(move |t_25: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup2<i32, i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup2::new(cast_to_i32_i32N((*t_25).1), cast_to_i32_i32N((*t_25).2)), Tup9::new((*t_25).0, (*t_25).1, (*t_25).2, (*t_25).3, (*t_25).4, (*t_25).5, (*t_25).6.clone(), (*t_25).7, (*t_25).8), )
        });
        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPStreamJoinOperator 26094(1482)
        let stream26094: Stream<_, WSet<Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream1365.stream_join(&stream26090, move |t_26: &Tup2<i32, i32>, t_24: &Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>, t_25: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>> {
            Tup7::new((*t_24).3.clone().clone(), (*t_24).4.clone().clone(), (*t_24).5.clone().clone(), (*t_24).6.clone().clone(), (*t_24).7.clone().clone(), (*t_24).2.clone().clone(), (*t_25).0)
        });
        // CREATE VIEW `DISTRICT_SELECT` AS
        // SELECT `D`.`D_STREET_1`, `D`.`D_STREET_2`, `D`.`D_CITY`, `D`.`D_STATE`, `D`.`D_ZIP`, `D`.`D_NAME`, `T`.`TXN_ID`
        // FROM `schema`.`DISTRICT_STATIC` AS `D`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `D`.`D_W_ID` = `T`.`W_ID` AND `D`.`D_ID` = `T`.`D_ID`
        // DBSPSinkOperator 26096(1508)
        let handle26096 = stream26094.output();

        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPFilterOperator 1575
        let stream1575: Stream<_, WSet<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream279.filter(move |t_29: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        bool {
            (!or_b_b(or_b_b((*t_29).0.is_none(), (*t_29).1.is_none()), (*t_29).2.is_none()))
        });
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPFilterOperator 1600
        let stream1600: Stream<_, WSet<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream332.filter(move |t_30: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        bool {
            (!or_b_b(or_b_b((*t_30).3.is_none(), (*t_30).4.is_none()), (*t_30).5.is_none()))
        });
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPMapIndexOperator 1836
        let stream1836: Stream<_, IndexedWSet<Tup3<i32, i32, i32>, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream1575.map_index(move |t_31: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        (Tup3<i32, i32, i32>, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, ) {
            (Tup3::new(cast_to_i32_i32N((*t_31).2), cast_to_i32_i32N((*t_31).1), cast_to_i32_i32N((*t_31).0)), Tup21::new((*t_31).0, (*t_31).1, (*t_31).2, (*t_31).3.clone(), (*t_31).4.clone(), (*t_31).5.clone(), (*t_31).6.clone(), (*t_31).7.clone(), (*t_31).8.clone(), (*t_31).9.clone(), (*t_31).10.clone(), (*t_31).11.clone(), (*t_31).12, (*t_31).13.clone(), (*t_31).14.clone(), (*t_31).15.clone(), (*t_31).16.clone(), (*t_31).17.clone(), (*t_31).18, (*t_31).19, (*t_31).20.clone()), )
        });
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPMapIndexOperator 1877
        let stream1877: Stream<_, IndexedWSet<Tup3<i32, i32, i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream1600.map_index(move |t_32: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup3<i32, i32, i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup3::new(cast_to_i32_i32N((*t_32).4), cast_to_i32_i32N((*t_32).5), cast_to_i32_i32N((*t_32).3)), Tup9::new((*t_32).0, (*t_32).1, (*t_32).2, (*t_32).3, (*t_32).4, (*t_32).5, (*t_32).6.clone(), (*t_32).7, (*t_32).8), )
        });
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPStreamJoinOperator 11102(2004)
        let stream11102: Stream<_, WSet<Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>>> = stream1836.stream_join(&stream1877, move |t_33: &Tup3<i32, i32, i32>, t_31: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_32: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>> {
            Tup15::new((*t_31).3.clone().clone(), (*t_31).4.clone().clone(), (*t_31).5.clone().clone(), (*t_31).6.clone().clone(), (*t_31).7.clone().clone(), (*t_31).8.clone().clone(), (*t_31).9.clone().clone(), (*t_31).10.clone().clone(), (*t_31).11.clone().clone(), (*t_31).13.clone().clone(), (*t_31).14.clone().clone(), (*t_31).15.clone().clone(), (*t_31).16.clone().clone(), (*t_31).12, (*t_32).0)
        });
        // CREATE VIEW `CUSTOMER_SELECT` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_LAST`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`, `T`.`TXN_ID`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_W_ID` = `T`.`C_W_ID` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_ID` = `T`.`C_ID`
        // DBSPSinkOperator 19242(2046)
        let handle19242 = stream11102.output();

        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPFilterOperator 19244(2087)
        let stream19244: Stream<_, WSet<Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>>> = stream11102.filter(move |t_36: &Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>, | -> 
        bool {
            (!(*t_36).14.is_none())
        });
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPFilterOperator 2100
        let stream2100: Stream<_, WSet<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream332.filter(move |t_37: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        bool {
            (!(*t_37).0.is_none())
        });
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPMapIndexOperator 19247(2280)
        let stream19247: Stream<_, IndexedWSet<Tup1<i32>, Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>>> = stream19244.map_index(move |t_38: &Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>, | -> 
        (Tup1<i32>, Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_38).14)), Tup15::new((*t_38).0.clone(), (*t_38).1.clone(), (*t_38).2.clone(), (*t_38).3.clone(), (*t_38).4.clone(), (*t_38).5.clone(), (*t_38).6.clone(), (*t_38).7.clone(), (*t_38).8.clone(), (*t_38).9.clone(), (*t_38).10.clone(), (*t_38).11.clone(), (*t_38).12.clone(), (*t_38).13, (*t_38).14), )
        });
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPMapIndexOperator 2321
        let stream2321: Stream<_, IndexedWSet<Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream2100.map_index(move |t_39: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_39).0)), Tup9::new((*t_39).0, (*t_39).1, (*t_39).2, (*t_39).3, (*t_39).4, (*t_39).5, (*t_39).6.clone(), (*t_39).7, (*t_39).8), )
        });
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPStreamJoinOperator 19251(2453)
        let stream19251: Stream<_, WSet<Tup14<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream19247.stream_join(&stream2321, move |t_40: &Tup1<i32>, t_38: &Tup15<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>, Option<i32>>, t_39: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup14<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>> {
            Tup14::new((*t_38).0.clone().clone(), (*t_38).1.clone().clone(), (*t_38).2.clone().clone(), (*t_38).3.clone().clone(), (*t_38).4.clone().clone(), (*t_38).5.clone().clone(), (*t_38).6.clone().clone(), (*t_38).7.clone().clone(), (*t_38).8.clone().clone(), (*t_38).9.clone().clone(), (*t_38).10.clone().clone(), (*t_38).11.clone().clone(), plus_decimalN_decimalN(cast_to_decimalN_decimalN((*t_38).12.clone().clone(), 13, 2), cast_to_decimalN_decimalN((*t_39).6.clone().clone(), 13, 2)), (*t_38).13)
        });
        // CREATE VIEW `CUSTOMER_UPDATE` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_LAST`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE` + `T`.`H_AMOUNT` AS `C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER_SELECT` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`TXN_ID` = `T`.`TXN_ID`
        // DBSPSinkOperator 19253(2493)
        let handle19253 = stream19251.output();

        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPFilterOperator 26098(2539)
        let stream26098: Stream<_, WSet<Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26086.filter(move |t_44: &Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        bool {
            (!(*t_44).6.is_none())
        });
        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPMapIndexOperator 26101(2658)
        let stream26101: Stream<_, IndexedWSet<Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream2100.map_index(move |t_45: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup1<i32>, Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_45).0)), Tup9::new((*t_45).0, (*t_45).1, (*t_45).2, (*t_45).3, (*t_45).4, (*t_45).5, (*t_45).6.clone(), (*t_45).7, (*t_45).8), )
        });
        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPMapIndexOperator 26105(2698)
        let stream26105: Stream<_, IndexedWSet<Tup1<i32>, Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26098.map_index(move |t_46: &Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        (Tup1<i32>, Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_46).6)), Tup7::new((*t_46).0.clone(), (*t_46).1.clone(), (*t_46).2.clone(), (*t_46).3.clone(), (*t_46).4.clone(), (*t_46).5.clone(), (*t_46).6), )
        });
        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPStreamJoinOperator 26109(2708)
        let stream26109: Stream<_, WSet<Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26101.stream_join(&stream26105, move |t_47: &Tup1<i32>, t_45: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, t_46: &Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>> {
            Tup16::new((*t_45).0, (*t_45).1, (*t_45).2, (*t_45).3, (*t_45).4, (*t_45).5, (*t_45).6.clone(), (*t_45).7, (*t_45).8, (*t_46).0.clone(), (*t_46).1.clone(), (*t_46).2.clone(), (*t_46).3.clone(), (*t_46).4.clone(), (*t_46).5.clone(), (*t_46).6)
        });
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPFilterOperator 26111(2750)
        let stream26111: Stream<_, WSet<Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26109.filter(move |t_49: &Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        bool {
            (!(*t_49).0.is_none())
        });
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPFilterOperator 26114(2763)
        let stream26114: Stream<_, WSet<Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26094.filter(move |t_50: &Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        bool {
            (!(*t_50).6.is_none())
        });
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPMapIndexOperator 26117(2936)
        let stream26117: Stream<_, IndexedWSet<Tup1<i32>, Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26111.map_index(move |t_51: &Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        (Tup1<i32>, Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_51).0)), Tup16::new((*t_51).0, (*t_51).1, (*t_51).2, (*t_51).3, (*t_51).4, (*t_51).5, (*t_51).6.clone(), (*t_51).7, (*t_51).8, (*t_51).9.clone(), (*t_51).10.clone(), (*t_51).11.clone(), (*t_51).12.clone(), (*t_51).13.clone(), (*t_51).14.clone(), (*t_51).15), )
        });
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPMapIndexOperator 26121(2976)
        let stream26121: Stream<_, IndexedWSet<Tup1<i32>, Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>>> = stream26114.map_index(move |t_52: &Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        (Tup1<i32>, Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_52).6)), Tup7::new((*t_52).0.clone(), (*t_52).1.clone(), (*t_52).2.clone(), (*t_52).3.clone(), (*t_52).4.clone(), (*t_52).5.clone(), (*t_52).6), )
        });
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPStreamJoinOperator 26125(3063)
        let stream26125: Stream<_, WSet<Tup8<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Timestamp>, Option<Decimal>, Option<String>>>> = stream26117.stream_join(&stream26121, move |t_53: &Tup1<i32>, t_51: &Tup16<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, t_52: &Tup7<Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<i32>>, | -> 
        Tup8<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Timestamp>, Option<Decimal>, Option<String>> {
            Tup8::new((*t_51).5, (*t_51).4, (*t_51).3, (*t_51).2, (*t_51).1, (*t_51).8, (*t_51).6.clone().clone(), concat_sN_sN((*t_51).14.clone().clone(), (*t_52).5.clone().clone()))
        });
        // CREATE VIEW `HISTORY_INSERT` AS
        // SELECT `T`.`C_D_ID` AS `H_C_D_ID`, `T`.`C_W_ID` AS `H_C_W_ID`, `T`.`C_ID` AS `H_C_ID`, `T`.`D_ID` AS `H_D_ID`, `T`.`W_ID` AS `H_W_ID`, `T`.`DATETIME_` AS `H_DATE`, `T`.`H_AMOUNT` AS `H_AMOUNT`, CONCAT(`W`.`W_NAME`, `D`.`D_NAME`) AS `H_DATA`
        // FROM `schema`.`TRANSACTION_PARAMETERS` AS `T`,
        // `schema`.`WAREHOUSE_SELECT` AS `W`,
        // `schema`.`DISTRICT_SELECT` AS `D`
        // WHERE `T`.`TXN_ID` = `W`.`TXN_ID` AND `T`.`TXN_ID` = `D`.`TXN_ID`
        // DBSPSinkOperator 26127(3091)
        let handle26127 = stream26125.output();

        Ok((handle49, handle67, handle120, handle143, handle166, handle279, handle332, handle19234, handle26088, handle19238, handle26096, handle19242, handle19253, handle26127, ))
    })?;
    Ok((circuit, streams))
}


