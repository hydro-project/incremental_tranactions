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

declare_tuples! {}

sqlvalue::to_sql_row_impl! {}

pub fn circuit(
    cconf: CircuitConfig,
) -> Result<
    (
        DBSPHandle,
        (
            ZSetHandle<Tup3<String, Option<i32>, Option<bool>>>,
            OutputHandle<WSet<Tup2<String, Option<bool>>>>,
            OutputHandle<WSet<Tup2<Option<bool>, i64>>>,
            OutputHandle<WSet<Tup2<i32, i64>>>,
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
        // DBSPFilterOperator 362(301)
        let stream362: Stream<_, WSet<Tup3<String, Option<i32>, Option<bool>>>> = stream24.filter(
            move |t_1: &Tup3<String, Option<i32>, Option<bool>>| -> bool {
                wrap_bool(gt_i32N_i32((*t_1).1, 18i32))
            },
        );
        // rel#43:LogicalProject.(input=LogicalFilter#41,inputs=0,exprs=[$2])
        // DBSPMapOperator 367(304)
        let stream367: Stream<_, WSet<Tup2<String, Option<bool>>>> = stream362.map(
            move |t_2: &Tup3<String, Option<i32>, Option<bool>>| -> Tup2<String, Option<bool>> {
                Tup2::new((*t_2).0.clone(), (*t_2).2)
            },
        );
        // CREATE VIEW `ADULT` AS
        // SELECT `PERSON`.`NAME`, `PERSON`.`PRESENT`
        // FROM `schema`.`PERSON` AS `PERSON`
        // WHERE `PERSON`.`AGE` > 18
        // DBSPSinkOperator 693(90)
        let handle693 = stream367.output();

        // rel#87:LogicalProject.(input=LogicalTableScan#46,exprs=[$1])
        // DBSPMapOperator 695(318)
        let stream695: Stream<_, WSet<Tup1<Option<bool>>>> = stream367.map(
            move |t_3: &Tup2<String, Option<bool>>| -> Tup1<Option<bool>> { Tup1::new((*t_3).1) },
        );
        // rel#89:LogicalAggregate.(input=LogicalProject#87,group={0},COUNT=COUNT())
        // DBSPMapIndexOperator 698(321)
        let stream698: Stream<_, IndexedWSet<Tup1<Option<bool>>, Tup1<Option<bool>>>> = stream695
            .map_index(
                move |t_4: &Tup1<Option<bool>>| -> (Tup1<Option<bool>>, Tup1<Option<bool>>) {
                    (Tup1::new((*t_4).0), Tup1::new((*t_4).0))
                },
            );
        // rel#89:LogicalAggregate.(input=LogicalProject#87,group={0},COUNT=COUNT())
        // DBSPAggregateOperator 1280(325)
        let stream1280: Stream<_, IndexedWSet<Tup1<Option<bool>>, Tup1<i64>>> = stream698
            .aggregate_linear(move |t_5: &Tup1<Option<bool>>| -> Tup1<i64> { Tup1::new(1i64) });
        // rel#89:LogicalAggregate.(input=LogicalProject#87,group={0},COUNT=COUNT())
        // DBSPMapOperator 1282(327)
        let stream1282: Stream<_, WSet<Tup2<Option<bool>, i64>>> = stream1280.map(
            move |t_7: (&Tup1<Option<bool>>, &Tup1<i64>)| -> Tup2<Option<bool>, i64> {
                Tup2::new((*t_7.0).0, (*t_7.1).0)
            },
        );
        // CREATE VIEW `NUMADULT` AS
        // SELECT `ADULT`.`PRESENT`, COUNT(*) AS `COUNT`
        // FROM `schema`.`ADULT` AS `ADULT`
        // GROUP BY `ADULT`.`PRESENT`
        // DBSPSinkOperator 1287(211)
        let handle1287 = stream1282.output();

        // rel#134:LogicalFilter.(input=LogicalTableScan#92,condition=$0)
        // DBSPFilterOperator 1289(343)
        let stream1289: Stream<_, WSet<Tup2<Option<bool>, i64>>> =
            stream1282.filter(move |t_8: &Tup2<Option<bool>, i64>| -> bool { wrap_bool((*t_8).0) });
        // rel#136:LogicalProject.(input=LogicalFilter#134,exprs=[1, $1])
        // DBSPMapOperator 1292(346)
        let stream1292: Stream<_, WSet<Tup2<i32, i64>>> =
            stream1289.map(move |t_9: &Tup2<Option<bool>, i64>| -> Tup2<i32, i64> {
                Tup2::new(1i32, (*t_9).1)
            });
        // CREATE VIEW `DERIVED` AS
        // SELECT 1 AS `TEST`, `NUMADULT`.`COUNT`
        // FROM `schema`.`NUMADULT` AS `NUMADULT`
        // WHERE `NUMADULT`.`PRESENT` = TRUE
        // DBSPSinkOperator 1295(265)
        let handle1295 = stream1292.output();

        Ok((handle24, handle693, handle1287, handle1295))
    })?;
    Ok((circuit, streams))
}
