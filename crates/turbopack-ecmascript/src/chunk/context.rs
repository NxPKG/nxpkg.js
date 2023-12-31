use turbo_tasks::Vc;
use turbopack_core::chunk::ChunkingContext;

/// [`EcmascriptChunkingContext`] must be implemented by [`ChunkingContext`]
/// implementors that want to operate on [`EcmascriptChunk`]s.
#[turbo_tasks::value_trait]
pub trait EcmascriptChunkingContext: ChunkingContext {
    /// Whether chunk items generated by this chunking context should include
    /// the `__turbopack_refresh__` argument.
    fn has_react_refresh(self: Vc<Self>) -> Vc<bool> {
        Vc::cell(false)
    }
}
