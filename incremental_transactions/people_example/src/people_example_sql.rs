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
}


sqlvalue::to_sql_row_impl! {
}


pub fn circuit(cconf: CircuitConfig) -> Result<(DBSPHandle, (ZSetHandle<Tup3<String, Option<i32>, Option<bool>>>, OutputHandle<WSet<Tup2<String, Option<bool>>>>, OutputHandle<WSet<Tup2<Option<bool>, i64>>>, OutputHandle<WSet<Tup2<i32, i64>>>, )), Error> {

    let (circuit, streams) = Runtime::init_circuit(cconf, |circuit| {
        // CREATE TABLE `PERSON` (`NAME` VARCHAR NOT NULL, `AGE` INTEGER, `PRESENT` BOOLEAN)
        // DBSPSourceMultisetOperator 24
        // CREATE TABLE `PERSON` (`NAME` VARCHAR NOT NULL, `AGE` INTEGER, `PRESENT` BOOLEAN)
        let (stream24, handle24) = circuit.add_input_zset::<Tup3<String, Option<i32>, Option<bool>>>();

        // rel#41:LogicalFilter.(input=LogicalTableScan#1,condition=>($1, 18))
        // DBSPFilterOperator 49
        let stream49: Stream<_, WSet<Tup3<String, Option<i32>, Option<bool>>>> = stream24.filter(move |t_1: &Tup3<String, Option<i32>, Option<bool>>, | -> 
        bool {
            wrap_bool(gt_i32N_i32((*t_1).1, 18i32))
        });
        // rel#43:LogicalProject.(input=LogicalFilter#41,inputs=0,exprs=[$2])
        // DBSPMapOperator 74
        let stream74: Stream<_, WSet<Tup2<String, Option<bool>>>> = stream49.map(move |t_2: &Tup3<String, Option<i32>, Option<bool>>, | -> 
        Tup2<String, Option<bool>> {
            Tup2::new((*t_2).0.clone(), (*t_2).2)
        });
        // CREATE VIEW `ADULT` AS
        // SELECT `PERSON`.`NAME`, `PERSON`.`PRESENT`
        // FROM `schema`.`PERSON` AS `PERSON`
        // WHERE `PERSON`.`AGE` > 18
        // DBSPSinkOperator 606(90)
        let handle606 = stream74.output();

        // rel#87:LogicalProject.(input=LogicalTableScan#46,exprs=[$1])
        // DBSPMapOperator 608(108)
        let stream608: Stream<_, WSet<Tup1<Option<bool>>>> = stream74.map(move |t_3: &Tup2<String, Option<bool>>, | -> 
        Tup1<Option<bool>> {
            Tup1::new((*t_3).1)
        });
        // rel#89:LogicalAggregate.(input=LogicalProject#87,group={0},COUNT=COUNT())
        // DBSPMapIndexOperator 611(166)
        let stream611: Stream<_, IndexedWSet<Tup1<Option<bool>>, Tup1<Option<bool>>>> = stream608.map_index(move |t_4: &Tup1<Option<bool>>, | -> 
        (Tup1<Option<bool>>, Tup1<Option<bool>>, ) {
            (Tup1::new((*t_4).0), Tup1::new((*t_4).0), )
        });
        // rel#89:LogicalAggregate.(input=LogicalProject#87,group={0},COUNT=COUNT())
        // DBSPStreamAggregateOperator 1424(171)
        let stream1424: Stream<_, IndexedWSet<Tup1<Option<bool>>, Tup1<i64>>> = stream611.stream_aggregate_linear(move |t_5: &Tup1<Option<bool>>, | -> 
        Tup1<i64> {
            Tup1::new(1i64)
        });
        // rel#89:LogicalAggregate.(input=LogicalProject#87,group={0},COUNT=COUNT())
        // DBSPMapOperator 1426(191)
        let stream1426: Stream<_, WSet<Tup2<Option<bool>, i64>>> = stream1424.map(move |t_7: (&Tup1<Option<bool>>, &Tup1<i64>, ), | -> 
        Tup2<Option<bool>, i64> {
            Tup2::new((*t_7.0).0, (*t_7.1).0)
        });
        // CREATE VIEW `NUMADULT` AS
        // SELECT `ADULT`.`PRESENT`, COUNT(*) AS `COUNT`
        // FROM `schema`.`ADULT` AS `ADULT`
        // GROUP BY `ADULT`.`PRESENT`
        // DBSPSinkOperator 1431(211)
        let handle1431 = stream1426.output();

        // rel#134:LogicalFilter.(input=LogicalTableScan#92,condition=$0)
        // DBSPFilterOperator 1436(226)
        let stream1436: Stream<_, WSet<Tup2<Option<bool>, i64>>> = stream1426.filter(move |t_8: &Tup2<Option<bool>, i64>, | -> 
        bool {
            wrap_bool((*t_8).0)
        });
        // rel#136:LogicalProject.(input=LogicalFilter#134,exprs=[1, $1])
        // DBSPMapOperator 1439(249)
        let stream1439: Stream<_, WSet<Tup2<i32, i64>>> = stream1436.map(move |t_9: &Tup2<Option<bool>, i64>, | -> 
        Tup2<i32, i64> {
            Tup2::new(1i32, (*t_9).1)
        });
        // CREATE VIEW `DERIVED` AS
        // SELECT 1 AS `TEST`, `NUMADULT`.`COUNT`
        // FROM `schema`.`NUMADULT` AS `NUMADULT`
        // WHERE `NUMADULT`.`PRESENT` = TRUE
        // DBSPSinkOperator 1442(265)
        let handle1442 = stream1439.output();

        Ok((handle24, handle606, handle1431, handle1442, ))
    })?;
    Ok((circuit, streams))
}


