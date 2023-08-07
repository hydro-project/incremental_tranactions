use crate::{
    codegen::{
        utils::{set_column_null, FunctionBuilderExt},
        Codegen, CodegenCtx,
    },
    ir::{ColumnType, LayoutId},
    utils::HashMap,
};
use cranelift::prelude::FunctionBuilder;
use cranelift_codegen::ir::{InstBuilder, MemFlags};
use cranelift_module::{FuncId, Module};

// The index of a column within a row
// TODO: Newtyping for column indices within the layout interfaces
type ColumnIdx = usize;

pub struct JsonMapping {
    pub layout: LayoutId,
    /// A map between column indices and the json pointer used to access them
    pub mappings: HashMap<ColumnIdx, String>,
}

impl Codegen {
    pub(crate) fn deserialize_json(&mut self, mappings: &JsonMapping) -> FuncId {
        let layout_id = mappings.layout;

        tracing::trace!(
            "creating json deserializer for {}",
            self.layout_cache.row_layout(layout_id),
        );

        // fn(*mut u8, *const JsonValue)
        let ptr_ty = self.module.isa().pointer_type();
        let func_id = self.create_function([ptr_ty; 2], None);

        self.set_comment_writer(
            &format!("deserialize_json_{layout_id}"),
            &format!(
                "fn(*mut {}, *const JsonValue)",
                self.layout_cache.row_layout(layout_id),
            ),
        );

        {
            let mut ctx = CodegenCtx::new(
                self.config,
                &mut self.module,
                &mut self.data_ctx,
                &mut self.data,
                self.layout_cache.clone(),
                self.intrinsics.import(self.comment_writer.clone()),
                self.comment_writer.clone(),
            );
            let mut builder =
                FunctionBuilder::new(&mut self.module_ctx.func, &mut self.function_ctx);

            // Create the entry block
            let entry_block = builder.create_entry_block();
            let [place, json_map]: [_; 2] = builder.block_params(entry_block).try_into().unwrap();

            let layout_cache = ctx.layout_cache.clone();
            let (layout, row_layout) = layout_cache.get_layouts(layout_id);

            // ctx.debug_assert_ptr_valid(place, layout.align(), &mut builder);
            // ctx.debug_assert_ptr_valid(json_map, align_of::<JsonValue>() as u32, &mut builder);

            for (column_idx, (column_ty, nullable)) in row_layout.iter().enumerate() {
                // TODO: Json pointers include `/`s to delimit each token, so
                // if a "pointer" doesn't have any `/`s then we can index
                // directly with that single ident, potentially saving work
                let json_pointer = &*mappings.mappings[&column_idx];
                assert!(
                    !json_pointer.is_empty(),
                    "json pointers cannot be empty (column {column_idx} of {layout_id})",
                );
                assert!(
                    json_pointer.starts_with('/'),
                    "json pointers must start with `/` (this restriction may be loosened in the future)",
                );

                // Add the json pointer to the function's data
                let (json_pointer, json_pointer_len) =
                    ctx.import_string(json_pointer, &mut builder);

                // Get a pointer to the column
                let column_offset = builder
                    .ins()
                    .iconst(ptr_ty, layout.offset_of(column_idx) as i64);
                let column_place = builder.ins().iadd(place, column_offset);

                match column_ty {
                    ColumnType::String => {
                        // Call the deserialization function
                        let deserialize_string =
                            ctx.imports
                                .get("deserialize_json_string", ctx.module, builder.func);
                        let value_is_null = builder.call_fn(
                            deserialize_string,
                            &[column_place, json_pointer, json_pointer_len, json_map],
                        );

                        // If the column is nullable, set its nullness
                        if nullable {
                            let set_null = builder.create_block();
                            let after = builder.create_block();

                            builder.ins().brif(value_is_null, set_null, &[], after, &[]);
                            builder.switch_to_block(set_null);

                            let null = builder.ins().iconst(ptr_ty, 0);
                            builder
                                .ins()
                                .store(MemFlags::trusted(), null, column_place, 0);

                            builder.ins().jump(after, &[]);
                            builder.switch_to_block(after);
                            builder.seal_block(set_null);

                        // Otherwise trap if deserialization fails or the field
                        // is null
                        // FIXME: This probably shouldn't be a debug assertion,
                        // and we probably want more graceful error handling in
                        // general
                        } else {
                            ctx.debug_assert_false(value_is_null, &mut builder);
                        }
                    }

                    ColumnType::I64 => {
                        // Call the deserialization function
                        let deserialize_i64 =
                            ctx.imports
                                .get("deserialize_json_i64", ctx.module, builder.func);
                        let value_is_null = builder.call_fn(
                            deserialize_i64,
                            &[column_place, json_pointer, json_pointer_len, json_map],
                        );

                        // If the column is nullable, set its nullness
                        if nullable {
                            set_column_null(
                                value_is_null,
                                column_idx,
                                place,
                                MemFlags::trusted(),
                                &layout,
                                &mut builder,
                            );

                        // Otherwise trap if deserialization fails or the field
                        // is null
                        // FIXME: This probably shouldn't be a debug assertion,
                        // and we probably want more graceful error handling in
                        // general
                        } else {
                            ctx.debug_assert_false(value_is_null, &mut builder);
                        }
                    }

                    ColumnType::F64 => {
                        // Call the deserialization function
                        let deserialize_i64 =
                            ctx.imports
                                .get("deserialize_json_f64", ctx.module, builder.func);
                        let value_is_null = builder.call_fn(
                            deserialize_i64,
                            &[column_place, json_pointer, json_pointer_len, json_map],
                        );

                        // If the column is nullable, set its nullness
                        if nullable {
                            set_column_null(
                                value_is_null,
                                column_idx,
                                place,
                                MemFlags::trusted(),
                                &layout,
                                &mut builder,
                            );

                        // Otherwise trap if deserialization fails or the field
                        // is null
                        // FIXME: This probably shouldn't be a debug assertion,
                        // and we probably want more graceful error handling in
                        // general
                        } else {
                            ctx.debug_assert_false(value_is_null, &mut builder);
                        }
                    }

                    _ => todo!(),
                }
            }

            builder.ins().return_(&[]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        self.finalize_function(func_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::{json::JsonMapping, Codegen, CodegenConfig},
        ir::{ColumnType, RowLayoutBuilder, RowLayoutCache},
        row::UninitRow,
        utils::{self, HashMap},
    };
    use std::mem::transmute;

    #[test]
    fn deserialize_json_smoke() {
        utils::test_logger();

        let layout_cache = RowLayoutCache::new();
        let layout = layout_cache.add(
            RowLayoutBuilder::new()
                .with_column(ColumnType::String, false)
                .with_column(ColumnType::String, true)
                .with_column(ColumnType::I64, false)
                .with_column(ColumnType::I64, true)
                .with_column(ColumnType::F64, false)
                .with_column(ColumnType::F64, true)
                .build(),
        );

        let mut codegen = Codegen::new(layout_cache, CodegenConfig::debug());

        let mut mappings = HashMap::default();
        mappings.insert(0, "/foo".to_owned());
        mappings.insert(1, "/bar".to_owned());
        mappings.insert(2, "/baz".to_owned());
        mappings.insert(3, "/bing".to_owned());
        mappings.insert(4, "/bop".to_owned());
        mappings.insert(5, "/boop".to_owned());

        let deserialize_json = codegen.deserialize_json(&JsonMapping { layout, mappings });
        let vtable = codegen.vtable_for(layout);

        let json_snippets = &[
            r#"{ "foo": "foo data string", "bar": "bar data string", "baz": 10, "bing": 100, "bop": 96.542, "boop": -1245.53 }"#,
            r#"{ "foo": "second foo data string", "bar": null, "baz": -10000, "bing": null, "bop": -0.0, "boop": null }"#,
            r#"{ "baz": -32, "bar": null, "foo": "woah, now we switched the field orderings", "bop": 0.3 }"#,
        ];

        let (jit, layout_cache) = codegen.finalize_definitions();
        let vtable = Box::into_raw(Box::new(vtable.marshalled(&jit)));

        {
            let deserialize_json = unsafe {
                transmute::<_, unsafe extern "C" fn(*mut u8, *const u8)>(
                    jit.get_finalized_function(deserialize_json),
                )
            };

            for &json in json_snippets {
                let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
                let mut uninit = UninitRow::new(unsafe { &*vtable });

                unsafe {
                    deserialize_json(
                        uninit.as_mut_ptr(),
                        &json_value as *const serde_json::Value as *const u8,
                    );
                }

                let row = unsafe { uninit.assume_init() };
                println!(
                    "input json: {json:?}\nrow value for {}: {row:?}",
                    layout_cache.row_layout(layout),
                );
            }
        }

        unsafe {
            drop(Box::from_raw(vtable));
            jit.free_memory();
        }
    }
}
