var Gs=Object.defineProperty;var di=n=>{throw TypeError(n)};var Us=(n,e,t)=>e in n?Gs(n,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):n[e]=t;var ue=(n,e,t)=>Us(n,typeof e!="symbol"?e+"":e,t),gr=(n,e,t)=>e.has(n)||di("Cannot "+t);var v=(n,e,t)=>(gr(n,e,"read from private field"),t?t.call(n):e.get(n)),O=(n,e,t)=>e.has(n)?di("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,t),F=(n,e,t,r)=>(gr(n,e,"write to private field"),r?r.call(n,t):e.set(n,t),t),D=(n,e,t)=>(gr(n,e,"access private method"),t);const zi={},ae=Symbol("uninitialized"),Li="http://www.w3.org/1999/xhtml",Ws="http://www.w3.org/2000/svg",Xs="http://www.w3.org/1998/Math/MathML",ji=!1;var or=Array.isArray,Ks=Array.prototype.indexOf,kt=Array.prototype.includes,lr=Array.from,Di=Object.defineProperty,Wn=Object.getOwnPropertyDescriptor,Pi=Object.getOwnPropertyDescriptors,Zs=Object.prototype,Js=Array.prototype,Kr=Object.getPrototypeOf,vi=Object.isExtensible;const gn=()=>{};function Qs(n){for(var e=0;e<n.length;e++)n[e]()}function Oi(){var n,e,t=new Promise((r,i)=>{n=r,e=i});return{promise:t,resolve:n,reject:e}}function $s(n,e){if(Array.isArray(n))return n;if(!(Symbol.iterator in n))return Array.from(n);const t=[];for(const r of n)if(t.push(r),t.length===e)break;return t}const ge=2,nt=4,ar=8,Ni=1<<24,Ge=16,Ke=32,En=64,Ir=128,Zr=256,Xe=512,ce=1024,ee=2048,Te=4096,be=8192,ve=16384,ot=32768,xt=1<<25,Hn=65536,Jt=1<<17,eo=1<<18,lt=1<<19,no=1<<20,rn=1<<25,Qt=1<<21,Xn=1<<22,Cn=1<<23,fn=Symbol("$state"),Ri=Symbol("component"),to=Symbol("legacy props"),ro=Symbol(""),Xt=Symbol("attributes"),Ar=Symbol("class"),Tr=Symbol("style"),ut=Symbol("text"),Nt=new class extends Error{constructor(){super(...arguments);ue(this,"name","StaleReactionError");ue(this,"message","The reaction that called `getAbortSignal()` was re-run or destroyed")}};var Ai;const io=!!((Ai=globalThis.document)!=null&&Ai.contentType)&&globalThis.document.contentType.includes("xml");function so(){console.warn("https://svelte.dev/e/derived_inert")}function oo(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function lo(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}function Hi(n){return n===this.v}function Fi(n,e){return n!=n?e==e:n!==e||n!==null&&typeof n=="object"||typeof n=="function"}function qi(n){return!Fi(n,this.v)}function ao(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function co(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function uo(n,e,t){throw new Error("https://svelte.dev/e/each_key_duplicate")}function vo(n){throw new Error("https://svelte.dev/e/effect_in_teardown")}function fo(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function ho(n){throw new Error("https://svelte.dev/e/effect_orphan")}function go(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function wo(n){throw new Error("https://svelte.dev/e/props_invalid_value")}function _o(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function po(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function mo(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function bo(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let ko=!1;function xo(n){let e=n.p;for(;e!==null&&e.c===null;)e=e.p;return(e==null?void 0:e.c)??null}function Vi(n,e){return n===null&&ao(),n.c??(n.c=new Map(xo(n)||void 0))}let fe=null;function tt(n){fe=n}function yo(n){return Vi(fe).get(n)}function Mo(n,e){return Vi(fe).set(n,e),e}function we(n,e=!1,t){fe={p:fe,i:!1,c:null,e:null,s:n,x:null,r:G,l:null}}function _e(n){var e=fe,t=e.e;if(t!==null){e.e=null;for(var r of t)vs(r)}return e.i=!0,fe=e.p,Jr(n)}function Jr(n={}){return Di(n,Ri,{value:!0}),n}function Yi(){return!0}let Bn=[];function Gi(){var n=Bn;Bn=[],Qs(n)}function dn(n){if(Bn.length===0&&!wt){var e=Bn;queueMicrotask(()=>{e===Bn&&Gi()})}Bn.push(n)}function Co(){for(;Bn.length>0;)Gi()}const So=-7169;function oe(n,e){n.f=n.f&So|e}function Qr(n){(n.f&Xe)!==0||n.deps===null?oe(n,ce):oe(n,Te)}function Ui(n,e){(n.f&ee)!==0?e.set(n,ee):(n.f&Te)!==0&&e.get(n)!==ee&&e.set(n,Te),oe(n,ce)}function Rt(n){var e=q,t=G;He(null),ln(null);try{return n()}finally{He(e),ln(t)}}function Eo(n,e,t,r){const i=yt;var s=n.filter(f=>!f.settled),o=e.map(i);if(t.length===0&&s.length===0){r(o);return}var l=G,a=Io(),c=s.length===1?s[0].promise:s.length>1?Promise.all(s.map(f=>f.promise)):null;function u(f){if((l.f&ve)===0){a();try{r([...o,...f])}catch(w){tn(w,l)}$t()}}var h=Wi();if(t.length===0){c.then(()=>u([])).finally(h);return}function _(){Promise.all(t.map(f=>Ao(f))).then(u).catch(f=>tn(f,l)).finally(h)}c?c.then(()=>{if((l.f&ve)!==0){h();return}a(),_(),$t()}):_()}function Io(){var n=G,e=q,t=fe,r=P;return function(s=!0){ln(n),He(e),tt(t),s&&(n.f&ve)===0&&(r==null||r.activate(),r==null||r.apply())}}function $t(n=!0){ln(null),He(null),tt(null),n&&(P==null||P.deactivate())}function Wi(){var n=G,e=n.b,t=P,r=!!(e!=null&&e.is_rendered());return e==null||e.update_pending_count(1,t),t.increment(r,n),()=>{e==null||e.update_pending_count(-1,t),t.decrement(r,n)}}function yt(n){var e=ge|ee;return G!==null&&(G.f|=lt),{ctx:fe,deps:null,effects:null,equals:Hi,f:e,fn:n,reactions:null,rv:0,v:ae,wv:0,parent:G,ac:null}}const dt=Symbol("obsolete");function Ao(n,e,t){let r=G;r===null&&co();var i=void 0,s=qn(ae),o=!q,l=new Set;return Yo(()=>{var f,w;var a=G,c=Oi();i=c.promise;try{Promise.resolve(n()).then(c.resolve,g=>{g!==Nt&&c.reject(g)}).finally($t)}catch(g){c.reject(g),$t()}var u=P;if(o){if((a.f&ot)!==0)var h=Wi();if((f=r.b)!=null&&f.is_rendered())(w=u.async_deriveds.get(a))==null||w.reject(dt);else for(const g of l.values())g.reject(dt);l.add(c),u.async_deriveds.set(a,c)}const _=(g,p=void 0)=>{h==null||h(),l.delete(c),p!==dt&&(u.activate(),p?(s.f|=Cn,rt(s,p)):((s.f&Cn)!==0&&(s.f^=Cn),rt(s,g)),u.deactivate())};c.promise.then(_,g=>_(null,g||"unknown"))}),ds(()=>{for(const a of l)a.reject(dt)}),new Promise(a=>{function c(u){function h(){u===i?a(s):c(i)}u.then(h,h)}c(i)})}function re(n){const e=yt(n);return ps(e),e}function Xi(n){const e=yt(n);return e.equals=qi,e}function To(n){var e=n.effects;if(e!==null){n.effects=null;for(var t=0;t<e.length;t+=1)ye(e[t])}}function $r(n){var e,t=G,r=n.parent;if(!wn&&r!==null&&n.v!==ae&&(r.f&(ve|be))!==0)return so(),n.v;ln(r);try{To(n),e=xs(n)}finally{ln(t)}return e}function Ki(n){var e=$r(n);if(!n.equals(e)&&(n.wv=bs(),(!(P!=null&&P.is_fork)||n.deps===null)&&(P!==null?(P.capture(n,e,!0),Sn==null||Sn.capture(n,e,!0)):n.v=e,n.deps===null))){oe(n,ce);return}wn||(de!==null?(ri()||P!=null&&P.is_fork)&&de.set(n,e):Qr(n))}function Bo(n){var e;if(n.effects!==null)for(const t of n.effects)(t.teardown||t.ac)&&((e=t.teardown)==null||e.call(t),t.ac!==null&&Rt(()=>{t.ac.abort(Nt),t.ac=null}),t.fn!==null&&(t.teardown=gn),Mt(t,0),oi(t))}function Zi(n){if(n.effects!==null)for(const e of n.effects)e.teardown&&e.fn!==null&&it(e)}let wr=null,Gn=null,P=null,Sn=null,de=null,Br=null,wt=!1,_r=!1,_t=null,Kt=null;var fi=0;let zo=1;var Kn,Zn,Jn,an,Qn,cn,St,Qe,$e,$n,jn,W,zr,Ji,Lr,vt,jr,Qi,$i,es,Un,Lo,ft;const rr=class rr{constructor(){O(this,W);ue(this,"id",zo++);ue(this,"started",!1);ue(this,"linked",!0);ue(this,"prev",null);ue(this,"next",null);ue(this,"async_deriveds",new Map);ue(this,"current",new Map);ue(this,"previous",new Map);O(this,Kn,new Set);O(this,Zn,new Set);O(this,Jn,0);O(this,an,new Map);O(this,Qn,null);O(this,cn,[]);O(this,St,[]);O(this,Qe,new Map);O(this,$e,new Map);O(this,$n,new Set);ue(this,"is_fork",!1);O(this,jn,!1);Gn===null?wr=Gn=this:(Gn.next=this,this.prev=Gn),Gn=this}skip_effect(e){v(this,$e).has(e)||v(this,$e).set(e,{d:[],m:[]}),v(this,$n).delete(e)}unskip_effect(e,t=r=>this.schedule(r)){var r=v(this,$e).get(e);if(r){v(this,$e).delete(e);for(var i of r.d)oe(i,ee),t(i);for(i of r.m)oe(i,Te),t(i)}v(this,$n).add(e)}capture(e,t,r=!1){e.v!==ae&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&Cn)===0&&(this.current.set(e,[t,r]),de==null||de.set(e,t)),this.is_fork||(e.v=t)}activate(){P=this}deactivate(){P=null,de=null}flush(){try{_r=!0,P=this,D(this,W,vt).call(this)}finally{fi=0,Br=null,_t=null,Kt=null,_r=!1,P=null,de=null,sn.clear()}}discard(){var e;for(const t of v(this,Zn))t(this);v(this,Zn).clear();for(const t of this.async_deriveds.values())t.reject(dt);D(this,W,ft).call(this),(e=v(this,Qn))==null||e.resolve()}register_created_effect(e){v(this,St).push(e)}increment(e,t){if(F(this,Jn,v(this,Jn)+1),e){let r=v(this,an).get(t)??0;v(this,an).set(t,r+1)}}decrement(e,t){if(F(this,Jn,v(this,Jn)-1),e){let r=v(this,an).get(t)??0;r===1?v(this,an).delete(t):v(this,an).set(t,r-1)}v(this,jn)||(F(this,jn,!0),dn(()=>{F(this,jn,!1),this.linked&&this.flush()}))}transfer_reactions(e){for(const[t,r]of e)this.add_dirty_reaction(t,r);e.clear()}add_dirty_reaction(e,t){(t===ee||v(this,Qe).get(e)!==ee)&&v(this,Qe).set(e,t)}oncommit(e){v(this,Kn).add(e)}ondiscard(e){v(this,Zn).add(e)}settled(){return(v(this,Qn)??F(this,Qn,Oi())).promise}static ensure(){if(P===null){const e=P=new rr;!_r&&!wt&&dn(()=>{e.started||e.flush()})}return P}apply(){{de=null;return}}schedule(e){var t;if(Br=e,(t=e.b)!=null&&t.is_pending&&(e.f&(nt|ar|Ni))!==0&&(e.f&ot)===0){e.b.defer_effect(e);return}v(this,cn).push(e)}};Kn=new WeakMap,Zn=new WeakMap,Jn=new WeakMap,an=new WeakMap,Qn=new WeakMap,cn=new WeakMap,St=new WeakMap,Qe=new WeakMap,$e=new WeakMap,$n=new WeakMap,jn=new WeakMap,W=new WeakSet,zr=function(){if(this.is_fork)return!0;for(const e of v(this,an).keys())if(!D(this,W,Ji).call(this,e))return!0;return!1},Ji=function(e){for(var t=e;t!==null;t=t.parent)if(v(this,$e).has(t))return!0;return!1},Lr=function(){var e=[];for(const s of v(this,cn))if(!((s.f&ve)!==0||(s.f&(ee|Te))===0)){for(var t=s,r=!1;t.parent!==null;){t=t.parent;var i=t.f;if((i&(En|Ke))!==0){if((i&ce)===0){r=!0;break}t.f^=ce}}r||e.push(t)}return F(this,cn,[]),e},vt=function(){var l,a,c,u;this.started=!0;for(const[h,_]of v(this,Qe))oe(h,_),this.schedule(h);this.apply();for(var e=_t=[],t=[],r=Kt=[];v(this,cn).length>0;){fi++>1e3&&(D(this,W,ft).call(this),Do());for(const h of D(this,W,Lr).call(this))try{D(this,W,jr).call(this,h,e,t)}catch(_){throw rs(h),D(this,W,zr).call(this)||this.discard(),_}}if(P=null,r.length>0){var i=rr.ensure();for(const h of r)i.schedule(h)}if(_t=null,Kt=null,D(this,W,zr).call(this)){D(this,W,Un).call(this,t),D(this,W,Un).call(this,e);for(const[h,_]of v(this,$e))ts(h,_);r.length>0&&D(l=P,W,vt).call(l);return}const s=D(this,W,Qi).call(this);if(s){D(this,W,Un).call(this,t),D(this,W,Un).call(this,e),D(a=s,W,es).call(a,this);return}v(this,Qe).clear();for(const h of v(this,Kn))h(this);v(this,Kn).clear(),Sn=this,hi(t),hi(e),Sn=null,(c=v(this,Qn))==null||c.resolve();var o=P;v(this,Jn)===0&&(v(this,cn).length===0||o!==null)&&D(this,W,ft).call(this),o!==null&&(sn.clear(),D(u=o,W,vt).call(u))},jr=function(e,t,r){e.f^=ce;for(var i=e.first;i!==null;){var s=i.f,o=(s&(Ke|En))!==0,l=o&&(s&ce)!==0,a=l||(s&be)!==0||v(this,$e).has(i);if(!a&&i.fn!==null){o?i.f^=ce:(s&nt)!==0?t.push(i):Ft(i)&&((s&Ge)!==0&&this.add_dirty_reaction(i,Te),it(i));var c=i.first;if(c!==null){i=c;continue}}for(;i!==null;){var u=i.next;if(u!==null){i=u;break}i=i.parent}}},Qi=function(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&D(this,W,$i).call(this,e))return e;return null},$i=function(e){for(const[t,[,r]]of this.current)if(!r&&e.current.has(t))return!0;return!1},es=function(e){var r;for(const[i,s]of e.current)!this.previous.has(i)&&e.previous.has(i)&&this.previous.set(i,e.previous.get(i)),this.current.set(i,s);for(const[i,s]of e.async_deriveds){const o=this.async_deriveds.get(i);o&&s.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(v(e,Qe));const t=i=>{var s=i.reactions;if(s!==null&&!((i.f&ge)!==0&&(i.f&(ee|Te))===0))for(const a of s){var o=a.f;if((o&ge)!==0)t(a);else{var l=a;o&(Xn|Ge)&&!this.async_deriveds.has(l)&&(v(this,Qe).delete(l),oe(l,ee),this.schedule(l))}}};for(const i of this.current.keys())t(i);this.oncommit(()=>e.discard()),D(r=e,W,ft).call(r),P=this,D(this,W,vt).call(this)},Un=function(e){for(var t=0;t<e.length;t+=1)Ui(e[t],v(this,Qe))},Lo=function(){var h,_;for(let f=wr;f!==null;f=f.next){var e=f.id<this.id,t=[];for(const[w,[g,p]]of this.current){if(f.current.has(w)){var r=f.current.get(w)[0];if(e&&g!==r)f.current.set(w,[g,p]);else continue}t.push(w)}if(e)for(const[w,g]of this.async_deriveds){const p=f.async_deriveds.get(w);p&&g.promise.then(p.resolve).catch(p.reject)}var i=[...f.current.keys()].filter(w=>!f.current.get(w)[1]);if(!(!f.started||i.length===0)){var s=i.filter(w=>!this.current.has(w));if(s.length===0)e&&f.discard();else if(t.length>0){if(e)for(const w of v(this,$n))f.unskip_effect(w,g=>{var p;(g.f&(Ge|Xn))!==0?f.schedule(g):D(p=f,W,Un).call(p,[g])});f.activate();var o=new Set,l=new Map;for(var a of t)ns(a,s,o,l);l=new Map;var c=[...f.current].filter(([w,g])=>{const p=this.current.get(w);return p?p[0]!==g[0]||p[1]!==g[1]:!0}).map(([w])=>w);if(c.length>0)for(const w of v(this,St))(w.f&(ve|be|Jt))===0&&ei(w,c,l)&&((w.f&(Xn|Ge))!==0?(oe(w,ee),f.schedule(w)):f.add_dirty_reaction(w,ee));if(v(f,cn).length>0&&!v(f,jn)){f.apply();for(var u of D(h=f,W,Lr).call(h))D(_=f,W,jr).call(_,u,[],[])}f.deactivate()}}}},ft=function(){if(this.linked){var e=this.prev,t=this.next;e===null?wr=t:e.next=t,t===null?Gn=e:t.prev=e,this.linked=!1}};let Fn=rr;function jo(n){var e=wt,t=Sn;Sn=null,wt=!0;try{for(var r;;){if(Co(),P===null)return r;P.flush()}}finally{wt=e,Sn=t}}function Do(){try{go()}catch(n){tn(n,Br)}}let Ve=null;function hi(n){var e=n.length;if(e!==0){for(var t=0;t<e;){var r=n[t++];if((r.f&(ve|be))===0&&Ft(r)&&(Ve=new Set,it(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&gs(r),(Ve==null?void 0:Ve.size)>0)){sn.clear();for(const i of Ve){if((i.f&(ve|be))!==0)continue;const s=[i];let o=i.parent;for(;o!==null;)Ve.has(o)&&(Ve.delete(o),s.push(o)),o=o.parent;for(let l=s.length-1;l>=0;l--){const a=s[l];(a.f&(ve|be))===0&&it(a)}}Ve.clear()}}Ve=null}}function ns(n,e,t,r){if(!t.has(n)&&(t.add(n),n.reactions!==null))for(const i of n.reactions){const s=i.f;(s&ge)!==0?ns(i,e,t,r):(s&(Xn|Ge))!==0&&(s&ee)===0&&ei(i,e,r)&&(oe(i,ee),ni(i))}}function ei(n,e,t){const r=t.get(n);if(r!==void 0)return r;if(n.deps!==null)for(const i of n.deps){if(kt.call(e,i))return!0;if((i.f&ge)!==0&&ei(i,e,t))return t.set(i,!0),!0}return t.set(n,!1),!1}function ni(n){P.schedule(n)}function ts(n,e){if(!((n.f&Ke)!==0&&(n.f&ce)!==0)){(n.f&ee)!==0?e.d.push(n):(n.f&Te)!==0&&e.m.push(n),oe(n,ce);for(var t=n.first;t!==null;)ts(t,e),t=t.next}}function rs(n){oe(n,ce);for(var e=n.first;e!==null;)rs(e),e=e.next}let er=new Set;const sn=new Map;let is=!1;function qn(n,e){var t={f:0,v:n,reactions:null,equals:Hi,rv:0,wv:0};return t}function $(n,e){const t=qn(n);return ps(t),t}function Po(n,e=!1,t=!0){const r=qn(n);return e||(r.equals=qi),r}function V(n,e,t=!1){q!==null&&(!We||(q.f&Jt)!==0)&&Yi()&&(q.f&(ge|Ge|Xn|Jt))!==0&&(on===null||!on.has(n))&&mo();let r=t?Ue(e):e;return rt(n,r,Kt)}var Tn=null,Dr=0;function rt(n,e,t=null){if(!n.equals(e)){wn?sn.set(n,e):sn.has(n)||sn.set(n,n.v);var r=Fn.ensure();if(r.capture(n,e),(n.f&ge)!==0){const i=n;(n.f&ee)!==0&&$r(i),de===null&&Qr(i)}n.wv=bs(),Tn=null,Dr=0,ss(n,ee,t),Tn=null,G!==null&&(G.f&ce)!==0&&(G.f&(Ke|En))===0&&(De===null?Uo([n]):De.push(n)),!r.is_fork&&er.size>0&&!is&&Oo()}return e}function Oo(){is=!1;for(const n of er){(n.f&ce)!==0&&oe(n,Te);let e;try{e=Ft(n)}catch{e=!0}e&&it(n)}er.clear()}function pt(n){V(n,n.v+1)}function ss(n,e,t){var r=n.reactions;if(r!==null){var i=r.length;if(Dr+=i,Dr>1e5&&Tn===null&&(Tn=new Set),Tn!==null){if(Tn.has(n))return;Tn.add(n)}for(var s=0;s<i;s++){var o=r[s],l=o.f,a=(l&ee)===0;if(a&&oe(o,e),(l&Jt)!==0)er.add(o);else if((l&ge)!==0){var c=o;de==null||de.delete(c),ss(c,Te,t)}else if(a){var u=o;(l&Ge)!==0&&Ve!==null&&Ve.add(u),t!==null?t.push(u):ni(u)}}}}function Ue(n){if(typeof n!="object"||n===null||fn in n||Ri in n)return n;const e=Kr(n);if(e!==Zs&&e!==Js)return n;var t=new Map,r=or(n),i=$(0),s=Rn,o=l=>{if(Rn===s)return l();var a=q,c=Rn;He(null),pi(s);var u=l();return He(a),pi(c),u};return r&&t.set("length",$(n.length)),new Proxy(n,{defineProperty(l,a,c){(!("value"in c)||c.configurable===!1||c.enumerable===!1||c.writable===!1)&&_o();var u=t.get(a);return u===void 0?o(()=>{var h=$(c.value);return t.set(a,h),h}):V(u,c.value,!0),!0},deleteProperty(l,a){var c=t.get(a);if(c===void 0){if(a in l){const u=o(()=>$(ae));t.set(a,u),pt(i)}}else V(c,ae),pt(i);return!0},get(l,a,c){var f;if(a===fn)return n;var u=t.get(a),h=a in l;if(u===void 0&&(!h||(f=Wn(l,a))!=null&&f.writable)&&(u=o(()=>{var w=Ue(h?l[a]:ae),g=$(w);return g}),t.set(a,u)),u!==void 0){var _=d(u);return _===ae?void 0:_}return Reflect.get(l,a,c)},getOwnPropertyDescriptor(l,a){var _;(_=this.has)==null||_.call(this,l,a);var c=Reflect.getOwnPropertyDescriptor(l,a),u=t.get(a);if(u!==void 0){var h=d(u);if(h===ae)return;if(c&&"value"in c)c.value=h;else return{enumerable:!0,configurable:!0,value:h,writable:!0}}return c},has(l,a){var _;if(a===fn)return!0;var c=t.get(a),u=c!==void 0&&c.v!==ae||Reflect.has(l,a);if(c!==void 0||(G!==null||q!==null)&&(!u||(_=Wn(l,a))!=null&&_.writable)){c===void 0&&(c=o(()=>{var f=u?Ue(l[a]):ae,w=$(f);return w}),t.set(a,c));var h=d(c);if(h===ae)return!1}return u},set(l,a,c,u){var y;var h=t.get(a),_=a in l;if(r&&a==="length")for(var f=c;f<h.v;f+=1){var w=t.get(f+"");w!==void 0?V(w,ae):f in l&&(w=o(()=>$(ae)),t.set(f+"",w))}if(h===void 0)(!_||(y=Wn(l,a))!=null&&y.writable)&&(h=o(()=>$(void 0)),V(h,Ue(c)),t.set(a,h));else{_=h.v!==ae;var g=o(()=>Ue(c));V(h,g)}var p=Reflect.getOwnPropertyDescriptor(l,a);if(p!=null&&p.set&&p.set.call(u,c),!_){if(r&&typeof a=="string"){var b=t.get("length"),m=Number(a);Number.isInteger(m)&&m>=b.v&&V(b,m+1)}pt(i)}return!0},ownKeys(l){d(i);var a=Reflect.ownKeys(l).filter(h=>{var _=t.get(h);return _===void 0||_.v!==ae});for(var[c,u]of t)u.v!==ae&&!(c in l)&&a.push(c);return a},setPrototypeOf(){po()}})}function gi(n){try{if(n!==null&&typeof n=="object"&&fn in n)return n[fn]}catch{}return n}function os(n,e){return Object.is(gi(n),gi(e))}var wi,ls,as,cs;function No(){if(wi===void 0){wi=window,ls=/Firefox/.test(navigator.userAgent);var n=Element.prototype,e=Node.prototype,t=Text.prototype;as=Wn(e,"firstChild").get,cs=Wn(e,"nextSibling").get,vi(n)&&(n[Ar]=void 0,n[Xt]=null,n[Tr]=void 0,n.__e=void 0),vi(t)&&(t[ut]=void 0)}}function hn(n=""){return document.createTextNode(n)}function vn(n){return as.call(n)}function Ht(n){return cs.call(n)}function z(n,e){return vn(n)}function ie(n,e=!1){{var t=vn(n);return t instanceof Comment&&t.data===""?Ht(t):t}}function se(n,e=!1){return vn(n)}function j(n,e=1,t=!1){let r=n;for(;e--;)r=Ht(r);return r}function Ro(n){n.textContent=""}function us(){return!1}function ti(n,e,t){return e==null||e===Li?t?document.createElement(n,{is:t}):document.createElement(n):t?document.createElementNS(e,n,{is:t}):document.createElementNS(e,n)}function Ho(n){var e=G;if(e===null)return q.f|=Cn,n;if((e.f&ot)===0&&(e.f&nt)===0)throw n;tn(n,e)}function tn(n,e){if(n===zi)throw n;if(!(e!==null&&(e.f&ve)!==0)){for(;e!==null;){if((e.f&Ir)!==0&&(e.f&(ve|xt))===0){if((e.f&ot)===0)throw n;try{e.b.error(n);return}catch(t){n=t}}e=e.parent}throw n}}function Fo(n){G===null&&(q===null&&ho(),fo()),wn&&vo()}function qo(n,e){var t=e.last;t===null?e.last=e.first=n:(t.next=n,n.prev=t,e.last=n)}function pn(n,e){var t=G;t!==null&&(t.f&be)!==0&&(n|=be);var r={ctx:fe,deps:null,nodes:null,f:n|ee|Xe,first:null,fn:e,last:null,next:null,parent:t,b:t&&t.b,prev:null,teardown:null,wv:0,ac:null};P==null||P.register_created_effect(r);var i=r;if((n&nt)!==0)_t!==null?_t.push(r):Fn.ensure().schedule(r);else if(e!==null){try{it(r)}catch(o){throw ye(r),o}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&lt)===0&&(i=i.first,(n&Ge)!==0&&(n&Hn)!==0&&i!==null&&(i.f|=Hn))}if(i!==null&&(i.parent=t,t!==null&&qo(i,t),q!==null&&(q.f&ge)!==0&&(n&En)===0)){var s=q;(s.effects??(s.effects=[])).push(i)}return r}function ri(){return q!==null&&!We}function ds(n){const e=pn(ar,null);return oe(e,ce),e.teardown=n,e}function mt(n){Fo();var e=G.f,t=!q&&(e&Ke)!==0&&fe!==null&&!fe.i;if(t){var r=fe;(r.e??(r.e=[])).push(n)}else return vs(n)}function vs(n){return pn(nt|no,n)}function Vo(n){Fn.ensure();const e=pn(En|lt,n);return(t={})=>new Promise(r=>{t.outro?Nn(e,()=>{ye(e),r(void 0)}):(ye(e),r(void 0))})}function ii(n){return pn(nt,n)}function Yo(n){return pn(Xn|lt,n)}function si(n,e=0){return pn(ar|e,n)}function L(n,e=[],t=[],r=[]){Eo(r,e,t,i=>{pn(ar,()=>{n(...i.map(d))})})}function cr(n,e=0){var t=pn(Ge|e,n);return t}function Ne(n){return pn(Ke|lt,n)}function fs(n){var e=n.teardown;if(e!==null){const t=wn,r=q;_i(!0),He(null);try{e.call(null)}catch(i){tn(i,n.parent)}finally{_i(t),He(r)}}}function oi(n,e=!1){var t=n.first;for(n.first=n.last=null;t!==null;){const i=t.ac;i!==null&&Rt(()=>{i.abort(Nt)});var r=t.next;(t.f&En)!==0?t.parent=null:ye(t,e),t=r}}function Go(n){for(var e=n.first;e!==null;){var t=e.next;(e.f&Ke)===0&&ye(e),e=t}}function ye(n,e=!0){var t=!1;(e||(n.f&eo)!==0)&&n.nodes!==null&&n.nodes.end!==null&&(hs(n.nodes.start,n.nodes.end),t=!0),n.f|=xt,oi(n,e&&!t),Mt(n,0);var r=n.nodes&&n.nodes.t;if(r!==null)for(const s of r)s.stop();fs(n),n.f^=xt,n.f|=ve;var i=n.parent;i!==null&&i.first!==null&&gs(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.fn=n.nodes=n.ac=n.b=null}function hs(n,e){for(;n!==null;){var t=n===e?null:Ht(n);n.remove(),n=t}}function gs(n){var e=n.parent,t=n.prev,r=n.next;t!==null&&(t.next=r),r!==null&&(r.prev=t),e!==null&&(e.first===n&&(e.first=r),e.last===n&&(e.last=t))}function Nn(n,e,t=!0){var r=[];n.f|=Zr,ws(n,r,!0);var i=()=>{t&&ye(n),e&&e()},s=r.length;if(s>0){var o=()=>--s||i();for(var l of r)l.out(o)}else i()}function ws(n,e,t){if((n.f&be)===0){n.f^=be;var r=n.nodes&&n.nodes.t;if(r!==null)for(const l of r)(l.is_global||t)&&e.push(l);for(var i=n.first;i!==null;){var s=i.next;if((i.f&En)===0){var o=(i.f&Hn)!==0||(i.f&Ke)!==0&&(n.f&Ge)!==0;ws(i,e,o?t:!1)}i=s}}}function nr(n){n.f&=~Zr,_s(n,!0)}function _s(n,e){if((n.f&Zr)===0&&(n.f&be)!==0){n.f^=be,(n.f&ce)===0&&(oe(n,ee),Fn.ensure().schedule(n));for(var t=n.first;t!==null;){var r=t.next,i=(t.f&Hn)!==0||(t.f&Ke)!==0;_s(t,i?e:!1),t=r}var s=n.nodes&&n.nodes.t;if(s!==null)for(const o of s)(o.is_global||e)&&o.in()}}function li(n,e){if(n.nodes)for(var t=n.nodes.start,r=n.nodes.end;t!==null;){var i=t===r?null:Ht(t);e.append(t),t=i}}let wn=!1;function _i(n){wn=n}let q=null,We=!1;function He(n){q=n}let G=null;function ln(n){G=n}let on=null;function ps(n){q!==null&&((q.f&Qt)!==0||(q.f&ge)!==0)&&(on??(on=new Set)).add(n)}let ke=null,Ae=0,De=null;function Uo(n){De=n}let ms=1,zn=0,Rn=zn;function pi(n){Rn=n}function bs(){return++ms}function Ft(n){var e=n.f;if((e&ee)!==0)return!0;if((e&Te)!==0){for(var t=n.deps,r=t.length,i=0;i<r;i++){var s=t[i];if(Ft(s)&&Ki(s),s.wv>n.wv)return!0}(e&Xe)!==0&&de===null&&oe(n,ce)}return!1}function ks(n,e,t=!0){var r=n.reactions;if(r!==null&&!(on!==null&&on.has(n)))for(var i=0;i<r.length;i++){var s=r[i];(s.f&ge)!==0?ks(s,e,!1):e===s&&(t?oe(s,ee):(s.f&ce)!==0&&oe(s,Te),ni(s))}}function xs(n){var e=ke,t=Ae,r=De,i=q,s=on,o=fe,l=We,a=Rn,c=n.f;ke=null,Ae=0,De=null,q=(c&(Ke|En))===0?n:null,on=null,tt(n.ctx),We=!1,Rn=++zn,n.ac!==null&&(Rt(()=>{n.ac.abort(Nt)}),n.ac=null);try{n.f|=Qt;var u=n.fn,h=u();n.f|=ot;var _=mi(n);if(Yi()&&De!==null&&!We&&_!==null&&(n.f&(ge|Te|ee))===0)for(var f=0;f<De.length;f++)ks(De[f],n);if(i!==null&&i!==n){if(zn++,i.deps!==null)for(let w=0;w<t;w+=1)i.deps[w].rv=zn;if(e!==null)for(const w of e)w.rv=zn;De!==null&&(r===null?r=De:r.push(...De))}return(n.f&Cn)!==0&&(n.f^=Cn),h}catch(w){return mi(n),Ho(w)}finally{n.f^=Qt,ke=e,Ae=t,De=r,q=i,on=s,tt(o),We=l,Rn=a}}function mi(n){var i;var e=n.deps,t=P==null?void 0:P.is_fork;if(ke!==null){var r;if(t||Mt(n,Ae),e!==null&&Ae>0)for(e.length=Ae+ke.length,r=0;r<ke.length;r++)e[Ae+r]=ke[r];else n.deps=e=ke;if(ri()&&(n.f&Xe)!==0)for(r=Ae;r<e.length;r++)((i=e[r]).reactions??(i.reactions=[])).push(n)}else!t&&e!==null&&Ae<e.length&&(Mt(n,Ae),e.length=Ae);return e}function Wo(n,e){let t=e.reactions;if(t!==null){var r=Ks.call(t,n);if(r!==-1){var i=t.length-1;i===0?t=e.reactions=null:(t[r]=t[i],t.pop())}}if(t===null&&(e.f&ge)!==0&&(ke===null||!kt.call(ke,e))){var s=e;(s.f&Xe)!==0&&(s.f^=Xe),s.v!==ae&&Qr(s),s.ac!==null&&Rt(()=>{s.ac.abort(Nt),s.ac=null,oe(s,ee)}),Bo(s),Mt(s,0)}}function Mt(n,e){var t=n.deps;if(t!==null)for(var r=e;r<t.length;r++)Wo(n,t[r])}function it(n){var e=n.f;if((e&ve)===0){oe(n,ce);var t=G;G=n;try{(e&(Ge|Ni))!==0?Go(n):oi(n),fs(n);var r=xs(n);n.teardown=typeof r=="function"?r:null,n.wv=ms;var i;ji&&ko&&(n.f&ee)!==0&&n.deps}finally{G=t}}}async function ys(){await Promise.resolve(),jo()}function d(n){var e=n.f,t=(e&ge)!==0;if(q!==null&&!We){var r=G!==null&&(G.f&ve)!==0;if(!r&&(on===null||!on.has(n))){var i=q.deps;if((q.f&Qt)!==0)n.rv<zn&&(n.rv=zn,ke===null&&i!==null&&i[Ae]===n?Ae++:ke===null?ke=[n]:ke.push(n));else{q.deps??(q.deps=[]),kt.call(q.deps,n)||q.deps.push(n);var s=n.reactions;s===null?n.reactions=[q]:kt.call(s,q)||s.push(q)}}}if(wn&&sn.has(n))return sn.get(n);if(t){var o=n;if(wn){var l=o.v;return((o.f&ce)===0&&o.reactions!==null||Cs(o))&&(l=$r(o)),sn.set(o,l),l}var a=(o.f&Xe)===0&&!We&&q!==null&&(q.f&Xe)!==0,c=(o.f&ot)===0;Ft(o)&&(a&&(o.f|=Xe),Ki(o)),a&&!c&&(Zi(o),Ms(o))}if(de!=null&&de.has(n))return de.get(n);if((n.f&Cn)!==0)throw n.v;return n.v}function Ms(n){if(n.f|=Xe,n.deps!==null)for(const t of n.deps){var e=t.reactions;e===null?t.reactions=[n]:kt.call(e,n)||e.push(n),(t.f&ge)!==0&&(t.f&Xe)===0&&(Zi(t),Ms(t))}}function Cs(n){if(n.v===ae)return!0;if(n.deps===null)return!1;for(const e of n.deps)if(sn.has(e)||(e.f&ge)!==0&&Cs(e))return!0;return!1}function ur(n){var e=We;try{return We=!0,n()}finally{We=e}}function Xo(n){if(!(typeof n!="object"||!n||n instanceof EventTarget)){if(fn in n)Pr(n);else if(!Array.isArray(n))for(let e in n){const t=n[e];typeof t=="object"&&t&&fn in t&&Pr(t)}}}function Pr(n,e=new Set){if(typeof n=="object"&&n!==null&&!(n instanceof EventTarget)&&!e.has(n)){e.add(n),n instanceof Date&&n.getTime();for(let r in n)try{Pr(n[r],e)}catch{}const t=Kr(n);if(t!==Object.prototype&&t!==Array.prototype&&t!==Map.prototype&&t!==Set.prototype&&t!==Date.prototype){const r=Pi(t);for(let i in r){const s=r[i].get;if(s)try{s.call(n)}catch{}}}}}const Ln=Symbol("events"),Ss=new Set,Or=new Set;function Ko(n,e,t,r={}){function i(s){if(r.capture||Nr.call(e,s),!s.cancelBubble)return Rt(()=>t==null?void 0:t.call(this,s))}return n.startsWith("pointer")||n.startsWith("touch")||n==="wheel"?(i.__removed=!1,dn(()=>{i.__removed||e.addEventListener(n,i,r)})):e.addEventListener(n,i,r),i}function Zo(n,e,t,r={}){var i=Ko(e,n,t,r);return()=>{i.__removed=!0,n.removeEventListener(e,i,r)}}function ne(n,e,t){(e[Ln]??(e[Ln]={}))[n]=t}function Ze(n){for(var e=0;e<n.length;e++)Ss.add(n[e]);for(var t of Or)t(n)}let pr=null,mr=!1;function Nr(n){var g,p;var e=this,t=e.ownerDocument,r=n.type,i=((g=n.composedPath)==null?void 0:g.call(n))||[],s=i[0]||n.target;pr=n,mr||(mr=!0,setTimeout(()=>{mr=!1,pr=null}));var o=0,l=pr===n&&n[Ln];if(l){var a=i.indexOf(l);if(a!==-1&&(e===document||e===window)){n[Ln]=e;return}var c=i.indexOf(e);if(c===-1)return;a<=c&&(o=a)}if(s=i[o]||n.target,s!==e){Di(n,"currentTarget",{configurable:!0,get(){return s||t}});var u=q,h=G;He(null),ln(null);try{for(var _,f=[];s!==null&&s!==e;){try{var w=(p=s[Ln])==null?void 0:p[r];w!=null&&(!s.disabled||n.target===s)&&w.call(s,n)}catch(b){_?f.push(b):_=b}if(n.cancelBubble)break;o++,s=o<i.length?i[o]:null}if(_){for(let b of f)queueMicrotask(()=>{throw b});throw _}}finally{n[Ln]=e,delete n.currentTarget,He(u),ln(h)}}}var Ti;const br=((Ti=globalThis==null?void 0:globalThis.window)==null?void 0:Ti.trustedTypes)&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:n=>n});function Jo(n){return(br==null?void 0:br.createHTML(n))??n}function Qo(n){var e=ti("template");return e.innerHTML=Jo(n.replaceAll("<!>","<!---->")),e.content}function st(n,e){var t=G;t.nodes===null&&(t.nodes={start:n,end:e,a:null,t:null})}function E(n,e){var t=(e&1)!==0,r=(e&2)!==0,i,s=!n.startsWith("<!>");return()=>{i===void 0&&(i=Qo(s?n:"<!>"+n),t||(i=vn(i)));var o=r||ls?document.importNode(i,!0):i.cloneNode(!0);if(t){var l=vn(o),a=o.lastChild;st(l,a)}else st(o,o);return o}}function ai(n=""){{var e=hn(n+"");return st(e,e),e}}function ze(){var n=document.createDocumentFragment(),e=document.createComment(""),t=hn();return n.append(e,t),st(e,t),n}function M(n,e){n!==null&&n.before(e)}function $o(){var n;return(n=window.__svelte??(window.__svelte={})).uid??(n.uid=1),`c${window.__svelte.uid++}`}const el=["touchstart","touchmove"];function nl(n){return el.includes(n)}function tl(n){let e=0,t=qn(0),r;return()=>{ri()&&(d(t),si(()=>(e===0&&(r=ur(()=>n(()=>pt(t)))),e+=1,()=>{dn(()=>{e-=1,e===0&&(r==null||r(),r=void 0,pt(t))})})))}}var rl=Hn|lt;function il(n,e,t,r){new sl(n,e,t,r)}var Pe,Xr,Oe,Dn,he,Se,me,Ee,en,Pn,Mn,et,Et,un,ir,X,ol,ll,Rr,yn,al,Hr,ht,Zt,Fr,qr;class sl{constructor(e,t,r,i){O(this,X);ue(this,"parent");ue(this,"is_pending",!1);ue(this,"transform_error");O(this,Pe);O(this,Xr,null);O(this,Oe);O(this,Dn);O(this,he);O(this,Se,null);O(this,me,null);O(this,Ee,null);O(this,en,null);O(this,Pn,0);O(this,Mn,0);O(this,et,!1);O(this,Et,new Map);O(this,un,null);O(this,ir,tl(()=>(F(this,un,qn(v(this,Pn))),()=>{F(this,un,null)})));var s;F(this,Pe,e),F(this,Oe,t),F(this,Dn,o=>{var l=G;l.b=this,l.f|=Ir,r(o)}),this.parent=G.b,this.transform_error=i??((s=this.parent)==null?void 0:s.transform_error)??(o=>o),F(this,he,cr(()=>{D(this,X,Hr).call(this)},rl))}defer_effect(e){Ui(e,v(this,Et))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!v(this,Oe).pending}update_pending_count(e,t){D(this,X,Fr).call(this,e,t),F(this,Pn,v(this,Pn)+e),!(!v(this,un)||v(this,et))&&(F(this,et,!0),dn(()=>{F(this,et,!1),v(this,un)&&rt(v(this,un),v(this,Pn))}))}get_effect_pending(){return v(this,ir).call(this),d(v(this,un))}error(e){if(e===zi||!v(this,Oe).onerror&&!v(this,Oe).failed)throw e;P!=null&&P.is_fork?(v(this,Se)&&P.skip_effect(v(this,Se)),v(this,me)&&P.skip_effect(v(this,me)),v(this,Ee)&&P.skip_effect(v(this,Ee)),P.oncommit(()=>{D(this,X,yn).call(this)||D(this,X,qr).call(this,e)})):D(this,X,qr).call(this,e)}}Pe=new WeakMap,Xr=new WeakMap,Oe=new WeakMap,Dn=new WeakMap,he=new WeakMap,Se=new WeakMap,me=new WeakMap,Ee=new WeakMap,en=new WeakMap,Pn=new WeakMap,Mn=new WeakMap,et=new WeakMap,Et=new WeakMap,un=new WeakMap,ir=new WeakMap,X=new WeakSet,ol=function(){try{F(this,Se,Ne(()=>v(this,Dn).call(this,v(this,Pe))))}catch(e){this.error(e)}},ll=function(e){const t=v(this,Oe).failed,{reset:r,invoke_onerror:i}=D(this,X,Rr).call(this,e);dn(i),t&&F(this,Ee,Ne(()=>{t(v(this,Pe),()=>e,()=>r)}))},Rr=function(e){var t=!1,r=!1;const i=()=>{if(!D(this,X,yn).call(this)){if(t){lo();return}t=!0,r&&bo(),v(this,Ee)!==null&&Nn(v(this,Ee),()=>{F(this,Ee,null)}),D(this,X,Zt).call(this,()=>{D(this,X,Hr).call(this)})}};return{reset:i,invoke_onerror:()=>{var o,l;if(!D(this,X,yn).call(this))try{r=!0,(l=(o=v(this,Oe)).onerror)==null||l.call(o,e,i),r=!1}catch(a){tn(a,v(this,he)&&v(this,he).parent)}}}},yn=function(){return(v(this,he).f&(ve|xt))!==0},al=function(){const e=v(this,Oe).pending;e&&(this.is_pending=!0,F(this,me,Ne(()=>e(v(this,Pe)))),dn(()=>{if(!D(this,X,yn).call(this)){var t=F(this,en,document.createDocumentFragment()),r=hn(),i=!1;if(t.append(r),F(this,Se,D(this,X,Zt).call(this,()=>{try{return Ne(()=>v(this,Dn).call(this,r))}catch(s){try{this.error(s),i=!0}catch(o){tn(o,v(this,he).parent)}return null}})),v(this,Se)===null){F(this,en,null),i&&D(this,X,ht).call(this,P);return}v(this,Mn)===0&&(v(this,Pe).before(t),F(this,en,null),Nn(v(this,me),()=>{F(this,me,null)}),D(this,X,ht).call(this,P))}}))},Hr=function(){try{if(this.is_pending=this.has_pending_snippet(),F(this,Mn,0),F(this,Pn,0),F(this,Se,Ne(()=>{v(this,Dn).call(this,v(this,Pe))})),v(this,Mn)>0){var e=F(this,en,document.createDocumentFragment());li(v(this,Se),e);const t=v(this,Oe).pending;F(this,me,Ne(()=>t(v(this,Pe))))}else D(this,X,ht).call(this,P)}catch(t){this.error(t)}},ht=function(e){this.is_pending=!1,e.transfer_reactions(v(this,Et))},Zt=function(e){var t=G,r=q,i=fe;ln(v(this,he)),He(v(this,he)),tt(v(this,he).ctx);try{return Fn.ensure(),e()}finally{ln(t),He(r),tt(i)}},Fr=function(e,t){var r;if(!this.has_pending_snippet()){this.parent&&D(r=this.parent,X,Fr).call(r,e,t);return}F(this,Mn,v(this,Mn)+e),v(this,Mn)===0&&(D(this,X,ht).call(this,t),v(this,me)&&Nn(v(this,me),()=>{F(this,me,null)}),v(this,en)&&(v(this,Pe).before(v(this,en)),F(this,en,null)))},qr=function(e){v(this,Se)&&(ye(v(this,Se)),F(this,Se,null)),v(this,me)&&(ye(v(this,me)),F(this,me,null)),v(this,Ee)&&(ye(v(this,Ee)),F(this,Ee,null));let t=v(this,Oe).failed;const r=i=>{if(D(this,X,yn).call(this))return;const{reset:s,invoke_onerror:o}=D(this,X,Rr).call(this,i);o(),t&&!D(this,X,yn).call(this)&&F(this,Ee,D(this,X,Zt).call(this,()=>{try{return Ne(()=>{var l=G;l.b=this,l.f|=Ir,t(v(this,Pe),()=>i,()=>s)})}catch(l){return tn(l,v(this,he).parent),null}}))};dn(()=>{if(!D(this,X,yn).call(this)){var i;try{i=this.transform_error(e)}catch(s){tn(s,v(this,he)&&v(this,he).parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(r,s=>tn(s,v(this,he)&&v(this,he).parent)):r(i)}})};function Z(n,e){var t=e==null?"":typeof e=="object"?`${e}`:e;t!==(n[ut]??(n[ut]=n.nodeValue))&&(n[ut]=t,n.nodeValue=`${t}`)}function Es(n,e){return cl(n,e)}const Ut=new Map;function cl(n,{target:e,anchor:t,props:r={},events:i,context:s,intro:o=!0,transformError:l}){No();var a=void 0,c=Vo(()=>{var u=t??e.appendChild(hn());il(u,{pending:()=>{}},f=>{we({});var w=fe;s&&(w.c=s),i&&(r.$$events=i),a=n(f,r)||Jr(),_e()},l);var h=new Set,_=f=>{for(var w=0;w<f.length;w++){var g=f[w];if(!h.has(g)){h.add(g);var p=nl(g);for(const y of[e,document]){var b=Ut.get(y);b===void 0&&(b=new Map,Ut.set(y,b));var m=b.get(g);m===void 0?(y.addEventListener(g,Nr,{passive:p}),b.set(g,1)):b.set(g,m+1)}}}};return _(lr(Ss)),Or.add(_),()=>{var p;for(var f of h)for(const b of[e,document]){var w=Ut.get(b),g=w.get(f);--g==0?(b.removeEventListener(f,Nr),w.delete(f),w.size===0&&Ut.delete(b)):w.set(f,g)}Or.delete(_),u!==t&&((p=u.parentNode)==null||p.removeChild(u))}});return Vr.set(a,c),a}let Vr=new WeakMap;function Is(n,e){const t=Vr.get(n);return t?(Vr.delete(n),t(e)):Promise.resolve()}var Ye,nn,Ie,On,It,At,sr;class As{constructor(e,t=!0){ue(this,"anchor");O(this,Ye,new Map);O(this,nn,new Map);O(this,Ie,new Map);O(this,On,new Set);O(this,It,!0);O(this,At,e=>{if(v(this,Ye).has(e)){var t=v(this,Ye).get(e),r=v(this,nn).get(t);if(r)nr(r),v(this,On).delete(t);else{var i=v(this,Ie).get(t);i&&(nr(i.effect),v(this,nn).set(t,i.effect),v(this,Ie).delete(t),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),r=i.effect)}for(const[s,o]of v(this,Ye)){if(v(this,Ye).delete(s),s===e)break;const l=v(this,Ie).get(o);l&&(ye(l.effect),v(this,Ie).delete(o))}for(const[s,o]of v(this,nn)){if(s===t||v(this,On).has(s))continue;const l=()=>{if(Array.from(v(this,Ye).values()).includes(s)){var c=document.createDocumentFragment();li(o,c),c.append(hn()),v(this,Ie).set(s,{effect:o,fragment:c})}else ye(o);v(this,On).delete(s),v(this,nn).delete(s)};v(this,It)||!r?(v(this,On).add(s),Nn(o,l,!1)):l()}}});O(this,sr,e=>{v(this,Ye).delete(e);const t=Array.from(v(this,Ye).values());for(const[r,i]of v(this,Ie))t.includes(r)||(ye(i.effect),v(this,Ie).delete(r))});this.anchor=e,F(this,It,t)}ensure(e,t){var r=P,i=us();if(t&&!v(this,nn).has(e)&&!v(this,Ie).has(e))if(i){var s=document.createDocumentFragment(),o=hn();s.append(o),v(this,Ie).set(e,{effect:Ne(()=>t(o)),fragment:s})}else v(this,nn).set(e,Ne(()=>t(this.anchor)));if(v(this,Ye).set(r,e),i){for(const[l,a]of v(this,nn))l===e?r.unskip_effect(a):r.skip_effect(a);for(const[l,a]of v(this,Ie))l===e?r.unskip_effect(a.effect):r.skip_effect(a.effect);r.oncommit(v(this,At)),r.ondiscard(v(this,sr))}else v(this,At).call(this,r)}}Ye=new WeakMap,nn=new WeakMap,Ie=new WeakMap,On=new WeakMap,It=new WeakMap,At=new WeakMap,sr=new WeakMap;function xe(n,e,...t){var r=new As(n);cr(()=>{const i=e()??null;r.ensure(i,i&&(s=>i(s,...t)))},Hn)}function Y(n,e,t=!1){var r=new As(n),i=t?Hn:0;function s(o,l){r.ensure(o,l)}cr(()=>{var o=!1;e((l,a=0)=>{o=!0,s(a,l)}),o||s(-1,null)},i)}function Ct(n,e){return e}function ul(n,e,t){for(var r=[],i=e.length,s,o=e.length,l=0;l<i;l++){let h=e[l];Nn(h,()=>{if(s){if(s.pending.delete(h),s.done.add(h),s.pending.size===0){var _=n.outrogroups;Yr(n,lr(s.done)),_.delete(s),_.size===0&&(n.outrogroups=null)}}else o-=1},!1)}if(o===0){var a=r.length===0&&t!==null&&n.pending.size===0;if(a){var c=t,u=c.parentNode;Ro(u),u.append(c),n.items.clear()}Yr(n,e,!a)}else s={pending:new Set(e),done:new Set},(n.outrogroups??(n.outrogroups=new Set)).add(s)}function Yr(n,e,t=!0){var r;if(n.pending.size>0){r=new Set;for(const o of n.pending.values())for(const l of o)r.add(n.items.get(l).e)}for(var i=0;i<e.length;i++){var s=e[i];if(r!=null&&r.has(s)){s.f|=rn;const o=document.createDocumentFragment();li(s,o)}else ye(e[i],t)}}var bi;function Re(n,e,t,r,i,s=null){var o=n,l=new Map,a=(e&4)!==0;if(a){var c=n;o=c.appendChild(hn())}var u=null,h=Xi(()=>{var m=t();return or(m)?m:m==null?[]:lr(m)}),_=new Map,f=!0;function w(m){if((b.effect.f&ve)===0){b.pending.delete(m);var y=d(h);b.fallback=u,dl(b,y,o,e,r),u!==null&&(y.length===0?(u.f&rn)===0?nr(u):(u.f^=rn,gt(u,null,o)):Nn(u,()=>{u=null}))}}function g(m){b.pending.delete(m)}var p=cr(()=>{for(var m=d(h),y=m.length,S=new Set,k=P,I=us(),x=0;x<y;x+=1){var A=m[x],N=r(A,x),K=f?null:l.get(N);K?(K.v&&rt(K.v,A),K.i&&rt(K.i,x),I&&k.unskip_effect(K.e)):(K=vl(l,f?o:bi??(bi=hn()),A,N,x,i,e,t),f||(K.e.f|=rn),l.set(N,K)),S.add(N)}if(y===0&&s&&!u&&(f?u=Ne(()=>s(o)):(u=Ne(()=>s(bi??(bi=hn()))),u.f|=rn)),y>S.size&&uo(),!f)if(_.set(k,S),I){for(const[te,B]of l)S.has(te)||k.skip_effect(B.e);k.oncommit(w),k.ondiscard(g)}else w(k);d(h)}),b={effect:p,items:l,pending:_,outrogroups:null,fallback:u};f=!1}function ct(n){for(;n!==null&&(n.f&Ke)===0;)n=n.next;return n}function dl(n,e,t,r,i){var K,te,B,H,C,T,U,J,pe;var s=(r&8)!==0,o=e.length,l=n.items,a=ct(n.effect.first),c,u=null,h,_=[],f=[],w,g,p,b;if(s)for(b=0;b<o;b+=1)w=e[b],g=i(w,b),p=l.get(g).e,(p.f&rn)===0&&((te=(K=p.nodes)==null?void 0:K.a)==null||te.measure(),(h??(h=new Set)).add(p));for(b=0;b<o;b+=1){if(w=e[b],g=i(w,b),p=l.get(g).e,n.outrogroups!==null)for(const le of n.outrogroups)le.pending.delete(p),le.done.delete(p);if((p.f&be)!==0&&(nr(p),s&&((H=(B=p.nodes)==null?void 0:B.a)==null||H.unfix(),(h??(h=new Set)).delete(p))),(p.f&rn)!==0)if(p.f^=rn,p===a)gt(p,null,t);else{var m=u?u.next:a;p===n.effect.last&&(n.effect.last=p.prev),p.prev&&(p.prev.next=p.next),p.next&&(p.next.prev=p.prev),xn(n,u,p),xn(n,p,m),gt(p,m,t),u=p,_=[],f=[],a=ct(u.next);continue}if(p!==a){if(c!==void 0&&c.has(p)){if(_.length<f.length){var y=f[0],S;u=y.prev;var k=_[0],I=_[_.length-1];for(S=0;S<_.length;S+=1)gt(_[S],y,t);for(S=0;S<f.length;S+=1)c.delete(f[S]);xn(n,k.prev,I.next),xn(n,u,k),xn(n,I,y),a=y,u=I,b-=1,_=[],f=[]}else c.delete(p),gt(p,a,t),xn(n,p.prev,p.next),xn(n,p,u===null?n.effect.first:u.next),xn(n,u,p),u=p;continue}for(_=[],f=[];a!==null&&a!==p;)(c??(c=new Set)).add(a),f.push(a),a=ct(a.next);if(a===null)continue}(p.f&rn)===0&&_.push(p),u=p,a=ct(p.next)}if(n.outrogroups!==null){for(const le of n.outrogroups)le.pending.size===0&&(Yr(n,lr(le.done)),(C=n.outrogroups)==null||C.delete(le));n.outrogroups.size===0&&(n.outrogroups=null)}if(a!==null||c!==void 0){var x=[];if(c!==void 0)for(p of c)(p.f&be)===0&&x.push(p);for(;a!==null;)(a.f&be)===0&&a!==n.fallback&&x.push(a),a=ct(a.next);var A=x.length;if(A>0){var N=(r&4)!==0&&o===0?t:null;if(s){for(b=0;b<A;b+=1)(U=(T=x[b].nodes)==null?void 0:T.a)==null||U.measure();for(b=0;b<A;b+=1)(pe=(J=x[b].nodes)==null?void 0:J.a)==null||pe.fix()}ul(n,x,N)}}s&&dn(()=>{var le,Le;if(h!==void 0)for(p of h)(Le=(le=p.nodes)==null?void 0:le.a)==null||Le.apply()})}function vl(n,e,t,r,i,s,o,l){var a=(o&1)!==0?(o&16)===0?Po(t,!1,!1):qn(t):null,c=(o&2)!==0?qn(i):null;return{v:a,i:c,e:Ne(()=>(s(e,a??t,c??i,l),()=>{n.delete(r)}))}}function gt(n,e,t){if(n.nodes)for(var r=n.nodes.start,i=n.nodes.end,s=e&&(e.f&rn)===0?e.nodes.start:t;r!==null;){var o=Ht(r);if(s.before(r),r===i)return;r=o}}function xn(n,e,t){e===null?n.effect.first=t:e.next=t,t===null?n.effect.last=e:t.prev=e}function fl(n,e,t=!1,r=!1,i=!1,s=!1){var o=n,l="";if(t)var a=n;L(()=>{var c=G;if(l!==(l=e()??"")){if(t){c.nodes=null,a.innerHTML=l,l!==""&&st(vn(a),a.lastChild);return}if(c.nodes!==null&&(hs(c.nodes.start,c.nodes.end),c.nodes=null),l!==""){var u=r?Ws:i?Xs:void 0,h=ti(r?"svg":i?"math":"template",u);h.innerHTML=l;var _=r||i?h:h.content;if(st(vn(_),_.lastChild),r||i)for(;vn(_);)o.before(vn(_));else o.before(_)}}})}function Ce(n,e){ii(()=>{var i,s;n=((s=(i=G==null?void 0:G.parent)==null?void 0:i.nodes)==null?void 0:s.start)??n;var t=n.getRootNode(),r=t.host?t:t.head??t.ownerDocument.head;if(!r.querySelector("#"+e.hash)){const o=ti("style");o.id=e.hash,o.textContent=e.code,r.appendChild(o)}})}function Gr(n,e,t){ii(()=>{var r=ur(()=>e(n,t==null?void 0:t())||{});if(t&&(r!=null&&r.update)){var i=!1,s={};si(()=>{var o=t();Xo(o),i&&Fi(s,o)&&(s=o,r.update(o))}),i=!0}if(r!=null&&r.destroy)return()=>r.destroy()})}const ki=[...` 	
\r\f \v\uFEFF`];function hl(n,e,t){var r=n==null?"":""+n;if(t){for(var i of Object.keys(t))if(t[i])r=r?r+" "+i:i;else if(r.length)for(var s=i.length,o=0;(o=r.indexOf(i,o))>=0;){var l=o+s;(o===0||ki.includes(r[o-1]))&&(l===r.length||ki.includes(r[l]))?r=(o===0?"":r.substring(0,o))+r.substring(l+1):o=l}}return r===""?null:r}function xi(n,e=!1){var t=e?" !important;":";",r="";for(var i of Object.keys(n)){var s=n[i];s!=null&&s!==""&&(r+=" "+i+": "+s+t)}return r}function gl(n,e){if(e){var t="",r,i;return Array.isArray(e)?(r=e[0],i=e[1]):r=e,r&&(t+=xi(r)),i&&(t+=xi(i,!0)),t=t.trim(),t===""?null:t}return String(n)}function _n(n,e,t,r,i,s){var o=n[Ar];if(o!==t||o===void 0){var l=hl(t,r,s);l==null?n.removeAttribute("class"):n.className=l,n[Ar]=t}else if(s&&i!==s)for(var a in s){var c=!!s[a];(i==null||c!==!!i[a])&&n.classList.toggle(a,c)}return s}function kr(n,e={},t,r){for(var i in t){var s=t[i];e[i]!==s&&(t[i]==null?n.style.removeProperty(i):n.style.setProperty(i,s,r))}}function qt(n,e,t,r){var i=n[Tr];if(i!==e){var s=gl(e,r);s==null?n.removeAttribute("style"):n.style.cssText=s,n[Tr]=e}else r&&(Array.isArray(r)?(kr(n,t==null?void 0:t[0],r[0]),kr(n,t==null?void 0:t[1],r[1],"important")):kr(n,t,r));return r}function wl(n,e){e?n.hasAttribute("selected")||n.setAttribute("selected",""):n.removeAttribute("selected")}function _l(n,e){var t=n.__defaultValue,r=n.multiple,i=r?t??[]:null;if(!(r&&!or(i))){n.selectedIndex;for(var s of n.options){var o=Ur(s);wl(s,r?i.includes(o):os(o,t))}}}function Ts(n,e,t=!1){if(n.multiple){if(e==null)return;if(!or(e))return oo();for(var r of n.options)r.selected=e.includes(Ur(r));return}for(r of n.options){var i=Ur(r);if(os(i,e)){r.selected=!0;return}}(!t||e!==void 0)&&(n.selectedIndex=-1)}function pl(n){var e=new MutationObserver(t=>{t.every(ml)||("__defaultValue"in n&&_l(n),"__value"in n&&Ts(n,n.__value))});e.observe(n,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),ds(()=>{e.disconnect()})}function Ur(n){return"__value"in n?n.__value:n.value}function ml(n){if(n.target.closest("selectedcontent")!==null)return!0;if(n.type==="childList"){var e=[...n.addedNodes,...n.removedNodes];return e.length>0&&e.every(t=>t.nodeName==="SELECTEDCONTENT")}return!1}const bl=Symbol("is custom element"),kl=Symbol("is html"),xl=io?"progress":"PROGRESS";function yl(n,e){var t=Bs(n);t.value===(t.value=e??void 0)||n.value===e&&(e!==0||n.nodeName!==xl)||(n.value=e??"")}function R(n,e,t,r){var i=Bs(n),s=i[e];s!==(i[e]=t)&&(e==="loading"&&(n[ro]=t),t==null?n.removeAttribute(e):typeof t!="string"&&Ml(n).has(e)?n[e]=t:n.setAttribute(e,t))}function Bs(n){return n[Xt]??(n[Xt]={[bl]:n.nodeName.includes("-"),[kl]:n.namespaceURI===Li})}var yi=new Map;function Ml(n){var e=n.getAttribute("is")||n.nodeName,t=yi.get(e);if(t)return t;yi.set(e,t=new Set);for(var r,i=n,s=Element.prototype;s!==i;){r=Pi(i);for(var o in r)r[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&t.add(o);i=Kr(i)}return t}function xr(n,e){return n===e||(n==null?void 0:n[fn])===e}function tr(n=Jr(),e,t,r){var i=fe.r,s=G;return ii(()=>{var o,l;return si(()=>{o=l,l=[],ur(()=>{xr(t(...l),n)||(e(n,...l),o&&xr(t(...o),n)&&e(null,...o))})}),()=>{let a=s;for(;a!==i&&a.parent!==null&&a.parent.f&xt;)a=a.parent;const c=()=>{l&&xr(t(...l),n)&&e(null,...l)},u=a.teardown;a.teardown=()=>{c(),u==null||u()}}}),n}let Wt=!1;function Cl(n){var e=Wt;try{return Wt=!1,[n(),Wt]}finally{Wt=e}}function Be(n,e,t,r){var S;var i=!0,s=(t&8)!==0,o=(t&16)!==0,l=r,a=!0,c=void 0,u=()=>o&&i?(c??(c=yt(r)),d(c)):(a&&(a=!1,l=o?ur(r):r),l);let h;if(s){var _=fn in n||to in n;h=((S=Wn(n,e))==null?void 0:S.set)??(_&&e in n?k=>n[e]=k:void 0)}var f,w=!1;s?[f,w]=Cl(()=>n[e]):f=n[e],f===void 0&&r!==void 0&&(f=u(),h&&(wo(),h(f)));var g;if(g=()=>{var k=n[e];return k===void 0?u():(a=!0,k)},(t&4)===0)return g;if(h){var p=n.$$legacy;return(function(k,I){return arguments.length>0?((!I||p||w)&&h(I?g():k),k):g()})}var b=!1,m=((t&1)!==0?yt:Xi)(()=>(b=!1,g()));s&&d(m);var y=G;return(function(k,I){if(arguments.length>0){const x=I?d(m):s?Ue(k):k;return V(m,x),b=!0,l!==void 0&&(l=x),k}return wn&&b||(y.f&ve)!==0?m.v:d(m)})}function Sl(n){return(e,t)=>{let r=t.onchange;const i=Ue({...t,onchange:o=>{i.value=o,r(o)}}),s=Es(n,{target:e,props:i});return{update({onchange:o,...l}){o&&(r=o),Object.assign(i,l)},destroy(){Is(s)}}}}const El="5";var Bi;typeof window<"u"&&((Bi=window.__svelte??(window.__svelte={})).v??(Bi.v=new Set)).add(El);const Il=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Al=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Tl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Bl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,zl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ll=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,jl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Dl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Pl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ol=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Nl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Rl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Hl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Fl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ql=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Vl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Yl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Gl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ul=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Kl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Zl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Jl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ql=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$l=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ea=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ta=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,va=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,fa=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,_a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ka=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ea=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ta=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,za=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,La=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Oa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,qa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Va=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Za=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`;var Qa=E('<span class="icon svelte-oymxvd"></span>');const $a={hash:"svelte-oymxvd",code:".icon.svelte-oymxvd {display:inline-flex;flex-shrink:0;}.icon.svelte-oymxvd svg {width:100%;height:100%;stroke-width:1.75;}"};function Me(n,e){we(e,!0),Ce(n,$a);const t=Object.assign({"./icons/add-image.svg":Il,"./icons/add-video.svg":Al,"./icons/add.svg":Tl,"./icons/alert.svg":Bl,"./icons/all_pages.svg":zl,"./icons/arrow-down.svg":Ll,"./icons/arrow-left.svg":jl,"./icons/arrow-right.svg":Dl,"./icons/arrow-up.svg":Pl,"./icons/block.svg":Ol,"./icons/bold.svg":Nl,"./icons/button.svg":Rl,"./icons/calendar.svg":Hl,"./icons/channel.svg":Fl,"./icons/check.svg":ql,"./icons/chevron-down.svg":Vl,"./icons/chevron-left.svg":Yl,"./icons/chevron-right.svg":Gl,"./icons/chevrons-up-down.svg":Ul,"./icons/close.svg":Wl,"./icons/copy-url.svg":Xl,"./icons/copy.svg":Kl,"./icons/crop.svg":Zl,"./icons/dashboard.svg":Jl,"./icons/default.svg":Ql,"./icons/delete.svg":$l,"./icons/desktop.svg":ea,"./icons/doc.svg":na,"./icons/download.svg":ta,"./icons/edit.svg":ra,"./icons/entries.svg":ia,"./icons/external.svg":sa,"./icons/file.svg":oa,"./icons/flip-h.svg":la,"./icons/flip-v.svg":aa,"./icons/focal.svg":ca,"./icons/folder-plus.svg":ua,"./icons/folder.svg":da,"./icons/globe.svg":va,"./icons/graphql.svg":fa,"./icons/grip.svg":ha,"./icons/help.svg":ga,"./icons/hide.svg":wa,"./icons/indent.svg":_a,"./icons/info.svg":pa,"./icons/italic.svg":ma,"./icons/link.svg":ba,"./icons/live-preview.svg":ka,"./icons/logout.svg":xa,"./icons/mail.svg":ya,"./icons/mobile.svg":Ma,"./icons/more.svg":Ca,"./icons/move.svg":Sa,"./icons/numbered.svg":Ea,"./icons/outdent.svg":Ia,"./icons/page.svg":Aa,"./icons/page_cat.svg":Ta,"./icons/phone.svg":Ba,"./icons/play.svg":za,"./icons/reload.svg":La,"./icons/replace.svg":ja,"./icons/reset.svg":Da,"./icons/rotate-left.svg":Pa,"./icons/rotate-right.svg":Oa,"./icons/save.svg":Na,"./icons/search.svg":Ra,"./icons/settings.svg":Ha,"./icons/sidebar.svg":Fa,"./icons/single.svg":qa,"./icons/size.svg":Va,"./icons/sparkles.svg":Ya,"./icons/structure.svg":Ga,"./icons/tablet.svg":Ua,"./icons/translate.svg":Wa,"./icons/unlink.svg":Xa,"./icons/upload.svg":Ka,"./icons/users.svg":Za,"./icons/view.svg":Ja});let r=Be(e,"size",3,16);var i=Qa();let s;fl(i,()=>t[`./icons/${e.name}.svg`],!0),L(()=>s=qt(i,"",s,{width:`${r()/16}rem`,height:`${r()/16}rem`})),M(n,i),_e()}var ec=E('<button type="button" class="svelte-eb3b2"><!> </button>'),nc=E('<span class="add-group svelte-eb3b2"><!> <!></span>');const tc={hash:"svelte-eb3b2",code:`.add-group.svelte-eb3b2 {display:inline-flex;flex-wrap:wrap;flex-shrink:0;max-width:100%;border:1px solid var(--border-strong);border-radius:var(--radius);background:var(--surface);}.add-group.svelte-eb3b2 button {border:none;border-radius:0;border-right:1px solid var(--line);color:var(--text);}.add-group.svelte-eb3b2 > button:where(.svelte-eb3b2):first-child {border-radius:var(--radius) 0 0 var(--radius);}.add-group.svelte-eb3b2 > :where(.svelte-eb3b2):last-child,
.add-group.svelte-eb3b2 > :where(.svelte-eb3b2):last-child button {border-right:none;border-radius:0 var(--radius) var(--radius) 0;}.add-group.svelte-eb3b2 > button:where(.svelte-eb3b2):only-child {border-radius:var(--radius);}.add-group.svelte-eb3b2 > button:where(.svelte-eb3b2):first-child .icon {color:var(--accent);}`};function rc(n,e){we(e,!0),Ce(n,tc);var t=nc(),r=z(t);Re(r,19,()=>e.options,s=>s.value,(s,o,l)=>{var a=ec(),c=z(a);{var u=_=>{Me(_,{name:"add",size:14})};Y(c,_=>{d(l)===0&&_(u)})}var h=j(c);L(()=>Z(h,` ${d(o).label??""}`)),ne("click",a,()=>e.onadd(d(o).value)),M(s,a)});var i=j(r,2);xe(i,()=>e.more??gn),M(n,t),_e()}Ze(["click"]);const ic={actions:"Actions",back:"Back",breadcrumbs:"Breadcrumbs",cantMove:"It can't move",changed:"Changed",clear:"Clear",close:"Close",collapse:"Collapse",date:"Date",disabled:"Disabled",dragOrClick:"drag, or click for the moves",done:"Done",duplicate:"Duplicate",expand:"Expand",forward:"Forward",hasErrors:"Has errors",hide:"Hide it on the website",monthAfter:"Month after",monthBefore:"Month before",move:"Move",moveDown:"Move down",moveEarlier:"Move earlier",moveIn:"Move under the one above",moveLater:"Move later",moveOut:"Move out a level",moveUp:"Move up",noDate:"No date",nothingFound:"Nothing found",nothingYet:"Nothing yet",pickDay:"Pick a day",pickMonth:"Pick a month",pickYear:"Pick a year",remove:"Remove",resize:"Drag to change the width, double click for the default",search:"Search",show:"Show it on the website",time:"time",today:"Today"},yr=Symbol.for("zipp-ui"),sc=Ue({texts:ic,locale:void 0}),Mr=globalThis,Q=Mr[yr]??(Mr[yr]=sc),Cr=Symbol.for("zipp-ui-layers"),Sr=globalThis,zs=Sr[Cr]??(Sr[Cr]={stack:[],onkeydown(n){const e=zs.stack;n.key!=="Escape"||!e.length||document.querySelector("dialog:modal")||(n.preventDefault(),e[e.length-1]())}});function oc(n){const{stack:e,onkeydown:t}=zs;return e.length||window.addEventListener("keydown",t),e.push(n),()=>{const r=e.indexOf(n);r>=0&&e.splice(r,1),e.length||window.removeEventListener("keydown",t)}}function Er(n,e){const t=document.activeElement;let r=e;const i=oc(()=>r());return{update(s){r=s},destroy(){i(),t!=null&&t.isConnected&&t.focus()}}}function lc(n,e,t,r=!1){if(t)switch(n){case"ArrowDown":return r?(e+1)%t:Math.min(e+1,t-1);case"ArrowUp":return r?e<=0?t-1:e-1:Math.max(e-1,0);case"Home":return 0;case"End":return t-1}}var ac=E('<div class="popup svelte-1hgkc3f" tabindex="-1"><!></div>'),cc=E('<div class="popover svelte-1hgkc3f"><!> <!></div>');const uc={hash:"svelte-1hgkc3f",code:".popover.svelte-1hgkc3f {position:relative;}.popup.svelte-1hgkc3f {position:fixed;z-index:calc(var(--z-overlay) + 1);display:flex;flex-direction:column;gap:2px;max-width:calc(100vw - 1rem);}"};function Ls(n,e){we(e,!0),Ce(n,uc);let t=Be(e,"open",15,!1),r=Be(e,"at",15,null),i=Be(e,"align",3,"start"),s=Be(e,"menu",3,!1),o=$(void 0),l=$(void 0);const a=()=>t(!t()),c=()=>t(!1);mt(()=>{t()||r(null)});const u=8,h=4;function _(m,y){let S=y;const k=()=>{const x=m.getBoundingClientRect();let A,N;if(S)A=S.x,N=S.y+x.height>innerHeight-u?S.y-x.height:S.y;else{const K=d(o).getBoundingClientRect();A=i()==="end"?K.right-x.width:i()==="center"?K.left+K.width/2-x.width/2:K.left;const te=K.bottom+h,B=K.top-h-x.height;N=te+x.height>innerHeight-u&&B>=u?B:te}A=Math.min(A,innerWidth-x.width-u),m.style.left=`${Math.max(A,u)}px`,m.style.top=`${Math.max(N,u)}px`};k(),addEventListener("scroll",k,!0),addEventListener("resize",k);const I=new ResizeObserver(k);return I.observe(m),{update(x){S=x,k()},destroy(){removeEventListener("scroll",k,!0),removeEventListener("resize",k),I.disconnect()}}}mt(()=>{if(!t())return;const m=y=>{var S;(S=d(o))!=null&&S.contains(y.target)||c()};return window.addEventListener("pointerdown",m),()=>window.removeEventListener("pointerdown",m)}),mt(()=>{var y,S;if(!t())return;const m=s()?"[role=menuitem]":"input";(S=(y=d(l))==null?void 0:y.querySelector(m))==null||S.focus()});function f(m){if(!s()||!d(l))return;const y=[...d(l).querySelectorAll("[role=menuitem]")],S=y.indexOf(document.activeElement),k=lc(m.key,S,y.length,!0);k!==void 0&&(m.preventDefault(),y[k].focus())}var w=cc(),g=z(w);xe(g,()=>e.trigger??gn,()=>({toggle:a,open:t()}));var p=j(g,2);{var b=m=>{var y=ac();let S;var k=z(y);xe(k,()=>e.children),tr(y,I=>V(l,I),()=>d(l)),Gr(y,(I,x)=>Er==null?void 0:Er(I,x),()=>c),Gr(y,(I,x)=>_==null?void 0:_(I,x),r),L(()=>{R(y,"role",s()?"menu":"dialog"),R(y,"aria-label",e.label),S=qt(y,"",S,{width:e.width})}),ne("keydown",y,f),M(m,y)};Y(p,m=>{t()&&m(b)})}tr(w,m=>V(o,m),()=>d(o)),M(n,w),_e()}Ze(["keydown"]);var dc=E('<button type="button" class="icon drag-handle svelte-nnt03w" data-drag-handle=""><!></button>'),vc=E('<button type="button" class="popup-item" role="menuitem"><!> </button>'),fc=E('<span class="popup-item note"> </span>');const hc={hash:"svelte-nnt03w",code:`.drag-handle.svelte-nnt03w {cursor:grab;color:var(--subtle);}

@media (hover: hover) {.drag-handle.svelte-nnt03w:hover {color:var(--text);}
}`};function gc(n,e){we(e,!0),Ce(n,hc);const t=re(()=>e.label??Q.texts.move),r=()=>typeof e.index=="function"?e.index():e.index;let i=$(!1);const s=[["up","moveUp","arrow-up"],["down","moveDown","arrow-down"],["out","moveOut","outdent"],["in","moveIn","indent"]],o={up:["moveEarlier","arrow-left"],down:["moveLater","arrow-right"]},l={ArrowUp:"up",ArrowDown:"down",ArrowLeft:"out",ArrowRight:"in"},a=re(()=>d(i)?s.filter(([g])=>e.sortable.stepDrop(r(),g)).map(([g,p,b])=>e.sortable.grid()&&o[g]?[g,...o[g]]:[g,p,b]):[]),c={ArrowLeft:"up",ArrowRight:"down"};function u(g){const p=e.sortable.grid()&&c[g.key]||l[g.key];p&&(g.preventDefault(),e.sortable.step(r(),p))}var h=ze(),_=ie(h);{var f=g=>{Ls(g,{width:"13rem",get label(){return d(t)},menu:!0,get open(){return d(i)},set open(b){V(i,b,!0)},trigger:b=>{var m=dc();qt(m,"",{},{"touch-action":"none"});var y=z(m);Me(y,{name:"grip",size:15}),L(()=>{R(m,"title",`${d(t)??""}: ${Q.texts.dragOrClick??""}`),R(m,"aria-label",d(t)),R(m,"aria-expanded",d(i))}),ne("pointerdown",m,S=>e.sortable.start(S,r())),ne("click",m,()=>!e.sortable.dragged&&V(i,!d(i))),ne("keydown",m,u),M(b,m)},children:(b,m)=>{var y=ze(),S=ie(y);Re(S,17,()=>d(a),([k,I,x])=>k,(k,I)=>{var x=re(()=>$s(d(I),3));let A=()=>d(x)[0],N=()=>d(x)[1],K=()=>d(x)[2];var te=vc(),B=z(te);Me(B,{get name(){return K()},size:14});var H=j(B);L(()=>Z(H,` ${Q.texts[N()]??""}`)),ne("click",te,()=>{V(i,!1),e.sortable.step(r(),A())}),M(k,te)},k=>{var I=fc(),x=se(I,!0);L(()=>Z(x,Q.texts.cantMove)),M(k,I)}),M(b,y)},$$slots:{trigger:!0,default:!0}})},w=re(()=>e.sortable.movable());Y(_,g=>{d(w)&&g(f)})}M(n,h),_e()}Ze(["pointerdown","click","keydown"]);function wc(n){return n.defaultPrevented||n.target.closest("input, textarea, [contenteditable]")?!1:(n.preventDefault(),n.stopPropagation(),!0)}function _c(n){return n.button===2||n.clientX?{x:n.clientX,y:n.clientY}:null}var pc=E('<a class="popup-item svelte-60nrgv" role="menuitem"><!> </a>'),mc=E('<button type="button" role="menuitem"><!> </button>');const bc={hash:"svelte-60nrgv",code:".popup-item.svelte-60nrgv {min-width:8.625rem;white-space:nowrap;}"};function kc(n,e){we(e,!0),Ce(n,bc);var t=ze(),r=ie(t);Re(r,17,()=>e.items,Ct,(i,s)=>{var o=ze(),l=ie(o);{var a=u=>{var h=pc(),_=z(h);Me(_,{get name(){return d(s).icon},size:14});var f=j(_);L(w=>{R(h,"href",d(s).href),R(h,"target",w),R(h,"download",d(s).download),Z(f,` ${d(s).label??""}`)},[()=>d(s).href.startsWith("/admin")||d(s).download?void 0:"_blank"]),ne("click",h,function(...w){var g;(g=e.close)==null||g.apply(this,w)}),M(u,h)},c=u=>{var h=mc();let _;var f=z(h);Me(f,{get name(){return d(s).icon},size:14});var w=j(f);L(()=>{_=_n(h,1,"popup-item svelte-60nrgv",null,_,{danger:d(s).danger}),Z(w,` ${d(s).label??""}`)}),ne("click",h,()=>{var g,p;e.close(),(p=(g=d(s)).onclick)==null||p.call(g)}),M(u,h)};Y(l,u=>{d(s).href?u(a):u(c,-1)})}M(i,o)}),M(n,t),_e()}Ze(["click"]);var xc=E('<button type="button" class="icon"><!></button>');function yc(n,e){we(e,!0);const t=re(()=>e.label??Q.texts.actions);let r=$(!1),i=$(null),s=$(void 0);mt(()=>{var u;const c=(u=d(s))==null?void 0:u.closest("[data-menu-area]");if(c)return Zo(c,"contextmenu",h=>{wc(h)&&(V(i,_c(h),!0),V(r,!0))})});var o=ze(),l=ie(o);{var a=c=>{Ls(c,{align:"end",get label(){return d(t)},menu:!0,get open(){return d(r)},set open(h){V(r,h,!0)},get at(){return d(i)},set at(h){V(i,h,!0)},trigger:(h,_)=>{let f=()=>(_==null?void 0:_()).toggle,w=()=>(_==null?void 0:_()).open;var g=xc(),p=z(g);Me(p,{name:"more"}),tr(g,b=>V(s,b),()=>d(s)),L(()=>{R(g,"title",d(t)),R(g,"aria-label",d(t)),R(g,"aria-expanded",w())}),ne("click",g,function(...b){var m;(m=f())==null||m.apply(this,b)}),M(h,g)},children:(h,_)=>{kc(h,{get items(){return e.items},close:()=>V(r,!1)})},$$slots:{trigger:!0,default:!0}})};Y(l,c=>{e.items.length&&c(a)})}M(n,o),_e()}Ze(["click"]);var Mi=E('<button type="button" class="icon svelte-sr742y"><!></button>'),Mc=E('<span class="menu svelte-sr742y"><!></span>'),Cc=E('<span><!> <!> <button type="button" class="icon remove svelte-sr742y"><!></button> <!></span>');const Sc={hash:"svelte-sr742y",code:`.item-actions.svelte-sr742y {display:flex;flex-shrink:0;}.menu.svelte-sr742y {display:none;}

@container (width < 30rem) {.many.svelte-sr742y > :where(.svelte-sr742y):not(.menu) {display:none;}.many.svelte-sr742y > .menu:where(.svelte-sr742y) {display:contents;}
}
@media (hover: hover) {.remove.svelte-sr742y:hover:not(:disabled) {color:var(--danger);}
}`};function Ec(n,e){we(e,!0),Ce(n,Sc);let t=Be(e,"off",3,!1);const r=re(()=>[...e.onswitch?[{label:t()?Q.texts.show:Q.texts.hide,icon:t()?"view":"hide",onclick:e.onswitch}]:[],...e.onduplicate?[{label:Q.texts.duplicate,icon:"copy",onclick:e.onduplicate}]:[],{label:e.removeLabel??Q.texts.remove,icon:"delete",onclick:e.onremove,danger:!0}]);var i=Cc();let s;var o=z(i);{var l=w=>{var g=Mi(),p=z(g);{let b=re(()=>t()?"view":"hide");Me(p,{get name(){return d(b)},size:15})}L(()=>{R(g,"title",t()?Q.texts.show:Q.texts.hide),R(g,"aria-label",t()?Q.texts.show:Q.texts.hide),R(g,"aria-pressed",t())}),ne("click",g,function(...b){var m;(m=e.onswitch)==null||m.apply(this,b)}),M(w,g)};Y(o,w=>{e.onswitch&&w(l)})}var a=j(o,2);{var c=w=>{var g=Mi(),p=z(g);Me(p,{name:"copy",size:15}),L(()=>{R(g,"title",Q.texts.duplicate),R(g,"aria-label",Q.texts.duplicate)}),ne("click",g,function(...b){var m;(m=e.onduplicate)==null||m.apply(this,b)}),M(w,g)};Y(a,w=>{e.onduplicate&&w(c)})}var u=j(a,2),h=z(u);Me(h,{name:"delete",size:15});var _=j(u,2);{var f=w=>{var g=Mc(),p=z(g);yc(p,{get items(){return d(r)}}),M(w,g)};Y(_,w=>{d(r).length>1&&w(f)})}L(()=>{s=_n(i,1,"item-actions svelte-sr742y",null,s,{many:d(r).length>1}),R(u,"title",e.removeLabel??Q.texts.remove),R(u,"aria-label",e.removeLabel??Q.texts.remove)}),ne("click",u,function(...w){var g;(g=e.onremove)==null||g.apply(this,w)}),M(n,i),_e()}Ze(["click"]);var Ic=E('<span class="changed-dot"></span>'),Ac=E('<span class="kind svelte-tpn3g1"><!> </span>'),Tc=E('<span class="badge draft"> </span>'),Bc=E('<span class="summary truncate svelte-tpn3g1"><!></span>'),zc=E('<span class="flag svelte-tpn3g1"><!></span>'),Lc=E('<span class="reveal"><!></span>'),jc=E('<div class="body svelte-tpn3g1"><!></div>'),Dc=E('<div><div class="head svelte-tpn3g1"><!> <button type="button" class="toggle plain svelte-tpn3g1"><!> <!> <!> <!> <!></button> <!> <button type="button" class="icon"><!></button></div> <!></div>');const Pc={hash:"svelte-tpn3g1",code:`.item-card.svelte-tpn3g1 {scroll-margin-top:4rem;}.item-card.invalid.svelte-tpn3g1 {border-color:var(--danger);}.item-card.off.svelte-tpn3g1 > .head:where(.svelte-tpn3g1) .kind:where(.svelte-tpn3g1), .item-card.off.svelte-tpn3g1 > .head:where(.svelte-tpn3g1) .summary:where(.svelte-tpn3g1) {opacity:0.55;}.head.svelte-tpn3g1 {display:flex;align-items:center;gap:2px;min-height:2.375rem;padding:0 0.375rem 0 0.25rem;}.open.svelte-tpn3g1 > .head:where(.svelte-tpn3g1) {border-bottom:1px solid var(--line);}.toggle.svelte-tpn3g1 {display:flex;align-items:center;gap:0.5rem;flex:1;min-width:0;padding:0.375rem;}.kind.svelte-tpn3g1 {display:inline-flex;align-items:center;gap:0.25rem;flex-shrink:0;padding:1px 0.4375rem;border-radius:var(--radius-sm);background:var(--accent-soft);color:var(--accent);font-size:var(--fs-xs);font-weight:600;}.summary.svelte-tpn3g1 {color:var(--text-2);font-weight:400;}.flag.svelte-tpn3g1 {display:inline-flex;color:var(--danger);}.body.svelte-tpn3g1 {padding:0.875rem 0.875rem 1rem;}

@container (width < 30rem) {.body.svelte-tpn3g1 {padding:0.625rem 0.625rem 0.75rem;}
}`};function Oc(n,e){we(e,!0),Ce(n,Pc);let t=Be(e,"open",15,!0),r=Be(e,"invalid",3,!1),i=Be(e,"changed",3,!1),s=Be(e,"off",3,!1);const o=()=>t(!t());var l=Dc();let a;var c=z(l),u=z(c);xe(u,()=>e.handle??gn);var h=j(u,2),_=z(h);{var f=B=>{var H=Ic();L(()=>R(H,"title",Q.texts.changed)),M(B,H)};Y(_,B=>{i()&&B(f)})}var w=j(_,2);{var g=B=>{var H=Ac(),C=z(H);xe(C,()=>e.kindIcon??gn);var T=j(C,1,!0);L(()=>Z(T,e.kind)),M(B,H)};Y(w,B=>{e.kind&&B(g)})}var p=j(w,2);{var b=B=>{var H=Tc(),C=se(H,!0);L(()=>Z(C,Q.texts.disabled)),M(B,H)};Y(p,B=>{s()&&B(b)})}var m=j(p,2);{var y=B=>{var H=Bc(),C=z(H);xe(C,()=>e.summary),M(B,H)};Y(m,B=>{e.summary&&B(y)})}var S=j(m,2);{var k=B=>{var H=zc(),C=z(H);Me(C,{name:"alert",size:14}),L(()=>R(H,"title",Q.texts.hasErrors)),M(B,H)};Y(S,B=>{r()&&B(k)})}var I=j(h,2);{var x=B=>{var H=Lc(),C=z(H);xe(C,()=>e.actions),M(B,H)};Y(I,B=>{e.actions&&B(x)})}var A=j(I,2),N=z(A);{let B=re(()=>t()?"chevron-down":"chevron-right");Me(N,{get name(){return d(B)}})}var K=j(c,2);{var te=B=>{var H=jc(),C=z(H);xe(C,()=>e.children),M(B,H)};Y(K,B=>{t()&&B(te)})}L(()=>{a=_n(l,1,"item-card box svelte-tpn3g1",null,a,{open:t(),invalid:r(),off:s()}),R(l,"id",e.id),R(h,"aria-expanded",t()),R(A,"title",t()?Q.texts.collapse:Q.texts.expand),R(A,"aria-label",t()?Q.texts.collapse:Q.texts.expand)}),ne("click",h,o),ne("click",A,o),M(n,l),_e()}Ze(["click"]);var Nc=E('<button type="button"><!></button>'),Rc=E('<div class="segmented svelte-49ceuw"></div>');const Hc={hash:"svelte-49ceuw",code:`.segmented.svelte-49ceuw {display:inline-flex;flex-wrap:wrap;gap:2px;padding:3px;background:var(--hover);border:1px solid var(--line);border-radius:var(--radius-lg);}.segmented.svelte-49ceuw button:where(.svelte-49ceuw) {gap:0.375rem;padding:0.3125rem 0.75rem;border:none;border-radius:var(--radius);background:none;color:var(--muted);}.segmented.svelte-49ceuw button.on:where(.svelte-49ceuw) {background:var(--surface);color:var(--text);box-shadow:var(--shadow);}
@media (hover: hover) {.segmented.svelte-49ceuw button:where(.svelte-49ceuw):hover:not(.on):not(:disabled) {color:var(--text);background:none;}
}`};function Fc(n,e){we(e,!0),Ce(n,Hc);let t=Be(e,"tabs",3,!1);const r=re(()=>e.options.some(o=>o.value===e.value));function i(o){const l={ArrowRight:1,ArrowDown:1,ArrowLeft:-1,ArrowUp:-1}[o.key];if(!l)return;o.preventDefault();const a=e.options.findIndex(h=>h.value===e.value),c=a<0?e.options[l>0?0:e.options.length-1]:e.options[(a+l+e.options.length)%e.options.length];e.onchange(c.value);const u=o.currentTarget.parentElement;ys().then(()=>{var h;return(h=u==null?void 0:u.querySelector('[tabindex="0"]'))==null?void 0:h.focus()})}var s=Rc();Re(s,23,()=>e.options,o=>o.value,(o,l,a)=>{var c=Nc();let u;var h=z(c);{var _=w=>{var g=ze(),p=ie(g);xe(p,()=>e.item,()=>d(l)),M(w,g)},f=w=>{var g=ai();L(()=>Z(g,d(l).label)),M(w,g)};Y(h,w=>{e.item?w(_):w(f,-1)})}L(()=>{R(c,"role",t()?"tab":"radio"),R(c,"aria-selected",t()?d(l).value===e.value:void 0),R(c,"aria-checked",t()?void 0:d(l).value===e.value),R(c,"tabindex",d(l).value===e.value||d(a)===0&&!d(r)?0:-1),u=_n(c,1,"svelte-49ceuw",null,u,{on:d(l).value===e.value})}),ne("click",c,()=>e.onchange(d(l).value)),ne("keydown",c,i),M(o,c)}),L(()=>{R(s,"role",t()?"tablist":"radiogroup"),R(s,"aria-label",e.label)}),M(n,s),_e()}Ze(["click","keydown"]);var qc=E('<div class="sort-line svelte-183yeqa"></div>');const Vc={hash:"svelte-183yeqa",code:".sort-line.svelte-183yeqa {position:absolute;z-index:var(--z-raised);border-radius:2px;background:var(--accent);pointer-events:none;}"};function Yc(n,e){we(e,!0),Ce(n,Vc);var t=ze(),r=ie(t);{var i=s=>{var o=qc();let l;L(()=>l=qt(o,"",l,{top:`${e.sortable.line.top??""}px`,left:`${e.sortable.line.left??""}px`,width:`${e.sortable.line.width??""}px`,height:`${e.sortable.line.height??""}px`})),M(s,o)};Y(r,s=>{e.sortable.line&&s(i)})}M(n,t),_e()}function bt(n,e){let t=1;for(;e+t<n.length&&n[e+t]>n[e];)t++;return t}function Gc(n,e,t,r,i){const s=n[e],o=bt(n,e),l=g=>g<e?g:g-o,a=g=>g+bt(n,g);let c=e-1;for(;c>=0&&n[c]>s;)c--;const u=c>=0&&n[c]===s?c:-1,h=e+o,_=h<n.length&&n[h]===s?h:-1;let f=e-1;for(;f>=0&&n[f]>=s;)f--;const w=Math.max(...n.slice(e,e+o))-s;switch(t){case"up":return u<0?null:{index:l(u),depth:s};case"down":{const g=n.length-o-i,p=_<0?-1:l(a(_));return p<0||p>g?null:{index:p,depth:s}}case"out":return s===0||f<0?null:{index:l(a(f)),depth:s-1};case"in":return u<0||s+1+w>r?null:{index:e,depth:s+1}}}var Tt,Bt;class Uc{constructor(e){O(this,Tt,$(null));O(this,Bt,$(null));ue(this,"drop",null);ue(this,"dragged",!1);this.o=e}get dragging(){return d(v(this,Tt))}set dragging(e){V(v(this,Tt),e,!0)}get line(){return d(v(this,Bt))}set line(e){V(v(this,Bt),e,!0)}rows(){if(this.o.rows)return this.o.rows();const e=this.o.container();return e?[...e.querySelectorAll("[data-sort]")].filter(t=>{var i;const r=(i=t.parentElement)==null?void 0:i.closest("[data-sort]");return!r||!e.contains(r)}):[]}movable(){return this.o.depths().length>1}grid(){var e,t;return((t=(e=this.o).grid)==null?void 0:t.call(e))??!1}moving(e){const t=this.dragging;return t===null?!1:e>=t&&e<t+bt(this.o.depths(),t)}start(e,t){if(e.button!==0)return;const r=e.clientX,i=e.clientY;let s=!1;const o=c=>{if(!s){if(Math.hypot(c.clientX-r,c.clientY-i)<5)return;s=!0,this.dragging=t}c.preventDefault(),this.over(c,t,c.clientX-r),Wc(this.o.container(),c.clientY)},l=c=>{window.removeEventListener("pointermove",o),window.removeEventListener("pointerup",l),window.removeEventListener("pointercancel",l),window.removeEventListener("keydown",a,!0);const u=this.drop;this.dragging=null,this.line=null,this.drop=null,s&&(this.dragged=!0,setTimeout(()=>this.dragged=!1),u&&c.type==="pointerup"&&this.o.onmove(t,u))},a=c=>{c.key!=="Escape"||!s||(c.preventDefault(),c.stopImmediatePropagation(),this.drop=null,l(new PointerEvent("pointercancel")))};window.addEventListener("pointermove",o),window.addEventListener("pointerup",l),window.addEventListener("pointercancel",l),window.addEventListener("keydown",a,!0)}over(e,t,r){var c,u,h;const i=(c=this.o.container())==null?void 0:c.getBoundingClientRect(),s=this.rows(),o=this.o.depths();if(!i||s.length!==o.length)return;const l=bt(o,t),a=s.filter((_,f)=>f<t||f>=t+l);(h=(u=this.o).grid)!=null&&h.call(u)?this.overGrid(e,i,a):this.overList(e,i,a,o,t,r)}overGrid(e,t,r){var c,u;let i=-1,s=1/0;if(r.forEach((h,_)=>{const f=h.getBoundingClientRect(),w=Math.hypot(e.clientX-(f.left+f.width/2),e.clientY-(f.top+f.height/2));w<s&&([i,s]=[_,w])}),i<0)return;const o=r[i].getBoundingClientRect(),l=e.clientX>o.left+o.width/2,a=r.length-(((u=(c=this.o).keepLast)==null?void 0:u.call(c))??0);this.drop={index:Math.min(i+(l?1:0),a),depth:0},this.line={top:o.top-t.top,left:(l?o.right+3:o.left-5)-t.left,width:2,height:o.height}}overList(e,t,r,i,s,o){var S,k,I,x;const l=bt(i,s),a=i.filter((A,N)=>N<s||N>=s+l),c=A=>A.getBoundingClientRect();let u=r.findIndex(A=>{const N=c(A);return e.clientY<N.top+N.height/2});u<0&&(u=r.length),u=Math.min(u,r.length-(((k=(S=this.o).keepLast)==null?void 0:k.call(S))??0));const h=i.slice(s,s+l),_=Math.max(...h)-i[s],f=this.o.indent??24,w=i[s]+Math.round(o/f),g=Math.min(u>0?a[u-1]+1:0,(((x=(I=this.o).maxDepth)==null?void 0:x.call(I))??0)-_),p=u<r.length?a[u]:0;if(p>g){this.drop=null,this.line=null;return}const b=Math.max(p,Math.min(w,g));this.drop={index:u,depth:b};const m=u<r.length?c(r[u]).top-3:r.length?c(r[r.length-1]).bottom+1:t.top,y=(this.o.inset??0)+b*f;this.line={top:m-t.top,left:y,width:t.width-y-(this.o.inset??0),height:2}}stepDrop(e,t){var r,i,s,o;return Gc(this.o.depths(),e,t,((i=(r=this.o).maxDepth)==null?void 0:i.call(r))??0,((o=(s=this.o).keepLast)==null?void 0:o.call(s))??0)}async step(e,t){var s;const r=this.stepDrop(e,t);if(!r)return;await this.o.onmove(e,r),await ys();const i=this.rows()[r.index];(s=i==null?void 0:i.querySelector("[data-drag-handle]"))==null||s.focus()}}Tt=new WeakMap,Bt=new WeakMap;const Ci=48,Si=12;function Wc(n,e){const t=Xc(n),r=t?t.getBoundingClientRect().top:0,i=t?t.getBoundingClientRect().bottom:innerHeight,s=e<r+Ci?-Si:e>i-Ci?Si:0;s&&(t??window).scrollBy(0,s)}function Xc(n){for(let e=n==null?void 0:n.parentElement;e;e=e.parentElement){const t=getComputedStyle(e).overflowY;if((t==="auto"||t==="scroll")&&e.scrollHeight>e.clientHeight)return e}return null}function Kc(n,e){const t=[];n.forEach((r,i)=>{const s=t.at(-1);s&&s.label===r.page?s.rows.push(i):t.push({label:r.page,rows:[i]})});for(const r of e)t.some(i=>i.label===r)||t.push({label:r,rows:[]});return t.length||t.push({label:"",rows:[]}),t}function Wr(n,e){const t=n.findLastIndex(i=>i.page===e.page),r=t<0?n.length:t+1;return[...n.slice(0,r),e,...n.slice(r)]}function Zc(n,e,t){const r=n.filter((i,s)=>s!==e);return Wr(r,{...n[e],page:t})}function Jc(n,e,t,r){const i=e.map(l=>n[l]),[s]=i.splice(t,1);i.splice(r,0,s);const o=[...n];return e.forEach((l,a)=>o[l]=i[a]),o}function Qc(n,e,t){return n.map(r=>r.page===e?{...r,page:t}:r)}function $c(n,e){return n.filter(t=>t.page!==e)}function Ei(n,e){let t=e;for(let r=2;n.includes(t);r++)t=`${e} ${r}`;return t}const Ii={en:{title:"Submissions",note:"What visitors sent with the forms, the newest first.",allForms:"All forms",form:"Form",date:"Date",site:"Site",content:"Content",none:"No submissions yet.",more:"Show more",delete:"Delete",confirmDelete:"Delete this submission for good?",empty:"–",page:"Page",pages:"Pages",addPage:"Add page",pageName:"Name of the page",pageNameNote:"The title of this step on the website.",deletePage:"Delete page",confirmDeletePage:"Delete this page and its fields?",moveTo:"Move to page",noFields:"No fields on this page yet."},de:{title:"Einsendungen",note:"Was Besucher mit den Formularen geschickt haben, die neusten zuerst.",allForms:"Alle Formulare",form:"Formular",date:"Datum",site:"Website",content:"Inhalt",none:"Noch keine Einsendungen.",more:"Mehr zeigen",delete:"Löschen",confirmDelete:"Diese Einsendung endgültig löschen?",empty:"–",page:"Seite",pages:"Seiten",addPage:"Seite hinzufügen",pageName:"Name der Seite",pageNameNote:"Der Titel dieses Schritts auf der Website.",deletePage:"Seite löschen",confirmDeletePage:"Diese Seite und ihre Felder löschen?",moveTo:"Auf Seite verschieben",noFields:"Noch keine Felder auf dieser Seite."}};function js(){return Ii[document.documentElement.lang]??Ii.en}var eu=E('<div class="page-head svelte-qbgxdw"><label class="stacked svelte-qbgxdw"> <input/></label> <button type="button" class="light danger"><!> </button></div> <p class="note svelte-qbgxdw"> </p>',1),nu=E('<div class="tabs"><!></div> <!>',1),tu=E("<option> </option>"),ru=E('<select class="move svelte-qbgxdw"><option disabled=""> </option><!></select>'),iu=E("<!> <!>",1),su=E("<div></div>"),ou=E('<div data-sort=""><!></div>'),lu=E('<p class="empty"> </p>'),au=E('<div class="add svelte-qbgxdw"><!> <button type="button" class="light"><!> </button></div>'),cu=E('<div class="form-fields svelte-qbgxdw"><!> <div class="items svelte-qbgxdw"><!> <!></div> <!></div>');const uu={hash:"svelte-qbgxdw",code:`.form-fields.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.75rem;}.page-head.svelte-qbgxdw {display:flex;align-items:flex-end;gap:0.75rem;}.page-head.svelte-qbgxdw label:where(.svelte-qbgxdw) {flex:1;}.note.svelte-qbgxdw {margin:-0.5rem 0 0;}.items.svelte-qbgxdw {position:relative;display:flex;flex-direction:column;gap:0.5rem;}.move.svelte-qbgxdw {width:auto;padding:0.125rem 0.375rem;font-size:var(--fs-sm);}.add.svelte-qbgxdw {display:flex;flex-wrap:wrap;align-items:center;gap:0.75rem;}

@media (max-width: 799px) {.page-head.svelte-qbgxdw {flex-direction:column;align-items:stretch;}
}`};function du(n,e){const t=$o();we(e,!0),Ce(n,uu);const r=js(),i=re(()=>e.value??[]);let s=$(Ue([]));const o=re(()=>Kc(d(i),d(s)));let l=$(0);const a=re(()=>d(o)[Math.min(d(l),d(o).length-1)]),c=(C,T)=>C.label||`${r.page} ${T+1}`,u=C=>e.onchange(C.length?C:null);function h(C){const T={page:d(a).label,block:e.blocks.create(C)};V(s,d(s).filter(U=>U!==d(a).label),!0),u(Wr(d(i),T))}function _(){const C=d(o).map(U=>U.label),T=Ei(C,`${r.page} ${d(o).length+1}`);V(s,[...d(s),T],!0),V(l,d(o).length,!0)}function f(C){const T=d(o).filter(J=>J!==d(a)).map(J=>J.label),U=Ei(T,C.currentTarget.value.trim());V(s,d(s).map(J=>J===d(a).label?U:J),!0),d(a).rows.length&&u(Qc(d(i),d(a).label,U))}function w(){d(a).rows.length&&!confirm(r.confirmDeletePage)||(V(s,d(s).filter(C=>C!==d(a).label),!0),d(a).rows.length&&u($c(d(i),d(a).label)),V(l,Math.max(0,d(l)-1),!0))}function g(C){const T=d(i)[C];u(Wr(d(i),{...T,block:e.blocks.copy(T.block)}))}function p(C){const{enabled:T,...U}=d(i)[C].block,J=[...d(i)];J[C]={...d(i)[C],block:T===!1?U:{...U,enabled:!1}},u(J)}function b(C,T){V(s,d(s).filter(U=>U!==T),!0),u(Zc(d(i),C,T))}let m=$(void 0);const y=new Uc({container:()=>d(m),depths:()=>d(a).rows.map(()=>0),onmove:(C,T)=>u(Jc(d(i),d(a).rows,C,T.index))}),S=C=>{var T;return((T=e.blocks.types.find(U=>U.handle===C))==null?void 0:T.name)??C};function k(C,T){const U=e.blocks.mount(C,T);return{destroy:()=>U.destroy()}}var I=cu(),x=z(I);{var A=C=>{var T=nu(),U=ie(T),J=z(U);{let Le=re(()=>d(o).map((mn,In)=>({value:In,label:c(mn,In)}))),Fe=re(()=>d(o).indexOf(d(a)));Fc(J,{tabs:!0,get label(){return r.pages},get options(){return d(Le)},get value(){return d(Fe)},onchange:mn=>V(l,mn,!0)})}var pe=j(U,2);{var le=Le=>{var Fe=eu(),mn=ie(Fe),In=z(mn),Vt=z(In),Vn=j(Vt),Je=j(In,2),je=z(Je);Me(je,{name:"delete"});var bn=j(je),An=j(mn,2),qe=se(An,!0);L(Yt=>{Z(Vt,`${r.pageName??""} `),R(Vn,"id",`${t}-name`),yl(Vn,d(a).label),R(Vn,"placeholder",Yt),Z(bn,` ${r.deletePage??""}`),Z(qe,r.pageNameNote)},[()=>c(d(a),d(o).indexOf(d(a)))]),ne("change",Vn,f),ne("click",Je,w),M(Le,Fe)};Y(pe,Le=>{e.readonly||Le(le)})}M(C,T)};Y(x,C=>{d(o).length>1&&C(A)})}var N=j(x,2),K=z(N);Re(K,19,()=>d(a).rows,C=>d(i)[C].block.id,(C,T,U)=>{const J=re(()=>d(i)[d(T)]);var pe=ou();let le;var Le=z(pe);{const Fe=Je=>{var je=ze(),bn=ie(je);{var An=qe=>{gc(qe,{get sortable(){return y},get index(){return d(U)}})};Y(bn,qe=>{e.readonly||qe(An)})}M(Je,je)},mn=Je=>{var je=ai();L(()=>Z(je,d(J).block.fields.label??"")),M(Je,je)},In=Je=>{var je=ze(),bn=ie(je);{var An=qe=>{var Yt=iu(),ci=ie(Yt);{var Ps=at=>{var kn=ru(),Yn=z(kn),Ns=se(Yn,!0);Yn.value=Yn.__value="";var Rs=j(Yn);Re(Rs,17,()=>d(o),Ct,(dr,vr,Hs)=>{var ui=ze(),Fs=ie(ui);{var qs=fr=>{var Gt=tu(),Vs=se(Gt,!0),hr={};L(Ys=>{Z(Vs,Ys),hr!==(hr=d(vr).label)&&(Gt.value=(Gt.__value=hr)??"")},[()=>c(d(vr),Hs)]),M(fr,Gt)};Y(Fs,fr=>{d(vr)!==d(a)&&fr(qs)})}M(dr,ui)}),kn.value=kn.__value="",L(()=>{R(kn,"aria-label",r.moveTo),R(kn,"title",r.moveTo),Z(Ns,r.moveTo)}),ne("change",kn,dr=>b(d(T),dr.currentTarget.value)),M(at,kn)};Y(ci,at=>{d(o).length>1&&at(Ps)})}var Os=j(ci,2);{let at=re(()=>d(J).block.enabled===!1);Ec(Os,{get off(){return d(at)},onswitch:()=>p(d(T)),onduplicate:()=>g(d(T)),onremove:()=>u(d(i).filter((kn,Yn)=>Yn!==d(T)))})}M(qe,Yt)};Y(bn,qe=>{e.readonly||qe(An)})}M(Je,je)};let Vt=re(()=>S(d(J).block.type)),Vn=re(()=>d(J).block.enabled===!1);Oc(Le,{get kind(){return d(Vt)},get off(){return d(Vn)},handle:Fe,summary:mn,actions:In,children:(Je,je)=>{var bn=su();Gr(bn,(An,qe)=>k==null?void 0:k(An,qe),()=>d(J).block.id),M(Je,bn)},$$slots:{handle:!0,summary:!0,actions:!0,default:!0}})}L(Fe=>le=_n(pe,1,"",null,le,{"sort-dragging":Fe}),[()=>y.moving(d(U))]),M(C,pe)},C=>{var T=lu(),U=se(T,!0);L(()=>Z(U,r.noFields)),M(C,T)});var te=j(K,2);Yc(te,{get sortable(){return y}}),tr(N,C=>V(m,C),()=>d(m));var B=j(N,2);{var H=C=>{var T=au(),U=z(T);{let Le=re(()=>e.blocks.types.map(Fe=>({value:Fe.handle,label:Fe.name})));rc(U,{get options(){return d(Le)},onadd:h})}var J=j(U,2),pe=z(J);Me(pe,{name:"add"});var le=j(pe);L(()=>Z(le,` ${r.addPage??""}`)),ne("click",J,_),M(C,T)};Y(B,C=>{e.readonly||C(H)})}M(n,I),_e()}Ze(["change","click"]);var vu=E("<span>/</span>"),fu=E("<a> </a>"),hu=E('<span class="here truncate svelte-wekzd8"> </span>'),gu=E("<!> <!>",1),wu=E('<nav class="crumbs svelte-wekzd8"></nav>');const _u={hash:"svelte-wekzd8",code:`.crumbs.svelte-wekzd8 {display:flex;align-items:center;gap:0.375rem;min-width:0;font-size:var(--fs-md);}a.svelte-wekzd8 {text-decoration:none;white-space:nowrap;}.sep.svelte-wekzd8 {color:var(--subtle);}.here.svelte-wekzd8 {color:var(--text);font-weight:500;}

@container page (width < 36rem) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}
@media (max-width: 799px) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}`};function pu(n,e){we(e,!0),Ce(n,_u);var t=wu();Re(t,21,()=>e.items,Ct,(r,i,s)=>{const o=re(()=>s<e.items.length-2);var l=gu(),a=ie(l);{var c=f=>{var w=vu();let g;L(()=>g=_n(w,1,"sep svelte-wekzd8",null,g,{far:s<e.items.length-1})),M(f,w)};Y(a,f=>{s&&f(c)})}var u=j(a,2);{var h=f=>{var w=fu();let g;var p=se(w,!0);L(()=>{R(w,"href",d(i).href),g=_n(w,1,"svelte-wekzd8",null,g,{far:d(o)}),Z(p,d(i).label)}),M(f,w)},_=f=>{var w=hu(),g=se(w,!0);L(()=>Z(g,d(i).label)),M(f,w)};Y(u,f=>{d(i).href&&s<e.items.length-1?f(h):f(_,-1)})}M(r,l)}),L(()=>R(t,"aria-label",Q.texts.breadcrumbs)),M(n,t),_e()}function Ds(n){const e=Symbol(n);return{provide:t=>void Mo(e,t),get:()=>yo(e)}}const mu=Ds("page-lead"),bu=Ds("flyout-bar");var ku=E('<span class="note svelte-z3zeui"> </span>'),xu=E('<span class="state svelte-z3zeui"><!></span>'),yu=E('<!> <h1 class="svelte-z3zeui"><!></h1> <!> <!> <span class="actions svelte-z3zeui"><!></span>',1),Mu=E("<div><!></div>");const Cu={hash:"svelte-z3zeui",code:`.page-top.svelte-z3zeui {position:sticky;top:0;z-index:var(--z-bar);display:flex;align-items:center;gap:0.5rem;flex-wrap:wrap;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.bleed.svelte-z3zeui {margin:0 calc(-1 * var(--page-pad)) 1.25rem;padding:0.5rem var(--page-pad);}h1.svelte-z3zeui {min-width:0;font-size:var(--fs-lg);line-height:1.25rem;}.state.svelte-z3zeui {display:flex;align-items:center;gap:0.375rem;}.actions.svelte-z3zeui {display:flex;align-items:center;gap:0.5rem;margin-left:auto;}

@container page (width < 36rem) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}
@media (max-width: 799px) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}`};function Su(n,e){we(e,!0),Ce(n,Cu);const t=c=>{var u=yu(),h=ie(u);{var _=x=>{var A=ze(),N=ie(A);xe(N,()=>(s==null?void 0:s.content)??gn),M(x,A)};Y(h,x=>{i||x(_)})}var f=j(h,2),w=z(f);{var g=x=>{pu(x,{get items(){return e.crumbs}})},p=x=>{var A=ai();L(()=>Z(A,e.title)),M(x,A)};Y(w,x=>{e.crumbs?x(g):x(p,-1)})}var b=j(f,2);{var m=x=>{var A=ku(),N=se(A,!0);L(()=>Z(N,e.note)),M(x,A)};Y(b,x=>{e.note&&x(m)})}var y=j(b,2);{var S=x=>{var A=xu(),N=z(A);xe(N,()=>e.state),M(x,A)};Y(y,x=>{e.state&&x(S)})}var k=j(y,2),I=z(k);xe(I,()=>e.actions??gn),M(c,u)};let r=Be(e,"bleed",3,!0);const i=bu.get(),s=mu.get();mt(()=>{if(i)return i.content=t,()=>i.content=null});var o=ze(),l=ie(o);{var a=c=>{var u=Mu();let h;var _=z(u);t(_),L(()=>h=_n(u,1,"page-top svelte-z3zeui",null,h,{bleed:r()})),M(c,u)};Y(l,c=>{i||c(a)})}M(n,o),_e()}var Eu=E("<col/>"),Iu=E("<colgroup></colgroup>"),Au=E('<th class="svelte-migw9g"> </th>'),Tu=E("<thead><tr></tr></thead>"),Bu=E('<tr class="detail svelte-migw9g"><td><!></td></tr>'),zu=E('<tr class="svelte-migw9g"><!></tr> <!>',1),Lu=E('<tr class="svelte-migw9g"><td class="empty svelte-migw9g"> </td></tr>'),ju=E('<div class="scroll svelte-migw9g"><table><!><!><tbody class="svelte-migw9g"></tbody></table></div>');const Du={hash:"svelte-migw9g",code:`.scroll.svelte-migw9g {overflow-x:auto;}table.svelte-migw9g {width:100%;border-collapse:collapse;}table.fixed.svelte-migw9g {table-layout:fixed;}th.svelte-migw9g,
tbody.svelte-migw9g td {padding:0.375rem 0.5rem;text-align:left;vertical-align:top;}th.svelte-migw9g {font-weight:600;}tbody.svelte-migw9g td {border-top:1px solid var(--line);}tbody.svelte-migw9g code {font-size:var(--fs-md);color:var(--muted);}tbody.svelte-migw9g tr:where(.svelte-migw9g).target {background:var(--hover);transition:background 0.3s;}tbody.svelte-migw9g tr[aria-expanded=true]:where(.svelte-migw9g),
.detail.svelte-migw9g {background:var(--surface-2);}.detail.svelte-migw9g td {border-top:none;padding:0.25rem 0.5rem 0.75rem;}.empty.svelte-migw9g {color:var(--muted);}.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail) {cursor:pointer;}

@media (hover: hover) {.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail):hover {background:var(--hover);}
}`};function Pu(n,e){we(e,!0),Ce(n,Du);let t=Be(e,"columns",19,()=>[]);function r(_,f){if(_.target.closest("a, button, input, select"))return;if(e.ontoggle)return e.ontoggle(f);if(!e.href)return;const w=e.href(f);if(_.metaKey||_.ctrlKey)return void window.open(w,"_blank");const p=_.currentTarget.querySelector(`a[href="${CSS.escape(w)}"]`);p?p.click():location.assign(w)}var i=ju(),s=z(i);let o;var l=z(s);{var a=_=>{var f=Iu();Re(f,21,()=>e.widths,Ct,(w,g)=>{var p=Eu();let b;L(()=>b=qt(p,"",b,{width:d(g)})),M(w,p)}),M(_,f)};Y(l,_=>{e.widths&&_(a)})}var c=j(l);{var u=_=>{var f=Tu(),w=z(f);Re(w,21,t,Ct,(g,p)=>{var b=Au(),m=se(b,!0);L(()=>Z(m,d(p))),M(g,b)}),M(_,f)};Y(c,_=>{t().length&&_(u)})}var h=j(c);Re(h,23,()=>e.rows,_=>e.key(_),(_,f,w)=>{var g=zu(),p=ie(g),b=z(p);xe(b,()=>e.row,()=>d(f),()=>d(w));var m=j(p,2);{var y=k=>{var I=Bu(),x=z(I),A=z(x);xe(A,()=>e.detail,()=>d(f)),L(()=>R(x,"colspan",t().length||1)),M(k,I)},S=re(()=>{var k;return e.detail&&((k=e.isOpen)==null?void 0:k.call(e,d(f)))});Y(m,k=>{d(S)&&k(y)})}L((k,I)=>{R(p,"id",k),R(p,"aria-expanded",I),R(p,"tabindex",e.ontoggle?0:void 0)},[()=>{var k;return(k=e.id)==null?void 0:k.call(e,d(f))},()=>{var k;return e.ontoggle?!!((k=e.isOpen)!=null&&k.call(e,d(f))):void 0}]),ne("click",p,k=>r(k,d(f))),ne("keydown",p,k=>{!e.ontoggle||k.target!==k.currentTarget||k.key!=="Enter"&&k.key!==" "||(k.preventDefault(),e.ontoggle(d(f)))}),M(_,g)},_=>{var f=Lu(),w=z(f),g=se(w,!0);L(()=>{R(w,"colspan",t().length||1),Z(g,e.empty??Q.texts.nothingYet)}),M(_,f)}),L(()=>o=_n(s,1,"svelte-migw9g",null,o,{links:!!e.href||!!e.ontoggle,fixed:!!e.widths})),M(n,i),_e()}Ze(["click","keydown"]);var zt,Lt,jt,Dt,Pt,Ot;class Ou{constructor(e){O(this,zt,$(Ue([])));O(this,Lt,$(null));O(this,jt,$(Ue([])));O(this,Dt,$(0));O(this,Pt,$(!1));O(this,Ot,$(""));this.host=e}get forms(){return d(v(this,zt))}set forms(e){V(v(this,zt),e,!0)}get form(){return d(v(this,Lt))}set form(e){V(v(this,Lt),e,!0)}get submissions(){return d(v(this,jt))}set submissions(e){V(v(this,jt),e,!0)}get total(){return d(v(this,Dt))}set total(e){V(v(this,Dt),e,!0)}get loading(){return d(v(this,Pt))}set loading(e){V(v(this,Pt),e,!0)}get error(){return d(v(this,Ot))}set error(e){V(v(this,Ot),e,!0)}async init(){await this.run(async()=>{const e=await this.host.request("GET","/entries?section=forms&limit=500&order=title");this.forms=e.entries.map(t=>({id:t.id,title:t.title}))}),await this.load()}async load(){await this.run(async()=>{const e=await this.request(0);this.submissions=e.submissions,this.total=e.total})}async more(){await this.run(async()=>{const e=await this.request(this.submissions.length);this.submissions=[...this.submissions,...e.submissions],this.total=e.total})}async delete(e){await this.run(async()=>{await this.host.request("DELETE",`/forms/submissions/${e}`),this.submissions=this.submissions.filter(t=>t.id!==e),this.total-=1})}request(e){const t=this.form===null?"":`&form=${this.form}`;return this.host.request("GET",`/forms/submissions?offset=${e}${t}`)}async run(e){this.loading=!0,this.error="";try{await e()}catch(t){this.error=t.message}finally{this.loading=!1}}}zt=new WeakMap,Lt=new WeakMap,jt=new WeakMap,Dt=new WeakMap,Pt=new WeakMap,Ot=new WeakMap;var Nu=E("<option> </option>"),Ru=E("<select><option> </option><!></select>"),Hu=E('<p class="error"> </p>'),Fu=E('<td class="nowrap svelte-1tixh7s"> </td> <td> </td> <td> </td> <td class="truncate"> </td>',1),qu=E('<dt class="svelte-1tixh7s"> </dt> <dd class="svelte-1tixh7s"> </dd>',1),Vu=E('<dl class="svelte-1tixh7s"></dl> <button class="danger"><!> </button>',1),Yu=E('<p class="more svelte-1tixh7s"><button class="light"> </button></p>'),Gu=E('<!> <!> <div class="card"><!></div> <!>',1);const Uu={hash:"svelte-1tixh7s",code:`.nowrap.svelte-1tixh7s {white-space:nowrap;}dl.svelte-1tixh7s {display:grid;grid-template-columns:minmax(8rem, max-content) 1fr;gap:0.375rem 1.5rem;margin:0 0 1rem;}dt.svelte-1tixh7s {color:var(--muted);}dd.svelte-1tixh7s {margin:0;white-space:pre-wrap;overflow-wrap:anywhere;}.more.svelte-1tixh7s {text-align:center;}

@media (max-width: 799px) {dl.svelte-1tixh7s {grid-template-columns:1fr;}dd.svelte-1tixh7s {margin-bottom:0.5rem;}
}`};function Wu(n,e){we(e,!0),Ce(n,Uu);const t=js(),r=new Ou(e.host);r.init();let i=$(null);const s=m=>{var y;return((y=e.schema.sites.find(S=>S.handle===m))==null?void 0:y.name)??m},o=m=>new Date(m).toLocaleString(document.documentElement.lang,{dateStyle:"medium",timeStyle:"short"}),l=m=>m.answers.map(y=>y.value).filter(Boolean).slice(0,3).join(" · ")||t.empty;function a(m){const y=m.currentTarget.value;r.form=y?Number(y):null,V(i,null),r.load()}function c(m){confirm(t.confirmDelete)&&r.delete(m.id)}var u=Gu(),h=ie(u);Su(h,{get title(){return t.title},get note(){return t.note},actions:y=>{var S=ze(),k=ie(S);{var I=x=>{var A=Ru(),N=z(A),K=se(N,!0);N.value=N.__value="";var te=j(N);Re(te,17,()=>r.forms,H=>H.id,(H,C)=>{var T=Nu(),U=se(T,!0),J={};L(()=>{Z(U,d(C).title),J!==(J=d(C).id)&&(T.value=(T.__value=J)??"")}),M(H,T)});var B;pl(A),L(()=>{Z(K,t.allForms),B!==(B=r.form??"")&&(A.value=(A.__value=B)??"",Ts(A,B))}),ne("change",A,a),M(x,A)};Y(k,x=>{r.forms.length>1&&x(I)})}M(y,S)},$$slots:{actions:!0}});var _=j(h,2);{var f=m=>{var y=Hu(),S=se(y,!0);L(()=>Z(S,r.error)),M(m,y)};Y(_,m=>{r.error&&m(f)})}var w=j(_,2),g=z(w);{const m=(I,x=gn)=>{var A=Fu(),N=ie(A),K=se(N,!0),te=j(N,2),B=se(te,!0),H=j(te,2),C=se(H,!0),T=j(H,2),U=se(T,!0);L((J,pe,le)=>{Z(K,J),Z(B,x().form),Z(C,pe),Z(U,le)},[()=>o(x().createdAt),()=>s(x().site),()=>l(x())]),M(I,A)},y=(I,x=gn)=>{var A=Vu(),N=ie(A);Re(N,21,()=>x().answers,H=>H.handle,(H,C)=>{var T=qu(),U=ie(T),J=se(U,!0),pe=j(U,2),le=se(pe,!0);L(()=>{Z(J,d(C).label),Z(le,d(C).value||t.empty)}),M(H,T)});var K=j(N,2),te=z(K);Me(te,{name:"delete"});var B=j(te);L(()=>Z(B,` ${t.delete??""}`)),ne("click",K,()=>c(x())),M(I,A)};let S=re(()=>[t.date,t.form,t.site,t.content]),k=re(()=>r.loading?"…":t.none);Pu(g,{get rows(){return r.submissions},key:I=>I.id,get columns(){return d(S)},get empty(){return d(k)},isOpen:I=>I.id===d(i),ontoggle:I=>V(i,d(i)===I.id?null:I.id,!0),row:m,detail:y,$$slots:{row:!0,detail:!0}})}var p=j(w,2);{var b=m=>{var y=Yu(),S=z(y),k=se(S,!0);L(()=>{S.disabled=r.loading,Z(k,t.more)}),ne("click",S,()=>r.more()),M(m,y)};Y(p,m=>{r.submissions.length<r.total&&m(b)})}M(n,u),_e()}Ze(["change","click"]);const Zu=Sl(du);function Ju(n,e){const t=Es(Wu,{target:n,props:e});return{destroy(){Is(t)}}}export{Zu as mount,Ju as page};
