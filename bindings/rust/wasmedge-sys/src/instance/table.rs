use crate::{types::WasmEdgeRefType, wasmedge, Error, WasmEdgeResult};
use std::ops::Range;

#[derive(Debug)]
pub struct Table {
    pub(crate) ctx: *mut wasmedge::WasmEdge_TableInstanceContext,
    pub(crate) registered: bool,
}
impl Table {
    pub fn create(ref_type: WasmEdgeRefType, limit: Range<u32>) -> WasmEdgeResult<Self> {
        let ctx = unsafe {
            let table_ty = wasmedge::WasmEdge_TableTypeCreate(
                wasmedge::WasmEdge_RefType::from(ref_type),
                wasmedge::WasmEdge_Limit::from(limit),
            );
            wasmedge::WasmEdge_TableInstanceCreate(table_ty)
        };
        match ctx.is_null() {
            true => Err(Error::OperationError(String::from(
                "fail to create Table instance",
            ))),
            false => Ok(Table {
                ctx,
                registered: false,
            }),
        }
    }
}
impl Drop for Table {
    fn drop(&mut self) {
        if !self.registered && !self.ctx.is_null() {
            unsafe { wasmedge::WasmEdge_TableInstanceDelete(self.ctx) };
        }
    }
}
