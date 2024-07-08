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


pub fn circuit(cconf: CircuitConfig) -> Result<(DBSPHandle, (ZSetHandle<Tup3<String, Option<i32>, Option<bool>>>, OutputHandle<WSet<Tup1<String>>>, OutputHandle<WSet<Tup1<i64>>>, )), Error> {

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
        // rel#43:LogicalProject.(input=LogicalFilter#41,inputs=0)
        // DBSPMapOperator 70
        let stream70: Stream<_, WSet<Tup1<String>>> = stream49.map(move |t_2: &Tup3<String, Option<i32>, Option<bool>>, | -> 
        Tup1<String> {
            Tup1::new((*t_2).0.clone())
        });
        // CREATE VIEW `ADULT` AS
        // SELECT `PERSON`.`NAME`
        // FROM `schema`.`PERSON` AS `PERSON`
        // WHERE `PERSON`.`AGE` > 18
        // DBSPSinkOperator 517(84)
        let handle517 = stream70.output();

        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPMapIndexOperator 519(135)
        let stream519: Stream<_, IndexedWSet<Tup0, Tup1<String>>> = stream70.map_index(move |t_3: &Tup1<String>, | -> 
        (Tup0, Tup1<String>, ) {
            (Tup0::new(), Tup1::new((*t_3).0.clone()), )
        });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPStreamAggregateOperator 1191(140)
        let stream1191: Stream<_, IndexedWSet<Tup0, Tup1<i64>>> = stream519.stream_aggregate_linear(move |t_4: &Tup1<String>, | -> 
        Tup1<i64> {
            Tup1::new(1i64)
        });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPMapOperator 1193(156)
        let stream1193: Stream<_, WSet<Tup1<i64>>> = stream1191.map(move |t_6: (&Tup0, &Tup1<i64>, ), | -> 
        Tup1<i64> {
            Tup1::new((*t_6.1).0)
        });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPMapOperator 1198(169)
        let stream1198: Stream<_, WSet<Tup1<i64>>> = stream1191.map(move |t_6: (&Tup0, &Tup1<i64>, ), | -> 
        Tup1<i64> {
            Tup1::new(0i64)
        });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPNegateOperator 1203(172)
        let stream1203: Stream<_, WSet<Tup1<i64>>> = stream1198.neg();
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        let stream178 = circuit.add_source(Generator::new(|| if Runtime::worker_index() == 0 {zset!(
            Tup1::new(0i64) => 1,
        )} else {zset!(
        )}));
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPSumOperator 1205(182)
        let stream1205: Stream<_, WSet<Tup1<i64>>> = stream178.sum([&stream1203, &stream1193]);
        // CREATE VIEW `NUMADULT` AS
        // SELECT COUNT(*) AS `COUNT`
        // FROM `schema`.`ADULT` AS `ADULT`
        // DBSPSinkOperator 1207(195)
        let handle1207 = stream1205.output();

        Ok((handle24, handle517, handle1207, ))
    })?;
    Ok((circuit, streams))
}


