var Di=Object.defineProperty;var kr=n=>{throw TypeError(n)};var Pi=(n,e,t)=>e in n?Di(n,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):n[e]=t;var K=(n,e,t)=>Pi(n,typeof e!="symbol"?e+"":e,t),Ot=(n,e,t)=>e.has(n)||kr("Cannot "+t);var l=(n,e,t)=>(Ot(n,e,"read from private field"),t?t.call(n):e.get(n)),S=(n,e,t)=>e.has(n)?kr("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,t),E=(n,e,t,r)=>(Ot(n,e,"write to private field"),r?r.call(n,t):e.set(n,t),t),b=(n,e,t)=>(Ot(n,e,"access private method"),t);const Dr={},V=Symbol("uninitialized"),Pr="http://www.w3.org/1999/xhtml",Ri="http://www.w3.org/2000/svg",Hi="http://www.w3.org/1998/Math/MathML",Rr=!1;var Tt=Array.isArray,Fi=Array.prototype.indexOf,Xe=Array.prototype.includes,Bt=Array.from,Hr=Object.defineProperty,me=Object.getOwnPropertyDescriptor,Vi=Object.getOwnPropertyDescriptors,qi=Object.prototype,Ui=Array.prototype,Fr=Object.getPrototypeOf,xr=Object.isExtensible;const Je=()=>{};function Yi(n){for(var e=0;e<n.length;e++)n[e]()}function Vr(){var n,e,t=new Promise((r,i)=>{n=r,e=i});return{promise:t,resolve:n,reject:e}}const Q=2,Ae=4,jt=8,qr=1<<24,Mn=16,In=32,ee=64,Ut=128,cr=256,En=512,q=1024,D=2048,gn=4096,rn=8192,Z=16384,Ne=32768,mt=1<<25,fe=65536,kt=1<<17,Gi=1<<18,Oe=1<<19,Zi=1<<20,Dn=1<<25,xt=1<<21,ke=1<<22,ne=1<<23,xe=Symbol("$state"),Ur=Symbol("component"),Ki=Symbol("legacy props"),Wi=Symbol(""),_t=Symbol("attributes"),Yt=Symbol("class"),Gt=Symbol("style"),Re=Symbol("text"),ut=new class extends Error{constructor(){super(...arguments);K(this,"name","StaleReactionError");K(this,"message","The reaction that called `getAbortSignal()` was re-run or destroyed")}};function Xi(){console.warn("https://svelte.dev/e/derived_inert")}function Ji(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function Qi(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}function Yr(n){return n===this.v}function $i(n,e){return n!=n?e==e:n!==e||n!==null&&typeof n=="object"||typeof n=="function"}function Gr(n){return!$i(n,this.v)}function ns(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function es(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function ts(n,e,t){throw new Error("https://svelte.dev/e/each_key_duplicate")}function rs(n){throw new Error("https://svelte.dev/e/effect_in_teardown")}function is(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function ss(n){throw new Error("https://svelte.dev/e/effect_orphan")}function os(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function ls(n){throw new Error("https://svelte.dev/e/props_invalid_value")}function as(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function cs(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function us(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function ds(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let fs=!1;function hs(n){let e=n.p;for(;e!==null&&e.c===null;)e=e.p;return(e==null?void 0:e.c)??null}function Zr(n,e){return n===null&&ns(),n.c??(n.c=new Map(hs(n)||void 0))}let J=null;function Te(n){J=n}function vs(n){return Zr(J).get(n)}function gs(n,e){return Zr(J).set(n,e),e}function Le(n,e=!1,t){J={p:J,i:!1,c:null,e:null,s:n,x:null,r:T,l:null}}function De(n){var e=J,t=e.e;if(t!==null){e.e=null;for(var r of t)_i(r)}return e.i=!0,J=e.p,Kr(n)}function Kr(n={}){return Hr(n,Ur,{value:!0}),n}function Wr(){return!0}let pe=[];function _s(){var n=pe;pe=[],Yi(n)}function $n(n){if(pe.length===0){var e=pe;queueMicrotask(()=>{e===pe&&_s()})}pe.push(n)}const ws=-7169;function H(n,e){n.f=n.f&ws|e}function ur(n){(n.f&En)!==0||n.deps===null?H(n,q):H(n,gn)}function Xr(n,e){(n.f&D)!==0?e.set(n,D):(n.f&gn)!==0&&e.get(n)!==D&&e.set(n,gn),H(n,q)}function zt(n){var e=I,t=T;bn(null),Hn(null);try{return n()}finally{bn(e),Hn(t)}}function ps(n,e,t,r){const i=Qe;var s=n.filter(u=>!u.settled),o=e.map(i);if(t.length===0&&s.length===0){r(o);return}var a=T,c=ms(),d=s.length===1?s[0].promise:s.length>1?Promise.all(s.map(u=>u.promise)):null;function f(u){if((a.f&Z)===0){c();try{r([...o,...u])}catch(v){Ln(v,a)}bt()}}var _=Jr();if(t.length===0){d.then(()=>f([])).finally(_);return}function g(){Promise.all(t.map(u=>ks(u))).then(f).catch(u=>Ln(u,a)).finally(_)}d?d.then(()=>{if((a.f&Z)!==0){_();return}c(),g(),bt()}):g()}function ms(){var n=T,e=I,t=J,r=y;return function(s=!0){Hn(n),bn(e),Te(t),s&&(n.f&Z)===0&&(r==null||r.activate(),r==null||r.apply())}}function bt(n=!0){Hn(null),bn(null),Te(null),n&&(y==null||y.deactivate())}function Jr(){var n=T,e=n.b,t=y,r=!!(e!=null&&e.is_rendered());return e==null||e.update_pending_count(1,t),t.increment(r,n),()=>{e==null||e.update_pending_count(-1,t),t.decrement(r,n)}}function Qe(n){var e=Q|D;return T!==null&&(T.f|=Oe),{ctx:J,deps:null,effects:null,equals:Yr,f:e,fn:n,reactions:null,rv:0,v:V,wv:0,parent:T,ac:null}}const He=Symbol("obsolete");function ks(n,e,t){let r=T;r===null&&es();var i=void 0,s=ve(V),o=!I,a=new Set;return Ls(()=>{var u,v;var c=T,d=Vr();i=d.promise;try{Promise.resolve(n()).then(d.resolve,w=>{w!==ut&&d.reject(w)}).finally(bt)}catch(w){d.reject(w),bt()}var f=y;if(o){if((c.f&Ne)!==0)var _=Jr();if((u=r.b)!=null&&u.is_rendered())(v=f.async_deriveds.get(c))==null||v.reject(He);else for(const w of a.values())w.reject(He);a.add(d),f.async_deriveds.set(c,d)}const g=(w,h=void 0)=>{_==null||_(),a.delete(d),h!==He&&(f.activate(),h?(s.f|=ne,Be(s,h)):((s.f&ne)!==0&&(s.f^=ne),Be(s,w)),f.deactivate())};d.promise.then(g,w=>g(null,w||"unknown"))}),gi(()=>{for(const c of a)c.reject(He)}),new Promise(c=>{function d(f){function _(){f===i?c(s):d(i)}f.then(_,_)}d(i)})}function yt(n){const e=Qe(n);return bi(e),e}function Qr(n){const e=Qe(n);return e.equals=Gr,e}function xs(n){var e=n.effects;if(e!==null){n.effects=null;for(var t=0;t<e.length;t+=1)on(e[t])}}function dr(n){var e,t=T,r=n.parent;if(!Kn&&r!==null&&n.v!==V&&(r.f&(Z|rn))!==0)return Xi(),n.v;Hn(r);try{xs(n),e=Si(n)}finally{Hn(t)}return e}function $r(n){var e=dr(n);if(!n.equals(e)&&(n.wv=Ci(),(!(y!=null&&y.is_fork)||n.deps===null)&&(y!==null?(y.capture(n,e,!0),Ye==null||Ye.capture(n,e,!0)):n.v=e,n.deps===null))){H(n,q);return}Kn||(G!==null?(gr()||y!=null&&y.is_fork)&&G.set(n,e):ur(n))}function bs(n){var e;if(n.effects!==null)for(const t of n.effects)(t.teardown||t.ac)&&((e=t.teardown)==null||e.call(t),t.ac!==null&&zt(()=>{t.ac.abort(ut),t.ac=null}),t.fn!==null&&(t.teardown=Je),$e(t,0),_r(t))}function ni(n){if(n.effects!==null)for(const e of n.effects)e.teardown&&e.fn!==null&&je(e)}let Lt=null,_e=null,y=null,Ye=null,G=null,Zt=null,Dt=!1,Ge=null,wt=null;var br=0;let ys=1;var ye,Ce,Me,Vn,Se,qn,nt,Bn,jn,Ee,oe,A,Kt,ei,Wt,Fe,Xt,ti,ri,ii,we,Cs,Ve;const Et=class Et{constructor(){S(this,A);K(this,"id",ys++);K(this,"started",!1);K(this,"linked",!0);K(this,"prev",null);K(this,"next",null);K(this,"async_deriveds",new Map);K(this,"current",new Map);K(this,"previous",new Map);S(this,ye,new Set);S(this,Ce,new Set);S(this,Me,0);S(this,Vn,new Map);S(this,Se,null);S(this,qn,[]);S(this,nt,[]);S(this,Bn,new Map);S(this,jn,new Map);S(this,Ee,new Set);K(this,"is_fork",!1);S(this,oe,!1);_e===null?Lt=_e=this:(_e.next=this,this.prev=_e),_e=this}skip_effect(e){l(this,jn).has(e)||l(this,jn).set(e,{d:[],m:[]}),l(this,Ee).delete(e)}unskip_effect(e,t=r=>this.schedule(r)){var r=l(this,jn).get(e);if(r){l(this,jn).delete(e);for(var i of r.d)H(i,D),t(i);for(i of r.m)H(i,gn),t(i)}l(this,Ee).add(e)}capture(e,t,r=!1){e.v!==V&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&ne)===0&&(this.current.set(e,[t,r]),G==null||G.set(e,t)),this.is_fork||(e.v=t)}activate(){y=this}deactivate(){y=null,G=null}flush(){try{Dt=!0,y=this,b(this,A,Fe).call(this)}finally{br=0,Zt=null,Ge=null,wt=null,Dt=!1,y=null,G=null,Pn.clear()}}discard(){var e;for(const t of l(this,Ce))t(this);l(this,Ce).clear();for(const t of this.async_deriveds.values())t.reject(He);b(this,A,Ve).call(this),(e=l(this,Se))==null||e.resolve()}register_created_effect(e){l(this,nt).push(e)}increment(e,t){if(E(this,Me,l(this,Me)+1),e){let r=l(this,Vn).get(t)??0;l(this,Vn).set(t,r+1)}}decrement(e,t){if(E(this,Me,l(this,Me)-1),e){let r=l(this,Vn).get(t)??0;r===1?l(this,Vn).delete(t):l(this,Vn).set(t,r-1)}l(this,oe)||(E(this,oe,!0),$n(()=>{E(this,oe,!1),this.linked&&this.flush()}))}transfer_reactions(e){for(const[t,r]of e)this.add_dirty_reaction(t,r);e.clear()}add_dirty_reaction(e,t){(t===D||l(this,Bn).get(e)!==D)&&l(this,Bn).set(e,t)}oncommit(e){l(this,ye).add(e)}ondiscard(e){l(this,Ce).add(e)}settled(){return(l(this,Se)??E(this,Se,Vr())).promise}static ensure(){if(y===null){const e=y=new Et;Dt||$n(()=>{e.started||e.flush()})}return y}apply(){{G=null;return}}schedule(e){var t;if(Zt=e,(t=e.b)!=null&&t.is_pending&&(e.f&(Ae|jt|qr))!==0&&(e.f&Ne)===0){e.b.defer_effect(e);return}l(this,qn).push(e)}};ye=new WeakMap,Ce=new WeakMap,Me=new WeakMap,Vn=new WeakMap,Se=new WeakMap,qn=new WeakMap,nt=new WeakMap,Bn=new WeakMap,jn=new WeakMap,Ee=new WeakMap,oe=new WeakMap,A=new WeakSet,Kt=function(){if(this.is_fork)return!0;for(const e of l(this,Vn).keys())if(!b(this,A,ei).call(this,e))return!0;return!1},ei=function(e){for(var t=e;t!==null;t=t.parent)if(l(this,jn).has(t))return!0;return!1},Wt=function(){var e=[];for(const s of l(this,qn))if(!((s.f&Z)!==0||(s.f&(D|gn))===0)){for(var t=s,r=!1;t.parent!==null;){t=t.parent;var i=t.f;if((i&(ee|In))!==0){if((i&q)===0){r=!0;break}t.f^=q}}r||e.push(t)}return E(this,qn,[]),e},Fe=function(){var a,c,d,f;this.started=!0;for(const[_,g]of l(this,Bn))H(_,g),this.schedule(_);this.apply();for(var e=Ge=[],t=[],r=wt=[];l(this,qn).length>0;){br++>1e3&&(b(this,A,Ve).call(this),Ms());for(const _ of b(this,A,Wt).call(this))try{b(this,A,Xt).call(this,_,e,t)}catch(g){throw li(_),b(this,A,Kt).call(this)||this.discard(),g}}if(y=null,r.length>0){var i=Et.ensure();for(const _ of r)i.schedule(_)}if(Ge=null,wt=null,b(this,A,Kt).call(this)){b(this,A,we).call(this,t),b(this,A,we).call(this,e);for(const[_,g]of l(this,jn))oi(_,g);r.length>0&&b(a=y,A,Fe).call(a);return}const s=b(this,A,ti).call(this);if(s){b(this,A,we).call(this,t),b(this,A,we).call(this,e),b(c=s,A,ii).call(c,this);return}l(this,Bn).clear();for(const _ of l(this,ye))_(this);l(this,ye).clear(),Ye=this,yr(t),yr(e),Ye=null,(d=l(this,Se))==null||d.resolve();var o=y;l(this,Me)===0&&(l(this,qn).length===0||o!==null)&&b(this,A,Ve).call(this),o!==null&&(Pn.clear(),b(f=o,A,Fe).call(f))},Xt=function(e,t,r){e.f^=q;for(var i=e.first;i!==null;){var s=i.f,o=(s&(In|ee))!==0,a=o&&(s&q)!==0,c=a||(s&rn)!==0||l(this,jn).has(i);if(!c&&i.fn!==null){o?i.f^=q:(s&Ae)!==0?t.push(i):ft(i)&&((s&Mn)!==0&&this.add_dirty_reaction(i,gn),je(i));var d=i.first;if(d!==null){i=d;continue}}for(;i!==null;){var f=i.next;if(f!==null){i=f;break}i=i.parent}}},ti=function(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&b(this,A,ri).call(this,e))return e;return null},ri=function(e){for(const[t,[,r]]of this.current)if(!r&&e.current.has(t))return!0;return!1},ii=function(e){var r;for(const[i,s]of e.current)!this.previous.has(i)&&e.previous.has(i)&&this.previous.set(i,e.previous.get(i)),this.current.set(i,s);for(const[i,s]of e.async_deriveds){const o=this.async_deriveds.get(i);o&&s.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(l(e,Bn));const t=i=>{var s=i.reactions;if(s!==null&&!((i.f&Q)!==0&&(i.f&(D|gn))===0))for(const c of s){var o=c.f;if((o&Q)!==0)t(c);else{var a=c;o&(ke|Mn)&&!this.async_deriveds.has(a)&&(l(this,Bn).delete(a),H(a,D),this.schedule(a))}}};for(const i of this.current.keys())t(i);this.oncommit(()=>e.discard()),b(r=e,A,Ve).call(r),y=this,b(this,A,Fe).call(this)},we=function(e){for(var t=0;t<e.length;t+=1)Xr(e[t],l(this,Bn))},Cs=function(){var _,g;for(let u=Lt;u!==null;u=u.next){var e=u.id<this.id,t=[];for(const[v,[w,h]]of this.current){if(u.current.has(v)){var r=u.current.get(v)[0];if(e&&w!==r)u.current.set(v,[w,h]);else continue}t.push(v)}if(e)for(const[v,w]of this.async_deriveds){const h=u.async_deriveds.get(v);h&&w.promise.then(h.resolve).catch(h.reject)}var i=[...u.current.keys()].filter(v=>!u.current.get(v)[1]);if(!(!u.started||i.length===0)){var s=i.filter(v=>!this.current.has(v));if(s.length===0)e&&u.discard();else if(t.length>0){if(e)for(const v of l(this,Ee))u.unskip_effect(v,w=>{var h;(w.f&(Mn|ke))!==0?u.schedule(w):b(h=u,A,we).call(h,[w])});u.activate();var o=new Set,a=new Map;for(var c of t)si(c,s,o,a);a=new Map;var d=[...u.current].filter(([v,w])=>{const h=this.current.get(v);return h?h[0]!==w[0]||h[1]!==w[1]:!0}).map(([v])=>v);if(d.length>0)for(const v of l(this,nt))(v.f&(Z|rn|kt))===0&&fr(v,d,a)&&((v.f&(ke|Mn))!==0?(H(v,D),u.schedule(v)):u.add_dirty_reaction(v,D));if(l(u,qn).length>0&&!l(u,oe)){u.apply();for(var f of b(_=u,A,Wt).call(_))b(g=u,A,Xt).call(g,f,[],[])}u.deactivate()}}}},Ve=function(){if(this.linked){var e=this.prev,t=this.next;e===null?Lt=t:e.next=t,t===null?_e=e:t.prev=e,this.linked=!1}};let he=Et;function Ms(){try{os()}catch(n){Ln(n,Zt)}}let yn=null;function yr(n){var e=n.length;if(e!==0){for(var t=0;t<e;){var r=n[t++];if((r.f&(Z|rn))===0&&ft(r)&&(yn=new Set,je(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&mi(r),(yn==null?void 0:yn.size)>0)){Pn.clear();for(const i of yn){if((i.f&(Z|rn))!==0)continue;const s=[i];let o=i.parent;for(;o!==null;)yn.has(o)&&(yn.delete(o),s.push(o)),o=o.parent;for(let a=s.length-1;a>=0;a--){const c=s[a];(c.f&(Z|rn))===0&&je(c)}}yn.clear()}}yn=null}}function si(n,e,t,r){if(!t.has(n)&&(t.add(n),n.reactions!==null))for(const i of n.reactions){const s=i.f;(s&Q)!==0?si(i,e,t,r):(s&(ke|Mn))!==0&&(s&D)===0&&fr(i,e,r)&&(H(i,D),hr(i))}}function fr(n,e,t){const r=t.get(n);if(r!==void 0)return r;if(n.deps!==null)for(const i of n.deps){if(Xe.call(e,i))return!0;if((i.f&Q)!==0&&fr(i,e,t))return t.set(i,!0),!0}return t.set(n,!1),!1}function hr(n){y.schedule(n)}function oi(n,e){if(!((n.f&In)!==0&&(n.f&q)!==0)){(n.f&D)!==0?e.d.push(n):(n.f&gn)!==0&&e.m.push(n),H(n,q);for(var t=n.first;t!==null;)oi(t,e),t=t.next}}function li(n){H(n,q);for(var e=n.first;e!==null;)li(e),e=e.next}let Ct=new Set;const Pn=new Map;let ai=!1;function ve(n,e){var t={f:0,v:n,reactions:null,equals:Yr,rv:0,wv:0};return t}function en(n,e){const t=ve(n);return bi(t),t}function Ss(n,e=!1,t=!0){const r=ve(n);return e||(r.equals=Gr),r}function X(n,e,t=!1){I!==null&&(!Sn||(I.f&kt)!==0)&&Wr()&&(I.f&(Q|Mn|ke|kt))!==0&&(Rn===null||!Rn.has(n))&&us();let r=t?Yn(e):e;return Be(n,r,wt)}var te=null,Jt=0;function Be(n,e,t=null){if(!n.equals(e)){Kn?Pn.set(n,e):Pn.has(n)||Pn.set(n,n.v);var r=he.ensure();if(r.capture(n,e),(n.f&Q)!==0){const i=n;(n.f&D)!==0&&dr(i),G===null&&ur(i)}n.wv=Ci(),te=null,Jt=0,ci(n,D,t),te=null,T!==null&&(T.f&q)!==0&&(T.f&(In|ee))===0&&(pn===null?Rs([n]):pn.push(n)),!r.is_fork&&Ct.size>0&&!ai&&Es()}return e}function Es(){ai=!1;for(const n of Ct){(n.f&q)!==0&&H(n,gn);let e;try{e=ft(n)}catch{e=!0}e&&je(n)}Ct.clear()}function Ze(n){X(n,n.v+1)}function ci(n,e,t){var r=n.reactions;if(r!==null){var i=r.length;if(Jt+=i,Jt>1e5&&te===null&&(te=new Set),te!==null){if(te.has(n))return;te.add(n)}for(var s=0;s<i;s++){var o=r[s],a=o.f,c=(a&D)===0;if(c&&H(o,e),(a&kt)!==0)Ct.add(o);else if((a&Q)!==0){var d=o;G==null||G.delete(d),ci(d,gn,t)}else if(c){var f=o;(a&Mn)!==0&&yn!==null&&yn.add(f),t!==null?t.push(f):hr(f)}}}}function Yn(n){if(typeof n!="object"||n===null||xe in n||Ur in n)return n;const e=Fr(n);if(e!==qi&&e!==Ui)return n;var t=new Map,r=Tt(n),i=en(0),s=de,o=a=>{if(de===s)return a();var c=I,d=de;bn(null),Er(s);var f=a();return bn(c),Er(d),f};return r&&t.set("length",en(n.length)),new Proxy(n,{defineProperty(a,c,d){(!("value"in d)||d.configurable===!1||d.enumerable===!1||d.writable===!1)&&as();var f=t.get(c);return f===void 0?o(()=>{var _=en(d.value);return t.set(c,_),_}):X(f,d.value,!0),!0},deleteProperty(a,c){var d=t.get(c);if(d===void 0){if(c in a){const f=o(()=>en(V));t.set(c,f),Ze(i)}}else X(d,V),Ze(i);return!0},get(a,c,d){var u;if(c===xe)return n;var f=t.get(c),_=c in a;if(f===void 0&&(!_||(u=me(a,c))!=null&&u.writable)&&(f=o(()=>{var v=Yn(_?a[c]:V),w=en(v);return w}),t.set(c,f)),f!==void 0){var g=C(f);return g===V?void 0:g}return Reflect.get(a,c,d)},getOwnPropertyDescriptor(a,c){var g;(g=this.has)==null||g.call(this,a,c);var d=Reflect.getOwnPropertyDescriptor(a,c),f=t.get(c);if(f!==void 0){var _=C(f);if(_===V)return;if(d&&"value"in d)d.value=_;else return{enumerable:!0,configurable:!0,value:_,writable:!0}}return d},has(a,c){var g;if(c===xe)return!0;var d=t.get(c),f=d!==void 0&&d.v!==V||Reflect.has(a,c);if(d!==void 0||(T!==null||I!==null)&&(!f||(g=me(a,c))!=null&&g.writable)){d===void 0&&(d=o(()=>{var u=f?Yn(a[c]):V,v=en(u);return v}),t.set(c,d));var _=C(d);if(_===V)return!1}return f},set(a,c,d,f){var M;var _=t.get(c),g=c in a;if(r&&c==="length")for(var u=d;u<_.v;u+=1){var v=t.get(u+"");v!==void 0?X(v,V):u in a&&(v=o(()=>en(V)),t.set(u+"",v))}if(_===void 0)(!g||(M=me(a,c))!=null&&M.writable)&&(_=o(()=>en(void 0)),X(_,Yn(d)),t.set(c,_));else{g=_.v!==V;var w=o(()=>Yn(d));X(_,w)}var h=Reflect.getOwnPropertyDescriptor(a,c);if(h!=null&&h.set&&h.set.call(f,d),!g){if(r&&typeof c=="string"){var p=t.get("length"),m=Number(c);Number.isInteger(m)&&m>=p.v&&X(p,m+1)}Ze(i)}return!0},ownKeys(a){C(i);var c=Reflect.ownKeys(a).filter(_=>{var g=t.get(_);return g===void 0||g.v!==V});for(var[d,f]of t)f.v!==V&&!(d in a)&&c.push(d);return c},setPrototypeOf(){cs()}})}function Cr(n){try{if(n!==null&&typeof n=="object"&&xe in n)return n[xe]}catch{}return n}function ui(n,e){return Object.is(Cr(n),Cr(e))}var Mr,di,fi,hi;function Is(){if(Mr===void 0){Mr=window,di=/Firefox/.test(navigator.userAgent);var n=Element.prototype,e=Node.prototype,t=Text.prototype;fi=me(e,"firstChild").get,hi=me(e,"nextSibling").get,xr(n)&&(n[Yt]=void 0,n[_t]=null,n[Gt]=void 0,n.__e=void 0),xr(t)&&(t[Re]=void 0)}}function Zn(n=""){return document.createTextNode(n)}function Gn(n){return fi.call(n)}function dt(n){return hi.call(n)}function tn(n,e){return Gn(n)}function On(n,e=!1){{var t=Gn(n);return t instanceof Comment&&t.data===""?dt(t):t}}function nn(n,e=!1){return Gn(n)}function U(n,e=1,t=!1){let r=n;for(;e--;)r=dt(r);return r}function As(n){n.textContent=""}function vi(){return!1}function vr(n,e,t){return e==null||e===Pr?t?document.createElement(n,{is:t}):document.createElement(n):t?document.createElementNS(e,n,{is:t}):document.createElementNS(e,n)}function Ts(n){var e=T;if(e===null)return I.f|=ne,n;if((e.f&Ne)===0&&(e.f&Ae)===0)throw n;Ln(n,e)}function Ln(n,e){if(n===Dr)throw n;if(!(e!==null&&(e.f&Z)!==0)){for(;e!==null;){if((e.f&Ut)!==0&&(e.f&(Z|mt))===0){if((e.f&Ne)===0)throw n;try{e.b.error(n);return}catch(t){n=t}}e=e.parent}throw n}}function Bs(n){T===null&&(I===null&&ss(),is()),Kn&&rs()}function js(n,e){var t=e.last;t===null?e.last=e.first=n:(t.next=n,n.prev=t,e.last=n)}function Wn(n,e){var t=T;t!==null&&(t.f&rn)!==0&&(n|=rn);var r={ctx:J,deps:null,nodes:null,f:n|D|En,first:null,fn:e,last:null,next:null,parent:t,b:t&&t.b,prev:null,teardown:null,wv:0,ac:null};y==null||y.register_created_effect(r);var i=r;if((n&Ae)!==0)Ge!==null?Ge.push(r):he.ensure().schedule(r);else if(e!==null){try{je(r)}catch(o){throw on(r),o}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&Oe)===0&&(i=i.first,(n&Mn)!==0&&(n&fe)!==0&&i!==null&&(i.f|=fe))}if(i!==null&&(i.parent=t,t!==null&&js(i,t),I!==null&&(I.f&Q)!==0&&(n&ee)===0)){var s=I;(s.effects??(s.effects=[])).push(i)}return r}function gr(){return I!==null&&!Sn}function gi(n){const e=Wn(jt,null);return H(e,q),e.teardown=n,e}function zs(n){Bs();var e=T.f,t=!I&&(e&In)!==0&&J!==null&&!J.i;if(t){var r=J;(r.e??(r.e=[])).push(n)}else return _i(n)}function _i(n){return Wn(Ae|Zi,n)}function Ns(n){he.ensure();const e=Wn(ee|Oe,n);return(t={})=>new Promise(r=>{t.outro?ue(e,()=>{on(e),r(void 0)}):(on(e),r(void 0))})}function Os(n){return Wn(Ae,n)}function Ls(n){return Wn(ke|Oe,n)}function Ds(n,e=0){return Wn(jt|e,n)}function R(n,e=[],t=[],r=[]){ps(r,e,t,i=>{Wn(jt,()=>{n(...i.map(C))})})}function Nt(n,e=0){var t=Wn(Mn|e,n);return t}function xn(n){return Wn(In|Oe,n)}function wi(n){var e=n.teardown;if(e!==null){const t=Kn,r=I;Sr(!0),bn(null);try{e.call(null)}catch(i){Ln(i,n.parent)}finally{Sr(t),bn(r)}}}function _r(n,e=!1){var t=n.first;for(n.first=n.last=null;t!==null;){const i=t.ac;i!==null&&zt(()=>{i.abort(ut)});var r=t.next;(t.f&ee)!==0?t.parent=null:on(t,e),t=r}}function Ps(n){for(var e=n.first;e!==null;){var t=e.next;(e.f&In)===0&&on(e),e=t}}function on(n,e=!0){var t=!1;(e||(n.f&Gi)!==0)&&n.nodes!==null&&n.nodes.end!==null&&(pi(n.nodes.start,n.nodes.end),t=!0),n.f|=mt,_r(n,e&&!t),$e(n,0);var r=n.nodes&&n.nodes.t;if(r!==null)for(const s of r)s.stop();wi(n),n.f^=mt,n.f|=Z;var i=n.parent;i!==null&&i.first!==null&&mi(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.fn=n.nodes=n.ac=n.b=null}function pi(n,e){for(;n!==null;){var t=n===e?null:dt(n);n.remove(),n=t}}function mi(n){var e=n.parent,t=n.prev,r=n.next;t!==null&&(t.next=r),r!==null&&(r.prev=t),e!==null&&(e.first===n&&(e.first=r),e.last===n&&(e.last=t))}function ue(n,e,t=!0){var r=[];n.f|=cr,ki(n,r,!0);var i=()=>{t&&on(n),e&&e()},s=r.length;if(s>0){var o=()=>--s||i();for(var a of r)a.out(o)}else i()}function ki(n,e,t){if((n.f&rn)===0){n.f^=rn;var r=n.nodes&&n.nodes.t;if(r!==null)for(const a of r)(a.is_global||t)&&e.push(a);for(var i=n.first;i!==null;){var s=i.next;if((i.f&ee)===0){var o=(i.f&fe)!==0||(i.f&In)!==0&&(n.f&Mn)!==0;ki(i,e,o?t:!1)}i=s}}}function Mt(n){n.f&=~cr,xi(n,!0)}function xi(n,e){if((n.f&cr)===0&&(n.f&rn)!==0){n.f^=rn,(n.f&q)===0&&(H(n,D),he.ensure().schedule(n));for(var t=n.first;t!==null;){var r=t.next,i=(t.f&fe)!==0||(t.f&In)!==0;xi(t,i?e:!1),t=r}var s=n.nodes&&n.nodes.t;if(s!==null)for(const o of s)(o.is_global||e)&&o.in()}}function wr(n,e){if(n.nodes)for(var t=n.nodes.start,r=n.nodes.end;t!==null;){var i=t===r?null:dt(t);e.append(t),t=i}}let Kn=!1;function Sr(n){Kn=n}let I=null,Sn=!1;function bn(n){I=n}let T=null;function Hn(n){T=n}let Rn=null;function bi(n){I!==null&&((I.f&xt)!==0||(I.f&Q)!==0)&&(Rn??(Rn=new Set)).add(n)}let sn=null,hn=0,pn=null;function Rs(n){pn=n}let yi=1,ie=0,de=ie;function Er(n){de=n}function Ci(){return++yi}function ft(n){var e=n.f;if((e&D)!==0)return!0;if((e&gn)!==0){for(var t=n.deps,r=t.length,i=0;i<r;i++){var s=t[i];if(ft(s)&&$r(s),s.wv>n.wv)return!0}(e&En)!==0&&G===null&&H(n,q)}return!1}function Mi(n,e,t=!0){var r=n.reactions;if(r!==null&&!(Rn!==null&&Rn.has(n)))for(var i=0;i<r.length;i++){var s=r[i];(s.f&Q)!==0?Mi(s,e,!1):e===s&&(t?H(s,D):(s.f&q)!==0&&H(s,gn),hr(s))}}function Si(n){var e=sn,t=hn,r=pn,i=I,s=Rn,o=J,a=Sn,c=de,d=n.f;sn=null,hn=0,pn=null,I=(d&(In|ee))===0?n:null,Rn=null,Te(n.ctx),Sn=!1,de=++ie,n.ac!==null&&(zt(()=>{n.ac.abort(ut)}),n.ac=null);try{n.f|=xt;var f=n.fn,_=f();n.f|=Ne;var g=Ir(n);if(Wr()&&pn!==null&&!Sn&&g!==null&&(n.f&(Q|gn|D))===0)for(var u=0;u<pn.length;u++)Mi(pn[u],n);if(i!==null&&i!==n){if(ie++,i.deps!==null)for(let v=0;v<t;v+=1)i.deps[v].rv=ie;if(e!==null)for(const v of e)v.rv=ie;pn!==null&&(r===null?r=pn:r.push(...pn))}return(n.f&ne)!==0&&(n.f^=ne),_}catch(v){return Ir(n),Ts(v)}finally{n.f^=xt,sn=e,hn=t,pn=r,I=i,Rn=s,Te(o),Sn=a,de=c}}function Ir(n){var i;var e=n.deps,t=y==null?void 0:y.is_fork;if(sn!==null){var r;if(t||$e(n,hn),e!==null&&hn>0)for(e.length=hn+sn.length,r=0;r<sn.length;r++)e[hn+r]=sn[r];else n.deps=e=sn;if(gr()&&(n.f&En)!==0)for(r=hn;r<e.length;r++)((i=e[r]).reactions??(i.reactions=[])).push(n)}else!t&&e!==null&&hn<e.length&&($e(n,hn),e.length=hn);return e}function Hs(n,e){let t=e.reactions;if(t!==null){var r=Fi.call(t,n);if(r!==-1){var i=t.length-1;i===0?t=e.reactions=null:(t[r]=t[i],t.pop())}}if(t===null&&(e.f&Q)!==0&&(sn===null||!Xe.call(sn,e))){var s=e;(s.f&En)!==0&&(s.f^=En),s.v!==V&&ur(s),s.ac!==null&&zt(()=>{s.ac.abort(ut),s.ac=null,H(s,D)}),bs(s),$e(s,0)}}function $e(n,e){var t=n.deps;if(t!==null)for(var r=e;r<t.length;r++)Hs(n,t[r])}function je(n){var e=n.f;if((e&Z)===0){H(n,q);var t=T;T=n;try{(e&(Mn|qr))!==0?Ps(n):_r(n),wi(n);var r=Si(n);n.teardown=typeof r=="function"?r:null,n.wv=yi;var i;Rr&&fs&&(n.f&D)!==0&&n.deps}finally{T=t}}}function C(n){var e=n.f,t=(e&Q)!==0;if(I!==null&&!Sn){var r=T!==null&&(T.f&Z)!==0;if(!r&&(Rn===null||!Rn.has(n))){var i=I.deps;if((I.f&xt)!==0)n.rv<ie&&(n.rv=ie,sn===null&&i!==null&&i[hn]===n?hn++:sn===null?sn=[n]:sn.push(n));else{I.deps??(I.deps=[]),Xe.call(I.deps,n)||I.deps.push(n);var s=n.reactions;s===null?n.reactions=[I]:Xe.call(s,I)||s.push(I)}}}if(Kn&&Pn.has(n))return Pn.get(n);if(t){var o=n;if(Kn){var a=o.v;return((o.f&q)===0&&o.reactions!==null||Ii(o))&&(a=dr(o)),Pn.set(o,a),a}var c=(o.f&En)===0&&!Sn&&I!==null&&(I.f&En)!==0,d=(o.f&Ne)===0;ft(o)&&(c&&(o.f|=En),$r(o)),c&&!d&&(ni(o),Ei(o))}if(G!=null&&G.has(n))return G.get(n);if((n.f&ne)!==0)throw n.v;return n.v}function Ei(n){if(n.f|=En,n.deps!==null)for(const t of n.deps){var e=t.reactions;e===null?t.reactions=[n]:Xe.call(e,n)||e.push(n),(t.f&Q)!==0&&(t.f&En)===0&&(ni(t),Ei(t))}}function Ii(n){if(n.v===V)return!0;if(n.deps===null)return!1;for(const e of n.deps)if(Pn.has(e)||(e.f&Q)!==0&&Ii(e))return!0;return!1}function Ai(n){var e=Sn;try{return Sn=!0,n()}finally{Sn=e}}const se=Symbol("events"),Ti=new Set,Qt=new Set;function Ke(n,e,t){(e[se]??(e[se]={}))[n]=t}function Bi(n){for(var e=0;e<n.length;e++)Ti.add(n[e]);for(var t of Qt)t(n)}let Pt=null,Rt=!1;function Ar(n){var w,h;var e=this,t=e.ownerDocument,r=n.type,i=((w=n.composedPath)==null?void 0:w.call(n))||[],s=i[0]||n.target;Pt=n,Rt||(Rt=!0,setTimeout(()=>{Rt=!1,Pt=null}));var o=0,a=Pt===n&&n[se];if(a){var c=i.indexOf(a);if(c!==-1&&(e===document||e===window)){n[se]=e;return}var d=i.indexOf(e);if(d===-1)return;c<=d&&(o=c)}if(s=i[o]||n.target,s!==e){Hr(n,"currentTarget",{configurable:!0,get(){return s||t}});var f=I,_=T;bn(null),Hn(null);try{for(var g,u=[];s!==null&&s!==e;){try{var v=(h=s[se])==null?void 0:h[r];v!=null&&(!s.disabled||n.target===s)&&v.call(s,n)}catch(p){g?u.push(p):g=p}if(n.cancelBubble)break;o++,s=o<i.length?i[o]:null}if(g){for(let p of u)queueMicrotask(()=>{throw p});throw g}}finally{n[se]=e,delete n.currentTarget,bn(f),Hn(_)}}}var Or;const Ht=((Or=globalThis==null?void 0:globalThis.window)==null?void 0:Or.trustedTypes)&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:n=>n});function Fs(n){return(Ht==null?void 0:Ht.createHTML(n))??n}function Vs(n){var e=vr("template");return e.innerHTML=Fs(n.replaceAll("<!>","<!---->")),e.content}function ze(n,e){var t=T;t.nodes===null&&(t.nodes={start:n,end:e,a:null,t:null})}function L(n,e){var t=(e&1)!==0,r=(e&2)!==0,i,s=!n.startsWith("<!>");return()=>{i===void 0&&(i=Vs(s?n:"<!>"+n),t||(i=Gn(i)));var o=r||di?document.importNode(i,!0):i.cloneNode(!0);if(t){var a=Gn(o),c=o.lastChild;ze(a,c)}else ze(o,o);return o}}function qs(n=""){{var e=Zn(n+"");return ze(e,e),e}}function $t(){var n=document.createDocumentFragment(),e=document.createComment(""),t=Zn();return n.append(e,t),ze(e,t),n}function O(n,e){n!==null&&n.before(e)}const Us=["touchstart","touchmove"];function Ys(n){return Us.includes(n)}function Gs(n){let e=0,t=ve(0),r;return()=>{gr()&&(C(t),Ds(()=>(e===0&&(r=Ai(()=>n(()=>Ze(t)))),e+=1,()=>{$n(()=>{e-=1,e===0&&(r==null||r(),r=void 0,Ze(t))})})))}}var Zs=fe|Oe;function Ks(n,e,t,r){new Ws(n,e,t,r)}var mn,ar,kn,le,W,un,$,dn,zn,ae,Qn,Ie,et,Un,It,j,Xs,Js,nr,Jn,Qs,er,qe,pt,tr,rr;class Ws{constructor(e,t,r,i){S(this,j);K(this,"parent");K(this,"is_pending",!1);K(this,"transform_error");S(this,mn);S(this,ar,null);S(this,kn);S(this,le);S(this,W);S(this,un,null);S(this,$,null);S(this,dn,null);S(this,zn,null);S(this,ae,0);S(this,Qn,0);S(this,Ie,!1);S(this,et,new Map);S(this,Un,null);S(this,It,Gs(()=>(E(this,Un,ve(l(this,ae))),()=>{E(this,Un,null)})));var s;E(this,mn,e),E(this,kn,t),E(this,le,o=>{var a=T;a.b=this,a.f|=Ut,r(o)}),this.parent=T.b,this.transform_error=i??((s=this.parent)==null?void 0:s.transform_error)??(o=>o),E(this,W,Nt(()=>{b(this,j,er).call(this)},Zs))}defer_effect(e){Xr(e,l(this,et))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!l(this,kn).pending}update_pending_count(e,t){b(this,j,tr).call(this,e,t),E(this,ae,l(this,ae)+e),!(!l(this,Un)||l(this,Ie))&&(E(this,Ie,!0),$n(()=>{E(this,Ie,!1),l(this,Un)&&Be(l(this,Un),l(this,ae))}))}get_effect_pending(){return l(this,It).call(this),C(l(this,Un))}error(e){if(e===Dr||!l(this,kn).onerror&&!l(this,kn).failed)throw e;y!=null&&y.is_fork?(l(this,un)&&y.skip_effect(l(this,un)),l(this,$)&&y.skip_effect(l(this,$)),l(this,dn)&&y.skip_effect(l(this,dn)),y.oncommit(()=>{b(this,j,Jn).call(this)||b(this,j,rr).call(this,e)})):b(this,j,rr).call(this,e)}}mn=new WeakMap,ar=new WeakMap,kn=new WeakMap,le=new WeakMap,W=new WeakMap,un=new WeakMap,$=new WeakMap,dn=new WeakMap,zn=new WeakMap,ae=new WeakMap,Qn=new WeakMap,Ie=new WeakMap,et=new WeakMap,Un=new WeakMap,It=new WeakMap,j=new WeakSet,Xs=function(){try{E(this,un,xn(()=>l(this,le).call(this,l(this,mn))))}catch(e){this.error(e)}},Js=function(e){const t=l(this,kn).failed,{reset:r,invoke_onerror:i}=b(this,j,nr).call(this,e);$n(i),t&&E(this,dn,xn(()=>{t(l(this,mn),()=>e,()=>r)}))},nr=function(e){var t=!1,r=!1;const i=()=>{if(!b(this,j,Jn).call(this)){if(t){Qi();return}t=!0,r&&ds(),l(this,dn)!==null&&ue(l(this,dn),()=>{E(this,dn,null)}),b(this,j,pt).call(this,()=>{b(this,j,er).call(this)})}};return{reset:i,invoke_onerror:()=>{var o,a;if(!b(this,j,Jn).call(this))try{r=!0,(a=(o=l(this,kn)).onerror)==null||a.call(o,e,i),r=!1}catch(c){Ln(c,l(this,W)&&l(this,W).parent)}}}},Jn=function(){return(l(this,W).f&(Z|mt))!==0},Qs=function(){const e=l(this,kn).pending;e&&(this.is_pending=!0,E(this,$,xn(()=>e(l(this,mn)))),$n(()=>{if(!b(this,j,Jn).call(this)){var t=E(this,zn,document.createDocumentFragment()),r=Zn(),i=!1;if(t.append(r),E(this,un,b(this,j,pt).call(this,()=>{try{return xn(()=>l(this,le).call(this,r))}catch(s){try{this.error(s),i=!0}catch(o){Ln(o,l(this,W).parent)}return null}})),l(this,un)===null){E(this,zn,null),i&&b(this,j,qe).call(this,y);return}l(this,Qn)===0&&(l(this,mn).before(t),E(this,zn,null),ue(l(this,$),()=>{E(this,$,null)}),b(this,j,qe).call(this,y))}}))},er=function(){try{if(this.is_pending=this.has_pending_snippet(),E(this,Qn,0),E(this,ae,0),E(this,un,xn(()=>{l(this,le).call(this,l(this,mn))})),l(this,Qn)>0){var e=E(this,zn,document.createDocumentFragment());wr(l(this,un),e);const t=l(this,kn).pending;E(this,$,xn(()=>t(l(this,mn))))}else b(this,j,qe).call(this,y)}catch(t){this.error(t)}},qe=function(e){this.is_pending=!1,e.transfer_reactions(l(this,et))},pt=function(e){var t=T,r=I,i=J;Hn(l(this,W)),bn(l(this,W)),Te(l(this,W).ctx);try{return he.ensure(),e()}finally{Hn(t),bn(r),Te(i)}},tr=function(e,t){var r;if(!this.has_pending_snippet()){this.parent&&b(r=this.parent,j,tr).call(r,e,t);return}E(this,Qn,l(this,Qn)+e),l(this,Qn)===0&&(b(this,j,qe).call(this,t),l(this,$)&&ue(l(this,$),()=>{E(this,$,null)}),l(this,zn)&&(l(this,mn).before(l(this,zn)),E(this,zn,null)))},rr=function(e){l(this,un)&&(on(l(this,un)),E(this,un,null)),l(this,$)&&(on(l(this,$)),E(this,$,null)),l(this,dn)&&(on(l(this,dn)),E(this,dn,null));let t=l(this,kn).failed;const r=i=>{if(b(this,j,Jn).call(this))return;const{reset:s,invoke_onerror:o}=b(this,j,nr).call(this,i);o(),t&&!b(this,j,Jn).call(this)&&E(this,dn,b(this,j,pt).call(this,()=>{try{return xn(()=>{var a=T;a.b=this,a.f|=Ut,t(l(this,mn),()=>i,()=>s)})}catch(a){return Ln(a,l(this,W).parent),null}}))};$n(()=>{if(!b(this,j,Jn).call(this)){var i;try{i=this.transform_error(e)}catch(s){Ln(s,l(this,W)&&l(this,W).parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(r,s=>Ln(s,l(this,W)&&l(this,W).parent)):r(i)}})};function Y(n,e){var t=e==null?"":typeof e=="object"?`${e}`:e;t!==(n[Re]??(n[Re]=n.nodeValue))&&(n[Re]=t,n.nodeValue=`${t}`)}function $s(n,e){return no(n,e)}const vt=new Map;function no(n,{target:e,anchor:t,props:r={},events:i,context:s,intro:o=!0,transformError:a}){Is();var c=void 0,d=Ns(()=>{var f=t??e.appendChild(Zn());Ks(f,{pending:()=>{}},u=>{Le({});var v=J;s&&(v.c=s),i&&(r.$$events=i),c=n(u,r)||Kr(),De()},a);var _=new Set,g=u=>{for(var v=0;v<u.length;v++){var w=u[v];if(!_.has(w)){_.add(w);var h=Ys(w);for(const M of[e,document]){var p=vt.get(M);p===void 0&&(p=new Map,vt.set(M,p));var m=p.get(w);m===void 0?(M.addEventListener(w,Ar,{passive:h}),p.set(w,1)):p.set(w,m+1)}}}};return g(Bt(Ti)),Qt.add(g),()=>{var h;for(var u of _)for(const p of[e,document]){var v=vt.get(p),w=v.get(u);--w==0?(p.removeEventListener(u,Ar),v.delete(u),v.size===0&&vt.delete(p)):v.set(u,w)}Qt.delete(g),f!==t&&((h=f.parentNode)==null||h.removeChild(f))}});return ir.set(c,d),c}let ir=new WeakMap;function eo(n,e){const t=ir.get(n);return t?(ir.delete(n),t(e)):Promise.resolve()}var Cn,Nn,fn,ce,tt,rt,At;class ji{constructor(e,t=!0){K(this,"anchor");S(this,Cn,new Map);S(this,Nn,new Map);S(this,fn,new Map);S(this,ce,new Set);S(this,tt,!0);S(this,rt,e=>{if(l(this,Cn).has(e)){var t=l(this,Cn).get(e),r=l(this,Nn).get(t);if(r)Mt(r),l(this,ce).delete(t);else{var i=l(this,fn).get(t);i&&(Mt(i.effect),l(this,Nn).set(t,i.effect),l(this,fn).delete(t),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),r=i.effect)}for(const[s,o]of l(this,Cn)){if(l(this,Cn).delete(s),s===e)break;const a=l(this,fn).get(o);a&&(on(a.effect),l(this,fn).delete(o))}for(const[s,o]of l(this,Nn)){if(s===t||l(this,ce).has(s))continue;const a=()=>{if(Array.from(l(this,Cn).values()).includes(s)){var d=document.createDocumentFragment();wr(o,d),d.append(Zn()),l(this,fn).set(s,{effect:o,fragment:d})}else on(o);l(this,ce).delete(s),l(this,Nn).delete(s)};l(this,tt)||!r?(l(this,ce).add(s),ue(o,a,!1)):a()}}});S(this,At,e=>{l(this,Cn).delete(e);const t=Array.from(l(this,Cn).values());for(const[r,i]of l(this,fn))t.includes(r)||(on(i.effect),l(this,fn).delete(r))});this.anchor=e,E(this,tt,t)}ensure(e,t){var r=y,i=vi();if(t&&!l(this,Nn).has(e)&&!l(this,fn).has(e))if(i){var s=document.createDocumentFragment(),o=Zn();s.append(o),l(this,fn).set(e,{effect:xn(()=>t(o)),fragment:s})}else l(this,Nn).set(e,xn(()=>t(this.anchor)));if(l(this,Cn).set(r,e),i){for(const[a,c]of l(this,Nn))a===e?r.unskip_effect(c):r.skip_effect(c);for(const[a,c]of l(this,fn))a===e?r.unskip_effect(c.effect):r.skip_effect(c.effect);r.oncommit(l(this,rt)),r.ondiscard(l(this,At))}else l(this,rt).call(this,r)}}Cn=new WeakMap,Nn=new WeakMap,fn=new WeakMap,ce=new WeakMap,tt=new WeakMap,rt=new WeakMap,At=new WeakMap;function We(n,e,...t){var r=new ji(n);Nt(()=>{const i=e()??null;r.ensure(i,i&&(s=>i(s,...t)))},fe)}function vn(n,e,t=!1){var r=new ji(n),i=t?fe:0;function s(o,a){r.ensure(o,a)}Nt(()=>{var o=!1;e((a,c=0)=>{o=!0,s(c,a)}),o||s(-1,null)},i)}function sr(n,e){return e}function to(n,e,t){for(var r=[],i=e.length,s,o=e.length,a=0;a<i;a++){let _=e[a];ue(_,()=>{if(s){if(s.pending.delete(_),s.done.add(_),s.pending.size===0){var g=n.outrogroups;or(n,Bt(s.done)),g.delete(s),g.size===0&&(n.outrogroups=null)}}else o-=1},!1)}if(o===0){var c=r.length===0&&t!==null&&n.pending.size===0;if(c){var d=t,f=d.parentNode;As(f),f.append(d),n.items.clear()}or(n,e,!c)}else s={pending:new Set(e),done:new Set},(n.outrogroups??(n.outrogroups=new Set)).add(s)}function or(n,e,t=!0){var r;if(n.pending.size>0){r=new Set;for(const o of n.pending.values())for(const a of o)r.add(n.items.get(a).e)}for(var i=0;i<e.length;i++){var s=e[i];if(r!=null&&r.has(s)){s.f|=Dn;const o=document.createDocumentFragment();wr(s,o)}else on(e[i],t)}}var Tr;function be(n,e,t,r,i,s=null){var o=n,a=new Map,c=(e&4)!==0;if(c){var d=n;o=d.appendChild(Zn())}var f=null,_=Qr(()=>{var m=t();return Tt(m)?m:m==null?[]:Bt(m)}),g=new Map,u=!0;function v(m){if((p.effect.f&Z)===0){p.pending.delete(m);var M=C(_);p.fallback=f,ro(p,M,o,e,r),f!==null&&(M.length===0?(f.f&Dn)===0?Mt(f):(f.f^=Dn,Ue(f,null,o)):ue(f,()=>{f=null}))}}function w(m){p.pending.delete(m)}var h=Nt(()=>{for(var m=C(_),M=m.length,z=new Set,k=y,N=vi(),x=0;x<M;x+=1){var B=m[x],P=r(B,x),F=u?null:a.get(P);F?(F.v&&Be(F.v,B),F.i&&Be(F.i,x),N&&k.unskip_effect(F.e)):(F=io(a,u?o:Tr??(Tr=Zn()),B,P,x,i,e,t),u||(F.e.f|=Dn),a.set(P,F)),z.add(P)}if(M===0&&s&&!f&&(u?f=xn(()=>s(o)):(f=xn(()=>s(Tr??(Tr=Zn()))),f.f|=Dn)),M>z.size&&ts(),!u)if(g.set(k,z),N){for(const[_n,ln]of a)z.has(_n)||k.skip_effect(ln.e);k.oncommit(v),k.ondiscard(w)}else v(k);C(_)}),p={effect:h,items:a,pending:g,outrogroups:null,fallback:f};u=!1}function Pe(n){for(;n!==null&&(n.f&In)===0;)n=n.next;return n}function ro(n,e,t,r,i){var F,_n,ln,an,An,wn,Fn,Tn,ge;var s=(r&8)!==0,o=e.length,a=n.items,c=Pe(n.effect.first),d,f=null,_,g=[],u=[],v,w,h,p;if(s)for(p=0;p<o;p+=1)v=e[p],w=i(v,p),h=a.get(w).e,(h.f&Dn)===0&&((_n=(F=h.nodes)==null?void 0:F.a)==null||_n.measure(),(_??(_=new Set)).add(h));for(p=0;p<o;p+=1){if(v=e[p],w=i(v,p),h=a.get(w).e,n.outrogroups!==null)for(const cn of n.outrogroups)cn.pending.delete(h),cn.done.delete(h);if((h.f&rn)!==0&&(Mt(h),s&&((an=(ln=h.nodes)==null?void 0:ln.a)==null||an.unfix(),(_??(_=new Set)).delete(h))),(h.f&Dn)!==0)if(h.f^=Dn,h===c)Ue(h,null,t);else{var m=f?f.next:c;h===n.effect.last&&(n.effect.last=h.prev),h.prev&&(h.prev.next=h.next),h.next&&(h.next.prev=h.prev),Xn(n,f,h),Xn(n,h,m),Ue(h,m,t),f=h,g=[],u=[],c=Pe(f.next);continue}if(h!==c){if(d!==void 0&&d.has(h)){if(g.length<u.length){var M=u[0],z;f=M.prev;var k=g[0],N=g[g.length-1];for(z=0;z<g.length;z+=1)Ue(g[z],M,t);for(z=0;z<u.length;z+=1)d.delete(u[z]);Xn(n,k.prev,N.next),Xn(n,f,k),Xn(n,N,M),c=M,f=N,p-=1,g=[],u=[]}else d.delete(h),Ue(h,c,t),Xn(n,h.prev,h.next),Xn(n,h,f===null?n.effect.first:f.next),Xn(n,f,h),f=h;continue}for(g=[],u=[];c!==null&&c!==h;)(d??(d=new Set)).add(c),u.push(c),c=Pe(c.next);if(c===null)continue}(h.f&Dn)===0&&g.push(h),f=h,c=Pe(h.next)}if(n.outrogroups!==null){for(const cn of n.outrogroups)cn.pending.size===0&&(or(n,Bt(cn.done)),(An=n.outrogroups)==null||An.delete(cn));n.outrogroups.size===0&&(n.outrogroups=null)}if(c!==null||d!==void 0){var x=[];if(d!==void 0)for(h of d)(h.f&rn)===0&&x.push(h);for(;c!==null;)(c.f&rn)===0&&c!==n.fallback&&x.push(c),c=Pe(c.next);var B=x.length;if(B>0){var P=(r&4)!==0&&o===0?t:null;if(s){for(p=0;p<B;p+=1)(Fn=(wn=x[p].nodes)==null?void 0:wn.a)==null||Fn.measure();for(p=0;p<B;p+=1)(ge=(Tn=x[p].nodes)==null?void 0:Tn.a)==null||ge.fix()}to(n,x,P)}}s&&$n(()=>{var cn,mr;if(_!==void 0)for(h of _)(mr=(cn=h.nodes)==null?void 0:cn.a)==null||mr.apply()})}function io(n,e,t,r,i,s,o,a){var c=(o&1)!==0?(o&16)===0?Ss(t,!1,!1):ve(t):null,d=(o&2)!==0?ve(i):null;return{v:c,i:d,e:xn(()=>(s(e,c??t,d??i,a),()=>{n.delete(r)}))}}function Ue(n,e,t){if(n.nodes)for(var r=n.nodes.start,i=n.nodes.end,s=e&&(e.f&Dn)===0?e.nodes.start:t;r!==null;){var o=dt(r);if(s.before(r),r===i)return;r=o}}function Xn(n,e,t){e===null?n.effect.first=t:e.next=t,t===null?n.effect.last=e:t.prev=e}function so(n,e,t=!1,r=!1,i=!1,s=!1){var o=n,a="";if(t)var c=n;R(()=>{var d=T;if(a!==(a=e()??"")){if(t){d.nodes=null,c.innerHTML=a,a!==""&&ze(Gn(c),c.lastChild);return}if(d.nodes!==null&&(pi(d.nodes.start,d.nodes.end),d.nodes=null),a!==""){var f=r?Ri:i?Hi:void 0,_=vr(r?"svg":i?"math":"template",f);_.innerHTML=a;var g=r||i?_:_.content;if(ze(Gn(g),g.lastChild),r||i)for(;Gn(g);)o.before(Gn(g));else o.before(g)}}})}function ht(n,e){Os(()=>{var i,s;n=((s=(i=T==null?void 0:T.parent)==null?void 0:i.nodes)==null?void 0:s.start)??n;var t=n.getRootNode(),r=t.host?t:t.head??t.ownerDocument.head;if(!r.querySelector("#"+e.hash)){const o=vr("style");o.id=e.hash,o.textContent=e.code,r.appendChild(o)}})}const Br=[...` 	
\r\f \v\uFEFF`];function oo(n,e,t){var r=n==null?"":""+n;if(t){for(var i of Object.keys(t))if(t[i])r=r?r+" "+i:i;else if(r.length)for(var s=i.length,o=0;(o=r.indexOf(i,o))>=0;){var a=o+s;(o===0||Br.includes(r[o-1]))&&(a===r.length||Br.includes(r[a]))?r=(o===0?"":r.substring(0,o))+r.substring(a+1):o=a}}return r===""?null:r}function jr(n,e=!1){var t=e?" !important;":";",r="";for(var i of Object.keys(n)){var s=n[i];s!=null&&s!==""&&(r+=" "+i+": "+s+t)}return r}function lo(n,e){if(e){var t="",r,i;return Array.isArray(e)?(r=e[0],i=e[1]):r=e,r&&(t+=jr(r)),i&&(t+=jr(i,!0)),t=t.trim(),t===""?null:t}return String(n)}function St(n,e,t,r,i,s){var o=n[Yt];if(o!==t||o===void 0){var a=oo(t,r,s);a==null?n.removeAttribute("class"):n.className=a,n[Yt]=t}else if(s&&i!==s)for(var c in s){var d=!!s[c];(i==null||d!==!!i[c])&&n.classList.toggle(c,d)}return s}function Ft(n,e={},t,r){for(var i in t){var s=t[i];e[i]!==s&&(t[i]==null?n.style.removeProperty(i):n.style.setProperty(i,s,r))}}function zi(n,e,t,r){var i=n[Gt];if(i!==e){var s=lo(e,r);s==null?n.removeAttribute("style"):n.style.cssText=s,n[Gt]=e}else r&&(Array.isArray(r)?(Ft(n,t==null?void 0:t[0],r[0]),Ft(n,t==null?void 0:t[1],r[1],"important")):Ft(n,t,r));return r}function ao(n,e){e?n.hasAttribute("selected")||n.setAttribute("selected",""):n.removeAttribute("selected")}function co(n,e){var t=n.__defaultValue,r=n.multiple,i=r?t??[]:null;if(!(r&&!Tt(i))){n.selectedIndex;for(var s of n.options){var o=lr(s);ao(s,r?i.includes(o):ui(o,t))}}}function Ni(n,e,t=!1){if(n.multiple){if(e==null)return;if(!Tt(e))return Ji();for(var r of n.options)r.selected=e.includes(lr(r));return}for(r of n.options){var i=lr(r);if(ui(i,e)){r.selected=!0;return}}(!t||e!==void 0)&&(n.selectedIndex=-1)}function uo(n){var e=new MutationObserver(t=>{t.every(fo)||("__defaultValue"in n&&co(n),"__value"in n&&Ni(n,n.__value))});e.observe(n,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),gi(()=>{e.disconnect()})}function lr(n){return"__value"in n?n.__value:n.value}function fo(n){if(n.target.closest("selectedcontent")!==null)return!0;if(n.type==="childList"){var e=[...n.addedNodes,...n.removedNodes];return e.length>0&&e.every(t=>t.nodeName==="SELECTEDCONTENT")}return!1}const ho=Symbol("is custom element"),vo=Symbol("is html");function re(n,e,t,r){var i=go(n),s=i[e];s!==(i[e]=t)&&(e==="loading"&&(n[Wi]=t),t==null?n.removeAttribute(e):typeof t!="string"&&_o(n).has(e)?n[e]=t:n.setAttribute(e,t))}function go(n){return n[_t]??(n[_t]={[ho]:n.nodeName.includes("-"),[vo]:n.namespaceURI===Pr})}var zr=new Map;function _o(n){var e=n.getAttribute("is")||n.nodeName,t=zr.get(e);if(t)return t;zr.set(e,t=new Set);for(var r,i=n,s=Element.prototype;s!==i;){r=Vi(i);for(var o in r)r[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&t.add(o);i=Fr(i)}return t}let gt=!1;function wo(n){var e=gt;try{return gt=!1,[n(),gt]}finally{gt=e}}function pr(n,e,t,r){var z;var i=!0,s=(t&8)!==0,o=(t&16)!==0,a=r,c=!0,d=void 0,f=()=>o&&i?(d??(d=Qe(r)),C(d)):(c&&(c=!1,a=o?Ai(r):r),a);let _;if(s){var g=xe in n||Ki in n;_=((z=me(n,e))==null?void 0:z.set)??(g&&e in n?k=>n[e]=k:void 0)}var u,v=!1;s?[u,v]=wo(()=>n[e]):u=n[e],u===void 0&&r!==void 0&&(u=f(),_&&(ls(),_(u)));var w;if(w=()=>{var k=n[e];return k===void 0?f():(c=!0,k)},(t&4)===0)return w;if(_){var h=n.$$legacy;return(function(k,N){return arguments.length>0?((!N||h||v)&&_(N?w():k),k):w()})}var p=!1,m=((t&1)!==0?Qe:Qr)(()=>(p=!1,w()));s&&C(m);var M=T;return(function(k,N){if(arguments.length>0){const x=N?C(m):s?Yn(k):k;return X(m,x),p=!0,a!==void 0&&(a=x),k}return Kn&&p||(M.f&Z)!==0?m.v:C(m)})}const po="5";var Lr;typeof window<"u"&&((Lr=window.__svelte??(window.__svelte={})).v??(Lr.v=new Set)).add(po);const mo={actions:"Actions",back:"Back",breadcrumbs:"Breadcrumbs",cantMove:"It can't move",changed:"Changed",clear:"Clear",close:"Close",collapse:"Collapse",date:"Date",disabled:"Disabled",dragOrClick:"drag, or click for the moves",done:"Done",duplicate:"Duplicate",expand:"Expand",forward:"Forward",hasErrors:"Has errors",hide:"Hide it on the website",monthAfter:"Month after",monthBefore:"Month before",move:"Move",moveDown:"Move down",moveEarlier:"Move earlier",moveIn:"Move under the one above",moveLater:"Move later",moveOut:"Move out a level",moveUp:"Move up",noDate:"No date",nothingFound:"Nothing found",nothingYet:"Nothing yet",pickDay:"Pick a day",pickMonth:"Pick a month",pickYear:"Pick a year",remove:"Remove",resize:"Drag to change the width, double click for the default",search:"Search",show:"Show it on the website",time:"time",today:"Today"},Vt=Symbol.for("zipp-ui"),ko=Yn({texts:mo,locale:void 0}),qt=globalThis,Oi=qt[Vt]??(qt[Vt]=ko);var xo=L("<span>/</span>"),bo=L("<a> </a>"),yo=L('<span class="here truncate svelte-wekzd8"> </span>'),Co=L("<!> <!>",1),Mo=L('<nav class="crumbs svelte-wekzd8"></nav>');const So={hash:"svelte-wekzd8",code:`.crumbs.svelte-wekzd8 {display:flex;align-items:center;gap:0.375rem;min-width:0;font-size:var(--fs-md);}a.svelte-wekzd8 {text-decoration:none;white-space:nowrap;}.sep.svelte-wekzd8 {color:var(--subtle);}.here.svelte-wekzd8 {color:var(--text);font-weight:500;}

@container page (width < 36rem) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}
@media (max-width: 799px) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}`};function Eo(n,e){Le(e,!0),ht(n,So);var t=Mo();be(t,21,()=>e.items,sr,(r,i,s)=>{const o=yt(()=>s<e.items.length-2);var a=Co(),c=On(a);{var d=u=>{var v=xo();let w;R(()=>w=St(v,1,"sep svelte-wekzd8",null,w,{far:s<e.items.length-1})),O(u,v)};vn(c,u=>{s&&u(d)})}var f=U(c,2);{var _=u=>{var v=bo();let w;var h=nn(v,!0);R(()=>{re(v,"href",C(i).href),w=St(v,1,"svelte-wekzd8",null,w,{far:C(o)}),Y(h,C(i).label)}),O(u,v)},g=u=>{var v=yo(),w=nn(v,!0);R(()=>Y(w,C(i).label)),O(u,v)};vn(f,u=>{C(i).href&&s<e.items.length-1?u(_):u(g,-1)})}O(r,a)}),R(()=>re(t,"aria-label",Oi.texts.breadcrumbs)),O(n,t),De()}function Li(n){const e=Symbol(n);return{provide:t=>void gs(e,t),get:()=>vs(e)}}const Io=Li("page-lead"),Ao=Li("flyout-bar");var To=L('<span class="note svelte-z3zeui"> </span>'),Bo=L('<span class="state svelte-z3zeui"><!></span>'),jo=L('<!> <h1 class="svelte-z3zeui"><!></h1> <!> <!> <span class="actions svelte-z3zeui"><!></span>',1),zo=L("<div><!></div>");const No={hash:"svelte-z3zeui",code:`.page-top.svelte-z3zeui {position:sticky;top:0;z-index:var(--z-bar);display:flex;align-items:center;gap:0.5rem;flex-wrap:wrap;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.bleed.svelte-z3zeui {margin:0 calc(-1 * var(--page-pad)) 1.25rem;padding:0.5rem var(--page-pad);}h1.svelte-z3zeui {min-width:0;font-size:var(--fs-lg);line-height:1.25rem;}.state.svelte-z3zeui {display:flex;align-items:center;gap:0.375rem;}.actions.svelte-z3zeui {display:flex;align-items:center;gap:0.5rem;margin-left:auto;}

@container page (width < 36rem) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}
@media (max-width: 799px) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}`};function Oo(n,e){Le(e,!0),ht(n,No);const t=d=>{var f=jo(),_=On(f);{var g=x=>{var B=$t(),P=On(B);We(P,()=>(s==null?void 0:s.content)??Je),O(x,B)};vn(_,x=>{i||x(g)})}var u=U(_,2),v=tn(u);{var w=x=>{Eo(x,{get items(){return e.crumbs}})},h=x=>{var B=qs();R(()=>Y(B,e.title)),O(x,B)};vn(v,x=>{e.crumbs?x(w):x(h,-1)})}var p=U(u,2);{var m=x=>{var B=To(),P=nn(B,!0);R(()=>Y(P,e.note)),O(x,B)};vn(p,x=>{e.note&&x(m)})}var M=U(p,2);{var z=x=>{var B=Bo(),P=tn(B);We(P,()=>e.state),O(x,B)};vn(M,x=>{e.state&&x(z)})}var k=U(M,2),N=tn(k);We(N,()=>e.actions??Je),O(d,f)};let r=pr(e,"bleed",3,!0);const i=Ao.get(),s=Io.get();zs(()=>{if(i)return i.content=t,()=>i.content=null});var o=$t(),a=On(o);{var c=d=>{var f=zo();let _;var g=tn(f);t(g),R(()=>_=St(f,1,"page-top svelte-z3zeui",null,_,{bleed:r()})),O(d,f)};vn(a,d=>{i||d(c)})}O(n,o),De()}var Lo=L("<col/>"),Do=L("<colgroup></colgroup>"),Po=L('<th class="svelte-migw9g"> </th>'),Ro=L("<thead><tr></tr></thead>"),Ho=L('<tr class="detail svelte-migw9g"><td><!></td></tr>'),Fo=L('<tr class="svelte-migw9g"><!></tr> <!>',1),Vo=L('<tr class="svelte-migw9g"><td class="empty svelte-migw9g"> </td></tr>'),qo=L('<div class="scroll svelte-migw9g"><table><!><!><tbody class="svelte-migw9g"></tbody></table></div>');const Uo={hash:"svelte-migw9g",code:`.scroll.svelte-migw9g {overflow-x:auto;}table.svelte-migw9g {width:100%;border-collapse:collapse;}table.fixed.svelte-migw9g {table-layout:fixed;}th.svelte-migw9g,
tbody.svelte-migw9g td {padding:0.375rem 0.5rem;text-align:left;vertical-align:top;}th.svelte-migw9g {font-weight:600;}tbody.svelte-migw9g td {border-top:1px solid var(--line);}tbody.svelte-migw9g code {font-size:var(--fs-md);color:var(--muted);}tbody.svelte-migw9g tr:where(.svelte-migw9g).target {background:var(--hover);transition:background 0.3s;}tbody.svelte-migw9g tr[aria-expanded=true]:where(.svelte-migw9g),
.detail.svelte-migw9g {background:var(--surface-2);}.detail.svelte-migw9g td {border-top:none;padding:0.25rem 0.5rem 0.75rem;}.empty.svelte-migw9g {color:var(--muted);}.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail) {cursor:pointer;}

