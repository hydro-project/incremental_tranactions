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
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
}

pipeline_types::deserialize_without_context!(Tup21, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);

sqlvalue::to_sql_row_impl! {
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
}


pub fn circuit(cconf: CircuitConfig) -> Result<(DBSPHandle, (ZSetHandle<Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup2<Option<i32>, Option<Decimal>>>, ZSetHandle<Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<i32>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>, ZSetHandle<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>, ZSetHandle<Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>, OutputHandle<WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>>, )), Error> {

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
        // DBSPStreamJoinOperator 1092(555)
        let stream1092: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> = stream451.stream_join(&stream492, move |t_5: &Tup1<i32>, t_3: &Tup2<Option<i32>, Option<Decimal>>, t_4: &Tup9<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup3<Option<i32>, Option<i32>, Option<Decimal>> {
            Tup3::new((*t_4).0, (*t_3).0, plus_decimalN_decimalN(cast_to_decimalN_decimalN((*t_3).1.clone().clone(), 13, 2), cast_to_decimalN_decimalN((*t_4).6.clone().clone(), 13, 2)))
        });
        // CREATE VIEW `WAREHOUSE_UPDATES` AS
        // SELECT `T`.`TXN_ID`, `W`.`W_ID`, `W`.`W_YTD` + `T`.`H_AMOUNT` AS `W_YTD`
        // FROM `schema`.`WAREHOUSE` AS `W`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `W`.`W_ID` = `T`.`W_ID`
        // DBSPSinkOperator 1371(573)
        let handle1371 = stream1092.output();

        Ok((handle49, handle67, handle120, handle143, handle166, handle279, handle332, handle1371, ))
    })?;
    Ok((circuit, streams))
}


