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

        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `C_LAST` VARCHAR(16), `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        // DBSPSourceMultisetOperator 337
        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `C_LAST` VARCHAR(16), `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        let (stream337, handle337) = circuit.add_input_zset::<Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>();

        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPFilterOperator 1858(1771)
        let stream1858: Stream<_, WSet<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream279.filter(move |t_1: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        bool {
            (!or_b_b(or_b_b((*t_1).1.is_none(), (*t_1).2.is_none()), (*t_1).5.is_none()))
        });
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPFilterOperator 1863(1774)
        let stream1863: Stream<_, WSet<Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream337.filter(move |t_2: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        bool {
            (!or_b_b(or_b_b((*t_2).4.is_none(), (*t_2).5.is_none()), (*t_2).6.is_none()))
        });
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 1868(1777)
        let stream1868: Stream<_, IndexedWSet<Tup3<String, i32, i32>, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream1858.map_index(move |t_3: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        (Tup3<String, i32, i32>, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, ) {
            (Tup3::new(cast_to_s_sN((*t_3).5.clone(), 16, false), cast_to_i32_i32N((*t_3).1), cast_to_i32_i32N((*t_3).2)), Tup21::new((*t_3).0, (*t_3).1, (*t_3).2, (*t_3).3.clone(), (*t_3).4.clone(), (*t_3).5.clone(), (*t_3).6.clone(), (*t_3).7.clone(), (*t_3).8.clone(), (*t_3).9.clone(), (*t_3).10.clone(), (*t_3).11.clone(), (*t_3).12, (*t_3).13.clone(), (*t_3).14.clone(), (*t_3).15.clone(), (*t_3).16.clone(), (*t_3).17.clone(), (*t_3).18, (*t_3).19, (*t_3).20.clone()), )
        });
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 1874(1781)
        let stream1874: Stream<_, IndexedWSet<Tup3<String, i32, i32>, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream1863.map_index(move |t_4: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup3<String, i32, i32>, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup3::new(cast_to_s_sN((*t_4).6.clone(), 16, false), cast_to_i32_i32N((*t_4).5), cast_to_i32_i32N((*t_4).4)), Tup10::new((*t_4).0, (*t_4).1, (*t_4).2, (*t_4).3, (*t_4).4, (*t_4).5, (*t_4).6.clone(), (*t_4).7.clone(), (*t_4).8, (*t_4).9), )
        });
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPJoinOperator 3427(1787)
        let stream3427: Stream<_, WSet<Tup2<Option<i32>, Option<String>>>> = stream1868.join(&stream1874, move |t_5: &Tup3<String, i32, i32>, t_3: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_4: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup2<Option<i32>, Option<String>> {
            Tup2::new(plus_i32N_i32N(plus_i32N_i32N((*t_3).0, (*t_3).2), (*t_3).1), (*t_3).3.clone().clone())
        });
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPMapIndexOperator 3429(1790)
        let stream3429: Stream<_, IndexedWSet<Tup0, Tup2<Option<i32>, Option<String>>>> = stream3427.map_index(move |t_8: &Tup2<Option<i32>, Option<String>>, | -> 
        (Tup0, Tup2<Option<i32>, Option<String>>, ) {
            (Tup0::new(), Tup2::new((*t_8).0, (*t_8).1.clone()), )
        });
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPAggregateOperator 13452(1794)
        let stream13452: Stream<_, IndexedWSet<Tup0, Tup1<Vec<Option<i32>>>>> = stream3429.aggregate(Fold::<_, _, Semigroup1<Vec<Option<i32>>, ConcatSemigroup<Vec<Option<i32>>>>, _, _>::with_output(Tup1::new(vec!(
        )), move |t_32: &mut Tup1<Vec<Option<i32>>>, t_9: &Tup2<Option<i32>, Option<String>>, t_34: Weight, | {
            (*t_32) = Tup1::new(array_agg(&mut (*t_32).0, (*t_9).0, t_34, false))
        }, move |t_33: Tup1<Vec<Option<i32>>>, | -> 
        Tup1<Vec<Option<i32>>> {
            Tup1::new(t_33.0)
        }));
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPMapOperator 13454(1796)
        let stream13454: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream13452.map(move |t_11: (&Tup0, &Tup1<Vec<Option<i32>>>, ), | -> 
        Tup1<Vec<Option<i32>>> {
            Tup1::new((*t_11.1).0.clone())
        });
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPMapOperator 13459(1801)
        let stream13459: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream13452.map(move |t_11: (&Tup0, &Tup1<Vec<Option<i32>>>, ), | -> 
        Tup1<Vec<Option<i32>>> {
            Tup1::new(vec!(
            ))
        });
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPNegateOperator 13464(1804)
        let stream13464: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream13459.neg();
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        let stream906 = circuit.add_source(Generator::new(|| if Runtime::worker_index() == 0 {zset!(
            Tup1::new(vec!(
            )) => 1,
        )} else {zset!(
        )}));
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPDifferentiateOperator 1915(906)
        let stream1915: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream906.differentiate();
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={},CUST_ARRAY=ARRAY_AGG($0) WITHIN GROUP ([1]))
        // DBSPSumOperator 13466(1806)
        let stream13466: Stream<_, WSet<Tup1<Vec<Option<i32>>>>> = stream1915.sum([&stream13464, &stream13454]);
        // CREATE VIEW `CUST_AGG` AS
        // SELECT ARRAY_AGG(`EXPR$0`.`C_ID` + `EXPR$0`.`C_W_ID` + `EXPR$0`.`C_D_ID` ORDER BY `EXPR$0`.`C_FIRST`) AS `CUST_ARRAY`
        // FROM (SELECT `C`.`C_ID`, `C`.`C_W_ID`, `C`.`C_D_ID`, `C`.`C_FIRST`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_LAST` = `T`.`C_LAST` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_W_ID` = `T`.`C_W_ID`
        // ORDER BY `C_FIRST`) AS `EXPR$0`
        // DBSPSinkOperator 13468(924)
        let handle13468 = stream13466.output();

        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition==(+(+($0, $2), $1), ITEM($21, +(/(ARRAY_LENGTH($21), 2), 1))),joinType=inner)
        // DBSPMapIndexOperator 1934(1819)
        let stream1934: Stream<_, IndexedWSet<Tup0, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream279.map_index(move |t_13: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        (Tup0, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, ) {
            (Tup0::new(), Tup21::new((*t_13).0, (*t_13).1, (*t_13).2, (*t_13).3.clone(), (*t_13).4.clone(), (*t_13).5.clone(), (*t_13).6.clone(), (*t_13).7.clone(), (*t_13).8.clone(), (*t_13).9.clone(), (*t_13).10.clone(), (*t_13).11.clone(), (*t_13).12, (*t_13).13.clone(), (*t_13).14.clone(), (*t_13).15.clone(), (*t_13).16.clone(), (*t_13).17.clone(), (*t_13).18, (*t_13).19, (*t_13).20.clone()), )
        });
        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition==(+(+($0, $2), $1), ITEM($21, +(/(ARRAY_LENGTH($21), 2), 1))),joinType=inner)
        // DBSPMapIndexOperator 13470(1823)
        let stream13470: Stream<_, IndexedWSet<Tup0, Tup1<Vec<Option<i32>>>>> = stream13466.map_index(move |t_14: &Tup1<Vec<Option<i32>>>, | -> 
        (Tup0, Tup1<Vec<Option<i32>>>, ) {
            (Tup0::new(), Tup1::new((*t_14).0.clone()), )
        });
        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition==(+(+($0, $2), $1), ITEM($21, +(/(ARRAY_LENGTH($21), 2), 1))),joinType=inner)
        // DBSPJoinFilterMap 14116(1829)
        let stream14116: Stream<_, WSet<Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>>> = stream1934.join_flatmap(&stream13470, move |t_16: &Tup0, t_13: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_14: &Tup1<Vec<Option<i32>>>, | -> 
        Option<Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>> {
            let tmp: Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>> = Tup22::new((*t_13).0, (*t_13).1, (*t_13).2, (*t_13).3.clone(), (*t_13).4.clone(), (*t_13).5.clone(), (*t_13).6.clone(), (*t_13).7.clone(), (*t_13).8.clone(), (*t_13).9.clone(), (*t_13).10.clone(), (*t_13).11.clone(), (*t_13).12, (*t_13).13.clone(), (*t_13).14.clone(), (*t_13).15.clone(), (*t_13).16.clone(), (*t_13).17.clone(), (*t_13).18, (*t_13).19, (*t_13).20.clone(), (*t_14).0.clone());
            (if wrap_bool(eq_i32N_i32N(plus_i32N_i32N(plus_i32N_i32N((*&tmp).0, (*&tmp).2), (*&tmp).1), index__N_((*&tmp).21.clone(), cast_to_u_i32(plus_i32_i32(div_i32_i32(cardinality((*&tmp).21.clone()), 2i32), 1i32)) - 1))) {
                Some(tmp)
            } else {
                None::<Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>>
            })
        });
        // rel#189:LogicalJoin.(left=LogicalJoin#186,right=LogicalTableScan#96,condition=true,joinType=inner)
        // DBSPMapIndexOperator 14194(1832)
        let stream14194: Stream<_, IndexedWSet<Tup0, Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>>> = stream14116.map_index(move |t_18: &Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>, | -> 
        (Tup0, Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>, ) {
            (Tup0::new(), Tup22::new((*t_18).0, (*t_18).1, (*t_18).2, (*t_18).3.clone(), (*t_18).4.clone(), (*t_18).5.clone(), (*t_18).6.clone(), (*t_18).7.clone(), (*t_18).8.clone(), (*t_18).9.clone(), (*t_18).10.clone(), (*t_18).11.clone(), (*t_18).12, (*t_18).13.clone(), (*t_18).14.clone(), (*t_18).15.clone(), (*t_18).16.clone(), (*t_18).17.clone(), (*t_18).18, (*t_18).19, (*t_18).20.clone(), (*t_18).21.clone()), )
        });
        // rel#189:LogicalJoin.(left=LogicalJoin#186,right=LogicalTableScan#96,condition=true,joinType=inner)
        // DBSPMapIndexOperator 1961(1836)
        let stream1961: Stream<_, IndexedWSet<Tup0, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream337.map_index(move |t_19: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup0, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup0::new(), Tup10::new((*t_19).0, (*t_19).1, (*t_19).2, (*t_19).3, (*t_19).4, (*t_19).5, (*t_19).6.clone(), (*t_19).7.clone(), (*t_19).8, (*t_19).9), )
        });
        // rel#189:LogicalJoin.(left=LogicalJoin#186,right=LogicalTableScan#96,condition=true,joinType=inner)
        // DBSPJoinOperator 14302(1842)
        let stream14302: Stream<_, WSet<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream14194.join(&stream1961, move |t_21: &Tup0, t_18: &Tup22<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Vec<Option<i32>>>, t_19: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>> {
            Tup14::new((*t_18).3.clone().clone(), (*t_18).4.clone().clone(), (*t_18).0, (*t_18).6.clone().clone(), (*t_18).7.clone().clone(), (*t_18).8.clone().clone(), (*t_18).9.clone().clone(), (*t_18).10.clone().clone(), (*t_18).11.clone().clone(), (*t_18).13.clone().clone(), (*t_18).14.clone().clone(), (*t_18).15.clone().clone(), (*t_18).16.clone().clone(), (*t_18).12)
        });
        // CREATE VIEW `CUST_BYNAME` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_ID`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`CUST_AGG` AS `A`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_ID` + `C`.`C_W_ID` + `C`.`C_D_ID` = `A`.`CUST_ARRAY`[ARRAY_LENGTH(`A`.`CUST_ARRAY`) / 2 + 1]
        // DBSPSinkOperator 14321(1708)
        let handle14321 = stream14302.output();

        Ok((handle49, handle67, handle120, handle143, handle166, handle279, handle337, handle13468, handle14321, ))
    })?;
    Ok((circuit, streams))
}