@media (hover: hover) {.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail):hover {background:var(--hover);}
}`};function Yo(n,e){Le(e,!0),ht(n,Uo);let t=pr(e,"columns",19,()=>[]);function r(g,u){if(g.target.closest("a, button, input, select"))return;if(e.ontoggle)return e.ontoggle(u);if(!e.href)return;const v=e.href(u);if(g.metaKey||g.ctrlKey)return void window.open(v,"_blank");const h=g.currentTarget.querySelector(`a[href="${CSS.escape(v)}"]`);h?h.click():location.assign(v)}var i=qo(),s=tn(i);let o;var a=tn(s);{var c=g=>{var u=Do();be(u,21,()=>e.widths,sr,(v,w)=>{var h=Lo();let p;R(()=>p=zi(h,"",p,{width:C(w)})),O(v,h)}),O(g,u)};vn(a,g=>{e.widths&&g(c)})}var d=U(a);{var f=g=>{var u=Ro(),v=tn(u);be(v,21,t,sr,(w,h)=>{var p=Po(),m=nn(p,!0);R(()=>Y(m,C(h))),O(w,p)}),O(g,u)};vn(d,g=>{t().length&&g(f)})}var _=U(d);be(_,23,()=>e.rows,g=>e.key(g),(g,u,v)=>{var w=Fo(),h=On(w),p=tn(h);We(p,()=>e.row,()=>C(u),()=>C(v));var m=U(h,2);{var M=k=>{var N=Ho(),x=tn(N),B=tn(x);We(B,()=>e.detail,()=>C(u)),R(()=>re(x,"colspan",t().length||1)),O(k,N)},z=yt(()=>{var k;return e.detail&&((k=e.isOpen)==null?void 0:k.call(e,C(u)))});vn(m,k=>{C(z)&&k(M)})}R((k,N)=>{re(h,"id",k),re(h,"aria-expanded",N),re(h,"tabindex",e.ontoggle?0:void 0)},[()=>{var k;return(k=e.id)==null?void 0:k.call(e,C(u))},()=>{var k;return e.ontoggle?!!((k=e.isOpen)!=null&&k.call(e,C(u))):void 0}]),Ke("click",h,k=>r(k,C(u))),Ke("keydown",h,k=>{!e.ontoggle||k.target!==k.currentTarget||k.key!=="Enter"&&k.key!==" "||(k.preventDefault(),e.ontoggle(C(u)))}),O(g,w)},g=>{var u=Vo(),v=tn(u),w=nn(v,!0);R(()=>{re(v,"colspan",t().length||1),Y(w,e.empty??Oi.texts.nothingYet)}),O(g,u)}),R(()=>o=St(s,1,"svelte-migw9g",null,o,{links:!!e.href||!!e.ontoggle,fixed:!!e.widths})),O(n,i),De()}Bi(["click","keydown"]);const Go=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-image-plus"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 5h6" />
  <path d="M19 2v6" />
  <path d="M21 11.5V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7.5" />
  <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
  <circle cx="9" cy="9" r="2" />
</svg>
`,Zo=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-clapperboard"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12.296 3.464 3.02 3.956" />
  <path d="M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3z" />
  <path d="M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
  <path d="m6.18 5.276 3.1 3.899" />
