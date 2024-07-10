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
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    Tup31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>,
}

pipeline_types::deserialize_without_context!(Tup21, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
pipeline_types::deserialize_without_context!(Tup14, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
pipeline_types::deserialize_without_context!(Tup31, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);

sqlvalue::to_sql_row_impl! {
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    Tup31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>,
}


pub fn circuit(cconf: CircuitConfig) -> Result<(DBSPHandle, (ZSetHandle<Tup8<Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup2<Option<i32>, Option<Decimal>>>, ZSetHandle<Tup9<Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<i32>>>, ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>, ZSetHandle<Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>, ZSetHandle<Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>, OutputHandle<WSet<Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>>, OutputHandle<WSet<Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>>, )), Error> {

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

        // rel#82:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 2357(2230)
        let stream2357: Stream<_, IndexedWSet<Tup0, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>>> = stream279.map_index(move |t_1: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        (Tup0, Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, ) {
            (Tup0::new(), Tup21::new((*t_1).0, (*t_1).1, (*t_1).2, (*t_1).3.clone(), (*t_1).4.clone(), (*t_1).5.clone(), (*t_1).6.clone(), (*t_1).7.clone(), (*t_1).8.clone(), (*t_1).9.clone(), (*t_1).10.clone(), (*t_1).11.clone(), (*t_1).12, (*t_1).13.clone(), (*t_1).14.clone(), (*t_1).15.clone(), (*t_1).16.clone(), (*t_1).17.clone(), (*t_1).18, (*t_1).19, (*t_1).20.clone()), )
        });
        // rel#82:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 2363(2234)
        let stream2363: Stream<_, IndexedWSet<Tup0, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream337.map_index(move |t_2: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup0, Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup0::new(), Tup10::new((*t_2).0, (*t_2).1, (*t_2).2, (*t_2).3, (*t_2).4, (*t_2).5, (*t_2).6.clone(), (*t_2).7.clone(), (*t_2).8, (*t_2).9), )
        });
        // rel#82:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPJoinFlatmapOperator 8081(2240)
        let stream8081: Stream<_, WSet<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream2357.join_flatmap(&stream2363, move |t_4: &Tup0, t_1: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_2: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Option<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>> {
            let tmp: Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>> = Tup31::new((*t_1).0, (*t_1).1, (*t_1).2, (*t_1).3.clone(), (*t_1).4.clone(), (*t_1).5.clone(), (*t_1).6.clone(), (*t_1).7.clone(), (*t_1).8.clone(), (*t_1).9.clone(), (*t_1).10.clone(), (*t_1).11.clone(), (*t_1).12, (*t_1).13.clone(), (*t_1).14.clone(), (*t_1).15.clone(), (*t_1).16.clone(), (*t_1).17.clone(), (*t_1).18, (*t_1).19, (*t_1).20.clone(), (*t_2).0, (*t_2).1, (*t_2).2, (*t_2).3, (*t_2).4, (*t_2).5, (*t_2).6.clone(), (*t_2).7.clone(), (*t_2).8, (*t_2).9);
            (if wrap_bool(and_bN_bN(and_bN_bN(eq_sN_sN(cast_to_sN_sN(tmp.5.clone(), 20, false), tmp.27.clone()), eq_i32N_i32N(tmp.1, tmp.26)), eq_i32N_i32N(tmp.2, tmp.25))) {
                Some(tmp)
            } else {
                None::<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>
            })
        });
        // rel#84:LogicalProject.(input=LogicalJoin#82,exprs=[$3, $4, $0, $6, $7, $8, $9, $10, $11, $13, $14, $15, $16, $12])
        // DBSPMapOperator 8174(2243)
        let stream8174: Stream<_, WSet<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream8081.map(move |t_6: &Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>> {
            Tup14::new((*t_6).3.clone(), (*t_6).4.clone(), (*t_6).0, (*t_6).6.clone(), (*t_6).7.clone(), (*t_6).8.clone(), (*t_6).9.clone(), (*t_6).10.clone(), (*t_6).11.clone(), (*t_6).13.clone(), (*t_6).14.clone(), (*t_6).15.clone(), (*t_6).16.clone(), (*t_6).12)
        });
        // rel#86:LogicalSort.(input=LogicalProject#84,sort0=$0,dir0=ASC,fetch=1)
        // DBSPMapIndexOperator 8273(2246)
        let stream8273: Stream<_, IndexedWSet<(), Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream8174.map_index(move |t_7: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, | -> 
        ((), Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, ) {
            ((), Tup14::new((*t_7).0.clone(), (*t_7).1.clone(), (*t_7).2, (*t_7).3.clone(), (*t_7).4.clone(), (*t_7).5.clone(), (*t_7).6.clone(), (*t_7).7.clone(), (*t_7).8.clone(), (*t_7).9.clone(), (*t_7).10.clone(), (*t_7).11.clone(), (*t_7).12.clone(), (*t_7).13), )
        });
        // rel#86:LogicalSort.(input=LogicalProject#84,sort0=$0,dir0=ASC,fetch=1)
        struct Cmpstream8284;
        impl CmpFunc<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>> for Cmpstream8284 {
            fn cmp(left: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, right: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>) -> std::cmp::Ordering {
                let ord = left.0.cmp(&right.0);
                if ord != Ordering::Equal { return ord };
                let ord = left.1.cmp(&right.1);
                if ord != Ordering::Equal { return ord };
                let ord = left.2.cmp(&right.2);
                if ord != Ordering::Equal { return ord };
                let ord = left.3.cmp(&right.3);
                if ord != Ordering::Equal { return ord };
                let ord = left.4.cmp(&right.4);
                if ord != Ordering::Equal { return ord };
                let ord = left.5.cmp(&right.5);
                if ord != Ordering::Equal { return ord };
                let ord = left.6.cmp(&right.6);
                if ord != Ordering::Equal { return ord };
                let ord = left.7.cmp(&right.7);
                if ord != Ordering::Equal { return ord };
                let ord = left.8.cmp(&right.8);
                if ord != Ordering::Equal { return ord };
                let ord = left.9.cmp(&right.9);
                if ord != Ordering::Equal { return ord };
                let ord = left.10.cmp(&right.10);
                if ord != Ordering::Equal { return ord };
                let ord = left.11.cmp(&right.11);
                if ord != Ordering::Equal { return ord };
                let ord = left.12.cmp(&right.12);
                if ord != Ordering::Equal { return ord };
                let ord = left.13.cmp(&right.13);
                if ord != Ordering::Equal { return ord };
                return Ordering::Equal;
            }
        }
        // DBSPIndexedTopKOperator 8284(914)
        let stream8284: Stream<_, IndexedWSet<(), Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream8273.topk_custom_order::<Cmpstream8284>(cast_to_u_i32(1i32));
        // rel#86:LogicalSort.(input=LogicalProject#84,sort0=$0,dir0=ASC,fetch=1)
        // DBSPAggregateOperator 8316(2256)
        let stream8316: Stream<_, IndexedWSet<(), Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>> = stream8284.aggregate(Fold::<_, _, UnimplementedSemigroup<_>, _, _>::new(Vec::new(), move |t_8: &mut Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>, t_9: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, t_0: Weight, | {
            weighted_push(t_8, t_9, t_0)
        }));
        // rel#86:LogicalSort.(input=LogicalProject#84,sort0=$0,dir0=ASC,fetch=1)
        // DBSPMapOperator 8332(2258)
        let stream8332: Stream<_, WSet<Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>> = stream8316.map(move |(k, v): (&(), &Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>)| -> Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>> {
            let ec = Extract::new(move |r: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>| r.0.clone());
            let comp = move |a: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, b: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>| { ec.compare(a, b) };let mut v = v.clone();
            v.sort_by(comp);
            v
        });
        // CREATE VIEW `CUST_ENUM` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_ID`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_LAST` = `T`.`C_LAST` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_W_ID` = `T`.`C_W_ID`
        // ORDER BY `C_FIRST`
        // FETCH NEXT 1 ROWS ONLY
        // ORDER BY `C_FIRST`
        // FETCH NEXT 1 ROWS ONLY
        // DBSPSinkOperator 9518(1008)
        let handle9518 = stream8332.output();

        // rel#228:LogicalJoin.(left=LogicalTableScan#89,right=LogicalTableScan#91,condition=AND(=(CAST($5):VARCHAR(20), $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPJoinFlatmapOperator 14957(2284)
        let stream14957: Stream<_, WSet<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream2357.join_flatmap(&stream2363, move |t_13: &Tup0, t_10: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, t_11: &Tup10<Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        Option<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>> {
            let tmp: Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>> = Tup31::new((*t_10).0, (*t_10).1, (*t_10).2, (*t_10).3.clone(), (*t_10).4.clone(), (*t_10).5.clone(), (*t_10).6.clone(), (*t_10).7.clone(), (*t_10).8.clone(), (*t_10).9.clone(), (*t_10).10.clone(), (*t_10).11.clone(), (*t_10).12, (*t_10).13.clone(), (*t_10).14.clone(), (*t_10).15.clone(), (*t_10).16.clone(), (*t_10).17.clone(), (*t_10).18, (*t_10).19, (*t_10).20.clone(), (*t_11).0, (*t_11).1, (*t_11).2, (*t_11).3, (*t_11).4, (*t_11).5, (*t_11).6.clone(), (*t_11).7.clone(), (*t_11).8, (*t_11).9);
            (if wrap_bool(and_bN_bN(and_bN_bN(eq_sN_sN(cast_to_sN_sN(tmp.5.clone(), 20, false), tmp.27.clone()), eq_i32N_i32N(tmp.1, tmp.26)), eq_i32N_i32N(tmp.2, tmp.25))) {
                Some(tmp)
            } else {
                None::<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>
            })
        });
        // rel#230:LogicalProject.(input=LogicalTableScan#89,exprs=[$3])
        // DBSPMapOperator 2436(2287)
        let stream2436: Stream<_, WSet<Tup1<Option<String>>>> = stream279.map(move |t_15: &Tup21<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>>, | -> 
        Tup1<Option<String>> {
            Tup1::new((*t_15).3.clone())
        });
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPMapIndexOperator 2441(2290)
        let stream2441: Stream<_, IndexedWSet<Tup0, Tup1<Option<String>>>> = stream2436.map_index(move |t_16: &Tup1<Option<String>>, | -> 
        (Tup0, Tup1<Option<String>>, ) {
            (Tup0::new(), Tup1::new((*t_16).0.clone()), )
        });
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPAggregateOperator 15046(2294)
        let stream15046: Stream<_, IndexedWSet<Tup0, Tup1<Option<String>>>> = stream2441.aggregate(Fold::<_, _, Semigroup1<Option<String>, MaxSemigroup<Option<String>>>, _, _>::with_output(Tup1::new(None::<String>), move |t_39: &mut Tup1<Option<String>>, t_17: &Tup1<Option<String>>, t_41: Weight, | {
            (*t_39) = Tup1::new(agg_max_sN_sN((*t_39).0.clone(), (*t_17).0.clone()))
        }, move |t_40: Tup1<Option<String>>, | -> 
        Tup1<Option<String>> {
            Tup1::new(t_40.0)
        }));
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPMapOperator 15048(2296)
        let stream15048: Stream<_, WSet<Tup1<Option<String>>>> = stream15046.map(move |t_19: (&Tup0, &Tup1<Option<String>>, ), | -> 
        Tup1<Option<String>> {
            Tup1::new((*t_19.1).0.clone())
        });
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPMapOperator 15053(2301)
        let stream15053: Stream<_, WSet<Tup1<Option<String>>>> = stream15046.map(move |t_19: (&Tup0, &Tup1<Option<String>>, ), | -> 
        Tup1<Option<String>> {
            Tup1::new(None::<String>)
        });
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPNegateOperator 15058(2304)
        let stream15058: Stream<_, WSet<Tup1<Option<String>>>> = stream15053.neg();
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        let stream1505 = circuit.add_source(Generator::new(|| if Runtime::worker_index() == 0 {zset!(
            Tup1::new(None::<String>) => 1,
        )} else {zset!(
        )}));
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPDifferentiateOperator 2467(1505)
        let stream2467: Stream<_, WSet<Tup1<Option<String>>>> = stream1505.differentiate();
        // rel#232:LogicalAggregate.(input=LogicalProject#230,group={},EXPR$0=MAX($0))
        // DBSPSumOperator 15060(2306)
        let stream15060: Stream<_, WSet<Tup1<Option<String>>>> = stream2467.sum([&stream15058, &stream15048]);
        // rel#234:LogicalJoin.(left=LogicalJoin#228,right=LogicalAggregate#232,condition==($3, $31),joinType=inner)
        // DBSPFilterOperator 14959(2308)
        let stream14959: Stream<_, WSet<Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream14957.filter(move |t_21: &Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        bool {
            (!(*t_21).3.is_none())
        });
        // rel#234:LogicalJoin.(left=LogicalJoin#228,right=LogicalAggregate#232,condition==($3, $31),joinType=inner)
        // DBSPFilterOperator 15062(2311)
        let stream15062: Stream<_, WSet<Tup1<Option<String>>>> = stream15060.filter(move |t_22: &Tup1<Option<String>>, | -> 
        bool {
            (!(*t_22).0.is_none())
        });
        // rel#234:LogicalJoin.(left=LogicalJoin#228,right=LogicalAggregate#232,condition==($3, $31),joinType=inner)
        // DBSPMapIndexOperator 14962(2314)
        let stream14962: Stream<_, IndexedWSet<Tup1<String>, Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>>> = stream14959.map_index(move |t_23: &Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, | -> 
        (Tup1<String>, Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, ) {
            (Tup1::new(cast_to_s_sN((*t_23).3.clone(), 16, false)), Tup31::new((*t_23).0, (*t_23).1, (*t_23).2, (*t_23).3.clone(), (*t_23).4.clone(), (*t_23).5.clone(), (*t_23).6.clone(), (*t_23).7.clone(), (*t_23).8.clone(), (*t_23).9.clone(), (*t_23).10.clone(), (*t_23).11.clone(), (*t_23).12, (*t_23).13.clone(), (*t_23).14.clone(), (*t_23).15.clone(), (*t_23).16.clone(), (*t_23).17.clone(), (*t_23).18, (*t_23).19, (*t_23).20.clone(), (*t_23).21, (*t_23).22, (*t_23).23, (*t_23).24, (*t_23).25, (*t_23).26, (*t_23).27.clone(), (*t_23).28.clone(), (*t_23).29, (*t_23).30), )
        });
        // rel#234:LogicalJoin.(left=LogicalJoin#228,right=LogicalAggregate#232,condition==($3, $31),joinType=inner)
        // DBSPMapIndexOperator 15065(2318)
        let stream15065: Stream<_, IndexedWSet<Tup1<String>, Tup1<Option<String>>>> = stream15062.map_index(move |t_24: &Tup1<Option<String>>, | -> 
        (Tup1<String>, Tup1<Option<String>>, ) {
            (Tup1::new(cast_to_s_sN((*t_24).0.clone(), 16, false)), Tup1::new((*t_24).0.clone()), )
        });
        // rel#234:LogicalJoin.(left=LogicalJoin#228,right=LogicalAggregate#232,condition==($3, $31),joinType=inner)
        // DBSPJoinOperator 15069(2324)
        let stream15069: Stream<_, WSet<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream14962.join(&stream15065, move |t_25: &Tup1<String>, t_23: &Tup31<Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Timestamp>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<Decimal>, Option<Timestamp>, Option<Timestamp>>, t_24: &Tup1<Option<String>>, | -> 
        Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>> {
            Tup14::new((*t_23).3.clone().clone(), (*t_23).4.clone().clone(), (*t_23).0, (*t_23).6.clone().clone(), (*t_23).7.clone().clone(), (*t_23).8.clone().clone(), (*t_23).9.clone().clone(), (*t_23).10.clone().clone(), (*t_23).11.clone().clone(), (*t_23).13.clone().clone(), (*t_23).14.clone().clone(), (*t_23).15.clone().clone(), (*t_23).16.clone().clone(), (*t_23).12)
        });
        // rel#238:LogicalSort.(input=LogicalProject#236,fetch=1)
        // DBSPMapIndexOperator 15071(2327)
        let stream15071: Stream<_, IndexedWSet<(), Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream15069.map_index(move |t_28: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, | -> 
        ((), Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, ) {
            ((), Tup14::new((*t_28).0.clone(), (*t_28).1.clone(), (*t_28).2, (*t_28).3.clone(), (*t_28).4.clone(), (*t_28).5.clone(), (*t_28).6.clone(), (*t_28).7.clone(), (*t_28).8.clone(), (*t_28).9.clone(), (*t_28).10.clone(), (*t_28).11.clone(), (*t_28).12.clone(), (*t_28).13), )
        });
        // rel#238:LogicalSort.(input=LogicalProject#236,fetch=1)
        struct Cmpstream15075;
        impl CmpFunc<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>> for Cmpstream15075 {
            fn cmp(left: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, right: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>) -> std::cmp::Ordering {
                let ord = left.0.cmp(&right.0);
                if ord != Ordering::Equal { return ord };
                let ord = left.1.cmp(&right.1);
                if ord != Ordering::Equal { return ord };
                let ord = left.2.cmp(&right.2);
                if ord != Ordering::Equal { return ord };
                let ord = left.3.cmp(&right.3);
                if ord != Ordering::Equal { return ord };
                let ord = left.4.cmp(&right.4);
                if ord != Ordering::Equal { return ord };
                let ord = left.5.cmp(&right.5);
                if ord != Ordering::Equal { return ord };
                let ord = left.6.cmp(&right.6);
                if ord != Ordering::Equal { return ord };
                let ord = left.7.cmp(&right.7);
                if ord != Ordering::Equal { return ord };
                let ord = left.8.cmp(&right.8);
                if ord != Ordering::Equal { return ord };
                let ord = left.9.cmp(&right.9);
                if ord != Ordering::Equal { return ord };
                let ord = left.10.cmp(&right.10);
                if ord != Ordering::Equal { return ord };
                let ord = left.11.cmp(&right.11);
                if ord != Ordering::Equal { return ord };
                let ord = left.12.cmp(&right.12);
                if ord != Ordering::Equal { return ord };
                let ord = left.13.cmp(&right.13);
                if ord != Ordering::Equal { return ord };
                return Ordering::Equal;
            }
        }
        // DBSPIndexedTopKOperator 15075(2071)
        let stream15075: Stream<_, IndexedWSet<(), Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>> = stream15071.topk_custom_order::<Cmpstream15075>(cast_to_u_i32(1i32));
        // rel#238:LogicalSort.(input=LogicalProject#236,fetch=1)
        // DBSPAggregateOperator 15077(2337)
        let stream15077: Stream<_, IndexedWSet<(), Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>> = stream15075.aggregate(Fold::<_, _, UnimplementedSemigroup<_>, _, _>::new(Vec::new(), move |t_29: &mut Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>, t_30: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, t_0: Weight, | {
            weighted_push(t_29, t_30, t_0)
        }));
        // rel#238:LogicalSort.(input=LogicalProject#236,fetch=1)
        // DBSPMapOperator 15079(2339)
        let stream15079: Stream<_, WSet<Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>>> = stream15077.map(move |(k, v): (&(), &Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>>)| -> Vec<Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>> {
            let ec = ;
            let comp = move |a: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>, b: &Tup14<Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Timestamp>>| { ec.compare(a, b) };let mut v = v.clone();
            v.sort_by(comp);
            v
        });
        // CREATE VIEW `CUST_MAX` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_ID`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_LAST` = `T`.`C_LAST` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_W_ID` = `T`.`C_W_ID` AND `C`.`C_FIRST` = (((SELECT MAX(`CUSTOMER`.`C_FIRST`)
        // FROM `schema`.`CUSTOMER` AS `CUSTOMER`)))
        // FETCH NEXT 1 ROWS ONLY
        // FETCH NEXT 1 ROWS ONLY
        // DBSPSinkOperator 15084(2165)
        let handle15084 = stream15079.output();

        Ok((handle49, handle67, handle120, handle143, handle166, handle279, handle337, handle9518, handle15084, ))
    })?;
    Ok((circuit, streams))
}


