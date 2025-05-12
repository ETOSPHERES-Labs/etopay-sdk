pub trait EnumOrderMap {
    fn order_to_variant_map() -> std::collections::BTreeMap<u64, String>;
}
