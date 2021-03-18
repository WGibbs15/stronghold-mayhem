(function() {var implementors = {};
implementors["communication"] = [{"text":"impl&lt;Req:&nbsp;Debug + Clone + Serialize + DeserializeOwned&gt; Serialize for RequestEnvelope&lt;Req&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Req: Debug + Clone + Serialize + DeserializeOwned,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["iota_stronghold"] = [{"text":"impl Serialize for SLIP10DeriveInput","synthetic":false,"types":[]},{"text":"impl Serialize for Procedure","synthetic":false,"types":[]},{"text":"impl Serialize for ProcResult","synthetic":false,"types":[]},{"text":"impl Serialize for SerdeProcResult","synthetic":false,"types":[]},{"text":"impl Serialize for SHRequest","synthetic":false,"types":[]},{"text":"impl Serialize for SHResults","synthetic":false,"types":[]},{"text":"impl Serialize for Client","synthetic":false,"types":[]},{"text":"impl Serialize for SnapshotState","synthetic":false,"types":[]},{"text":"impl Serialize for ClientId","synthetic":false,"types":[]},{"text":"impl Serialize for VaultId","synthetic":false,"types":[]},{"text":"impl Serialize for ID","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Serialize for ResultMessage&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Serialize,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Serialize for Location","synthetic":false,"types":[]}];
implementors["runtime"] = [{"text":"impl&lt;T:&nbsp;Bytes&gt; Serialize for GuardedVec&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Serialize,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["store"] = [{"text":"impl&lt;K, V&gt; Serialize for Cache&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Clone + Debug,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Serialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Serialize,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Serialize for Value&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Serialize,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["vault"] = [{"text":"impl&lt;T:&nbsp;BoxProvider&gt; Serialize for Key&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Serialize for Transaction","synthetic":false,"types":[]},{"text":"impl Serialize for RecordHint","synthetic":false,"types":[]},{"text":"impl Serialize for ChainId","synthetic":false,"types":[]},{"text":"impl Serialize for TransactionId","synthetic":false,"types":[]},{"text":"impl Serialize for BlobId","synthetic":false,"types":[]},{"text":"impl Serialize for Kind","synthetic":false,"types":[]},{"text":"impl Serialize for ReadResult","synthetic":false,"types":[]},{"text":"impl Serialize for RecordId","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()