</svg>
`,Ko=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-plus"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 12h14" />
  <path d="M12 5v14" />
</svg>
`,Wo=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-circle-alert"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <line x1="12" x2="12" y1="8" y2="12" />
  <line x1="12" x2="12.01" y1="16" y2="16" />
</svg>
`,Xo=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-layers"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z" />
  <path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12" />
  <path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17" />
</svg>
`,Jo=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-arrow-down"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5v14" />
  <path d="m19 12-7 7-7-7" />
</svg>
`,Qo=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-arrow-left"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 19-7-7 7-7" />
  <path d="M19 12H5" />
</svg>
`,$o=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-arrow-right"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 12h14" />
  <path d="m12 5 7 7-7 7" />
</svg>
`,nl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-arrow-up"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5 12 7-7 7 7" />
  <path d="M12 19V5" />
</svg>
`,el=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-box"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" />
  <path d="m3.3 7 8.7 5 8.7-5" />
  <path d="M12 22V12" />
</svg>
`,tl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-bold"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 12h9a4 4 0 010 8H7a1 1 0 01-1-1V5a1 1 0 011-1h7a4 4 0 010 8" />
</svg>
`,rl=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-square-mouse-pointer"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.034 12.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.943l-3.444 1.068a1 1 0 0 0-.66.66l-1.067 3.443a.5.5 0 0 1-.943.033z" />
  <path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" />
