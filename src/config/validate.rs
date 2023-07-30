use crate::Result;

// TODO: Implement `Validate` trait for eligible types.

pub(crate) trait Validate<Ctx: ?Sized> {
    type Error: Into<crate::Error>;

    unsafe fn validate<'data>(
        value: *const Self,
        context: &mut Ctx,
    ) -> Result<&'data Self, Self::Error>;
}
