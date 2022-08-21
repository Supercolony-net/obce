use obce::codegen::ExtensionDescription;

#[test]
fn definition_extension_id_by_name() {
    #[obce::definition(id = "pallet-assets-chain-extension@v0.1")]
    pub trait Trait {}

    assert_eq!(<dyn Trait as ExtensionDescription>::ID, 0x48f6);
}

#[test]
fn definition_extension_id_by_number() {
    #[obce::definition(id = 0x13)]
    pub trait Trait {}

    assert_eq!(<dyn Trait as ExtensionDescription>::ID, 0x13);
}