</svg>
`,il=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-calendar"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v3" />
  <path d="M16 2v3" />
  <rect x="3" y="3" width="18" height="18" rx="2" />
  <path d="M3 9h18" />
</svg>
`,sl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-list"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 5h.01" />
  <path d="M3 12h.01" />
  <path d="M3 19h.01" />
  <path d="M8 5h13" />
  <path d="M8 12h13" />
  <path d="M8 19h13" />
</svg>
`,ol=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-check"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 6 9 17l-5-5" />
</svg>
`,ll=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-chevron-down"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 9 6 6 6-6" />
</svg>
`,al=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-chevron-left"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 18-6-6 6-6" />
</svg>
`,cl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-chevron-right"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 18 6-6-6-6" />
</svg>
`,ul=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-chevrons-up-down"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 15 5 5 5-5" />
  <path d="m7 9 5-5 5 5" />
</svg>
`,dl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-x"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 6 6 18" />
  <path d="m6 6 12 12" />
</svg>
`,fl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-link"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
  <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
</svg>
`,hl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-copy"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
`,vl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-crop"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 2v14a2 2 0 0 0 2 2h14" />
  <path d="M18 22V8a2 2 0 0 0-2-2H2" />
</svg>
`,gl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-image"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <circle cx="9" cy="9" r="2" />
  <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
