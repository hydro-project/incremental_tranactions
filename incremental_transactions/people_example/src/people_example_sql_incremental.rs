// Automatically-generated file
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unconditional_panic)]
#![allow(non_camel_case_types)]

use ::serde::{Deserialize, Serialize};
use compare::{Compare, Extract};
use core::cmp::Ordering;
use dbsp::declare_tuples;
use dbsp::{
    algebra::{
        AddAssignByRef, AddByRef, DefaultSemigroup, HasZero, MulByRef, NegByRef, Semigroup,
        SemigroupValue, UnimplementedSemigroup, ZRingValue, ZSet, F32, F64,
    },
    circuit::{checkpointer::Checkpoint, Circuit, CircuitConfig, Stream},
    dynamic::{DynData, DynDataTyped},
    indexed_zset,
    operator::{
        time_series::{OrdPartitionedIndexedZSet, RelOffset, RelRange},
        CmpFunc, FilterMap, Fold, Generator, MaxSemigroup, MinSemigroup,
    },
    utils::*,
    zset, DBData, DBSPHandle, DBWeight, Error, MapHandle, NumEntries, OrdIndexedZSet, OrdZSet,
    OutputHandle, Runtime, TypedBox, ZSetHandle,
};
use dbsp_adapters::Catalog;
#[cfg(test)]
use hashing::*;
use json::*;
use pipeline_types::{deserialize_table_record, serialize_table_record};
#[cfg(test)]
use readers::*;
use rust_decimal::Decimal;
use size_of::*;
use sqllib::{
    aggregates::*, array::*, binary::*, casts::*, geopoint::*, interval::*, operators::*,
    string::*, timestamp::*, *,
};
use sqlvalue::*;
#[cfg(test)]
use sqlx::{any::AnyRow, AnyConnection, Row};
use std::{
    collections::BTreeMap,
    convert::identity,
    fmt::{Debug, Formatter, Result as FmtResult},
    marker::PhantomData,
    ops::Neg,
    path::Path,
};

/* #[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[allow(non_upper_case_globals)]
#[export_name = "malloc_conf"]
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0"; */

declare_tuples! {}

sqlvalue::to_sql_row_impl! {}

pub fn circuit(
    cconf: CircuitConfig,
) -> Result<
    (
        DBSPHandle,
        (
            ZSetHandle<Tup3<String, Option<i32>, Option<bool>>>,
            OutputHandle<WSet<Tup1<String>>>,
            OutputHandle<WSet<Tup1<i64>>>,
        ),
    ),
    Error,
> {
    let (circuit, streams) = Runtime::init_circuit(cconf, |circuit| {
        // CREATE TABLE `PERSON` (`NAME` VARCHAR NOT NULL, `AGE` INTEGER, `PRESENT` BOOLEAN)
        // DBSPSourceMultisetOperator 24
        // CREATE TABLE `PERSON` (`NAME` VARCHAR NOT NULL, `AGE` INTEGER, `PRESENT` BOOLEAN)
        let (stream24, handle24) =
            circuit.add_input_zset::<Tup3<String, Option<i32>, Option<bool>>>();

        // rel#41:LogicalFilter.(input=LogicalTableScan#1,condition=>($1, 18))
        // DBSPFilterOperator 267(219)
        let stream267: Stream<_, WSet<Tup3<String, Option<i32>, Option<bool>>>> = stream24.filter(
            move |t_1: &Tup3<String, Option<i32>, Option<bool>>| -> bool {
                wrap_bool(gt_i32N_i32((*t_1).1, 18i32))
            },
        );
        // rel#43:LogicalProject.(input=LogicalFilter#41,inputs=0)
        // DBSPMapOperator 272(222)
        let stream272: Stream<_, WSet<Tup1<String>>> = stream267.map(
            move |t_2: &Tup3<String, Option<i32>, Option<bool>>| -> Tup1<String> {
                Tup1::new((*t_2).0.clone())
            },
        );
        // CREATE VIEW `ADULT` AS
        // SELECT `PERSON`.`NAME`
        // FROM `schema`.`PERSON` AS `PERSON`
        // WHERE `PERSON`.`AGE` > 18
        // DBSPSinkOperator 593(84)
        let handle593 = stream272.output();

        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPMapIndexOperator 595(236)
        let stream595: Stream<_, IndexedWSet<Tup0, Tup1<String>>> =
            stream272.map_index(move |t_3: &Tup1<String>| -> (Tup0, Tup1<String>) {
                (Tup0::new(), Tup1::new((*t_3).0.clone()))
            });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPAggregateOperator 1067(240)
        let stream1067: Stream<_, IndexedWSet<Tup0, Tup1<i64>>> =
            stream595.aggregate_linear(move |t_4: &Tup1<String>| -> Tup1<i64> { Tup1::new(1i64) });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPMapOperator 1069(242)
        let stream1069: Stream<_, WSet<Tup1<i64>>> =
            stream1067.map(move |t_6: (&Tup0, &Tup1<i64>)| -> Tup1<i64> { Tup1::new((*t_6.1).0) });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPMapOperator 1074(247)
        let stream1074: Stream<_, WSet<Tup1<i64>>> =
            stream1067.map(move |t_6: (&Tup0, &Tup1<i64>)| -> Tup1<i64> { Tup1::new(0i64) });
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPNegateOperator 1079(250)
        let stream1079: Stream<_, WSet<Tup1<i64>>> = stream1074.neg();
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        let stream178 = circuit.add_source(Generator::new(|| {
            if Runtime::worker_index() == 0 {
                zset!(
                    Tup1::new(0i64) => 1,
                )
            } else {
                zset!()
            }
        }));
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPDifferentiateOperator 314(178)
        let stream314: Stream<_, WSet<Tup1<i64>>> = stream178.differentiate();
        // rel#71:LogicalAggregate.(input=LogicalTableScan#46,group={},COUNT=COUNT())
        // DBSPSumOperator 1081(252)
        let stream1081: Stream<_, WSet<Tup1<i64>>> = stream314.sum([&stream1079, &stream1069]);
        // CREATE VIEW `NUMADULT` AS
        // SELECT COUNT(*) AS `COUNT`
        // FROM `schema`.`ADULT` AS `ADULT`
        // DBSPSinkOperator 1083(195)
        let handle1083 = stream1081.output();

        Ok((handle24, handle593, handle1083))
    })?;
    Ok((circuit, streams))
}
