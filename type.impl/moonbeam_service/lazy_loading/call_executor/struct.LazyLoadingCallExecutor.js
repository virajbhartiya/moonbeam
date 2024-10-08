(function() {var type_impls = {
"moonbeam_service":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#47-138\">source</a><a href=\"#impl-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block: BlockT + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.210/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>, B, E&gt; <a class=\"struct\" href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html\" title=\"struct moonbeam_service::lazy_loading::call_executor::LazyLoadingCallExecutor\">LazyLoadingCallExecutor</a>&lt;Block, B, E&gt;<div class=\"where\">where\n    E: CodeExecutor + RuntimeVersionOf + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'static,\n    B: Backend&lt;Block&gt;,\n    Block::Hash: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;H256&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#54-82\">source</a><h4 class=\"code-header\">pub fn <a href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html#tymethod.new\" class=\"fn\">new</a>(\n    backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;B&gt;,\n    lazy_loading_config: &amp;<a class=\"struct\" href=\"moonbeam_cli_opt/struct.LazyLoadingConfig.html\" title=\"struct moonbeam_cli_opt::LazyLoadingConfig\">LazyLoadingConfig</a>,\n    executor: E,\n    client_config: ClientConfig&lt;Block&gt;,\n    execution_extensions: ExecutionExtensions&lt;Block&gt;\n) -&gt; Result&lt;Self&gt;</h4></section></summary><div class=\"docblock\"><p>Creates new instance of local call executor.</p>\n</div></details></div></details>",0,"moonbeam_service::lazy_loading::TLazyLoadingCallExecutor"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#140-156\">source</a><a href=\"#impl-Clone-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block: BlockT + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.210/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>, B, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html\" title=\"struct moonbeam_service::lazy_loading::call_executor::LazyLoadingCallExecutor\">LazyLoadingCallExecutor</a>&lt;Block, B, E&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#145-155\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Self</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","moonbeam_service::lazy_loading::TLazyLoadingCallExecutor"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GetNativeVersion-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#321-330\">source</a><a href=\"#impl-GetNativeVersion-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, B, E&gt; GetNativeVersion for <a class=\"struct\" href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html\" title=\"struct moonbeam_service::lazy_loading::call_executor::LazyLoadingCallExecutor\">LazyLoadingCallExecutor</a>&lt;Block, B, E&gt;<div class=\"where\">where\n    B: Backend&lt;Block&gt;,\n    E: CodeExecutor + GetNativeVersion + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'static,\n    Block: BlockT + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.210/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.native_version\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#327-329\">source</a><a href=\"#method.native_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">native_version</a>(&amp;self) -&gt; &amp;NativeVersion</h4></section></summary><div class='docblock'>Returns the version of the native runtime.</div></details></div></details>","GetNativeVersion","moonbeam_service::lazy_loading::TLazyLoadingCallExecutor"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RuntimeVersionOf-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#295-307\">source</a><a href=\"#impl-RuntimeVersionOf-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B, E, Block&gt; RuntimeVersionOf for <a class=\"struct\" href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html\" title=\"struct moonbeam_service::lazy_loading::call_executor::LazyLoadingCallExecutor\">LazyLoadingCallExecutor</a>&lt;Block, B, E&gt;<div class=\"where\">where\n    E: RuntimeVersionOf,\n    Block: BlockT,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.runtime_version\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#300-306\">source</a><a href=\"#method.runtime_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">runtime_version</a>(\n    &amp;self,\n    ext: &amp;mut dyn Externalities,\n    runtime_code: &amp;RuntimeCode&lt;'_&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;RuntimeVersion, Error&gt;</h4></section></summary><div class='docblock'>Extract [<code>RuntimeVersion</code>] of the given <code>runtime_code</code>.</div></details></div></details>","RuntimeVersionOf","moonbeam_service::lazy_loading::TLazyLoadingCallExecutor"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CallExecutor%3CBlock%3E-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#158-293\">source</a><a href=\"#impl-CallExecutor%3CBlock%3E-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B, E, Block&gt; CallExecutor&lt;Block&gt; for <a class=\"struct\" href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html\" title=\"struct moonbeam_service::lazy_loading::call_executor::LazyLoadingCallExecutor\">LazyLoadingCallExecutor</a>&lt;Block, B, E&gt;<div class=\"where\">where\n    B: Backend&lt;Block&gt;,\n    E: CodeExecutor + RuntimeVersionOf + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'static,\n    Block: BlockT + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.210/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,\n    Block::Hash: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;H256&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = &lt;E as CodeExecutor&gt;::Error</h4></section></summary><div class='docblock'>Externalities error type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Backend\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Backend\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Backend</a> = B</h4></section></summary><div class='docblock'>The backend used by the node.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.execution_extensions\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#169-171\">source</a><a href=\"#method.execution_extensions\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">execution_extensions</a>(&amp;self) -&gt; &amp;ExecutionExtensions&lt;Block&gt;</h4></section></summary><div class='docblock'>Returns the [<code>ExecutionExtensions</code>].</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.call\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#173-209\">source</a><a href=\"#method.call\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">call</a>(\n    &amp;self,\n    at_hash: Block::Hash,\n    method: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.str.html\">str</a>,\n    call_data: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>],\n    context: CallContext\n) -&gt; Result&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>&gt;&gt;</h4></section></summary><div class='docblock'>Execute a call to a contract on top of state in a block of given hash. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.contextual_call\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#211-249\">source</a><a href=\"#method.contextual_call\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">contextual_call</a>(\n    &amp;self,\n    at_hash: Block::Hash,\n    method: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.str.html\">str</a>,\n    call_data: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>],\n    changes: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/core/cell/struct.RefCell.html\" title=\"struct core::cell::RefCell\">RefCell</a>&lt;OverlayedChanges&lt;HashingFor&lt;Block&gt;&gt;&gt;,\n    _recorder: &amp;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;ProofRecorder&lt;Block&gt;&gt;,\n    call_context: CallContext,\n    extensions: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/core/cell/struct.RefCell.html\" title=\"struct core::cell::RefCell\">RefCell</a>&lt;Extensions&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>&gt;, Error&gt;</h4></section></summary><div class='docblock'>Execute a contextual call on top of state in a block of a given hash. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.runtime_version\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#251-260\">source</a><a href=\"#method.runtime_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">runtime_version</a>(&amp;self, at_hash: Block::Hash) -&gt; Result&lt;RuntimeVersion&gt;</h4></section></summary><div class='docblock'>Extract RuntimeVersion of given block <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prove_execution\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#262-292\">source</a><a href=\"#method.prove_execution\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">prove_execution</a>(\n    &amp;self,\n    at_hash: Block::Hash,\n    method: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.str.html\">str</a>,\n    call_data: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>]\n) -&gt; Result&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.u8.html\">u8</a>&gt;, StorageProof)&gt;</h4></section></summary><div class='docblock'>Prove the execution of the given <code>method</code>. <a>Read more</a></div></details></div></details>","CallExecutor<Block>","moonbeam_service::lazy_loading::TLazyLoadingCallExecutor"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GetRuntimeVersionAt%3CBlock%3E-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#309-319\">source</a><a href=\"#impl-GetRuntimeVersionAt%3CBlock%3E-for-LazyLoadingCallExecutor%3CBlock,+B,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, B, E&gt; GetRuntimeVersionAt&lt;Block&gt; for <a class=\"struct\" href=\"moonbeam_service/lazy_loading/call_executor/struct.LazyLoadingCallExecutor.html\" title=\"struct moonbeam_service::lazy_loading::call_executor::LazyLoadingCallExecutor\">LazyLoadingCallExecutor</a>&lt;Block, B, E&gt;<div class=\"where\">where\n    B: Backend&lt;Block&gt;,\n    E: CodeExecutor + RuntimeVersionOf + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'static,\n    Block: BlockT + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.210/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,\n    Block::Hash: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;H256&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.runtime_version\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/moonbeam_service/lazy_loading/call_executor.rs.html#316-318\">source</a><a href=\"#method.runtime_version\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">runtime_version</a>(&amp;self, at: Block::Hash) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;RuntimeVersion, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;</h4></section></summary><div class='docblock'>Returns the version of runtime at the given block.</div></details></div></details>","GetRuntimeVersionAt<Block>","moonbeam_service::lazy_loading::TLazyLoadingCallExecutor"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()