</svg>
`,_l=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-file"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" />
  <path d="M14 2v5a1 1 0 0 0 1 1h5" />
</svg>
`,wl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-trash-2"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 11v6" />
  <path d="M14 11v6" />
  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
  <path d="M3 6h18" />
  <path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
</svg>
`,pl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-desktop"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <line x1="8" x2="16" y1="21" y2="21" />
  <line x1="12" x2="12" y1="17" y2="21" />
</svg>
`,ml=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-book-open"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5v16" />
  <path d="M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z" />
</svg>
`,kl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-download"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 15V3" />
  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
  <path d="m7 10 5 5 5-5" />
</svg>
`,xl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-pencil"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" />
  <path d="m15 5 4 4" />
</svg>
`,bl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-folder-open"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" />
</svg>
`,yl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-external-link"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 3h6v6" />
  <path d="M10 14 21 3" />
  <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
</svg>
`,Cl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-file"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" />
  <path d="M14 2v5a1 1 0 0 0 1 1h5" />
</svg>
`,Ml=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-flip-horizontal-2"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 12H8" />
  <path d="M16 12h-2" />
  <path d="M22 12h-2" />
  <path d="M4 12H2" />
  <path d="M7.298 20.288A1 1 0 008 22h8a1 1 0 00.703-1.712l-3.991-3.99a1 1 0 00-1.424-.001z" />
  <path d="M7.298 3.712A1 1 0 018 2h8a1 1 0 01.703 1.712l-3.991 3.99a1 1 0 01-1.424.001z" />
