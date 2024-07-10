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

#[derive(Clone)]
pub struct Semigroup1<T0, TS0>(PhantomData<(T0, TS0)>);

impl<T0, TS0> Semigroup<Tup1<T0, >> for Semigroup1<T0, TS0>
where
    TS0: Semigroup<T0>
{
    fn combine(left: &Tup1<T0, >, right:&Tup1<T0, >) -> Tup1<T0, > {
        Tup1::new(
            TS0::combine(&left.0, &right.0),
        )
    }
}
declare_tuples! {
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
}

pipeline_types::deserialize_without_context!(Tup21, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
pipeline_types::deserialize_without_context!(Tup22, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
pipeline_types::deserialize_without_context!(Tup14, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);

sqlvalue::to_sql_row_impl! {
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
}


pub fn circuit(cconf: CircuitConfig) -> Result<(DBSPHandle, (ZSetHandle<Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup2<Option<i32>, Option<Decimal>>>, ZSetHandle<Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<i32>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>, ZSetHandle<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>, ZSetHandle<Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>, OutputHandle<WSet<Tup1<Vec<Option<i32>>>>>, OutputHandle<WSet<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>, )), Error> {

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

        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `C_LAST` VARCHAR(20), `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        // DBSPSourceMultisetOperator 337
        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `C_LAST` VARCHAR(20), `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        let (stream337, handle337) = circuit.add_input_zset::<Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>();

        // rel#84:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 1790(1706)
        let stream1790: Stream<_, IndexedWSet<Tup0, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream279.map_index(move |t_1: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        (Tup0, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, ) {
            (Tup0::new(), Tup21::new((*t_1).0, (*t_1).1, (*t_1).2, (*t_1).3.clone(), (*t_1).4.clone(), (*t_1).5.clone(), (*t_1).6.clone(), (*t_1).7.clone(), (*t_1).8.clone(), (*t_1).9.clone(), (*t_1).10.clone(), (*t_1).11.clone(), (*t_1).12, (*t_1).13.clone(), (*t_1).14.clone(), (*t_1).15.clone(), (*t_1).16.clone(), (*t_1).17.clone(), (*t_1).18, (*t_1).19, (*t_1).20.clone()), )
        });
        // rel#84:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 1796(1710)
        let stream1796: Stream<_, IndexedWSet<Tup0, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream337.map_index(move |t_2: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup0, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup0::new(), Tup10::new((*t_2).0, (*t_2).1, (*t_2).2, (*t_2).3, (*t_2).4, (*t_2).5, (*t_2).6.clone(), (*t_2).7.clone(), (*t_2).8, (*t_2).9), )
        });
        // rel#84:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPJoinFilterMap 15012(1719)
        let stream15012: Stream<_, WSet<Tup2<Option<i32>, Option<String>>>> = stream1790.join_flatmap(&stream1796, move |t_4: &Tup0, t_1: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_2: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Option<Tup2<Option<i32>, Option<String>>> {
            (if wrap_bool(and_bN_bN(and_bN_bN(eq_sN_sN(cast_to_sN_sN((*t_1).5.clone().clone(), 20, false), (*t_2).6.clone().clone()), eq_i32N_i32N((*t_1).1, (*t_2).5)), eq_i32N_i32N((*t_1).2, (*t_2).4))) {
                Some(Tup2::new((*t_1).0, (*t_1).3.clone().clone()))
            } else {
                None::<Tup2<Option<i32>, Option<String>>>
            })
        });
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPMapIndexOperator 15014(1722)
        let stream15014: Stream<_, IndexedWSet<Tup0, Tup2<Option<i32>, Option<String>>>> = stream15012.map_index(move |t_7: &Tup2<Option<i32>, Option<String>>, | -> 
        (Tup0, Tup2<Option<i32>, Option<String>>, ) {
            (Tup0::new(), Tup2::new((*t_7).0, (*t_7).1.clone()), )
        });
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPAggregateOperator 15081(1726)
        let stream15081: Stream<_, IndexedWSet<Tup0, Tup1<Vec<Option<i32>>>>> = stream15014.aggregate(Fold::<_, _, Semigroup1<Vec<Option<i32>>, ConcatSemigroup<Vec<Option<i32>>>>, _, _>::with_output(Tup1::new(vec!(
        )), move |t_31: &mut Tup1<Vec<Option<i32>>>, t_8: &Tup2<Option<i32>, Option<String>>, t_33: Weight, | {
            (*t_31) = Tup1::new(array_agg(&mut (*t_31).0, (*t_8).0, t_33, false))
        }, move |t_32: Tup1<Vec<Option<i32>>>, | -> 
        Tup1<Vec<Option<i32>>> {
            Tup1::new(t_32.0)
        }));
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPMapOperator 15083(1728)
        let stream15083: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream15081.map(move |t_10: (&Tup0, &Tup1<Vec<Option<i32>>>, ), | -> 
        Tup1<Vec<Option<i32>>> {
            Tup1::new((*t_10.1).0.clone())
        });
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPMapOperator 15088(1733)
        let stream15088: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream15081.map(move |t_10: (&Tup0, &Tup1<Vec<Option<i32>>>, ), | -> 
        Tup1<Vec<Option<i32>>> {
            Tup1::new(vec!(
            ))
        });
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPNegateOperator 15093(1736)
        let stream15093: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream15088.neg();
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        let stream861 = circuit.add_source(Generator::new(|| if Runtime::worker_index() == 0 {zset!(
            Tup1::new(vec!(
            )) => 1,
        )} else {zset!(
        )}));
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPDifferentiateOperator 1842(861)
        let stream1842: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream861.differentiate();
        // rel#88:LogicalAggregate.(input=LogicalProject#86,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPSumOperator 15095(1738)
        let stream15095: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream1842.sum([&stream15093, &stream15083]);
        // CREATE VIEW `CUST_AGG` AS
        // SELECT ARRAY_AGG(`EXPR$0`.`C_ID` ORDER BY `EXPR$0`.`C_FIRST`) AS `CUST_ARRAY`
        // FROM (SELECT `C`.`C_ID`, `C`.`C_FIRST`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_LAST` = `T`.`C_LAST` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_W_ID` = `T`.`C_W_ID`
        // ORDER BY `C_FIRST`) AS `EXPR$0`
        // DBSPSinkOperator 15097(879)
        let handle15097 = stream15095.output();

        // rel#187:LogicalJoin.(left=LogicalTableScan#91,right=LogicalTableScan#93,condition==($0, ITEM($21, +(/(ARRAY_LENGTH($21), 2), 1))),joinType=inner)
        // DBSPMapIndexOperator 15099(1755)
        let stream15099: Stream<_, IndexedWSet<Tup0, Tup1<Vec<Option<i32>>>>> = stream15095.map_index(move |t_13: &Tup1<Vec<Option<i32>>>, | -> 
        (Tup0, Tup1<Vec<Option<i32>>>, ) {
            (Tup0::new(), Tup1::new((*t_13).0.clone()), )
        });
        // rel#187:LogicalJoin.(left=LogicalTableScan#91,right=LogicalTableScan#93,condition==($0, ITEM($21, +(/(ARRAY_LENGTH($21), 2), 1))),joinType=inner)
        // DBSPJoinFilterMap 15619(1761)
        let stream15619: Stream<_, WSet<Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>>> = stream1790.join_flatmap(&stream15099, move |t_15: &Tup0, t_12: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_13: &Tup1<Vec<Option<i32>>>, | -> 
        Option<Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>> {
            let tmp: Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>> = Tup22::new((*t_12).0, (*t_12).1, (*t_12).2, (*t_12).3.clone(), (*t_12).4.clone(), (*t_12).5.clone(), (*t_12).6.clone(), (*t_12).7.clone(), (*t_12).8.clone(), (*t_12).9.clone(), (*t_12).10.clone(), (*t_12).11.clone(), (*t_12).12, (*t_12).13.clone(), (*t_12).14.clone(), (*t_12).15.clone(), (*t_12).16.clone(), (*t_12).17.clone(), (*t_12).18, (*t_12).19, (*t_12).20.clone(), (*t_13).0.clone());
            (if wrap_bool(eq_i32N_i32N((*&tmp).0, index__N_((*&tmp).21.clone(), cast_to_u_i32(plus_i32_i32(div_i32_i32(cardinality((*&tmp).21.clone()), 2i32), 1i32)) - 1))) {
                Some(tmp)
            } else {
                None::<Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>>
            })
        });
        // rel#190:LogicalJoin.(left=LogicalJoin#187,right=LogicalTableScan#97,condition=true,joinType=inner)
        // DBSPMapIndexOperator 15697(1764)
        let stream15697: Stream<_, IndexedWSet<Tup0, Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>>> = stream15619.map_index(move |t_17: &Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>, | -> 
        (Tup0, Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>, ) {
            (Tup0::new(), Tup22::new((*t_17).0, (*t_17).1, (*t_17).2, (*t_17).3.clone(), (*t_17).4.clone(), (*t_17).5.clone(), (*t_17).6.clone(), (*t_17).7.clone(), (*t_17).8.clone(), (*t_17).9.clone(), (*t_17).10.clone(), (*t_17).11.clone(), (*t_17).12, (*t_17).13.clone(), (*t_17).14.clone(), (*t_17).15.clone(), (*t_17).16.clone(), (*t_17).17.clone(), (*t_17).18, (*t_17).19, (*t_17).20.clone(), (*t_17).21.clone()), )
        });
        // rel#190:LogicalJoin.(left=LogicalJoin#187,right=LogicalTableScan#97,condition=true,joinType=inner)
        // DBSPJoinOperator 15769(1774)
        let stream15769: Stream<_, WSet<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream15697.join(&stream1796, move |t_20: &Tup0, t_17: &Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>, t_18: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>> {
            Tup14::new((*t_17).3.clone().clone(), (*t_17).4.clone().clone(), (*t_17).0, (*t_17).6.clone().clone(), (*t_17).7.clone().clone(), (*t_17).8.clone().clone(), (*t_17).9.clone().clone(), (*t_17).10.clone().clone(), (*t_17).11.clone().clone(), (*t_17).13.clone().clone(), (*t_17).14.clone().clone(), (*t_17).15.clone().clone(), (*t_17).16.clone().clone(), (*t_17).12)
        });
        // CREATE VIEW `CUST_MED` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_ID`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`CUST_AGG` AS `A`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_ID` = `A`.`CUST_ARRAY`[ARRAY_LENGTH(`A`.`CUST_ARRAY`) / 2 + 1]
        // DBSPSinkOperator 15788(1643)
        let handle15788 = stream15769.output();

        Ok((handle49, handle67, handle120, handle143, handle166, handle279, handle337, handle15097, handle15788, ))
    })?;
    Ok((circuit, streams))
}


