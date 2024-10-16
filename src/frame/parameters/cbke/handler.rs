pub mod calculate_smacs;
pub mod calculate_smacs283k1;
pub mod dsa_sign;
pub mod dsa_verify;
pub mod generate_cbke_keys;
pub mod generate_cbke_keys283k1;

#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    CalculateSmacs(calculate_smacs::Handler),
    CalculateSmacs283k1(calculate_smacs283k1::Handler),
    DsaSign(dsa_sign::Handler),
    DsaVerify(dsa_verify::Handler),
    GenerateCbkeKeys(generate_cbke_keys::Handler),
    GenerateCbkeKeys283k1(generate_cbke_keys283k1::Handler),
}