</svg>
`,Sl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-flip-vertical-2"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 14v2" />
  <path d="M12 20v2" />
  <path d="M12 2v2" />
  <path d="M12 8v2" />
  <path d="M20.288 16.703A1 1 0 0022 16V8a1 1 0 00-1.712-.703l-3.99 3.991a1 1 0 00-.001 1.424z" />
  <path d="M3.712 16.703A1 1 0 012 16V8a1 1 0 011.712-.703l3.99 3.991a1 1 0 01.001 1.424z" />
</svg>
`,El=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-crosshair"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <line x1="22" x2="18" y1="12" y2="12" />
  <line x1="6" x2="2" y1="12" y2="12" />
  <line x1="12" x2="12" y1="6" y2="2" />
  <line x1="12" x2="12" y1="22" y2="18" />
</svg>
`,Il=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-folder-plus"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 10v6" />
  <path d="M9 13h6" />
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
</svg>
`,Al=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-folder"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
</svg>
`,Tl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-globe"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
  <path d="M2 12h20" />
</svg>
`,Bl=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-braces"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1" />
  <path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1" />
</svg>
`,jl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-grip-vertical"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="9" cy="12" r="1" />
  <circle cx="9" cy="5" r="1" />
  <circle cx="9" cy="19" r="1" />
  <circle cx="15" cy="12" r="1" />
  <circle cx="15" cy="5" r="1" />
  <circle cx="15" cy="19" r="1" />
</svg>
`,zl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-circle-help"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
  <path d="M12 17h.01" />
