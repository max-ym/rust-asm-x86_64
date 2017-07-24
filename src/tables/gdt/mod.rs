use super::*;

/// GDT controller. All operations on GDT are made through methods of this
/// controller.
pub struct GdtCtrl {
    limit   : u16,
    addr    : u64,
}
