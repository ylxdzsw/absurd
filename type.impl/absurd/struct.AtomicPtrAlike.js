(function() {var type_impls = {
"absurd":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AtomicPtrAlike%3CT,+Option%3CS%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#114-147\">source</a><a href=\"#impl-AtomicPtrAlike%3CT,+Option%3CS%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, S&gt; <a class=\"struct\" href=\"absurd/struct.AtomicPtrAlike.html\" title=\"struct absurd::AtomicPtrAlike\">AtomicPtrAlike</a>&lt;T, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;&gt;<div class=\"where\">where\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;: <a class=\"trait\" href=\"absurd/trait.PtrAlike.html\" title=\"trait absurd::PtrAlike\">PtrAlike</a>&lt;T&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.none\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#115-117\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.none\" class=\"fn\">none</a>() -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.take_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#120-122\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.take_with_order\" class=\"fn\">take_with_order</a>(&amp;self, order: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><section id=\"method.take\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#124-126\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.take\" class=\"fn\">take</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;</h4></section><section id=\"method.take_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#128-130\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.take_seqcst\" class=\"fn\">take_seqcst</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_insert_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#133-138\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.try_insert_with_order\" class=\"fn\">try_insert_with_order</a>(\n    &amp;self,\n    val: S,\n    success: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>,\n    failure: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, S&gt;</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><section id=\"method.try_insert\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#140-142\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.try_insert\" class=\"fn\">try_insert</a>(&amp;self, val: S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, S&gt;</h4></section><section id=\"method.try_insert_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#144-146\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.try_insert_seqcst\" class=\"fn\">try_insert_seqcst</a>(&amp;self, val: S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, S&gt;</h4></section></div></details>",0,"absurd::atomic::AtomicOptionBox","absurd::atomic::AtomicOptionRef","absurd::atomic::AtomicOptionMutRef"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AtomicPtrAlike%3CT,+P%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#18-56\">source</a><a href=\"#impl-AtomicPtrAlike%3CT,+P%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, P: <a class=\"trait\" href=\"absurd/trait.PtrAlike.html\" title=\"trait absurd::PtrAlike\">PtrAlike</a>&lt;T&gt;&gt; <a class=\"struct\" href=\"absurd/struct.AtomicPtrAlike.html\" title=\"struct absurd::AtomicPtrAlike\">AtomicPtrAlike</a>&lt;T, P&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#19-24\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.new\" class=\"fn\">new</a>(val: P) -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.swap_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#27-29\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.swap_with_order\" class=\"fn\">swap_with_order</a>(&amp;self, val: P, order: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>) -&gt; P</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><section id=\"method.swap\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#31-33\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.swap\" class=\"fn\">swap</a>(&amp;self, val: P) -&gt; P</h4></section><section id=\"method.swap_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#35-37\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.swap_seqcst\" class=\"fn\">swap_seqcst</a>(&amp;self, val: P) -&gt; P</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.store_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#40-42\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.store_with_order\" class=\"fn\">store_with_order</a>(&amp;self, val: P, order: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>)</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><section id=\"method.store\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#44-46\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.store\" class=\"fn\">store</a>(&amp;self, val: P)</h4></section><section id=\"method.store_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#48-50\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.store_seqcst\" class=\"fn\">store_seqcst</a>(&amp;self, val: P)</h4></section><section id=\"method.into_inner\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#52-55\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.into_inner\" class=\"fn\">into_inner</a>(self) -&gt; P</h4></section></div></details>",0,"absurd::atomic::AtomicBox","absurd::atomic::AtomicOptionBox","absurd::atomic::AtomicRef","absurd::atomic::AtomicOptionRef","absurd::atomic::AtomicMutRef","absurd::atomic::AtomicOptionMutRef"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AtomicPtrAlike%3CT,+P%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#65-112\">source</a><a href=\"#impl-AtomicPtrAlike%3CT,+P%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, P: <a class=\"trait\" href=\"absurd/trait.PtrAlike.html\" title=\"trait absurd::PtrAlike\">PtrAlike</a>&lt;T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>&gt; <a class=\"struct\" href=\"absurd/struct.AtomicPtrAlike.html\" title=\"struct absurd::AtomicPtrAlike\">AtomicPtrAlike</a>&lt;T, P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.compare_exchange_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#67-71\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.compare_exchange_with_order\" class=\"fn\">compare_exchange_with_order</a>(\n    &amp;self,\n    current: P,\n    new: P,\n    success: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>,\n    failure: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;P, P&gt;</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compare_exchange\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#74-76\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.compare_exchange\" class=\"fn\">compare_exchange</a>(&amp;self, current: P, new: P) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;P, P&gt;</h4></section></summary><div class=\"docblock\"><p>Compare the value and swap if it is equal to the current value</p>\n</div></details><section id=\"method.compare_exchange_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#78-80\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.compare_exchange_seqcst\" class=\"fn\">compare_exchange_seqcst</a>(&amp;self, current: P, new: P) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;P, P&gt;</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.compare_exchange_weak_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#83-87\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.compare_exchange_weak_with_order\" class=\"fn\">compare_exchange_weak_with_order</a>(\n    &amp;self,\n    current: P,\n    new: P,\n    success: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>,\n    failure: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;P, P&gt;</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compare_exchange_weak\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#90-92\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.compare_exchange_weak\" class=\"fn\">compare_exchange_weak</a>(&amp;self, current: P, new: P) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;P, P&gt;</h4></section></summary><div class=\"docblock\"><p>Compare the value and swap if it is equal to the current value, but may fail spuriously</p>\n</div></details><section id=\"method.compare_exchange_weak_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#94-96\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.compare_exchange_weak_seqcst\" class=\"fn\">compare_exchange_weak_seqcst</a>(&amp;self, current: P, new: P) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;P, P&gt;</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.load_with_order\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#99-101\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.load_with_order\" class=\"fn\">load_with_order</a>(&amp;self, order: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html\" title=\"enum core::sync::atomic::Ordering\">Ordering</a>) -&gt; P</h4></section></summary><div class=\"docblock\"><p>Safety: relaxed ordering may cause invalid pointers</p>\n</div></details><section id=\"method.load\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#103-105\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.load\" class=\"fn\">load</a>(&amp;self) -&gt; P</h4></section><section id=\"method.load_seqcst\" class=\"method\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#107-109\">source</a><h4 class=\"code-header\">pub fn <a href=\"absurd/struct.AtomicPtrAlike.html#tymethod.load_seqcst\" class=\"fn\">load_seqcst</a>(&amp;self) -&gt; P</h4></section></div></details>",0,"absurd::atomic::AtomicBox","absurd::atomic::AtomicOptionBox","absurd::atomic::AtomicRef","absurd::atomic::AtomicOptionRef","absurd::atomic::AtomicMutRef","absurd::atomic::AtomicOptionMutRef"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-AtomicPtrAlike%3CT,+Option%3CS%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#149-153\">source</a><a href=\"#impl-Default-for-AtomicPtrAlike%3CT,+Option%3CS%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"absurd/struct.AtomicPtrAlike.html\" title=\"struct absurd::AtomicPtrAlike\">AtomicPtrAlike</a>&lt;T, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;&gt;<div class=\"where\">where\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;: <a class=\"trait\" href=\"absurd/trait.PtrAlike.html\" title=\"trait absurd::PtrAlike\">PtrAlike</a>&lt;T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#150-152\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","absurd::atomic::AtomicOptionBox","absurd::atomic::AtomicOptionRef","absurd::atomic::AtomicOptionMutRef"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-AtomicPtrAlike%3CT,+P%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#58-63\">source</a><a href=\"#impl-Drop-for-AtomicPtrAlike%3CT,+P%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, P: <a class=\"trait\" href=\"absurd/trait.PtrAlike.html\" title=\"trait absurd::PtrAlike\">PtrAlike</a>&lt;T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"absurd/struct.AtomicPtrAlike.html\" title=\"struct absurd::AtomicPtrAlike\">AtomicPtrAlike</a>&lt;T, P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#59-62\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","absurd::atomic::AtomicBox","absurd::atomic::AtomicOptionBox","absurd::atomic::AtomicRef","absurd::atomic::AtomicOptionRef","absurd::atomic::AtomicMutRef","absurd::atomic::AtomicOptionMutRef"],["<section id=\"impl-Sync-for-AtomicPtrAlike%3CT,+P%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/absurd/atomic.rs.html#16\">source</a><a href=\"#impl-Sync-for-AtomicPtrAlike%3CT,+P%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>, P: <a class=\"trait\" href=\"absurd/trait.PtrAlike.html\" title=\"trait absurd::PtrAlike\">PtrAlike</a>&lt;T&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"absurd/struct.AtomicPtrAlike.html\" title=\"struct absurd::AtomicPtrAlike\">AtomicPtrAlike</a>&lt;T, P&gt;</h3></section>","Sync","absurd::atomic::AtomicBox","absurd::atomic::AtomicOptionBox","absurd::atomic::AtomicRef","absurd::atomic::AtomicOptionRef","absurd::atomic::AtomicMutRef","absurd::atomic::AtomicOptionMutRef"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()