</svg>
`,Nl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-eye-off"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49" />
  <path d="M14.084 14.158a3 3 0 0 1-4.242-4.242" />
  <path d="M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143" />
  <path d="m2 2 20 20" />
</svg>
`,Ol=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-corner-down-right"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 10 5 5-5 5" />
  <path d="M4 4v7a4 4 0 0 0 4 4h12" />
</svg>
`,Ll=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-info"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 16v-4" />
  <path d="M12 8h.01" />
</svg>
`,Dl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-italic"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="19" x2="10" y1="4" y2="4" />
  <line x1="14" x2="5" y1="20" y2="20" />
  <line x1="15" x2="9" y1="4" y2="20" />
</svg>
`,Pl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-link"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
  <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
</svg>
`,Rl=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-panel-right"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M15 3v18" />
</svg>
`,Hl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-log-out"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 17 5-5-5-5" />
  <path d="M21 12H9" />
  <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
</svg>
`,Fl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-mail"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7" />
  <rect x="2" y="4" width="20" height="16" rx="2" />
</svg>
`,Vl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-mobile"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
  <path d="M12 18h.01" />
</svg>
`,ql=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-ellipsis"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="1" />
  <circle cx="19" cy="12" r="1" />
  <circle cx="5" cy="12" r="1" />
</svg>
`,Ul=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-grip-vertical"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="9" cy="12" r="1" />
  <circle cx="9" cy="5" r="1" />
  <circle cx="9" cy="19" r="1" />
  <circle cx="15" cy="12" r="1" />
  <circle cx="15" cy="5" r="1" />
  <circle cx="15" cy="19" r="1" />
</svg>
`,Yl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-list-ordered"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 5h10" />
  <path d="M11 12h10" />
  <path d="M11 19h10" />
  <path d="M4 4h1v5" />
  <path d="M4 9h2" />
  <path d="M6.5 20H3.4c0-1 2.6-1.925 2.6-3.5a1.5 1.5 0 0 0-2.6-1.02" />
</svg>
`,Gl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-corner-up-left"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20v-7a4 4 0 0 0-4-4H4" />
  <path d="M9 14 4 9l5-5" />
</svg>
`,Zl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-file-text"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" />
  <path d="M14 2v5a1 1 0 0 0 1 1h5" />
  <path d="M10 9H8" />
  <path d="M16 13H8" />
  <path d="M16 17H8" />
</svg>
`,Kl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-file"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" />
  <path d="M14 2v5a1 1 0 0 0 1 1h5" />
</svg>
`,Wl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-phone"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384" />
</svg>
`,Xl=`<!-- @license lucide-static v1.65.0 - ISC -->
<svg
  class="lucide lucide-play"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z" />
</svg>
`,Jl=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-reload"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
  <path d="M21 3v5h-5" />
</svg>
`,Ql=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-refresh-cw"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
  <path d="M21 3v5h-5" />
  <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
  <path d="M8 16H3v5" />
</svg>
`,$l=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-undo-2"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 14 4 9l5-5" />
  <path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5a5.5 5.5 0 0 1-5.5 5.5H11" />
</svg>
`,na=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-rotate-ccw"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
  <path d="M3 3v5h5" />
</svg>
`,ea=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-rotate-cw"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
  <path d="M21 3v5h-5" />
</svg>
`,ta=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-check"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 6 9 17l-5-5" />
</svg>
`,ra=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-search"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21 21-4.34-4.34" />
  <circle cx="11" cy="11" r="8" />
</svg>
`,ia=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-settings"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915" />
  <circle cx="12" cy="12" r="3" />
</svg>
`,sa=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-panel-left"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 3v18" />
</svg>
`,oa=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-file"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" />
  <path d="M14 2v5a1 1 0 0 0 1 1h5" />
</svg>
`,la=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-scaling"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
  <path d="M14 15H9v-5" />
  <path d="M16 3h5v5" />
  <path d="M21 3 9 15" />
</svg>
`,aa=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-sparkles"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z" />
  <path d="M20 2v4" />
  <path d="M22 4h-4" />
  <circle cx="4" cy="20" r="2" />
</svg>
`,ca=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-list-tree"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 5h13" />
  <path d="M13 12h8" />
  <path d="M13 19h8" />
  <path d="M3 10a2 2 0 0 0 2 2h3" />
  <path d="M3 5v12a2 2 0 0 0 2 2h3" />
</svg>
`,ua=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-tablet"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="16" height="20" x="4" y="2" rx="2" ry="2" />
  <line x1="12" x2="12.01" y1="18" y2="18" />
