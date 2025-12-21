use std::marker::PhantomData;

use crate::{
    autocmds::AutoCmds,
    keymaps::Mapping, //
};

pub struct KeymapsUnset;
pub struct KeymapsSet;
pub struct AutocmdsUnset;
pub struct AutocmdsSet;

// Init builder
#[derive(Default)]
pub struct Init<K, A> {
    _keymaps: PhantomData<K>,
    _autocmds: PhantomData<A>,
}

// Initial state
impl Init<KeymapsUnset, AutocmdsUnset> {
    pub fn new() -> Self {
        Self { _keymaps: PhantomData, _autocmds: PhantomData }
    }
}

// keymaps(): callable only once
impl<A> Init<KeymapsUnset, A> {
    pub fn keymaps(self) -> Init<KeymapsSet, A> {
        let _ = Mapping::init();

        Init { _keymaps: PhantomData, _autocmds: PhantomData }
    }
}

// autocmds(): callable only once
impl<K> Init<K, AutocmdsUnset> {
    pub fn autocmds(self) -> Init<K, AutocmdsSet> {
        let _ = AutoCmds::new("ABSTRACT_AUTOCMDS");

        Init { _keymaps: PhantomData, _autocmds: PhantomData }
    }
}

// Optional: finalize step
impl Init<KeymapsSet, AutocmdsSet> {
    pub fn build(self) {}
}
