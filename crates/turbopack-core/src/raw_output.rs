use turbo_tasks::Vc;

use crate::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    output::OutputAsset,
    source::Source,
};

/// A module where source code doesn't need to be parsed but can be used as is.
/// This module has no references to other modules.
#[turbo_tasks::value]
pub struct RawOutput {
    source: Vc<Box<dyn Source>>,
}

#[turbo_tasks::value_impl]
impl OutputAsset for RawOutput {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.source.ident()
    }
}

#[turbo_tasks::value_impl]
impl Asset for RawOutput {
    #[turbo_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}

#[turbo_tasks::value_impl]
impl RawOutput {
    #[turbo_tasks::function]
    pub fn new(source: Vc<Box<dyn Source>>) -> Vc<RawOutput> {
        RawOutput { source }.cell()
    }
}