</svg>
`,da=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-languages"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5 8 6 6" />
  <path d="m4 14 6-6 2-3" />
  <path d="M2 5h12" />
  <path d="M7 2h1" />
  <path d="m22 22-5-10-5 10" />
  <path d="M14 18h6" />
</svg>
`,fa=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-unlink"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71" />
  <path d="m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71" />
  <line x1="8" x2="8" y1="2" y2="5" />
  <line x1="2" x2="5" y1="8" y2="8" />
  <line x1="16" x2="16" y1="19" y2="22" />
  <line x1="19" x2="22" y1="16" y2="16" />
</svg>
`,ha=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-upload"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3v12" />
  <path d="m17 8-5-5-5 5" />
  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
</svg>
`,va=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-users"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
  <path d="M16 3.128a4 4 0 0 1 0 7.744" />
  <path d="M22 21v-2a4 4 0 0 0-3-3.87" />
  <circle cx="9" cy="7" r="4" />
</svg>
`,ga=`<!-- @license lucide-static v1.64.0 - ISC -->
<svg
  class="lucide lucide-eye"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0" />
  <circle cx="12" cy="12" r="3" />
</svg>
`;var _a=L('<span class="icon svelte-oymxvd"></span>');const wa={hash:"svelte-oymxvd",code:".icon.svelte-oymxvd {display:inline-flex;flex-shrink:0;}.icon.svelte-oymxvd svg {width:100%;height:100%;stroke-width:1.75;}"};function pa(n,e){Le(e,!0),ht(n,wa);const t=Object.assign({"./icons/add-image.svg":Go,"./icons/add-video.svg":Zo,"./icons/add.svg":Ko,"./icons/alert.svg":Wo,"./icons/all_pages.svg":Xo,"./icons/arrow-down.svg":Jo,"./icons/arrow-left.svg":Qo,"./icons/arrow-right.svg":$o,"./icons/arrow-up.svg":nl,"./icons/block.svg":el,"./icons/bold.svg":tl,"./icons/button.svg":rl,"./icons/calendar.svg":il,"./icons/channel.svg":sl,"./icons/check.svg":ol,"./icons/chevron-down.svg":ll,"./icons/chevron-left.svg":al,"./icons/chevron-right.svg":cl,"./icons/chevrons-up-down.svg":ul,"./icons/close.svg":dl,"./icons/copy-url.svg":fl,"./icons/copy.svg":hl,"./icons/crop.svg":vl,"./icons/dashboard.svg":gl,"./icons/default.svg":_l,"./icons/delete.svg":wl,"./icons/desktop.svg":pl,"./icons/doc.svg":ml,"./icons/download.svg":kl,"./icons/edit.svg":xl,"./icons/entries.svg":bl,"./icons/external.svg":yl,"./icons/file.svg":Cl,"./icons/flip-h.svg":Ml,"./icons/flip-v.svg":Sl,"./icons/focal.svg":El,"./icons/folder-plus.svg":Il,"./icons/folder.svg":Al,"./icons/globe.svg":Tl,"./icons/graphql.svg":Bl,"./icons/grip.svg":jl,"./icons/help.svg":zl,"./icons/hide.svg":Nl,"./icons/indent.svg":Ol,"./icons/info.svg":Ll,"./icons/italic.svg":Dl,"./icons/link.svg":Pl,"./icons/live-preview.svg":Rl,"./icons/logout.svg":Hl,"./icons/mail.svg":Fl,"./icons/mobile.svg":Vl,"./icons/more.svg":ql,"./icons/move.svg":Ul,"./icons/numbered.svg":Yl,"./icons/outdent.svg":Gl,"./icons/page.svg":Zl,"./icons/page_cat.svg":Kl,"./icons/phone.svg":Wl,"./icons/play.svg":Xl,"./icons/reload.svg":Jl,"./icons/replace.svg":Ql,"./icons/reset.svg":$l,"./icons/rotate-left.svg":na,"./icons/rotate-right.svg":ea,"./icons/save.svg":ta,"./icons/search.svg":ra,"./icons/settings.svg":ia,"./icons/sidebar.svg":sa,"./icons/single.svg":oa,"./icons/size.svg":la,"./icons/sparkles.svg":aa,"./icons/structure.svg":ca,"./icons/tablet.svg":ua,"./icons/translate.svg":da,"./icons/unlink.svg":fa,"./icons/upload.svg":ha,"./icons/users.svg":va,"./icons/view.svg":ga});let r=pr(e,"size",3,16);var i=_a();let s;so(i,()=>t[`./icons/${e.name}.svg`],!0),R(()=>s=zi(i,"",s,{width:`${r()/16}rem`,height:`${r()/16}rem`})),O(n,i),De()}var it,st,ot,lt,at,ct;class ma{constructor(e){S(this,it,en(Yn([])));S(this,st,en(null));S(this,ot,en(Yn([])));S(this,lt,en(0));S(this,at,en(!1));S(this,ct,en(""));this.host=e}get forms(){return C(l(this,it))}set forms(e){X(l(this,it),e,!0)}get form(){return C(l(this,st))}set form(e){X(l(this,st),e,!0)}get submissions(){return C(l(this,ot))}set submissions(e){X(l(this,ot),e,!0)}get total(){return C(l(this,lt))}set total(e){X(l(this,lt),e,!0)}get loading(){return C(l(this,at))}set loading(e){X(l(this,at),e,!0)}get error(){return C(l(this,ct))}set error(e){X(l(this,ct),e,!0)}async init(){await this.run(async()=>{const e=await this.host.request("GET","/entries?section=forms&limit=500&order=title");this.forms=e.entries.map(t=>({id:t.id,title:t.title}))}),await this.load()}async load(){await this.run(async()=>{const e=await this.request(0);this.submissions=e.submissions,this.total=e.total})}async more(){await this.run(async()=>{const e=await this.request(this.submissions.length);this.submissions=[...this.submissions,...e.submissions],this.total=e.total})}async delete(e){await this.run(async()=>{await this.host.request("DELETE",`/forms/submissions/${e}`),this.submissions=this.submissions.filter(t=>t.id!==e),this.total-=1})}request(e){const t=this.form===null?"":`&form=${this.form}`;return this.host.request("GET",`/forms/submissions?offset=${e}${t}`)}async run(e){this.loading=!0,this.error="";try{await e()}catch(t){this.error=t.message}finally{this.loading=!1}}}it=new WeakMap,st=new WeakMap,ot=new WeakMap,lt=new WeakMap,at=new WeakMap,ct=new WeakMap;const Nr={en:{title:"Submissions",note:"What visitors sent with the forms, the newest first.",allForms:"All forms",form:"Form",date:"Date",site:"Site",content:"Content",none:"No submissions yet.",more:"Show more",delete:"Delete",confirmDelete:"Delete this submission for good?",empty:"–"},de:{title:"Einsendungen",note:"Was Besucher mit den Formularen geschickt haben, die neusten zuerst.",allForms:"Alle Formulare",form:"Formular",date:"Datum",site:"Website",content:"Inhalt",none:"Noch keine Einsendungen.",more:"Mehr zeigen",delete:"Löschen",confirmDelete:"Diese Einsendung endgültig löschen?",empty:"–"}};function ka(){return Nr[document.documentElement.lang]??Nr.en}var xa=L("<option> </option>"),ba=L("<select><option> </option><!></select>"),ya=L('<p class="error"> </p>'),Ca=L('<td class="nowrap svelte-1tixh7s"> </td> <td> </td> <td> </td> <td class="truncate"> </td>',1),Ma=L('<dt class="svelte-1tixh7s"> </dt> <dd class="svelte-1tixh7s"> </dd>',1),Sa=L('<dl class="svelte-1tixh7s"></dl> <button class="danger"><!> </button>',1),Ea=L('<p class="more svelte-1tixh7s"><button class="light"> </button></p>'),Ia=L('<!> <!> <div class="card"><!></div> <!>',1);const Aa={hash:"svelte-1tixh7s",code:`.nowrap.svelte-1tixh7s {white-space:nowrap;}dl.svelte-1tixh7s {display:grid;grid-template-columns:minmax(8rem, max-content) 1fr;gap:0.375rem 1.5rem;margin:0 0 1rem;}dt.svelte-1tixh7s {color:var(--muted);}dd.svelte-1tixh7s {margin:0;white-space:pre-wrap;overflow-wrap:anywhere;}.more.svelte-1tixh7s {text-align:center;}

@media (max-width: 799px) {dl.svelte-1tixh7s {grid-template-columns:1fr;}dd.svelte-1tixh7s {margin-bottom:0.5rem;}
}`};function Ta(n,e){Le(e,!0),ht(n,Aa);const t=ka(),r=new ma(e.host);r.init();let i=en(null);const s=m=>{var M;return((M=e.schema.sites.find(z=>z.handle===m))==null?void 0:M.name)??m},o=m=>new Date(m).toLocaleString(document.documentElement.lang,{dateStyle:"medium",timeStyle:"short"}),a=m=>m.answers.map(M=>M.value).filter(Boolean).slice(0,3).join(" · ")||t.empty;function c(m){const M=m.currentTarget.value;r.form=M?Number(M):null,X(i,null),r.load()}function d(m){confirm(t.confirmDelete)&&r.delete(m.id)}var f=Ia(),_=On(f);Oo(_,{get title(){return t.title},get note(){return t.note},actions:M=>{var z=$t(),k=On(z);{var N=x=>{var B=ba(),P=tn(B),F=nn(P,!0);P.value=P.__value="";var _n=U(P);be(_n,17,()=>r.forms,an=>an.id,(an,An)=>{var wn=xa(),Fn=nn(wn,!0),Tn={};R(()=>{Y(Fn,C(An).title),Tn!==(Tn=C(An).id)&&(wn.value=(wn.__value=Tn)??"")}),O(an,wn)});var ln;uo(B),R(()=>{Y(F,t.allForms),ln!==(ln=r.form??"")&&(B.value=(B.__value=ln)??"",Ni(B,ln))}),Ke("change",B,c),O(x,B)};vn(k,x=>{r.forms.length>1&&x(N)})}O(M,z)},$$slots:{actions:!0}});var g=U(_,2);{var u=m=>{var M=ya(),z=nn(M,!0);R(()=>Y(z,r.error)),O(m,M)};vn(g,m=>{r.error&&m(u)})}var v=U(g,2),w=tn(v);{const m=(N,x=Je)=>{var B=Ca(),P=On(B),F=nn(P,!0),_n=U(P,2),ln=nn(_n,!0),an=U(_n,2),An=nn(an,!0),wn=U(an,2),Fn=nn(wn,!0);R((Tn,ge,cn)=>{Y(F,Tn),Y(ln,x().form),Y(An,ge),Y(Fn,cn)},[()=>o(x().createdAt),()=>s(x().site),()=>a(x())]),O(N,B)},M=(N,x=Je)=>{var B=Sa(),P=On(B);be(P,21,()=>x().answers,an=>an.handle,(an,An)=>{var wn=Ma(),Fn=On(wn),Tn=nn(Fn,!0),ge=U(Fn,2),cn=nn(ge,!0);R(()=>{Y(Tn,C(An).label),Y(cn,C(An).value||t.empty)}),O(an,wn)});var F=U(P,2),_n=tn(F);pa(_n,{name:"delete"});var ln=U(_n);R(()=>Y(ln,` ${t.delete??""}`)),Ke("click",F,()=>d(x())),O(N,B)};let z=yt(()=>[t.date,t.form,t.site,t.content]),k=yt(()=>r.loading?"…":t.none);Yo(w,{get rows(){return r.submissions},key:N=>N.id,get columns(){return C(z)},get empty(){return C(k)},isOpen:N=>N.id===C(i),ontoggle:N=>X(i,C(i)===N.id?null:N.id,!0),row:m,detail:M,$$slots:{row:!0,detail:!0}})}var h=U(v,2);{var p=m=>{var M=Ea(),z=tn(M),k=nn(z,!0);R(()=>{z.disabled=r.loading,Y(k,t.more)}),Ke("click",z,()=>r.more()),O(m,M)};vn(h,m=>{r.submissions.length<r.total&&m(p)})}O(n,f),De()}Bi(["change","click"]);function za(n,e){const t=$s(Ta,{target:n,props:e});return{destroy(){eo(t)}}}export{za as page};
