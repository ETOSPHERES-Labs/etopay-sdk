pub trait Migrate {
    type Next;
    fn migrate(self) -> Self::Next;
}
