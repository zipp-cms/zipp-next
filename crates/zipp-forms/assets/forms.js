var vo=Object.defineProperty;var Ci=n=>{throw TypeError(n)};var fo=(n,e,t)=>e in n?vo(n,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):n[e]=t;var we=(n,e,t)=>fo(n,typeof e!="symbol"?e+"":e,t),Mr=(n,e,t)=>e.has(n)||Ci("Cannot "+t);var f=(n,e,t)=>(Mr(n,e,"read from private field"),t?t.call(n):e.get(n)),q=(n,e,t)=>e.has(n)?Ci("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,t),H=(n,e,t,r)=>(Mr(n,e,"write to private field"),r?r.call(n,t):e.set(n,t),t),P=(n,e,t)=>(Mr(n,e,"access private method"),t);const Wi={},ve=Symbol("uninitialized"),Xi="http://www.w3.org/1999/xhtml",ho="http://www.w3.org/2000/svg",go="http://www.w3.org/1998/Math/MathML",Ki=!1;var Ut=Array.isArray,wo=Array.prototype.indexOf,At=Array.prototype.includes,pr=Array.from,Zi=Object.defineProperty,Qn=Object.getOwnPropertyDescriptor,Ji=Object.getOwnPropertyDescriptors,Qi=Object.prototype,po=Array.prototype,_r=Object.getPrototypeOf,Si=Object.isExtensible;const lt=()=>{};function _o(n){for(var e=0;e<n.length;e++)n[e]()}function $i(){var n,e,t=new Promise((r,i)=>{n=r,e=i});return{promise:t,resolve:n,reject:e}}function mo(n,e){if(Array.isArray(n))return n;if(!(Symbol.iterator in n))return Array.from(n);const t=[];for(const r of n)if(t.push(r),t.length===e)break;return t}const ye=2,at=4,mr=8,es=1<<24,en=16,ln=32,Dn=64,Or=128,ri=256,sn=512,fe=1024,se=2048,Re=4096,Ae=8192,me=16384,ht=32768,It=1<<25,Wn=65536,lr=1<<17,bo=1<<18,gt=1<<19,xo=1<<20,gn=1<<25,ar=1<<21,$n=1<<22,Bn=1<<23,Cn=Symbol("$state"),ns=Symbol("component"),ko=Symbol("legacy props"),yo=Symbol(""),rr=Symbol("attributes"),Nr=Symbol("class"),qr=Symbol("style"),_t=Symbol("text"),Gt=new class extends Error{constructor(){super(...arguments);we(this,"name","StaleReactionError");we(this,"message","The reaction that called `getAbortSignal()` was re-run or destroyed")}};var Yi;const Mo=!!((Yi=globalThis.document)!=null&&Yi.contentType)&&globalThis.document.contentType.includes("xml");function Co(){console.warn("https://svelte.dev/e/derived_inert")}function So(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function Eo(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}function ts(n){return n===this.v}function rs(n,e){return n!=n?e==e:n!==e||n!==null&&typeof n=="object"||typeof n=="function"}function is(n){return!rs(n,this.v)}function Ao(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function Io(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function zo(n,e,t){throw new Error("https://svelte.dev/e/each_key_duplicate")}function To(n){throw new Error("https://svelte.dev/e/effect_in_teardown")}function Bo(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function Lo(n){throw new Error("https://svelte.dev/e/effect_orphan")}function Do(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function Po(n){throw new Error("https://svelte.dev/e/props_invalid_value")}function jo(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function Oo(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function No(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function qo(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let Ro=!1;const Fo=[];function Cr(n,e=!1,t=!1){return ir(n,new Map,"",Fo,null,t)}function ir(n,e,t,r,i=null,s=!1){if(typeof n=="object"&&n!==null){var o=e.get(n);if(o!==void 0)return o;if(n instanceof Map)return new Map(n);if(n instanceof Set)return new Set(n);if(Ut(n)){var l=Array(n.length);e.set(n,l),i!==null&&e.set(i,l);for(var a=0;a<n.length;a+=1){var d=n[a];a in n&&(l[a]=ir(d,e,t,r,null,s))}return l}if(_r(n)===Qi){l={},e.set(n,l),i!==null&&e.set(i,l);for(var u of Object.keys(n))l[u]=ir(n[u],e,t,r,null,s);return l}if(n instanceof Date)return n.getTime(),structuredClone(n);if(typeof n.toJSON=="function"&&!s)return ir(n.toJSON(),e,t,r,n)}if(n instanceof EventTarget)return n;try{return structuredClone(n)}catch{return n}}function Ho(n){let e=n.p;for(;e!==null&&e.c===null;)e=e.p;return(e==null?void 0:e.c)??null}function ss(n,e){return n===null&&Ao(),n.c??(n.c=new Map(Ho(n)||void 0))}let be=null;function ct(n){be=n}function Vo(n){return ss(be).get(n)}function Yo(n,e){return ss(be).set(n,e),e}function Me(n,e=!1,t){be={p:be,i:!1,c:null,e:null,s:n,x:null,r:U,l:null}}function Ce(n){var e=be,t=e.e;if(t!==null){e.e=null;for(var r of t)As(r)}return e.i=!0,be=e.p,ii(n)}function ii(n={}){return Zi(n,ns,{value:!0}),n}function os(){return!0}let jn=[];function ls(){var n=jn;jn=[],_o(n)}function yn(n){if(jn.length===0&&!Mt){var e=jn;queueMicrotask(()=>{e===jn&&ls()})}jn.push(n)}function Uo(){for(;jn.length>0;)ls()}const Go=-7169;function ue(n,e){n.f=n.f&Go|e}function si(n){(n.f&sn)!==0||n.deps===null?ue(n,fe):ue(n,Re)}function as(n,e){(n.f&se)!==0?e.set(n,se):(n.f&Re)!==0&&e.get(n)!==se&&e.set(n,Re),ue(n,fe)}function Wt(n){var e=V,t=U;We(null),mn(null);try{return n()}finally{We(e),mn(t)}}function Wo(n,e,t,r){const i=zt;var s=n.filter(v=>!v.settled),o=e.map(i);if(t.length===0&&s.length===0){r(o);return}var l=U,a=Xo(),d=s.length===1?s[0].promise:s.length>1?Promise.all(s.map(v=>v.promise)):null;function u(v){if((l.f&me)===0){a();try{r([...o,...v])}catch(p){hn(p,l)}cr()}}var g=cs();if(t.length===0){d.then(()=>u([])).finally(g);return}function _(){Promise.all(t.map(v=>Ko(v))).then(u).catch(v=>hn(v,l)).finally(g)}d?d.then(()=>{if((l.f&me)!==0){g();return}a(),_(),cr()}):_()}function Xo(){var n=U,e=V,t=be,r=j;return function(s=!0){mn(n),We(e),ct(t),s&&(n.f&me)===0&&(r==null||r.activate(),r==null||r.apply())}}function cr(n=!0){mn(null),We(null),ct(null),n&&(j==null||j.deactivate())}function cs(){var n=U,e=n.b,t=j,r=!!(e!=null&&e.is_rendered());return e==null||e.update_pending_count(1,t),t.increment(r,n),()=>{e==null||e.update_pending_count(-1,t),t.decrement(r,n)}}function zt(n){var e=ye|se;return U!==null&&(U.f|=gt),{ctx:be,deps:null,effects:null,equals:ts,f:e,fn:n,reactions:null,rv:0,v:ve,wv:0,parent:U,ac:null}}const mt=Symbol("obsolete");function Ko(n,e,t){let r=U;r===null&&Io();var i=void 0,s=Kn(ve),o=!V,l=new Set;return dl(()=>{var v,p;var a=U,d=$i();i=d.promise;try{Promise.resolve(n()).then(d.resolve,h=>{h!==Gt&&d.reject(h)}).finally(cr)}catch(h){d.reject(h),cr()}var u=j;if(o){if((a.f&ht)!==0)var g=cs();if((v=r.b)!=null&&v.is_rendered())(p=u.async_deriveds.get(a))==null||p.reject(mt);else for(const h of l.values())h.reject(mt);l.add(d),u.async_deriveds.set(a,d)}const _=(h,w=void 0)=>{g==null||g(),l.delete(d),w!==mt&&(u.activate(),w?(s.f|=Bn,dt(s,w)):((s.f&Bn)!==0&&(s.f^=Bn),dt(s,h)),u.deactivate())};d.promise.then(_,h=>_(null,h||"unknown"))}),Es(()=>{for(const a of l)a.reject(mt)}),new Promise(a=>{function d(u){function g(){u===i?a(s):d(i)}u.then(g,g)}d(i)})}function J(n){const e=zt(n);return Ds(e),e}function ds(n){const e=zt(n);return e.equals=is,e}function Zo(n){var e=n.effects;if(e!==null){n.effects=null;for(var t=0;t<e.length;t+=1)Be(e[t])}}function oi(n){var e,t=U,r=n.parent;if(!En&&r!==null&&n.v!==ve&&(r.f&(me|Ae))!==0)return Co(),n.v;mn(r);try{Zo(n),e=Ns(n)}finally{mn(t)}return e}function us(n){var e=oi(n);if(!n.equals(e)&&(n.wv=js(),(!(j!=null&&j.is_fork)||n.deps===null)&&(j!==null?(j.capture(n,e,!0),Ln==null||Ln.capture(n,e,!0)):n.v=e,n.deps===null))){ue(n,fe);return}En||(_e!==null?(di()||j!=null&&j.is_fork)&&_e.set(n,e):si(n))}function Jo(n){var e;if(n.effects!==null)for(const t of n.effects)(t.teardown||t.ac)&&((e=t.teardown)==null||e.call(t),t.ac!==null&&Wt(()=>{t.ac.abort(Gt),t.ac=null}),t.fn!==null&&(t.teardown=lt),Tt(t,0),fi(t))}function vs(n){if(n.effects!==null)for(const e of n.effects)e.teardown&&e.fn!==null&&ut(e)}let Sr=null,Zn=null,j=null,Ln=null,_e=null,Rr=null,Mt=!1,Er=!1,Ct=null,sr=null;var Ei=0;let Qo=1;var nt,tt,rt,bn,it,xn,Lt,dn,un,st,Rn,K,Fr,fs,Hr,bt,Vr,hs,gs,ws,Jn,$o,xt;const hr=class hr{constructor(){q(this,K);we(this,"id",Qo++);we(this,"started",!1);we(this,"linked",!0);we(this,"prev",null);we(this,"next",null);we(this,"async_deriveds",new Map);we(this,"current",new Map);we(this,"previous",new Map);q(this,nt,new Set);q(this,tt,new Set);q(this,rt,0);q(this,bn,new Map);q(this,it,null);q(this,xn,[]);q(this,Lt,[]);q(this,dn,new Map);q(this,un,new Map);q(this,st,new Set);we(this,"is_fork",!1);q(this,Rn,!1);Zn===null?Sr=Zn=this:(Zn.next=this,this.prev=Zn),Zn=this}skip_effect(e){f(this,un).has(e)||f(this,un).set(e,{d:[],m:[]}),f(this,st).delete(e)}unskip_effect(e,t=r=>this.schedule(r)){var r=f(this,un).get(e);if(r){f(this,un).delete(e);for(var i of r.d)ue(i,se),t(i);for(i of r.m)ue(i,Re),t(i)}f(this,st).add(e)}capture(e,t,r=!1){e.v!==ve&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&Bn)===0&&(this.current.set(e,[t,r]),_e==null||_e.set(e,t)),this.is_fork||(e.v=t)}activate(){j=this}deactivate(){j=null,_e=null}flush(){try{Er=!0,j=this,P(this,K,bt).call(this)}finally{Ei=0,Rr=null,Ct=null,sr=null,Er=!1,j=null,_e=null,pn.clear()}}discard(){var e;for(const t of f(this,tt))t(this);f(this,tt).clear();for(const t of this.async_deriveds.values())t.reject(mt);P(this,K,xt).call(this),(e=f(this,it))==null||e.resolve()}register_created_effect(e){f(this,Lt).push(e)}increment(e,t){if(H(this,rt,f(this,rt)+1),e){let r=f(this,bn).get(t)??0;f(this,bn).set(t,r+1)}}decrement(e,t){if(H(this,rt,f(this,rt)-1),e){let r=f(this,bn).get(t)??0;r===1?f(this,bn).delete(t):f(this,bn).set(t,r-1)}f(this,Rn)||(H(this,Rn,!0),yn(()=>{H(this,Rn,!1),this.linked&&this.flush()}))}transfer_reactions(e){for(const[t,r]of e)this.add_dirty_reaction(t,r);e.clear()}add_dirty_reaction(e,t){(t===se||f(this,dn).get(e)!==se)&&f(this,dn).set(e,t)}oncommit(e){f(this,nt).add(e)}ondiscard(e){f(this,tt).add(e)}settled(){return(f(this,it)??H(this,it,$i())).promise}static ensure(){if(j===null){const e=j=new hr;!Er&&!Mt&&yn(()=>{e.started||e.flush()})}return j}apply(){{_e=null;return}}schedule(e){var t;if(Rr=e,(t=e.b)!=null&&t.is_pending&&(e.f&(at|mr|es))!==0&&(e.f&ht)===0){e.b.defer_effect(e);return}f(this,xn).push(e)}};nt=new WeakMap,tt=new WeakMap,rt=new WeakMap,bn=new WeakMap,it=new WeakMap,xn=new WeakMap,Lt=new WeakMap,dn=new WeakMap,un=new WeakMap,st=new WeakMap,Rn=new WeakMap,K=new WeakSet,Fr=function(){if(this.is_fork)return!0;for(const e of f(this,bn).keys())if(!P(this,K,fs).call(this,e))return!0;return!1},fs=function(e){for(var t=e;t!==null;t=t.parent)if(f(this,un).has(t))return!0;return!1},Hr=function(){var e=[];for(const s of f(this,xn))if(!((s.f&me)!==0||(s.f&(se|Re))===0)){for(var t=s,r=!1;t.parent!==null;){t=t.parent;var i=t.f;if((i&(Dn|ln))!==0){if((i&fe)===0){r=!0;break}t.f^=fe}}r||e.push(t)}return H(this,xn,[]),e},bt=function(){var l,a,d,u;this.started=!0;for(const[g,_]of f(this,dn))ue(g,_),this.schedule(g);this.apply();for(var e=Ct=[],t=[],r=sr=[];f(this,xn).length>0;){Ei++>1e3&&(P(this,K,xt).call(this),nl());for(const g of P(this,K,Hr).call(this))try{P(this,K,Vr).call(this,g,e,t)}catch(_){throw ms(g),P(this,K,Fr).call(this)||this.discard(),_}}if(j=null,r.length>0){var i=hr.ensure();for(const g of r)i.schedule(g)}if(Ct=null,sr=null,P(this,K,Fr).call(this)){P(this,K,Jn).call(this,t),P(this,K,Jn).call(this,e);for(const[g,_]of f(this,un))_s(g,_);r.length>0&&P(l=j,K,bt).call(l);return}const s=P(this,K,hs).call(this);if(s){P(this,K,Jn).call(this,t),P(this,K,Jn).call(this,e),P(a=s,K,ws).call(a,this);return}f(this,dn).clear();for(const g of f(this,nt))g(this);f(this,nt).clear(),Ln=this,Ai(t),Ai(e),Ln=null,(d=f(this,it))==null||d.resolve();var o=j;f(this,rt)===0&&(f(this,xn).length===0||o!==null)&&P(this,K,xt).call(this),o!==null&&(pn.clear(),P(u=o,K,bt).call(u))},Vr=function(e,t,r){e.f^=fe;for(var i=e.first;i!==null;){var s=i.f,o=(s&(ln|Dn))!==0,l=o&&(s&fe)!==0,a=l||(s&Ae)!==0||f(this,un).has(i);if(!a&&i.fn!==null){o?i.f^=fe:(s&at)!==0?t.push(i):Zt(i)&&((s&en)!==0&&this.add_dirty_reaction(i,Re),ut(i));var d=i.first;if(d!==null){i=d;continue}}for(;i!==null;){var u=i.next;if(u!==null){i=u;break}i=i.parent}}},hs=function(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&P(this,K,gs).call(this,e))return e;return null},gs=function(e){for(const[t,[,r]]of this.current)if(!r&&e.current.has(t))return!0;return!1},ws=function(e){var r;for(const[i,s]of e.current)!this.previous.has(i)&&e.previous.has(i)&&this.previous.set(i,e.previous.get(i)),this.current.set(i,s);for(const[i,s]of e.async_deriveds){const o=this.async_deriveds.get(i);o&&s.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(f(e,dn));const t=i=>{var s=i.reactions;if(s!==null&&!((i.f&ye)!==0&&(i.f&(se|Re))===0))for(const a of s){var o=a.f;if((o&ye)!==0)t(a);else{var l=a;o&($n|en)&&!this.async_deriveds.has(l)&&(f(this,dn).delete(l),ue(l,se),this.schedule(l))}}};for(const i of this.current.keys())t(i);this.oncommit(()=>e.discard()),P(r=e,K,xt).call(r),j=this,P(this,K,bt).call(this)},Jn=function(e){for(var t=0;t<e.length;t+=1)as(e[t],f(this,dn))},$o=function(){var g,_;for(let v=Sr;v!==null;v=v.next){var e=v.id<this.id,t=[];for(const[p,[h,w]]of this.current){if(v.current.has(p)){var r=v.current.get(p)[0];if(e&&h!==r)v.current.set(p,[h,w]);else continue}t.push(p)}if(e)for(const[p,h]of this.async_deriveds){const w=v.async_deriveds.get(p);w&&h.promise.then(w.resolve).catch(w.reject)}var i=[...v.current.keys()].filter(p=>!v.current.get(p)[1]);if(!(!v.started||i.length===0)){var s=i.filter(p=>!this.current.has(p));if(s.length===0)e&&v.discard();else if(t.length>0){if(e)for(const p of f(this,st))v.unskip_effect(p,h=>{var w;(h.f&(en|$n))!==0?v.schedule(h):P(w=v,K,Jn).call(w,[h])});v.activate();var o=new Set,l=new Map;for(var a of t)ps(a,s,o,l);l=new Map;var d=[...v.current].filter(([p,h])=>{const w=this.current.get(p);return w?w[0]!==h[0]||w[1]!==h[1]:!0}).map(([p])=>p);if(d.length>0)for(const p of f(this,Lt))(p.f&(me|Ae|lr))===0&&li(p,d,l)&&((p.f&($n|en))!==0?(ue(p,se),v.schedule(p)):v.add_dirty_reaction(p,se));if(f(v,xn).length>0&&!f(v,Rn)){v.apply();for(var u of P(g=v,K,Hr).call(g))P(_=v,K,Vr).call(_,u,[],[])}v.deactivate()}}}},xt=function(){if(this.linked){var e=this.prev,t=this.next;e===null?Sr=t:e.next=t,t===null?Zn=e:t.prev=e,this.linked=!1}};let Xn=hr;function el(n){var e=Mt,t=Ln;Ln=null,Mt=!0;try{for(var r;;){if(Uo(),j===null)return r;j.flush()}}finally{Mt=e,Ln=t}}function nl(){try{Do()}catch(n){hn(n,Rr)}}let Qe=null;function Ai(n){var e=n.length;if(e!==0){for(var t=0;t<e;){var r=n[t++];if((r.f&(me|Ae))===0&&Zt(r)&&(Qe=new Set,ut(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&Ts(r),(Qe==null?void 0:Qe.size)>0)){pn.clear();for(const i of Qe){if((i.f&(me|Ae))!==0)continue;const s=[i];let o=i.parent;for(;o!==null;)Qe.has(o)&&(Qe.delete(o),s.push(o)),o=o.parent;for(let l=s.length-1;l>=0;l--){const a=s[l];(a.f&(me|Ae))===0&&ut(a)}}Qe.clear()}}Qe=null}}function ps(n,e,t,r){if(!t.has(n)&&(t.add(n),n.reactions!==null))for(const i of n.reactions){const s=i.f;(s&ye)!==0?ps(i,e,t,r):(s&($n|en))!==0&&(s&se)===0&&li(i,e,r)&&(ue(i,se),ai(i))}}function li(n,e,t){const r=t.get(n);if(r!==void 0)return r;if(n.deps!==null)for(const i of n.deps){if(At.call(e,i))return!0;if((i.f&ye)!==0&&li(i,e,t))return t.set(i,!0),!0}return t.set(n,!1),!1}function ai(n){j.schedule(n)}function _s(n,e){if(!((n.f&ln)!==0&&(n.f&fe)!==0)){(n.f&se)!==0?e.d.push(n):(n.f&Re)!==0&&e.m.push(n),ue(n,fe);for(var t=n.first;t!==null;)_s(t,e),t=t.next}}function ms(n){ue(n,fe);for(var e=n.first;e!==null;)ms(e),e=e.next}let dr=new Set;const pn=new Map;let bs=!1;function Kn(n,e){var t={f:0,v:n,reactions:null,equals:ts,rv:0,wv:0};return t}function $(n,e){const t=Kn(n);return Ds(t),t}function tl(n,e=!1,t=!0){const r=Kn(n);return e||(r.equals=is),r}function O(n,e,t=!1){V!==null&&(!tn||(V.f&lr)!==0)&&os()&&(V.f&(ye|en|$n|lr))!==0&&(_n===null||!_n.has(n))&&No();let r=t?nn(e):e;return dt(n,r,sr)}var Pn=null,Yr=0;function dt(n,e,t=null){if(!n.equals(e)){En?pn.set(n,e):pn.has(n)||pn.set(n,n.v);var r=Xn.ensure();if(r.capture(n,e),(n.f&ye)!==0){const i=n;(n.f&se)!==0&&oi(i),_e===null&&si(i)}n.wv=js(),Pn=null,Yr=0,xs(n,se,t),Pn=null,U!==null&&(U.f&fe)!==0&&(U.f&(ln|Dn))===0&&(He===null?vl([n]):He.push(n)),!r.is_fork&&dr.size>0&&!bs&&rl()}return e}function rl(){bs=!1;for(const n of dr){(n.f&fe)!==0&&ue(n,Re);let e;try{e=Zt(n)}catch{e=!0}e&&ut(n)}dr.clear()}function St(n){O(n,n.v+1)}function xs(n,e,t){var r=n.reactions;if(r!==null){var i=r.length;if(Yr+=i,Yr>1e5&&Pn===null&&(Pn=new Set),Pn!==null){if(Pn.has(n))return;Pn.add(n)}for(var s=0;s<i;s++){var o=r[s],l=o.f,a=(l&se)===0;if(a&&ue(o,e),(l&lr)!==0)dr.add(o);else if((l&ye)!==0){var d=o;_e==null||_e.delete(d),xs(d,Re,t)}else if(a){var u=o;(l&en)!==0&&Qe!==null&&Qe.add(u),t!==null?t.push(u):ai(u)}}}}function nn(n){if(typeof n!="object"||n===null||Cn in n||ns in n)return n;const e=_r(n);if(e!==Qi&&e!==po)return n;var t=new Map,r=Ut(n),i=$(0),s=Gn,o=l=>{if(Gn===s)return l();var a=V,d=Gn;We(null),Bi(s);var u=l();return We(a),Bi(d),u};return r&&t.set("length",$(n.length)),new Proxy(n,{defineProperty(l,a,d){(!("value"in d)||d.configurable===!1||d.enumerable===!1||d.writable===!1)&&jo();var u=t.get(a);return u===void 0?o(()=>{var g=$(d.value);return t.set(a,g),g}):O(u,d.value,!0),!0},deleteProperty(l,a){var d=t.get(a);if(d===void 0){if(a in l){const u=o(()=>$(ve));t.set(a,u),St(i)}}else O(d,ve),St(i);return!0},get(l,a,d){var v;if(a===Cn)return n;var u=t.get(a),g=a in l;if(u===void 0&&(!g||(v=Qn(l,a))!=null&&v.writable)&&(u=o(()=>{var p=nn(g?l[a]:ve),h=$(p);return h}),t.set(a,u)),u!==void 0){var _=c(u);return _===ve?void 0:_}return Reflect.get(l,a,d)},getOwnPropertyDescriptor(l,a){var _;(_=this.has)==null||_.call(this,l,a);var d=Reflect.getOwnPropertyDescriptor(l,a),u=t.get(a);if(u!==void 0){var g=c(u);if(g===ve)return;if(d&&"value"in d)d.value=g;else return{enumerable:!0,configurable:!0,value:g,writable:!0}}return d},has(l,a){var _;if(a===Cn)return!0;var d=t.get(a),u=d!==void 0&&d.v!==ve||Reflect.has(l,a);if(d!==void 0||(U!==null||V!==null)&&(!u||(_=Qn(l,a))!=null&&_.writable)){d===void 0&&(d=o(()=>{var v=u?nn(l[a]):ve,p=$(v);return p}),t.set(a,d));var g=c(d);if(g===ve)return!1}return u},set(l,a,d,u){var M;var g=t.get(a),_=a in l;if(r&&a==="length")for(var v=d;v<g.v;v+=1){var p=t.get(v+"");p!==void 0?O(p,ve):v in l&&(p=o(()=>$(ve)),t.set(v+"",p))}if(g===void 0)(!_||(M=Qn(l,a))!=null&&M.writable)&&(g=o(()=>$(void 0)),O(g,nn(d)),t.set(a,g));else{_=g.v!==ve;var h=o(()=>nn(d));O(g,h)}var w=Reflect.getOwnPropertyDescriptor(l,a);if(w!=null&&w.set&&w.set.call(u,d),!_){if(r&&typeof a=="string"){var k=t.get("length"),b=Number(a);Number.isInteger(b)&&b>=k.v&&O(k,b+1)}St(i)}return!0},ownKeys(l){c(i);var a=Reflect.ownKeys(l).filter(g=>{var _=t.get(g);return _===void 0||_.v!==ve});for(var[d,u]of t)u.v!==ve&&!(d in l)&&a.push(d);return a},setPrototypeOf(){Oo()}})}function Ii(n){try{if(n!==null&&typeof n=="object"&&Cn in n)return n[Cn]}catch{}return n}function ks(n,e){return Object.is(Ii(n),Ii(e))}var zi,ys,Ms,Cs;function il(){if(zi===void 0){zi=window,ys=/Firefox/.test(navigator.userAgent);var n=Element.prototype,e=Node.prototype,t=Text.prototype;Ms=Qn(e,"firstChild").get,Cs=Qn(e,"nextSibling").get,Si(n)&&(n[Nr]=void 0,n[rr]=null,n[qr]=void 0,n.__e=void 0),Si(t)&&(t[_t]=void 0)}}function Sn(n=""){return document.createTextNode(n)}function Mn(n){return Ms.call(n)}function Xt(n){return Cs.call(n)}function L(n,e){return Mn(n)}function ae(n,e=!1){{var t=Mn(n);return t instanceof Comment&&t.data===""?Xt(t):t}}function te(n,e=!1){return Mn(n)}function D(n,e=1,t=!1){let r=n;for(;e--;)r=Xt(r);return r}function sl(n){n.textContent=""}function Ss(){return!1}function ci(n,e,t){return e==null||e===Xi?t?document.createElement(n,{is:t}):document.createElement(n):t?document.createElementNS(e,n,{is:t}):document.createElementNS(e,n)}function ol(n){var e=U;if(e===null)return V.f|=Bn,n;if((e.f&ht)===0&&(e.f&at)===0)throw n;hn(n,e)}function hn(n,e){if(n===Wi)throw n;if(!(e!==null&&(e.f&me)!==0)){for(;e!==null;){if((e.f&Or)!==0&&(e.f&(me|It))===0){if((e.f&ht)===0)throw n;try{e.b.error(n);return}catch(t){n=t}}e=e.parent}throw n}}function ll(n){U===null&&(V===null&&Lo(),Bo()),En&&To()}function al(n,e){var t=e.last;t===null?e.last=e.first=n:(t.next=n,n.prev=t,e.last=n)}function An(n,e){var t=U;t!==null&&(t.f&Ae)!==0&&(n|=Ae);var r={ctx:be,deps:null,nodes:null,f:n|se|sn,first:null,fn:e,last:null,next:null,parent:t,b:t&&t.b,prev:null,teardown:null,wv:0,ac:null};j==null||j.register_created_effect(r);var i=r;if((n&at)!==0)Ct!==null?Ct.push(r):Xn.ensure().schedule(r);else if(e!==null){try{ut(r)}catch(o){throw Be(r),o}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&gt)===0&&(i=i.first,(n&en)!==0&&(n&Wn)!==0&&i!==null&&(i.f|=Wn))}if(i!==null&&(i.parent=t,t!==null&&al(i,t),V!==null&&(V.f&ye)!==0&&(n&Dn)===0)){var s=V;(s.effects??(s.effects=[])).push(i)}return r}function di(){return V!==null&&!tn}function Es(n){const e=An(mr,null);return ue(e,fe),e.teardown=n,e}function Yn(n){ll();var e=U.f,t=!V&&(e&ln)!==0&&be!==null&&!be.i;if(t){var r=be;(r.e??(r.e=[])).push(n)}else return As(n)}function As(n){return An(at|xo,n)}function cl(n){Xn.ensure();const e=An(Dn|gt,n);return(t={})=>new Promise(r=>{t.outro?Un(e,()=>{Be(e),r(void 0)}):(Be(e),r(void 0))})}function ui(n){return An(at,n)}function dl(n){return An($n|gt,n)}function vi(n,e=0){return An(mr|e,n)}function B(n,e=[],t=[],r=[]){Wo(r,e,t,i=>{An(mr,()=>{n(...i.map(c))})})}function Kt(n,e=0){var t=An(en|e,n);return t}function Ue(n){return An(ln|gt,n)}function Is(n){var e=n.teardown;if(e!==null){const t=En,r=V;Ti(!0),We(null);try{e.call(null)}catch(i){hn(i,n.parent)}finally{Ti(t),We(r)}}}function fi(n,e=!1){var t=n.first;for(n.first=n.last=null;t!==null;){const i=t.ac;i!==null&&Wt(()=>{i.abort(Gt)});var r=t.next;(t.f&Dn)!==0?t.parent=null:Be(t,e),t=r}}function ul(n){for(var e=n.first;e!==null;){var t=e.next;(e.f&ln)===0&&Be(e),e=t}}function Be(n,e=!0){var t=!1;(e||(n.f&bo)!==0)&&n.nodes!==null&&n.nodes.end!==null&&(zs(n.nodes.start,n.nodes.end),t=!0),n.f|=It,fi(n,e&&!t),Tt(n,0);var r=n.nodes&&n.nodes.t;if(r!==null)for(const s of r)s.stop();Is(n),n.f^=It,n.f|=me;var i=n.parent;i!==null&&i.first!==null&&Ts(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.fn=n.nodes=n.ac=n.b=null}function zs(n,e){for(;n!==null;){var t=n===e?null:Xt(n);n.remove(),n=t}}function Ts(n){var e=n.parent,t=n.prev,r=n.next;t!==null&&(t.next=r),r!==null&&(r.prev=t),e!==null&&(e.first===n&&(e.first=r),e.last===n&&(e.last=t))}function Un(n,e,t=!0){var r=[];n.f|=ri,Bs(n,r,!0);var i=()=>{t&&Be(n),e&&e()},s=r.length;if(s>0){var o=()=>--s||i();for(var l of r)l.out(o)}else i()}function Bs(n,e,t){if((n.f&Ae)===0){n.f^=Ae;var r=n.nodes&&n.nodes.t;if(r!==null)for(const l of r)(l.is_global||t)&&e.push(l);for(var i=n.first;i!==null;){var s=i.next;if((i.f&Dn)===0){var o=(i.f&Wn)!==0||(i.f&ln)!==0&&(n.f&en)!==0;Bs(i,e,o?t:!1)}i=s}}}function ur(n){n.f&=~ri,Ls(n,!0)}function Ls(n,e){if((n.f&ri)===0&&(n.f&Ae)!==0){n.f^=Ae,(n.f&fe)===0&&(ue(n,se),Xn.ensure().schedule(n));for(var t=n.first;t!==null;){var r=t.next,i=(t.f&Wn)!==0||(t.f&ln)!==0;Ls(t,i?e:!1),t=r}var s=n.nodes&&n.nodes.t;if(s!==null)for(const o of s)(o.is_global||e)&&o.in()}}function hi(n,e){if(n.nodes)for(var t=n.nodes.start,r=n.nodes.end;t!==null;){var i=t===r?null:Xt(t);e.append(t),t=i}}let En=!1;function Ti(n){En=n}let V=null,tn=!1;function We(n){V=n}let U=null;function mn(n){U=n}let _n=null;function Ds(n){V!==null&&((V.f&ar)!==0||(V.f&ye)!==0)&&(_n??(_n=new Set)).add(n)}let Te=null,qe=0,He=null;function vl(n){He=n}let Ps=1,On=0,Gn=On;function Bi(n){Gn=n}function js(){return++Ps}function Zt(n){var e=n.f;if((e&se)!==0)return!0;if((e&Re)!==0){for(var t=n.deps,r=t.length,i=0;i<r;i++){var s=t[i];if(Zt(s)&&us(s),s.wv>n.wv)return!0}(e&sn)!==0&&_e===null&&ue(n,fe)}return!1}function Os(n,e,t=!0){var r=n.reactions;if(r!==null&&!(_n!==null&&_n.has(n)))for(var i=0;i<r.length;i++){var s=r[i];(s.f&ye)!==0?Os(s,e,!1):e===s&&(t?ue(s,se):(s.f&fe)!==0&&ue(s,Re),ai(s))}}function Ns(n){var e=Te,t=qe,r=He,i=V,s=_n,o=be,l=tn,a=Gn,d=n.f;Te=null,qe=0,He=null,V=(d&(ln|Dn))===0?n:null,_n=null,ct(n.ctx),tn=!1,Gn=++On,n.ac!==null&&(Wt(()=>{n.ac.abort(Gt)}),n.ac=null);try{n.f|=ar;var u=n.fn,g=u();n.f|=ht;var _=Li(n);if(os()&&He!==null&&!tn&&_!==null&&(n.f&(ye|Re|se))===0)for(var v=0;v<He.length;v++)Os(He[v],n);if(i!==null&&i!==n){if(On++,i.deps!==null)for(let p=0;p<t;p+=1)i.deps[p].rv=On;if(e!==null)for(const p of e)p.rv=On;He!==null&&(r===null?r=He:r.push(...He))}return(n.f&Bn)!==0&&(n.f^=Bn),g}catch(p){return Li(n),ol(p)}finally{n.f^=ar,Te=e,qe=t,He=r,V=i,_n=s,ct(o),tn=l,Gn=a}}function Li(n){var i;var e=n.deps,t=j==null?void 0:j.is_fork;if(Te!==null){var r;if(t||Tt(n,qe),e!==null&&qe>0)for(e.length=qe+Te.length,r=0;r<Te.length;r++)e[qe+r]=Te[r];else n.deps=e=Te;if(di()&&(n.f&sn)!==0)for(r=qe;r<e.length;r++)((i=e[r]).reactions??(i.reactions=[])).push(n)}else!t&&e!==null&&qe<e.length&&(Tt(n,qe),e.length=qe);return e}function fl(n,e){let t=e.reactions;if(t!==null){var r=wo.call(t,n);if(r!==-1){var i=t.length-1;i===0?t=e.reactions=null:(t[r]=t[i],t.pop())}}if(t===null&&(e.f&ye)!==0&&(Te===null||!At.call(Te,e))){var s=e;(s.f&sn)!==0&&(s.f^=sn),s.v!==ve&&si(s),s.ac!==null&&Wt(()=>{s.ac.abort(Gt),s.ac=null,ue(s,se)}),Jo(s),Tt(s,0)}}function Tt(n,e){var t=n.deps;if(t!==null)for(var r=e;r<t.length;r++)fl(n,t[r])}function ut(n){var e=n.f;if((e&me)===0){ue(n,fe);var t=U;U=n;try{(e&(en|es))!==0?ul(n):fi(n),Is(n);var r=Ns(n);n.teardown=typeof r=="function"?r:null,n.wv=Ps;var i;Ki&&Ro&&(n.f&se)!==0&&n.deps}finally{U=t}}}async function qs(){await Promise.resolve(),el()}function c(n){var e=n.f,t=(e&ye)!==0;if(V!==null&&!tn){var r=U!==null&&(U.f&me)!==0;if(!r&&(_n===null||!_n.has(n))){var i=V.deps;if((V.f&ar)!==0)n.rv<On&&(n.rv=On,Te===null&&i!==null&&i[qe]===n?qe++:Te===null?Te=[n]:Te.push(n));else{V.deps??(V.deps=[]),At.call(V.deps,n)||V.deps.push(n);var s=n.reactions;s===null?n.reactions=[V]:At.call(s,V)||s.push(V)}}}if(En&&pn.has(n))return pn.get(n);if(t){var o=n;if(En){var l=o.v;return((o.f&fe)===0&&o.reactions!==null||Fs(o))&&(l=oi(o)),pn.set(o,l),l}var a=(o.f&sn)===0&&!tn&&V!==null&&(V.f&sn)!==0,d=(o.f&ht)===0;Zt(o)&&(a&&(o.f|=sn),us(o)),a&&!d&&(vs(o),Rs(o))}if(_e!=null&&_e.has(n))return _e.get(n);if((n.f&Bn)!==0)throw n.v;return n.v}function Rs(n){if(n.f|=sn,n.deps!==null)for(const t of n.deps){var e=t.reactions;e===null?t.reactions=[n]:At.call(e,n)||e.push(n),(t.f&ye)!==0&&(t.f&sn)===0&&(vs(t),Rs(t))}}function Fs(n){if(n.v===ve)return!0;if(n.deps===null)return!1;for(const e of n.deps)if(pn.has(e)||(e.f&ye)!==0&&Fs(e))return!0;return!1}function br(n){var e=tn;try{return tn=!0,n()}finally{tn=e}}function hl(n){if(!(typeof n!="object"||!n||n instanceof EventTarget)){if(Cn in n)Ur(n);else if(!Array.isArray(n))for(let e in n){const t=n[e];typeof t=="object"&&t&&Cn in t&&Ur(t)}}}function Ur(n,e=new Set){if(typeof n=="object"&&n!==null&&!(n instanceof EventTarget)&&!e.has(n)){e.add(n),n instanceof Date&&n.getTime();for(let r in n)try{Ur(n[r],e)}catch{}const t=_r(n);if(t!==Object.prototype&&t!==Array.prototype&&t!==Map.prototype&&t!==Set.prototype&&t!==Date.prototype){const r=Ji(t);for(let i in r){const s=r[i].get;if(s)try{s.call(n)}catch{}}}}}const Nn=Symbol("events"),Hs=new Set,Gr=new Set;function gl(n,e,t,r={}){function i(s){if(r.capture||Wr.call(e,s),!s.cancelBubble)return Wt(()=>t==null?void 0:t.call(this,s))}return n.startsWith("pointer")||n.startsWith("touch")||n==="wheel"?(i.__removed=!1,yn(()=>{i.__removed||e.addEventListener(n,i,r)})):e.addEventListener(n,i,r),i}function wl(n,e,t,r={}){var i=gl(e,n,t,r);return()=>{i.__removed=!0,n.removeEventListener(e,i,r)}}function Q(n,e,t){(e[Nn]??(e[Nn]={}))[n]=t}function an(n){for(var e=0;e<n.length;e++)Hs.add(n[e]);for(var t of Gr)t(n)}let Ar=null,Ir=!1;function Wr(n){var h,w;var e=this,t=e.ownerDocument,r=n.type,i=((h=n.composedPath)==null?void 0:h.call(n))||[],s=i[0]||n.target;Ar=n,Ir||(Ir=!0,setTimeout(()=>{Ir=!1,Ar=null}));var o=0,l=Ar===n&&n[Nn];if(l){var a=i.indexOf(l);if(a!==-1&&(e===document||e===window)){n[Nn]=e;return}var d=i.indexOf(e);if(d===-1)return;a<=d&&(o=a)}if(s=i[o]||n.target,s!==e){Zi(n,"currentTarget",{configurable:!0,get(){return s||t}});var u=V,g=U;We(null),mn(null);try{for(var _,v=[];s!==null&&s!==e;){try{var p=(w=s[Nn])==null?void 0:w[r];p!=null&&(!s.disabled||n.target===s)&&p.call(s,n)}catch(k){_?v.push(k):_=k}if(n.cancelBubble)break;o++,s=o<i.length?i[o]:null}if(_){for(let k of v)queueMicrotask(()=>{throw k});throw _}}finally{n[Nn]=e,delete n.currentTarget,We(u),mn(g)}}}var Ui;const zr=((Ui=globalThis==null?void 0:globalThis.window)==null?void 0:Ui.trustedTypes)&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:n=>n});function pl(n){return(zr==null?void 0:zr.createHTML(n))??n}function _l(n){var e=ci("template");return e.innerHTML=pl(n.replaceAll("<!>","<!---->")),e.content}function vt(n,e){var t=U;t.nodes===null&&(t.nodes={start:n,end:e,a:null,t:null})}function I(n,e){var t=(e&1)!==0,r=(e&2)!==0,i,s=!n.startsWith("<!>");return()=>{i===void 0&&(i=_l(s?n:"<!>"+n),t||(i=Mn(i)));var o=r||ys?document.importNode(i,!0):i.cloneNode(!0);if(t){var l=Mn(o),a=o.lastChild;vt(l,a)}else vt(o,o);return o}}function Vs(n=""){{var e=Sn(n+"");return vt(e,e),e}}function Xe(){var n=document.createDocumentFragment(),e=document.createComment(""),t=Sn();return n.append(e,t),vt(e,t),n}function S(n,e){n!==null&&n.before(e)}function ml(){var n;return(n=window.__svelte??(window.__svelte={})).uid??(n.uid=1),`c${window.__svelte.uid++}`}const bl=["touchstart","touchmove"];function xl(n){return bl.includes(n)}function kl(n){let e=0,t=Kn(0),r;return()=>{di()&&(c(t),vi(()=>(e===0&&(r=br(()=>n(()=>St(t)))),e+=1,()=>{yn(()=>{e-=1,e===0&&(r==null||r(),r=void 0,St(t))})})))}}var yl=Wn|gt;function Ml(n,e,t,r){new Cl(n,e,t,r)}var Ve,ti,Ye,Fn,ke,je,Ee,Oe,vn,Hn,Tn,ot,Dt,kn,gr,Z,Sl,El,Xr,zn,Al,Kr,kt,or,Zr,Jr;class Cl{constructor(e,t,r,i){q(this,Z);we(this,"parent");we(this,"is_pending",!1);we(this,"transform_error");q(this,Ve);q(this,ti,null);q(this,Ye);q(this,Fn);q(this,ke);q(this,je,null);q(this,Ee,null);q(this,Oe,null);q(this,vn,null);q(this,Hn,0);q(this,Tn,0);q(this,ot,!1);q(this,Dt,new Map);q(this,kn,null);q(this,gr,kl(()=>(H(this,kn,Kn(f(this,Hn))),()=>{H(this,kn,null)})));var s;H(this,Ve,e),H(this,Ye,t),H(this,Fn,o=>{var l=U;l.b=this,l.f|=Or,r(o)}),this.parent=U.b,this.transform_error=i??((s=this.parent)==null?void 0:s.transform_error)??(o=>o),H(this,ke,Kt(()=>{P(this,Z,Kr).call(this)},yl))}defer_effect(e){as(e,f(this,Dt))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!f(this,Ye).pending}update_pending_count(e,t){P(this,Z,Zr).call(this,e,t),H(this,Hn,f(this,Hn)+e),!(!f(this,kn)||f(this,ot))&&(H(this,ot,!0),yn(()=>{H(this,ot,!1),f(this,kn)&&dt(f(this,kn),f(this,Hn))}))}get_effect_pending(){return f(this,gr).call(this),c(f(this,kn))}error(e){if(e===Wi||!f(this,Ye).onerror&&!f(this,Ye).failed)throw e;j!=null&&j.is_fork?(f(this,je)&&j.skip_effect(f(this,je)),f(this,Ee)&&j.skip_effect(f(this,Ee)),f(this,Oe)&&j.skip_effect(f(this,Oe)),j.oncommit(()=>{P(this,Z,zn).call(this)||P(this,Z,Jr).call(this,e)})):P(this,Z,Jr).call(this,e)}}Ve=new WeakMap,ti=new WeakMap,Ye=new WeakMap,Fn=new WeakMap,ke=new WeakMap,je=new WeakMap,Ee=new WeakMap,Oe=new WeakMap,vn=new WeakMap,Hn=new WeakMap,Tn=new WeakMap,ot=new WeakMap,Dt=new WeakMap,kn=new WeakMap,gr=new WeakMap,Z=new WeakSet,Sl=function(){try{H(this,je,Ue(()=>f(this,Fn).call(this,f(this,Ve))))}catch(e){this.error(e)}},El=function(e){const t=f(this,Ye).failed,{reset:r,invoke_onerror:i}=P(this,Z,Xr).call(this,e);yn(i),t&&H(this,Oe,Ue(()=>{t(f(this,Ve),()=>e,()=>r)}))},Xr=function(e){var t=!1,r=!1;const i=()=>{if(!P(this,Z,zn).call(this)){if(t){Eo();return}t=!0,r&&qo(),f(this,Oe)!==null&&Un(f(this,Oe),()=>{H(this,Oe,null)}),P(this,Z,or).call(this,()=>{P(this,Z,Kr).call(this)})}};return{reset:i,invoke_onerror:()=>{var o,l;if(!P(this,Z,zn).call(this))try{r=!0,(l=(o=f(this,Ye)).onerror)==null||l.call(o,e,i),r=!1}catch(a){hn(a,f(this,ke)&&f(this,ke).parent)}}}},zn=function(){return(f(this,ke).f&(me|It))!==0},Al=function(){const e=f(this,Ye).pending;e&&(this.is_pending=!0,H(this,Ee,Ue(()=>e(f(this,Ve)))),yn(()=>{if(!P(this,Z,zn).call(this)){var t=H(this,vn,document.createDocumentFragment()),r=Sn(),i=!1;if(t.append(r),H(this,je,P(this,Z,or).call(this,()=>{try{return Ue(()=>f(this,Fn).call(this,r))}catch(s){try{this.error(s),i=!0}catch(o){hn(o,f(this,ke).parent)}return null}})),f(this,je)===null){H(this,vn,null),i&&P(this,Z,kt).call(this,j);return}f(this,Tn)===0&&(f(this,Ve).before(t),H(this,vn,null),Un(f(this,Ee),()=>{H(this,Ee,null)}),P(this,Z,kt).call(this,j))}}))},Kr=function(){try{if(this.is_pending=this.has_pending_snippet(),H(this,Tn,0),H(this,Hn,0),H(this,je,Ue(()=>{f(this,Fn).call(this,f(this,Ve))})),f(this,Tn)>0){var e=H(this,vn,document.createDocumentFragment());hi(f(this,je),e);const t=f(this,Ye).pending;H(this,Ee,Ue(()=>t(f(this,Ve))))}else P(this,Z,kt).call(this,j)}catch(t){this.error(t)}},kt=function(e){this.is_pending=!1,e.transfer_reactions(f(this,Dt))},or=function(e){var t=U,r=V,i=be;mn(f(this,ke)),We(f(this,ke)),ct(f(this,ke).ctx);try{return Xn.ensure(),e()}finally{mn(t),We(r),ct(i)}},Zr=function(e,t){var r;if(!this.has_pending_snippet()){this.parent&&P(r=this.parent,Z,Zr).call(r,e,t);return}H(this,Tn,f(this,Tn)+e),f(this,Tn)===0&&(P(this,Z,kt).call(this,t),f(this,Ee)&&Un(f(this,Ee),()=>{H(this,Ee,null)}),f(this,vn)&&(f(this,Ve).before(f(this,vn)),H(this,vn,null)))},Jr=function(e){f(this,je)&&(Be(f(this,je)),H(this,je,null)),f(this,Ee)&&(Be(f(this,Ee)),H(this,Ee,null)),f(this,Oe)&&(Be(f(this,Oe)),H(this,Oe,null));let t=f(this,Ye).failed;const r=i=>{if(P(this,Z,zn).call(this))return;const{reset:s,invoke_onerror:o}=P(this,Z,Xr).call(this,i);o(),t&&!P(this,Z,zn).call(this)&&H(this,Oe,P(this,Z,or).call(this,()=>{try{return Ue(()=>{var l=U;l.b=this,l.f|=Or,t(f(this,Ve),()=>i,()=>s)})}catch(l){return hn(l,f(this,ke).parent),null}}))};yn(()=>{if(!P(this,Z,zn).call(this)){var i;try{i=this.transform_error(e)}catch(s){hn(s,f(this,ke)&&f(this,ke).parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(r,s=>hn(s,f(this,ke)&&f(this,ke).parent)):r(i)}})};function Y(n,e){var t=e==null?"":typeof e=="object"?`${e}`:e;t!==(n[_t]??(n[_t]=n.nodeValue))&&(n[_t]=t,n.nodeValue=`${t}`)}function Ys(n,e){return Il(n,e)}const nr=new Map;function Il(n,{target:e,anchor:t,props:r={},events:i,context:s,intro:o=!0,transformError:l}){il();var a=void 0,d=cl(()=>{var u=t??e.appendChild(Sn());Ml(u,{pending:()=>{}},v=>{Me({});var p=be;s&&(p.c=s),i&&(r.$$events=i),a=n(v,r)||ii(),Ce()},l);var g=new Set,_=v=>{for(var p=0;p<v.length;p++){var h=v[p];if(!g.has(h)){g.add(h);var w=xl(h);for(const M of[e,document]){var k=nr.get(M);k===void 0&&(k=new Map,nr.set(M,k));var b=k.get(h);b===void 0?(M.addEventListener(h,Wr,{passive:w}),k.set(h,1)):k.set(h,b+1)}}}};return _(pr(Hs)),Gr.add(_),()=>{var w;for(var v of g)for(const k of[e,document]){var p=nr.get(k),h=p.get(v);--h==0?(k.removeEventListener(v,Wr),p.delete(v),p.size===0&&nr.delete(k)):p.set(v,h)}Gr.delete(_),u!==t&&((w=u.parentNode)==null||w.removeChild(u))}});return Qr.set(a,d),a}let Qr=new WeakMap;function Us(n,e){const t=Qr.get(n);return t?(Qr.delete(n),t(e)):Promise.resolve()}var $e,fn,Ne,Vn,Pt,jt,wr;class gi{constructor(e,t=!0){we(this,"anchor");q(this,$e,new Map);q(this,fn,new Map);q(this,Ne,new Map);q(this,Vn,new Set);q(this,Pt,!0);q(this,jt,e=>{if(f(this,$e).has(e)){var t=f(this,$e).get(e),r=f(this,fn).get(t);if(r)ur(r),f(this,Vn).delete(t);else{var i=f(this,Ne).get(t);i&&(ur(i.effect),f(this,fn).set(t,i.effect),f(this,Ne).delete(t),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),r=i.effect)}for(const[s,o]of f(this,$e)){if(f(this,$e).delete(s),s===e)break;const l=f(this,Ne).get(o);l&&(Be(l.effect),f(this,Ne).delete(o))}for(const[s,o]of f(this,fn)){if(s===t||f(this,Vn).has(s))continue;const l=()=>{if(Array.from(f(this,$e).values()).includes(s)){var d=document.createDocumentFragment();hi(o,d),d.append(Sn()),f(this,Ne).set(s,{effect:o,fragment:d})}else Be(o);f(this,Vn).delete(s),f(this,fn).delete(s)};f(this,Pt)||!r?(f(this,Vn).add(s),Un(o,l,!1)):l()}}});q(this,wr,e=>{f(this,$e).delete(e);const t=Array.from(f(this,$e).values());for(const[r,i]of f(this,Ne))t.includes(r)||(Be(i.effect),f(this,Ne).delete(r))});this.anchor=e,H(this,Pt,t)}ensure(e,t){var r=j,i=Ss();if(t&&!f(this,fn).has(e)&&!f(this,Ne).has(e))if(i){var s=document.createDocumentFragment(),o=Sn();s.append(o),f(this,Ne).set(e,{effect:Ue(()=>t(o)),fragment:s})}else f(this,fn).set(e,Ue(()=>t(this.anchor)));if(f(this,$e).set(r,e),i){for(const[l,a]of f(this,fn))l===e?r.unskip_effect(a):r.skip_effect(a);for(const[l,a]of f(this,Ne))l===e?r.unskip_effect(a.effect):r.skip_effect(a.effect);r.oncommit(f(this,jt)),r.ondiscard(f(this,wr))}else f(this,jt).call(this,r)}}$e=new WeakMap,fn=new WeakMap,Ne=new WeakMap,Vn=new WeakMap,Pt=new WeakMap,jt=new WeakMap,wr=new WeakMap;function Fe(n,e,...t){var r=new gi(n);Kt(()=>{const i=e()??null;r.ensure(i,i&&(s=>i(s,...t)))},Wn)}function W(n,e,t=!1){var r=new gi(n),i=t?Wn:0;function s(o,l){r.ensure(o,l)}Kt(()=>{var o=!1;e((l,a=0)=>{o=!0,s(a,l)}),o||s(-1,null)},i)}const zl=Symbol("NaN");function Tl(n,e,t){var r=new gi(n);Kt(()=>{var i=e();i!==i&&(i=zl),r.ensure(i,t)})}function vr(n,e){return e}function Bl(n,e,t){for(var r=[],i=e.length,s,o=e.length,l=0;l<i;l++){let g=e[l];Un(g,()=>{if(s){if(s.pending.delete(g),s.done.add(g),s.pending.size===0){var _=n.outrogroups;$r(n,pr(s.done)),_.delete(s),_.size===0&&(n.outrogroups=null)}}else o-=1},!1)}if(o===0){var a=r.length===0&&t!==null&&n.pending.size===0;if(a){var d=t,u=d.parentNode;sl(u),u.append(d),n.items.clear()}$r(n,e,!a)}else s={pending:new Set(e),done:new Set},(n.outrogroups??(n.outrogroups=new Set)).add(s)}function $r(n,e,t=!0){var r;if(n.pending.size>0){r=new Set;for(const o of n.pending.values())for(const l of o)r.add(n.items.get(l).e)}for(var i=0;i<e.length;i++){var s=e[i];if(r!=null&&r.has(s)){s.f|=gn;const o=document.createDocumentFragment();hi(s,o)}else Be(e[i],t)}}var Di;function Ge(n,e,t,r,i,s=null){var o=n,l=new Map,a=(e&4)!==0;if(a){var d=n;o=d.appendChild(Sn())}var u=null,g=ds(()=>{var b=t();return Ut(b)?b:b==null?[]:pr(b)}),_=new Map,v=!0;function p(b){if((k.effect.f&me)===0){k.pending.delete(b);var M=c(g);k.fallback=u,Ll(k,M,o,e,r),u!==null&&(M.length===0?(u.f&gn)===0?ur(u):(u.f^=gn,yt(u,null,o)):Un(u,()=>{u=null}))}}function h(b){k.pending.delete(b)}var w=Kt(()=>{for(var b=c(g),M=b.length,E=new Set,y=j,A=Ss(),x=0;x<M;x+=1){var T=b[x],N=r(T,x),G=v?null:l.get(N);G?(G.v&&dt(G.v,T),G.i&&dt(G.i,x),A&&y.unskip_effect(G.e)):(G=Dl(l,v?o:Di??(Di=Sn()),T,N,x,i,e,t),v||(G.e.f|=gn),l.set(N,G)),E.add(N)}if(M===0&&s&&!u&&(v?u=Ue(()=>s(o)):(u=Ue(()=>s(Di??(Di=Sn()))),u.f|=gn)),M>E.size&&zo(),!v)if(_.set(y,E),A){for(const[ne,oe]of l)E.has(ne)||y.skip_effect(oe.e);y.oncommit(p),y.ondiscard(h)}else p(y);c(g)}),k={effect:w,items:l,pending:_,outrogroups:null,fallback:u};v=!1}function pt(n){for(;n!==null&&(n.f&ln)===0;)n=n.next;return n}function Ll(n,e,t,r,i){var G,ne,oe,le,he,ge,F,re,Ie;var s=(r&8)!==0,o=e.length,l=n.items,a=pt(n.effect.first),d,u=null,g,_=[],v=[],p,h,w,k;if(s)for(k=0;k<o;k+=1)p=e[k],h=i(p,k),w=l.get(h).e,(w.f&gn)===0&&((ne=(G=w.nodes)==null?void 0:G.a)==null||ne.measure(),(g??(g=new Set)).add(w));for(k=0;k<o;k+=1){if(p=e[k],h=i(p,k),w=l.get(h).e,n.outrogroups!==null)for(const pe of n.outrogroups)pe.pending.delete(w),pe.done.delete(w);if((w.f&Ae)!==0&&(ur(w),s&&((le=(oe=w.nodes)==null?void 0:oe.a)==null||le.unfix(),(g??(g=new Set)).delete(w))),(w.f&gn)!==0)if(w.f^=gn,w===a)yt(w,null,t);else{var b=u?u.next:a;w===n.effect.last&&(n.effect.last=w.prev),w.prev&&(w.prev.next=w.next),w.next&&(w.next.prev=w.prev),In(n,u,w),In(n,w,b),yt(w,b,t),u=w,_=[],v=[],a=pt(u.next);continue}if(w!==a){if(d!==void 0&&d.has(w)){if(_.length<v.length){var M=v[0],E;u=M.prev;var y=_[0],A=_[_.length-1];for(E=0;E<_.length;E+=1)yt(_[E],M,t);for(E=0;E<v.length;E+=1)d.delete(v[E]);In(n,y.prev,A.next),In(n,u,y),In(n,A,M),a=M,u=A,k-=1,_=[],v=[]}else d.delete(w),yt(w,a,t),In(n,w.prev,w.next),In(n,w,u===null?n.effect.first:u.next),In(n,u,w),u=w;continue}for(_=[],v=[];a!==null&&a!==w;)(d??(d=new Set)).add(a),v.push(a),a=pt(a.next);if(a===null)continue}(w.f&gn)===0&&_.push(w),u=w,a=pt(w.next)}if(n.outrogroups!==null){for(const pe of n.outrogroups)pe.pending.size===0&&($r(n,pr(pe.done)),(he=n.outrogroups)==null||he.delete(pe));n.outrogroups.size===0&&(n.outrogroups=null)}if(a!==null||d!==void 0){var x=[];if(d!==void 0)for(w of d)(w.f&Ae)===0&&x.push(w);for(;a!==null;)(a.f&Ae)===0&&a!==n.fallback&&x.push(a),a=pt(a.next);var T=x.length;if(T>0){var N=(r&4)!==0&&o===0?t:null;if(s){for(k=0;k<T;k+=1)(F=(ge=x[k].nodes)==null?void 0:ge.a)==null||F.measure();for(k=0;k<T;k+=1)(Ie=(re=x[k].nodes)==null?void 0:re.a)==null||Ie.fix()}Bl(n,x,N)}}s&&yn(()=>{var pe,Jt;if(g!==void 0)for(w of g)(Jt=(pe=w.nodes)==null?void 0:pe.a)==null||Jt.apply()})}function Dl(n,e,t,r,i,s,o,l){var a=(o&1)!==0?(o&16)===0?tl(t,!1,!1):Kn(t):null,d=(o&2)!==0?Kn(i):null;return{v:a,i:d,e:Ue(()=>(s(e,a??t,d??i,l),()=>{n.delete(r)}))}}function yt(n,e,t){if(n.nodes)for(var r=n.nodes.start,i=n.nodes.end,s=e&&(e.f&gn)===0?e.nodes.start:t;r!==null;){var o=Xt(r);if(s.before(r),r===i)return;r=o}}function In(n,e,t){e===null?n.effect.first=t:e.next=t,t===null?n.effect.last=e:t.prev=e}function Pl(n,e,t=!1,r=!1,i=!1,s=!1){var o=n,l="";if(t)var a=n;B(()=>{var d=U;if(l!==(l=e()??"")){if(t){d.nodes=null,a.innerHTML=l,l!==""&&vt(Mn(a),a.lastChild);return}if(d.nodes!==null&&(zs(d.nodes.start,d.nodes.end),d.nodes=null),l!==""){var u=r?ho:i?go:void 0,g=ci(r?"svg":i?"math":"template",u);g.innerHTML=l;var _=r||i?g:g.content;if(vt(Mn(_),_.lastChild),r||i)for(;Mn(_);)o.before(Mn(_));else o.before(_)}}})}function Le(n,e){ui(()=>{var i,s;n=((s=(i=U==null?void 0:U.parent)==null?void 0:i.nodes)==null?void 0:s.start)??n;var t=n.getRootNode(),r=t.host?t:t.head??t.ownerDocument.head;if(!r.querySelector("#"+e.hash)){const o=ci("style");o.id=e.hash,o.textContent=e.code,r.appendChild(o)}})}function Bt(n,e,t){ui(()=>{var r=br(()=>e(n,t==null?void 0:t())||{});if(t&&(r!=null&&r.update)){var i=!1,s={};vi(()=>{var o=t();hl(o),i&&rs(s,o)&&(s=o,r.update(o))}),i=!0}if(r!=null&&r.destroy)return()=>r.destroy()})}const Pi=[...` 	
\r\f \v\uFEFF`];function jl(n,e,t){var r=n==null?"":""+n;if(t){for(var i of Object.keys(t))if(t[i])r=r?r+" "+i:i;else if(r.length)for(var s=i.length,o=0;(o=r.indexOf(i,o))>=0;){var l=o+s;(o===0||Pi.includes(r[o-1]))&&(l===r.length||Pi.includes(r[l]))?r=(o===0?"":r.substring(0,o))+r.substring(l+1):o=l}}return r===""?null:r}function ji(n,e=!1){var t=e?" !important;":";",r="";for(var i of Object.keys(n)){var s=n[i];s!=null&&s!==""&&(r+=" "+i+": "+s+t)}return r}function Ol(n,e){if(e){var t="",r,i;return Array.isArray(e)?(r=e[0],i=e[1]):r=e,r&&(t+=ji(r)),i&&(t+=ji(i,!0)),t=t.trim(),t===""?null:t}return String(n)}function Ke(n,e,t,r,i,s){var o=n[Nr];if(o!==t||o===void 0){var l=jl(t,r,s);l==null?n.removeAttribute("class"):n.className=l,n[Nr]=t}else if(s&&i!==s)for(var a in s){var d=!!s[a];(i==null||d!==!!i[a])&&n.classList.toggle(a,d)}return s}function Tr(n,e={},t,r){for(var i in t){var s=t[i];e[i]!==s&&(t[i]==null?n.style.removeProperty(i):n.style.setProperty(i,s,r))}}function wt(n,e,t,r){var i=n[qr];if(i!==e){var s=Ol(e,r);s==null?n.removeAttribute("style"):n.style.cssText=s,n[qr]=e}else r&&(Array.isArray(r)?(Tr(n,t==null?void 0:t[0],r[0]),Tr(n,t==null?void 0:t[1],r[1],"important")):Tr(n,t,r));return r}function Nl(n,e){e?n.hasAttribute("selected")||n.setAttribute("selected",""):n.removeAttribute("selected")}function ql(n,e){var t=n.__defaultValue,r=n.multiple,i=r?t??[]:null;if(!(r&&!Ut(i))){n.selectedIndex;for(var s of n.options){var o=ei(s);Nl(s,r?i.includes(o):ks(o,t))}}}function Gs(n,e,t=!1){if(n.multiple){if(e==null)return;if(!Ut(e))return So();for(var r of n.options)r.selected=e.includes(ei(r));return}for(r of n.options){var i=ei(r);if(ks(i,e)){r.selected=!0;return}}(!t||e!==void 0)&&(n.selectedIndex=-1)}function Rl(n){var e=new MutationObserver(t=>{t.every(Fl)||("__defaultValue"in n&&ql(n),"__value"in n&&Gs(n,n.__value))});e.observe(n,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),Es(()=>{e.disconnect()})}function ei(n){return"__value"in n?n.__value:n.value}function Fl(n){if(n.target.closest("selectedcontent")!==null)return!0;if(n.type==="childList"){var e=[...n.addedNodes,...n.removedNodes];return e.length>0&&e.every(t=>t.nodeName==="SELECTEDCONTENT")}return!1}const Hl=Symbol("is custom element"),Vl=Symbol("is html"),Yl=Mo?"progress":"PROGRESS";function Ws(n,e){var t=Xs(n);t.value===(t.value=e??void 0)||n.value===e&&(e!==0||n.nodeName!==Yl)||(n.value=e??"")}function X(n,e,t,r){var i=Xs(n),s=i[e];s!==(i[e]=t)&&(e==="loading"&&(n[yo]=t),t==null?n.removeAttribute(e):typeof t!="string"&&Ul(n).has(e)?n[e]=t:n.setAttribute(e,t))}function Xs(n){return n[rr]??(n[rr]={[Hl]:n.nodeName.includes("-"),[Vl]:n.namespaceURI===Xi})}var Oi=new Map;function Ul(n){var e=n.getAttribute("is")||n.nodeName,t=Oi.get(e);if(t)return t;Oi.set(e,t=new Set);for(var r,i=n,s=Element.prototype;s!==i;){r=Ji(i);for(var o in r)r[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&t.add(o);i=_r(i)}return t}function Br(n,e){return n===e||(n==null?void 0:n[Cn])===e}function ft(n=ii(),e,t,r){var i=be.r,s=U;return ui(()=>{var o,l;return vi(()=>{o=l,l=[],br(()=>{Br(t(...l),n)||(e(n,...l),o&&Br(t(...o),n)&&e(null,...o))})}),()=>{let a=s;for(;a!==i&&a.parent!==null&&a.parent.f&It;)a=a.parent;const d=()=>{l&&Br(t(...l),n)&&e(null,...l)},u=a.teardown;a.teardown=()=>{d(),u==null||u()}}}),n}let tr=!1;function Gl(n){var e=tr;try{return tr=!1,[n(),tr]}finally{tr=e}}function rn(n,e,t,r){var E;var i=!0,s=(t&8)!==0,o=(t&16)!==0,l=r,a=!0,d=void 0,u=()=>o&&i?(d??(d=zt(r)),c(d)):(a&&(a=!1,l=o?br(r):r),l);let g;if(s){var _=Cn in n||ko in n;g=((E=Qn(n,e))==null?void 0:E.set)??(_&&e in n?y=>n[e]=y:void 0)}var v,p=!1;s?[v,p]=Gl(()=>n[e]):v=n[e],v===void 0&&r!==void 0&&(v=u(),g&&(Po(),g(v)));var h;if(h=()=>{var y=n[e];return y===void 0?u():(a=!0,y)},(t&4)===0)return h;if(g){var w=n.$$legacy;return(function(y,A){return arguments.length>0?((!A||w||p)&&g(A?h():y),y):h()})}var k=!1,b=((t&1)!==0?zt:ds)(()=>(k=!1,h()));s&&c(b);var M=U;return(function(y,A){if(arguments.length>0){const x=A?c(b):s?nn(y):y;return O(b,x),k=!0,l!==void 0&&(l=x),y}return En&&k||(M.f&me)!==0?b.v:c(b)})}function Ks(n){return(e,t)=>{let r=t.onchange;const i=nn({...t,onchange:o=>{i.value=o,r(o)}}),s=Ys(n,{target:e,props:i});return{update({onchange:o,...l}){o&&(r=o),Object.assign(i,l)},destroy(){Us(s)}}}}const Wl="5";var Gi;typeof window<"u"&&((Gi=window.__svelte??(window.__svelte={})).v??(Gi.v=new Set)).add(Wl);const Xl={actions:"Actions",back:"Back",breadcrumbs:"Breadcrumbs",cantMove:"It can't move",changed:"Changed",clear:"Clear",close:"Close",collapse:"Collapse",date:"Date",disabled:"Disabled",dragOrClick:"drag, or click for the moves",done:"Done",duplicate:"Duplicate",expand:"Expand",forward:"Forward",hasErrors:"Has errors",hide:"Hide it on the website",monthAfter:"Month after",monthBefore:"Month before",move:"Move",moveDown:"Move down",moveEarlier:"Move earlier",moveIn:"Move under the one above",moveLater:"Move later",moveOut:"Move out a level",moveUp:"Move up",noDate:"No date",nothingFound:"Nothing found",nothingYet:"Nothing yet",pickDay:"Pick a day",pickMonth:"Pick a month",pickYear:"Pick a year",remove:"Remove",resize:"Drag to change the width, double click for the default",search:"Search",show:"Show it on the website",time:"time",today:"Today"},Lr=Symbol.for("zipp-ui"),Kl=nn({texts:Xl,locale:void 0}),Dr=globalThis,wn=Dr[Lr]??(Dr[Lr]=Kl),Zl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Jl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Ql=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$l=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ea=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ta=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sa=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,oa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,la=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,aa=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,va=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,_a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ea=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,za=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ta=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,La=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Pa=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Oa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,qa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Va=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Za=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Qa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ec=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,nc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,tc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,rc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ic=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,oc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,lc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ac=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,cc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,dc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,uc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,vc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,fc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,hc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,gc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,wc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,pc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,_c=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,mc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`;var bc=I('<span class="icon svelte-oymxvd"></span>');const xc={hash:"svelte-oymxvd",code:".icon.svelte-oymxvd {display:inline-flex;flex-shrink:0;}.icon.svelte-oymxvd svg {width:100%;height:100%;stroke-width:1.75;}"};function on(n,e){Me(e,!0),Le(n,xc);const t=Object.assign({"./icons/add-image.svg":Zl,"./icons/add-video.svg":Jl,"./icons/add.svg":Ql,"./icons/alert.svg":$l,"./icons/all_pages.svg":ea,"./icons/arrow-down.svg":na,"./icons/arrow-left.svg":ta,"./icons/arrow-right.svg":ra,"./icons/arrow-up.svg":ia,"./icons/block.svg":sa,"./icons/bold.svg":oa,"./icons/button.svg":la,"./icons/calendar.svg":aa,"./icons/channel.svg":ca,"./icons/check.svg":da,"./icons/chevron-down.svg":ua,"./icons/chevron-left.svg":va,"./icons/chevron-right.svg":fa,"./icons/chevrons-up-down.svg":ha,"./icons/close.svg":ga,"./icons/copy-url.svg":wa,"./icons/copy.svg":pa,"./icons/crop.svg":_a,"./icons/dashboard.svg":ma,"./icons/default.svg":ba,"./icons/delete.svg":xa,"./icons/desktop.svg":ka,"./icons/doc.svg":ya,"./icons/download.svg":Ma,"./icons/edit.svg":Ca,"./icons/entries.svg":Sa,"./icons/external.svg":Ea,"./icons/file.svg":Aa,"./icons/flip-h.svg":Ia,"./icons/flip-v.svg":za,"./icons/focal.svg":Ta,"./icons/folder-plus.svg":Ba,"./icons/folder.svg":La,"./icons/globe.svg":Da,"./icons/graphql.svg":Pa,"./icons/grip.svg":ja,"./icons/help.svg":Oa,"./icons/hide.svg":Na,"./icons/indent.svg":qa,"./icons/info.svg":Ra,"./icons/italic.svg":Fa,"./icons/link.svg":Ha,"./icons/live-preview.svg":Va,"./icons/logout.svg":Ya,"./icons/mail.svg":Ua,"./icons/mobile.svg":Ga,"./icons/more.svg":Wa,"./icons/move.svg":Xa,"./icons/numbered.svg":Ka,"./icons/outdent.svg":Za,"./icons/page.svg":Ja,"./icons/page_cat.svg":Qa,"./icons/phone.svg":$a,"./icons/play.svg":ec,"./icons/reload.svg":nc,"./icons/replace.svg":tc,"./icons/reset.svg":rc,"./icons/rotate-left.svg":ic,"./icons/rotate-right.svg":sc,"./icons/save.svg":oc,"./icons/search.svg":lc,"./icons/settings.svg":ac,"./icons/sidebar.svg":cc,"./icons/single.svg":dc,"./icons/size.svg":uc,"./icons/sparkles.svg":vc,"./icons/structure.svg":fc,"./icons/tablet.svg":hc,"./icons/translate.svg":gc,"./icons/unlink.svg":wc,"./icons/upload.svg":pc,"./icons/users.svg":_c,"./icons/view.svg":mc});let r=rn(e,"size",3,16);var i=bc();let s;Pl(i,()=>t[`./icons/${e.name}.svg`],!0),B(()=>s=wt(i,"",s,{width:`${r()/16}rem`,height:`${r()/16}rem`})),S(n,i),Ce()}const Pr=Symbol.for("zipp-ui-layers"),jr=globalThis,Zs=jr[Pr]??(jr[Pr]={stack:[],onkeydown(n){const e=Zs.stack;n.key!=="Escape"||!e.length||document.querySelector("dialog:modal")||(n.preventDefault(),e[e.length-1]())}});function kc(n){const{stack:e,onkeydown:t}=Zs;return e.length||window.addEventListener("keydown",t),e.push(n),()=>{const r=e.indexOf(n);r>=0&&e.splice(r,1),e.length||window.removeEventListener("keydown",t)}}function et(n,e){const t=document.activeElement;let r=e;const i=kc(()=>r());return{update(s){r=s},destroy(){i(),t!=null&&t.isConnected&&t.focus()}}}const yc='a[href], button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]';function Mc(n,e){if(n.key!=="Tab")return;const t=[...e.querySelectorAll(yc)].filter(o=>o.offsetParent!==null&&!o.matches('[tabindex="-1"]'));if(!t.length)return;const[r,i]=[t[0],t[t.length-1]],s=document.activeElement;n.shiftKey&&(s===r||s===e)?(n.preventDefault(),i.focus()):!n.shiftKey&&s===i&&(n.preventDefault(),r.focus())}function Cc(n,e,t,r=!1){if(t)switch(n){case"ArrowDown":return r?(e+1)%t:Math.min(e+1,t-1);case"ArrowUp":return r?e<=0?t-1:e-1:Math.max(e-1,0);case"Home":return 0;case"End":return t-1}}var Sc=I('<div class="popup svelte-1hgkc3f" tabindex="-1"><!></div>'),Ec=I('<div class="popover svelte-1hgkc3f"><!> <!></div>');const Ac={hash:"svelte-1hgkc3f",code:".popover.svelte-1hgkc3f {position:relative;}.popup.svelte-1hgkc3f {position:fixed;z-index:calc(var(--z-overlay) + 1);display:flex;flex-direction:column;gap:2px;max-width:calc(100vw - 1rem);}"};function wi(n,e){Me(e,!0),Le(n,Ac);let t=rn(e,"open",15,!1),r=rn(e,"at",15,null),i=rn(e,"align",3,"start"),s=rn(e,"menu",3,!1),o=$(void 0),l=$(void 0);const a=()=>t(!t()),d=()=>t(!1);Yn(()=>{t()||r(null)});const u=8,g=4;function _(b,M){let E=M;const y=()=>{const x=b.getBoundingClientRect();let T,N;if(E)T=E.x,N=E.y+x.height>innerHeight-u?E.y-x.height:E.y;else{const G=c(o).getBoundingClientRect();T=i()==="end"?G.right-x.width:i()==="center"?G.left+G.width/2-x.width/2:G.left;const ne=G.bottom+g,oe=G.top-g-x.height;N=ne+x.height>innerHeight-u&&oe>=u?oe:ne}T=Math.min(T,innerWidth-x.width-u),b.style.left=`${Math.max(T,u)}px`,b.style.top=`${Math.max(N,u)}px`};y(),addEventListener("scroll",y,!0),addEventListener("resize",y);const A=new ResizeObserver(y);return A.observe(b),{update(x){E=x,y()},destroy(){removeEventListener("scroll",y,!0),removeEventListener("resize",y),A.disconnect()}}}Yn(()=>{if(!t())return;const b=M=>{var E;(E=c(o))!=null&&E.contains(M.target)||d()};return window.addEventListener("pointerdown",b),()=>window.removeEventListener("pointerdown",b)}),Yn(()=>{var M,E;if(!t())return;const b=s()?"[role=menuitem]":"input";(E=(M=c(l))==null?void 0:M.querySelector(b))==null||E.focus()});function v(b){if(!s()||!c(l))return;const M=[...c(l).querySelectorAll("[role=menuitem]")],E=M.indexOf(document.activeElement),y=Cc(b.key,E,M.length,!0);y!==void 0&&(b.preventDefault(),M[y].focus())}var p=Ec(),h=L(p);Fe(h,()=>e.trigger??lt,()=>({toggle:a,open:t()}));var w=D(h,2);{var k=b=>{var M=Sc();let E;var y=L(M);Fe(y,()=>e.children),ft(M,A=>O(l,A),()=>c(l)),Bt(M,(A,x)=>et==null?void 0:et(A,x),()=>d),Bt(M,(A,x)=>_==null?void 0:_(A,x),r),B(()=>{X(M,"role",s()?"menu":"dialog"),X(M,"aria-label",e.label),E=wt(M,"",E,{width:e.width})}),Q("keydown",M,v),S(b,M)};W(w,b=>{t()&&b(k)})}ft(p,b=>O(o,b),()=>c(o)),S(n,p),Ce()}an(["keydown"]);var Ic=I('<button type="button" class="icon drag-handle svelte-nnt03w" data-drag-handle=""><!></button>'),zc=I('<button type="button" class="popup-item" role="menuitem"><!> </button>'),Tc=I('<span class="popup-item note"> </span>');const Bc={hash:"svelte-nnt03w",code:`.drag-handle.svelte-nnt03w {cursor:grab;color:var(--subtle);}

@media (hover: hover) {.drag-handle.svelte-nnt03w:hover {color:var(--text);}
}`};function Js(n,e){Me(e,!0),Le(n,Bc);const t=J(()=>e.label??wn.texts.move),r=()=>typeof e.index=="function"?e.index():e.index;let i=$(!1);const s=[["up","moveUp","arrow-up"],["down","moveDown","arrow-down"],["out","moveOut","outdent"],["in","moveIn","indent"]],o={up:["moveEarlier","arrow-left"],down:["moveLater","arrow-right"]},l={ArrowUp:"up",ArrowDown:"down",ArrowLeft:"out",ArrowRight:"in"},a=J(()=>c(i)?s.filter(([h])=>e.sortable.stepDrop(r(),h)).map(([h,w,k])=>e.sortable.grid()&&o[h]?[h,...o[h]]:[h,w,k]):[]),d={ArrowLeft:"up",ArrowRight:"down"};function u(h){const w=e.sortable.grid()&&d[h.key]||l[h.key];w&&(h.preventDefault(),e.sortable.step(r(),w))}var g=Xe(),_=ae(g);{var v=h=>{wi(h,{width:"13rem",get label(){return c(t)},menu:!0,get open(){return c(i)},set open(k){O(i,k,!0)},trigger:k=>{var b=Ic();wt(b,"",{},{"touch-action":"none"});var M=L(b);on(M,{name:"grip",size:15}),B(()=>{X(b,"title",`${c(t)??""}: ${wn.texts.dragOrClick??""}`),X(b,"aria-label",c(t)),X(b,"aria-expanded",c(i))}),Q("pointerdown",b,E=>e.sortable.start(E,r())),Q("click",b,()=>!e.sortable.dragged&&O(i,!c(i))),Q("keydown",b,u),S(k,b)},children:(k,b)=>{var M=Xe(),E=ae(M);Ge(E,17,()=>c(a),([y,A,x])=>y,(y,A)=>{var x=J(()=>mo(c(A),3));let T=()=>c(x)[0],N=()=>c(x)[1],G=()=>c(x)[2];var ne=zc(),oe=L(ne);on(oe,{get name(){return G()},size:14});var le=D(oe);B(()=>Y(le,` ${wn.texts[N()]??""}`)),Q("click",ne,()=>{O(i,!1),e.sortable.step(r(),T())}),S(y,ne)},y=>{var A=Tc(),x=te(A,!0);B(()=>Y(x,wn.texts.cantMove)),S(y,A)}),S(k,M)},$$slots:{trigger:!0,default:!0}})},p=J(()=>e.sortable.movable());W(_,h=>{c(p)&&h(v)})}S(n,g),Ce()}an(["pointerdown","click","keydown"]);var Lc=I('<span class="info svelte-l93724"><!></span>'),Dc=I('<label class="svelte-l93724"><span> </span> <!></label>'),Pc=I('<div class="field-row svelte-l93724"><!> <div><!></div></div>');const jc={hash:"svelte-l93724",code:`.field-row.svelte-l93724 {display:grid;grid-template-columns:8.125rem minmax(0, 1fr);gap:1.25rem;align-items:center;min-width:0;}label.svelte-l93724 {align-self:start;display:flex;flex-direction:column;gap:1px;padding-top:0.4375rem;font-size:var(--fs-md);font-weight:600;color:var(--text);}.info.svelte-l93724 {color:var(--muted);font-size:var(--fs-sm);font-weight:400;}.value.svelte-l93724 {min-width:0;}.alone.svelte-l93724 {grid-column:2;}

@container (width < 26rem) {.field-row.svelte-l93724 {display:flex;flex-direction:column;align-items:stretch;gap:0.375rem;}label.svelte-l93724 {padding-top:0;}
}`};function Oc(n,e){Le(n,jc);let t=rn(e,"label",3,"");var r=Pc(),i=L(r);{var s=d=>{var u=Dc(),g=L(u),_=te(g,!0),v=D(g,2);{var p=h=>{var w=Lc(),k=L(w);Fe(k,()=>e.info),S(h,w)};W(v,h=>{e.info&&h(p)})}B(()=>{X(u,"for",e.for),Y(_,t())}),S(d,u)};W(i,d=>{t()&&d(s)})}var o=D(i,2);let l;var a=L(o);Fe(a,()=>e.children),B(()=>l=Ke(o,1,"value svelte-l93724",null,l,{alone:!t()})),S(n,r)}function Qs(n){const e=Symbol(n);return{provide:t=>void Yo(e,t),get:()=>Vo(e)}}const Nc=Qs("page-lead"),$s=Qs("flyout-bar");var qc=I('<div class="page svelte-22zuw3"><!></div> <button type="button" class="icon"><!></button>',1),Rc=I('<div class="actions svelte-22zuw3"><!></div>'),Fc=I('<button type="button" class="primary"><!> </button>'),Hc=I('<strong class="truncate"> </strong> <!>',1),Vc=I('<div class="backdrop" role="presentation"></div> <div class="flyout svelte-22zuw3" role="dialog" aria-modal="true" tabindex="-1"><div class="bar svelte-22zuw3"><!></div> <div><!></div></div>',1);const Yc={hash:"svelte-22zuw3",code:`.flyout.svelte-22zuw3 {position:fixed;top:0;right:0;bottom:0;z-index:var(--z-overlay);display:flex;flex-direction:column;width:min(65rem, 92vw);background:var(--bg);box-shadow:var(--shadow-flyout);outline:none;
  animation: svelte-22zuw3-slide 0.18s ease-out;}

@media (max-width: 799px) {.flyout.svelte-22zuw3 {width:100vw;}
}
@keyframes svelte-22zuw3-slide {
  from {
    transform: translateX(2.5rem);
    opacity: 0;
  }
}.bar.svelte-22zuw3 {position:relative;z-index:var(--z-bar);display:flex;align-items:center;justify-content:space-between;gap:0.75rem;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.actions.svelte-22zuw3 {display:flex;gap:0.5rem;}

/* the page inside scrolls, its top bar sticks to this */.body.svelte-22zuw3 {flex:1;min-height:0;padding:0 var(--page-pad) var(--page-pad);overflow-y:auto;}.page-in.svelte-22zuw3 {padding-top:1.25rem;}.page.svelte-22zuw3 {display:flex;align-items:center;gap:0.5rem;flex:1;flex-wrap:wrap;min-width:0;}`};function Uc(n,e){Me(e,!0),Le(n,Yc);const t=nn({content:null});$s.provide(t);let r=$(void 0);Yn(()=>{var h;return(h=c(r))==null?void 0:h.focus()});function i(h){c(r)&&Mc(h,c(r))}var s=Vc(),o=ae(s),l=D(o,2),a=L(l),d=L(a);{var u=h=>{var w=qc(),k=ae(w),b=L(k);Fe(b,()=>t.content);var M=D(k,2),E=L(M);on(E,{name:"close"}),B(()=>{X(M,"title",wn.texts.close),X(M,"aria-label",wn.texts.close)}),Q("click",M,function(...y){var A;(A=e.onclose)==null||A.apply(this,y)}),S(h,w)},g=h=>{var w=Hc(),k=ae(w),b=te(k,!0),M=D(k,2);{var E=A=>{var x=Rc(),T=L(x);Fe(T,()=>e.actions),S(A,x)},y=A=>{var x=Fc(),T=L(x);on(T,{name:"close",size:14});var N=D(T);B(()=>Y(N,` ${wn.texts.done??""}`)),Q("click",x,function(...G){var ne;(ne=e.onclose)==null||ne.apply(this,G)}),S(A,x)};W(M,A=>{e.actions?A(E):A(y,-1)})}B(()=>Y(b,e.title)),S(h,w)};W(d,h=>{t.content?h(u):h(g,-1)})}var _=D(a,2);let v;var p=L(_);Fe(p,()=>e.children),ft(l,h=>O(r,h),()=>c(r)),Bt(l,(h,w)=>et==null?void 0:et(h,w),()=>e.onclose),B(()=>{X(l,"aria-label",e.title),v=Ke(_,1,"body svelte-22zuw3",null,v,{"page-in":!!t.content})}),Q("click",o,function(...h){var w;(w=e.onclose)==null||w.apply(this,h)}),Q("keydown",l,i),S(n,s),Ce()}an(["click","keydown"]);function Gc(n){return n.defaultPrevented||n.target.closest("input, textarea, [contenteditable]")?!1:(n.preventDefault(),n.stopPropagation(),!0)}function Wc(n){return n.button===2||n.clientX?{x:n.clientX,y:n.clientY}:null}var Xc=I('<a class="popup-item svelte-60nrgv" role="menuitem"><!> </a>'),Kc=I('<button type="button" role="menuitem"><!> </button>');const Zc={hash:"svelte-60nrgv",code:".popup-item.svelte-60nrgv {min-width:8.625rem;white-space:nowrap;}"};function Jc(n,e){Me(e,!0),Le(n,Zc);var t=Xe(),r=ae(t);Ge(r,17,()=>e.items,vr,(i,s)=>{var o=Xe(),l=ae(o);{var a=u=>{var g=Xc(),_=L(g);on(_,{get name(){return c(s).icon},size:14});var v=D(_);B(p=>{X(g,"href",c(s).href),X(g,"target",p),X(g,"download",c(s).download),Y(v,` ${c(s).label??""}`)},[()=>c(s).href.startsWith("/admin")||c(s).download?void 0:"_blank"]),Q("click",g,function(...p){var h;(h=e.close)==null||h.apply(this,p)}),S(u,g)},d=u=>{var g=Kc();let _;var v=L(g);on(v,{get name(){return c(s).icon},size:14});var p=D(v);B(()=>{_=Ke(g,1,"popup-item svelte-60nrgv",null,_,{danger:c(s).danger}),Y(p,` ${c(s).label??""}`)}),Q("click",g,()=>{var h,w;e.close(),(w=(h=c(s)).onclick)==null||w.call(h)}),S(u,g)};W(l,u=>{c(s).href?u(a):u(d,-1)})}S(i,o)}),S(n,t),Ce()}an(["click"]);var Qc=I('<button type="button" class="icon"><!></button>');function eo(n,e){Me(e,!0);const t=J(()=>e.label??wn.texts.actions);let r=$(!1),i=$(null),s=$(void 0);Yn(()=>{var u;const d=(u=c(s))==null?void 0:u.closest("[data-menu-area]");if(d)return wl(d,"contextmenu",g=>{Gc(g)&&(O(i,Wc(g),!0),O(r,!0))})});var o=Xe(),l=ae(o);{var a=d=>{wi(d,{align:"end",get label(){return c(t)},menu:!0,get open(){return c(r)},set open(g){O(r,g,!0)},get at(){return c(i)},set at(g){O(i,g,!0)},trigger:(g,_)=>{let v=()=>(_==null?void 0:_()).toggle,p=()=>(_==null?void 0:_()).open;var h=Qc(),w=L(h);on(w,{name:"more"}),ft(h,k=>O(s,k),()=>c(s)),B(()=>{X(h,"title",c(t)),X(h,"aria-label",c(t)),X(h,"aria-expanded",p())}),Q("click",h,function(...k){var b;(b=v())==null||b.apply(this,k)}),S(g,h)},children:(g,_)=>{Jc(g,{get items(){return e.items},close:()=>O(r,!1)})},$$slots:{trigger:!0,default:!0}})};W(l,d=>{e.items.length&&d(a)})}S(n,o),Ce()}an(["click"]);var $c=I('<button type="button"><!></button>'),ed=I('<div class="segmented svelte-49ceuw"></div>');const nd={hash:"svelte-49ceuw",code:`.segmented.svelte-49ceuw {display:inline-flex;flex-wrap:wrap;gap:2px;padding:3px;background:var(--hover);border:1px solid var(--line);border-radius:var(--radius-lg);}.segmented.svelte-49ceuw button:where(.svelte-49ceuw) {gap:0.375rem;padding:0.3125rem 0.75rem;border:none;border-radius:var(--radius);background:none;color:var(--muted);}.segmented.svelte-49ceuw button.on:where(.svelte-49ceuw) {background:var(--surface);color:var(--text);box-shadow:var(--shadow);}
@media (hover: hover) {.segmented.svelte-49ceuw button:where(.svelte-49ceuw):hover:not(.on):not(:disabled) {color:var(--text);background:none;}
}`};function td(n,e){Me(e,!0),Le(n,nd);let t=rn(e,"tabs",3,!1);const r=J(()=>e.options.some(o=>o.value===e.value));function i(o){const l={ArrowRight:1,ArrowDown:1,ArrowLeft:-1,ArrowUp:-1}[o.key];if(!l)return;o.preventDefault();const a=e.options.findIndex(g=>g.value===e.value),d=a<0?e.options[l>0?0:e.options.length-1]:e.options[(a+l+e.options.length)%e.options.length];e.onchange(d.value);const u=o.currentTarget.parentElement;qs().then(()=>{var g;return(g=u==null?void 0:u.querySelector('[tabindex="0"]'))==null?void 0:g.focus()})}var s=ed();Ge(s,23,()=>e.options,o=>o.value,(o,l,a)=>{var d=$c();let u;var g=L(d);{var _=p=>{var h=Xe(),w=ae(h);Fe(w,()=>e.item,()=>c(l)),S(p,h)},v=p=>{var h=Vs();B(()=>Y(h,c(l).label)),S(p,h)};W(g,p=>{e.item?p(_):p(v,-1)})}B(()=>{X(d,"role",t()?"tab":"radio"),X(d,"aria-selected",t()?c(l).value===e.value:void 0),X(d,"aria-checked",t()?void 0:c(l).value===e.value),X(d,"tabindex",c(l).value===e.value||c(a)===0&&!c(r)?0:-1),u=Ke(d,1,"svelte-49ceuw",null,u,{on:c(l).value===e.value})}),Q("click",d,()=>e.onchange(c(l).value)),Q("keydown",d,i),S(o,d)}),B(()=>{X(s,"role",t()?"tablist":"radiogroup"),X(s,"aria-label",e.label)}),S(n,s),Ce()}an(["click","keydown"]);var rd=I('<div class="sort-line svelte-183yeqa"></div>');const id={hash:"svelte-183yeqa",code:".sort-line.svelte-183yeqa {position:absolute;z-index:var(--z-raised);border-radius:2px;background:var(--accent);pointer-events:none;}"};function Ni(n,e){Me(e,!0),Le(n,id);var t=Xe(),r=ae(t);{var i=s=>{var o=rd();let l;B(()=>l=wt(o,"",l,{top:`${e.sortable.line.top??""}px`,left:`${e.sortable.line.left??""}px`,width:`${e.sortable.line.width??""}px`,height:`${e.sortable.line.height??""}px`})),S(s,o)};W(r,s=>{e.sortable.line&&s(i)})}S(n,t),Ce()}function Et(n,e){let t=1;for(;e+t<n.length&&n[e+t]>n[e];)t++;return t}function sd(n,e,t,r,i){const s=n[e],o=Et(n,e),l=h=>h<e?h:h-o,a=h=>h+Et(n,h);let d=e-1;for(;d>=0&&n[d]>s;)d--;const u=d>=0&&n[d]===s?d:-1,g=e+o,_=g<n.length&&n[g]===s?g:-1;let v=e-1;for(;v>=0&&n[v]>=s;)v--;const p=Math.max(...n.slice(e,e+o))-s;switch(t){case"up":return u<0?null:{index:l(u),depth:s};case"down":{const h=n.length-o-i,w=_<0?-1:l(a(_));return w<0||w>h?null:{index:w,depth:s}}case"out":return s===0||v<0?null:{index:l(a(v)),depth:s-1};case"in":return u<0||s+1+p>r?null:{index:e,depth:s+1}}}var Ot,Nt;class qi{constructor(e){q(this,Ot,$(null));q(this,Nt,$(null));we(this,"drop",null);we(this,"dragged",!1);this.o=e}get dragging(){return c(f(this,Ot))}set dragging(e){O(f(this,Ot),e,!0)}get line(){return c(f(this,Nt))}set line(e){O(f(this,Nt),e,!0)}rows(){if(this.o.rows)return this.o.rows();const e=this.o.container();return e?[...e.querySelectorAll("[data-sort]")].filter(t=>{var i;const r=(i=t.parentElement)==null?void 0:i.closest("[data-sort]");return!r||!e.contains(r)}):[]}movable(){return this.o.depths().length>1}grid(){var e,t;return((t=(e=this.o).grid)==null?void 0:t.call(e))??!1}moving(e){const t=this.dragging;return t===null?!1:e>=t&&e<t+Et(this.o.depths(),t)}start(e,t){if(e.button!==0)return;const r=e.clientX,i=e.clientY;let s=!1;const o=d=>{if(!s){if(Math.hypot(d.clientX-r,d.clientY-i)<5)return;s=!0,this.dragging=t}d.preventDefault(),this.over(d,t,d.clientX-r),od(this.o.container(),d.clientY)},l=d=>{window.removeEventListener("pointermove",o),window.removeEventListener("pointerup",l),window.removeEventListener("pointercancel",l),window.removeEventListener("keydown",a,!0);const u=this.drop;this.dragging=null,this.line=null,this.drop=null,s&&(this.dragged=!0,setTimeout(()=>this.dragged=!1),u&&d.type==="pointerup"&&this.o.onmove(t,u))},a=d=>{d.key!=="Escape"||!s||(d.preventDefault(),d.stopImmediatePropagation(),this.drop=null,l(new PointerEvent("pointercancel")))};window.addEventListener("pointermove",o),window.addEventListener("pointerup",l),window.addEventListener("pointercancel",l),window.addEventListener("keydown",a,!0)}over(e,t,r){var d,u,g;const i=(d=this.o.container())==null?void 0:d.getBoundingClientRect(),s=this.rows(),o=this.o.depths();if(!i||s.length!==o.length)return;const l=Et(o,t),a=s.filter((_,v)=>v<t||v>=t+l);(g=(u=this.o).grid)!=null&&g.call(u)?this.overGrid(e,i,a):this.overList(e,i,a,o,t,r)}overGrid(e,t,r){var d,u;let i=-1,s=1/0;if(r.forEach((g,_)=>{const v=g.getBoundingClientRect(),p=Math.hypot(e.clientX-(v.left+v.width/2),e.clientY-(v.top+v.height/2));p<s&&([i,s]=[_,p])}),i<0)return;const o=r[i].getBoundingClientRect(),l=e.clientX>o.left+o.width/2,a=r.length-(((u=(d=this.o).keepLast)==null?void 0:u.call(d))??0);this.drop={index:Math.min(i+(l?1:0),a),depth:0},this.line={top:o.top-t.top,left:(l?o.right+3:o.left-5)-t.left,width:2,height:o.height}}overList(e,t,r,i,s,o){var E,y,A,x;const l=Et(i,s),a=i.filter((T,N)=>N<s||N>=s+l),d=T=>T.getBoundingClientRect();let u=r.findIndex(T=>{const N=d(T);return e.clientY<N.top+N.height/2});u<0&&(u=r.length),u=Math.min(u,r.length-(((y=(E=this.o).keepLast)==null?void 0:y.call(E))??0));const g=i.slice(s,s+l),_=Math.max(...g)-i[s],v=this.o.indent??24,p=i[s]+Math.round(o/v),h=Math.min(u>0?a[u-1]+1:0,(((x=(A=this.o).maxDepth)==null?void 0:x.call(A))??0)-_),w=u<r.length?a[u]:0;if(w>h){this.drop=null,this.line=null;return}const k=Math.max(w,Math.min(p,h));this.drop={index:u,depth:k};const b=u<r.length?d(r[u]).top-3:r.length?d(r[r.length-1]).bottom+1:t.top,M=(this.o.inset??0)+k*v;this.line={top:b-t.top,left:M,width:t.width-M-(this.o.inset??0),height:2}}stepDrop(e,t){var r,i,s,o;return sd(this.o.depths(),e,t,((i=(r=this.o).maxDepth)==null?void 0:i.call(r))??0,((o=(s=this.o).keepLast)==null?void 0:o.call(s))??0)}async step(e,t){var s;const r=this.stepDrop(e,t);if(!r)return;await this.o.onmove(e,r),await qs();const i=this.rows()[r.index];(s=i==null?void 0:i.querySelector("[data-drag-handle]"))==null||s.focus()}}Ot=new WeakMap,Nt=new WeakMap;const Ri=48,Fi=12;function od(n,e){const t=ld(n),r=t?t.getBoundingClientRect().top:0,i=t?t.getBoundingClientRect().bottom:innerHeight,s=e<r+Ri?-Fi:e>i-Ri?Fi:0;s&&(t??window).scrollBy(0,s)}function ld(n){for(let e=n==null?void 0:n.parentElement;e;e=e.parentElement){const t=getComputedStyle(e).overflowY;if((t==="auto"||t==="scroll")&&e.scrollHeight>e.clientHeight)return e}return null}const ni="formPage",ad=[25,33,50,67,75,100];function cd(n){const e=[];for(const t of n)t.block.type===ni?e.push({page:t,fields:[]}):e.length?e.at(-1).fields.push(t):e.push({page:null,fields:[t]});return e.length||e.push({page:null,fields:[]}),e}function dd(n,e){return n.flatMap((t,r)=>{const i=t.page??(r>0||n.length>1?e():null);return i?[i,...t.fields]:t.fields})}function qn(n,e){return n&&(n[e]||Object.values(n).find(Boolean))||""}const ud=(n,e)=>{var t;return qn((t=n.page)==null?void 0:t.block.fields.label,e)};function vd(n,e,t,r,i){return n.map((s,o)=>{if(o!==e)return s;const l=s.page??i(),a={...l.block.fields.label,[r]:t},d={...l.block,fields:{...l.block.fields,label:a}};return{...s,page:{...l,block:d}}})}function Hi(n,e,t){const r=[...n],[i]=r.splice(e,1);return r.splice(t,0,i),r}function fr(n,e,t){return n.map((r,i)=>i===e?{...r,fields:t(r.fields)}:r)}function fd(n,e,t,r){const i=n[e].fields[t],s=fr(n,e,o=>o.filter((l,a)=>a!==t));return fr(s,r,o=>[...o,i])}const hd=n=>Math.round((n??100)/100*12),Vi={en:{title:"Submissions",note:"What visitors sent with the forms, the newest first.",allForms:"All forms",form:"Form",date:"Date",site:"Site",content:"Content",none:"No submissions yet.",more:"Show more",delete:"Delete",confirmDelete:"Delete this submission for good?",empty:"–",page:"Page",pages:"Pages",addPage:"Add page",renamePage:"Rename",pageName:"Name of the page",deletePage:"Delete page",confirmDeletePage:"Delete this page and its fields?",moveLeft:"Move left",moveRight:"Move right",movePage:"Move page",addField:"Add field",edit:"Edit",duplicate:"Duplicate",switchOff:"Switch off",switchOn:"Switch on",off:"off",remove:"Delete",moveField:"Move field",toPage:"To {page}",width:"Width",full:"Full",noFields:"No fields on this page yet.",cancel:"Cancel",apply:"Apply",confirmDiscard:"Discard the changes of this field?",fieldsActions:"Field actions",pageActions:"Page actions"},de:{title:"Einsendungen",note:"Was Besucher mit den Formularen geschickt haben, die neusten zuerst.",allForms:"Alle Formulare",form:"Formular",date:"Datum",site:"Website",content:"Inhalt",none:"Noch keine Einsendungen.",more:"Mehr zeigen",delete:"Löschen",confirmDelete:"Diese Einsendung endgültig löschen?",empty:"–",page:"Seite",pages:"Seiten",addPage:"Seite hinzufügen",renamePage:"Umbenennen",pageName:"Name der Seite",deletePage:"Seite löschen",confirmDeletePage:"Diese Seite und ihre Felder löschen?",moveLeft:"Nach links",moveRight:"Nach rechts",movePage:"Seite verschieben",addField:"Feld hinzufügen",edit:"Bearbeiten",duplicate:"Duplizieren",switchOff:"Ausschalten",switchOn:"Einschalten",off:"aus",remove:"Löschen",moveField:"Feld verschieben",toPage:"Auf {page}",width:"Breite",full:"Ganz",noFields:"Noch keine Felder auf dieser Seite.",cancel:"Abbrechen",apply:"Übernehmen",confirmDiscard:"Die Änderungen an diesem Feld verwerfen?",fieldsActions:"Aktionen des Felds",pageActions:"Aktionen der Seite"}};function pi(){return Vi[document.documentElement.lang]??Vi.en}var gd=I('<span class="grip svelte-1rirqtx"><!></span>'),wd=I('<span class="star svelte-1rirqtx">*</span>'),pd=I('<span class="badge"> </span>'),_d=I('<p class="hint truncate svelte-1rirqtx"> </p>'),md=I('<div role="button" tabindex="0" data-menu-area=""><div class="top svelte-1rirqtx"><!> <span class="label truncate svelte-1rirqtx"> <!></span> <span class="type svelte-1rirqtx"> </span> <!> <!></div> <div><span class="truncate"> </span></div> <!></div>');const bd={hash:"svelte-1rirqtx",code:`.field-card.svelte-1rirqtx {display:flex;flex-direction:column;gap:0.375rem;height:100%;padding:0.625rem 0.75rem 0.75rem;background:var(--surface);border:1px solid var(--border);border-radius:var(--radius-lg);cursor:pointer;transition:border-color 0.12s, box-shadow 0.12s;}
@media (hover: hover) {.field-card.svelte-1rirqtx {border-color:var(--accent);}
}.field-card.svelte-1rirqtx:focus-visible {outline:2px solid var(--accent);outline-offset:2px;}.field-card.selected.svelte-1rirqtx {border-color:var(--accent);box-shadow:0 0 0 1px var(--accent);}.field-card.off.svelte-1rirqtx {opacity:0.55;}.top.svelte-1rirqtx {display:flex;align-items:center;gap:0.375rem;min-height:1.75rem;}.grip.svelte-1rirqtx {display:flex;margin-left:-0.375rem;color:var(--muted);}.label.svelte-1rirqtx {flex:1;min-width:0;font-weight:600;font-size:var(--fs-base);}.star.svelte-1rirqtx {margin-left:0.125rem;color:var(--danger);}.type.svelte-1rirqtx {color:var(--muted);font-size:var(--fs-xs);white-space:nowrap;}.preview.svelte-1rirqtx {display:flex;align-items:center;height:2rem;padding:0 0.625rem;color:var(--muted);font-size:var(--fs-md);background:var(--hover);border:1px solid var(--line);border-radius:var(--radius);}.preview.textarea.svelte-1rirqtx {align-items:flex-start;height:4.5rem;padding-top:0.375rem;}.hint.svelte-1rirqtx {margin:0;color:var(--muted);font-size:var(--fs-xs);}`};function xd(n,e){Me(e,!0),Le(n,bd);let t=rn(e,"selected",3,!1),r=rn(e,"readonly",3,!1);const i=pi(),s=J(()=>e.item.block.fields),o=J(()=>e.item.block.enabled===!1),l=J(()=>e.item.block.type==="formTextarea");function a(F){F.target.closest("button, a, input, select")||e.onedit()}function d(F){F.target===F.currentTarget&&(F.key==="Enter"||F.key===" ")&&(F.preventDefault(),e.onedit())}var u=md();let g;var _=L(u),v=L(_);{var p=F=>{var re=gd(),Ie=L(re);Js(Ie,{get sortable(){return e.sortable},get index(){return e.index}}),S(F,re)};W(v,F=>{r()||F(p)})}var h=D(v,2),w=L(h),k=D(w);{var b=F=>{var re=wd();S(F,re)};W(k,F=>{c(s).required&&F(b)})}var M=D(h,2),E=te(M,!0),y=D(M,2);{var A=F=>{var re=pd(),Ie=te(re,!0);B(()=>Y(Ie,i.off)),S(F,re)};W(y,F=>{c(o)&&F(A)})}var x=D(y,2);{var T=F=>{eo(F,{get items(){return e.menu},get label(){return i.fieldsActions}})};W(x,F=>{r()||F(T)})}var N=D(_,2);let G;var ne=L(N),oe=te(ne,!0),le=D(N,2);{var he=F=>{var re=_d(),Ie=te(re,!0);B(pe=>Y(Ie,pe),[()=>qn(c(s).instructions,e.site)]),S(F,re)},ge=J(()=>qn(c(s).instructions,e.site));W(le,F=>{c(ge)&&F(he)})}B((F,re)=>{g=Ke(u,1,"field-card svelte-1rirqtx",null,g,{selected:t(),off:c(o)}),Y(w,F),Y(E,e.typeName),G=Ke(N,1,"preview svelte-1rirqtx",null,G,{textarea:c(l)}),Y(oe,re)},[()=>qn(c(s).label,e.site)||e.typeName,()=>qn(c(s).placeholder,e.site)]),Q("click",u,a),Q("keydown",u,d),S(n,u),Ce()}an(["click","keydown"]);var kd=I('<button type="button"> </button> <button type="button" class="primary"><!> </button>',1),yd=I('<span class="grip svelte-qbgxdw"><!></span>'),Md=I('<input class="rename svelte-qbgxdw"/>'),Cd=I('<button type="button" role="tab" class="plain name svelte-qbgxdw"> <span class="count svelte-qbgxdw"> </span></button>'),Sd=I('<div data-sort="" data-menu-area=""><!> <!> <!></div>'),Ed=I('<button type="button" class="plain add-page svelte-qbgxdw"><!> <span> </span></button>'),Ad=I('<div data-sort=""><!></div>'),Id=I('<p class="empty"> </p>'),zd=I('<button type="button" class="add-field svelte-qbgxdw"><!> </button>'),Td=I('<button type="button" class="popup-item" role="menuitem"> </button>'),Bd=I('<div class="cell add svelte-qbgxdw"><!></div>'),Ld=I("<div></div>"),Dd=I('<div class="settings svelte-qbgxdw"><!> <!></div>'),Pd=I('<div class="builder svelte-qbgxdw"><div class="tabs svelte-qbgxdw" role="tablist"><!> <!> <!></div> <div class="grid svelte-qbgxdw"><!> <!> <!></div></div> <!>',1);const jd={hash:"svelte-qbgxdw",code:`.builder.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.75rem;}.tabs.svelte-qbgxdw {position:relative;display:flex;flex-wrap:wrap;align-items:center;gap:0.25rem;padding-bottom:0.5rem;border-bottom:1px solid var(--line);}.tab.svelte-qbgxdw {display:flex;align-items:center;gap:0.125rem;padding:0.125rem 0.25rem;border:1px solid transparent;border-radius:var(--radius);}.tab.on.svelte-qbgxdw {background:var(--accent-soft);border-color:var(--accent-ring);}.tab.svelte-qbgxdw .grip:where(.svelte-qbgxdw) {display:flex;color:var(--muted);}.name.svelte-qbgxdw {display:flex;align-items:center;gap:0.375rem;padding:0.25rem 0.375rem;font-weight:500;}.on.svelte-qbgxdw .name:where(.svelte-qbgxdw) {color:var(--accent);}.count.svelte-qbgxdw {min-width:1.25rem;padding:0 0.3125rem;color:var(--muted);font-size:var(--fs-xs);text-align:center;background:var(--hover);border-radius:999px;}.rename.svelte-qbgxdw {width:12rem;padding:0.25rem 0.5rem;}.add-page.svelte-qbgxdw {display:flex;align-items:center;gap:0.25rem;padding:0.375rem 0.5rem;color:var(--muted);}
@media (hover: hover) {.add-page.svelte-qbgxdw {color:var(--accent);}
}.grid.svelte-qbgxdw {position:relative;display:grid;grid-template-columns:repeat(12, minmax(0, 1fr));gap:0.625rem;}.cell.svelte-qbgxdw {grid-column:span var(--span, 12);min-width:0;}.cell.add.svelte-qbgxdw {grid-column:span 12;}.add-field.svelte-qbgxdw {display:flex;align-items:center;justify-content:center;gap:0.375rem;width:100%;padding:0.75rem;color:var(--muted);background:transparent;border:1px dashed var(--border-strong);border-radius:var(--radius-lg);}
@media (hover: hover) {.add-field.svelte-qbgxdw {color:var(--accent);border-color:var(--accent);}
}.settings.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.625rem;}

@media (max-width: 799px) {.cell.svelte-qbgxdw {grid-column:span 12;}
}`};function Od(n,e){Me(e,!0),Le(n,jd);const t=m=>{var C=kd(),z=ae(C),R=te(z,!0),ee=D(z,2),ie=L(ee);on(ie,{name:"check",size:14});var Se=D(ie);B(()=>{Y(R,r.cancel),Y(Se,` ${r.apply??""}`)}),Q("click",z,oe),Q("click",ee,ne),S(m,C)},r=pi(),i=J(()=>e.value??[]),s=J(()=>cd(c(i)));let o=$(0);const l=J(()=>Math.min(c(o),c(s).length-1)),a=J(()=>c(s)[c(l)]),d=J(()=>e.blocks.types.filter(m=>m.handle!==ni)),u=m=>{var C;return((C=e.blocks.types.find(z=>z.handle===m))==null?void 0:C.name)??m},g=(m,C)=>ud(m,e.site)||`${r.page} ${C+1}`,_=()=>({block:e.blocks.create(ni)});function v(m){const C=dd(m,_);e.onchange(C.length?C:null)}let p=$(null);function h(){const m=_();m.block.fields.label={[e.site]:`${r.page} ${c(s).length+1}`},v([...c(s),{page:m,fields:[]}]),O(o,c(s).length,!0)}function w(m,C){c(p)===m&&(O(p,null),v(vd(c(s),m,C.trim(),e.site,_)))}function k(m,C){let z=!1;const R=setTimeout(()=>{m.focus(),m.select(),z=!0},50),ee=()=>z&&w(C,m.value);return m.addEventListener("blur",ee),{destroy(){clearTimeout(R),m.removeEventListener("blur",ee)}}}function b(m,C){v(Hi(c(s),m,C)),O(o,C,!0)}function M(m){c(s)[m].fields.length&&!confirm(r.confirmDeletePage)||(v(c(s).filter((C,z)=>z!==m)),O(o,Math.max(0,m-1),!0))}const E=m=>[{label:r.renamePage,icon:"edit",onclick:()=>O(p,m,!0)},...m>0?[{label:r.moveLeft,icon:"arrow-left",onclick:()=>b(m,m-1)}]:[],...m<c(s).length-1?[{label:r.moveRight,icon:"arrow-right",onclick:()=>b(m,m+1)}]:[],...c(s).length>1?[{label:r.deletePage,icon:"delete",danger:!0,onclick:()=>M(m)}]:[]];let y=$(void 0);const A=new qi({container:()=>c(y),depths:()=>c(s).map(()=>0),grid:()=>!0,onmove:(m,C)=>b(m,C.index)});let x=$(null),T=null,N=null;function G(m){T=N=Cr(e.value),O(x,m,!0)}function ne(){O(x,null)}function oe(){JSON.stringify(Cr(e.value))!==JSON.stringify(N)&&!confirm(r.confirmDiscard)||(O(x,null),e.onchange(T))}const le=J(()=>c(x)&&c(s).flatMap((m,C)=>m.fields.map((z,R)=>({item:z,gi:C,fi:R}))).find(m=>m.item.block.id===c(x))),he=m=>v(fr(c(s),c(l),m));let ge=null;function F(m){const C=e.blocks.create(m),z=Cr(e.value);he(R=>[...R,{width:100,block:C}]),ge=C.id,G(C.id),T=z}function re(m){const C=c(a).fields[m],z={...C,block:e.blocks.copy(C.block)};he(R=>[...R.slice(0,m+1),z,...R.slice(m+1)])}function Ie(m){he(C=>C.map((z,R)=>{if(R!==m)return z;const{enabled:ee,...ie}=z.block;return{...z,block:ee===!1?ie:{...ie,enabled:!1}}}))}function pe(m){c(x)===c(a).fields[m].block.id&&O(x,null),he(C=>C.filter((z,R)=>R!==m))}function Jt(m,C,z){v(fr(c(s),m,R=>R.map((ee,ie)=>ie===C?{...ee,width:z}:ee)))}const no=(m,C)=>[{label:r.edit,icon:"edit",onclick:()=>O(x,m.block.id,!0)},{label:r.duplicate,icon:"copy",onclick:()=>re(C)},...c(s).map((z,R)=>({g:z,gi:R})).filter(({gi:z})=>z!==c(l)).map(({g:z,gi:R})=>({label:r.toPage.replace("{page}",g(z,R)),icon:"move",onclick:()=>v(fd(c(s),c(l),C,R))})),{label:m.block.enabled===!1?r.switchOn:r.switchOff,icon:m.block.enabled===!1?"view":"hide",onclick:()=>Ie(C)},{label:r.remove,icon:"delete",danger:!0,onclick:()=>pe(C)}];let xr=$(void 0),Qt={x:0,y:0};Yn(()=>{const m=C=>Qt={x:C.clientX,y:C.clientY};return window.addEventListener("pointermove",m),()=>window.removeEventListener("pointermove",m)});function to(){var R;const m=(R=document.elementFromPoint(Qt.x,Qt.y))==null?void 0:R.closest(".cell[data-sort]");if(!m||m.classList.contains("sort-dragging")||getComputedStyle(m).getPropertyValue("--span")==="12")return!1;const C=m.getBoundingClientRect(),z=(Qt.y-C.top)/C.height;return z>.25&&z<.75}const kr=new qi({container:()=>c(xr),depths:()=>c(a).fields.map(()=>0),grid:to,onmove:(m,C)=>he(z=>Hi(z,m,C.index))});function yr(m,C){const z=e.blocks.mount(m,C);return ge===C&&(ge=null,setTimeout(()=>{var R;return(R=m.querySelector("input"))==null?void 0:R.focus()},50)),{destroy:()=>z.destroy()}}const ro=m=>({25:"¼",33:"⅓",50:"½",67:"⅔",75:"¾",100:r.full})[m];var _i=Pd(),mi=ae(_i),$t=L(mi),bi=L($t);Ge(bi,19,()=>c(s),m=>{var C;return((C=m.page)==null?void 0:C.block.id)??"first"},(m,C,z)=>{var R=Sd();let ee;var ie=L(R);{var Se=ce=>{var de=yd(),ze=L(de);Js(ze,{get sortable(){return A},get index(){return c(z)}}),S(ce,de)};W(ie,ce=>{!e.readonly&&c(s).length>1&&ce(Se)})}var De=D(ie,2);{var xe=ce=>{var de=Md();Bt(de,(ze,Je)=>k==null?void 0:k(ze,Je),()=>c(z)),B(ze=>{var Je,er;X(de,"aria-label",r.pageName),Ws(de,((er=(Je=c(C).page)==null?void 0:Je.block.fields.label)==null?void 0:er[e.site])??""),X(de,"placeholder",ze)},[()=>g(c(C),c(z))]),Q("keydown",de,ze=>{ze.key==="Enter"&&w(c(z),ze.currentTarget.value),ze.key==="Escape"&&O(p,null)}),S(ce,de)},Ze=ce=>{var de=Cd(),ze=L(de),Je=D(ze),er=te(Je,!0);B(uo=>{X(de,"aria-selected",c(z)===c(l)),Y(ze,`${uo??""} `),Y(er,c(C).fields.length)},[()=>g(c(C),c(z))]),Q("click",de,()=>O(o,c(z),!0)),Q("dblclick",de,()=>!e.readonly&&O(p,c(z),!0)),S(ce,de)};W(De,ce=>{c(p)===c(z)?ce(xe):ce(Ze,-1)})}var Pe=D(De,2);{var cn=ce=>{{let de=J(()=>E(c(z)));eo(ce,{get items(){return c(de)},get label(){return r.pageActions}})}};W(Pe,ce=>{!e.readonly&&c(z)===c(l)&&c(p)!==c(z)&&ce(cn)})}B(ce=>ee=Ke(R,1,"tab svelte-qbgxdw",null,ee,{on:c(z)===c(l),"sort-dragging":ce}),[()=>A.moving(c(z))]),S(m,R)});var xi=D(bi,2);{var io=m=>{var C=Ed(),z=L(C);on(z,{name:"add"});var R=D(z,2),ee=te(R,!0);B(()=>{X(C,"title",r.addPage),Y(ee,r.addPage)}),Q("click",C,h),S(m,C)};W(xi,m=>{e.readonly||m(io)})}var so=D(xi,2);Ni(so,{get sortable(){return A}}),ft($t,m=>O(y,m),()=>c(y));var ki=D($t,2),yi=L(ki);Ge(yi,19,()=>c(a).fields,m=>m.block.id,(m,C,z)=>{var R=Ad();let ee,ie;var Se=L(R);{let De=J(()=>u(c(C).block.type)),xe=J(()=>c(x)===c(C).block.id),Ze=J(()=>no(c(C),c(z)));xd(Se,{get item(){return c(C)},get site(){return e.site},get typeName(){return c(De)},get selected(){return c(xe)},get readonly(){return e.readonly},get sortable(){return kr},get index(){return c(z)},get menu(){return c(Ze)},onedit:()=>G(c(C).block.id)})}B((De,xe)=>{ee=Ke(R,1,"cell svelte-qbgxdw",null,ee,{"sort-dragging":De}),ie=wt(R,"",ie,{"--span":xe})},[()=>kr.moving(c(z)),()=>hd(c(C).width)]),S(m,R)},m=>{var C=Xe(),z=ae(C);{var R=ee=>{var ie=Id(),Se=te(ie,!0);B(()=>Y(Se,r.noFields)),S(ee,ie)};W(z,ee=>{e.readonly&&ee(R)})}S(m,C)});var Mi=D(yi,2);{var oo=m=>{var C=Bd(),z=L(C);wi(z,{menu:!0,get label(){return r.addField},trigger:(ee,ie)=>{let Se=()=>(ie==null?void 0:ie()).toggle,De=()=>(ie==null?void 0:ie()).open;var xe=zd(),Ze=L(xe);on(Ze,{name:"add"});var Pe=D(Ze);B(()=>{X(xe,"aria-expanded",De()),Y(Pe,` ${r.addField??""}`)}),Q("click",xe,function(...cn){var ce;(ce=Se())==null||ce.apply(this,cn)}),S(ee,xe)},children:(ee,ie)=>{var Se=Xe(),De=ae(Se);Ge(De,17,()=>c(d),xe=>xe.handle,(xe,Ze)=>{var Pe=Td(),cn=te(Pe,!0);B(()=>Y(cn,c(Ze).name)),Q("click",Pe,()=>F(c(Ze).handle)),S(xe,Pe)}),S(ee,Se)},$$slots:{trigger:!0,default:!0}}),S(m,C)};W(Mi,m=>{e.readonly||m(oo)})}var lo=D(Mi,2);Ni(lo,{get sortable(){return kr}}),ft(ki,m=>O(xr,m),()=>c(xr));var ao=D(mi,2);{var co=m=>{{let C=J(()=>qn(c(le).item.block.fields.label,e.site)||u(c(le).item.block.type)),z=J(()=>e.readonly?ne:oe),R=J(()=>e.readonly?void 0:t);Uc(m,{get title(){return c(C)},get onclose(){return c(z)},get actions(){return c(R)},children:(ee,ie)=>{var Se=Dd(),De=L(Se);{var xe=Pe=>{Oc(Pe,{get label(){return r.width},children:(cn,ce)=>{{let de=J(()=>ad.map(Je=>({value:Je,label:ro(Je)}))),ze=J(()=>c(le).item.width??100);td(cn,{get label(){return r.width},get options(){return c(de)},get value(){return c(ze)},onchange:Je=>Jt(c(le).gi,c(le).fi,Je)})}},$$slots:{default:!0}})};W(De,Pe=>{e.readonly||Pe(xe)})}var Ze=D(De,2);Tl(Ze,()=>c(x),Pe=>{var cn=Ld();Bt(cn,(ce,de)=>yr==null?void 0:yr(ce,de),()=>c(x)),S(Pe,cn)}),S(ee,Se)},$$slots:{default:!0}})}};W(ao,m=>{c(le)&&m(co)})}B(()=>X($t,"aria-label",r.pages)),S(n,_i),Ce()}an(["click","keydown","dblclick"]);var Nd=I("<input/>");function qd(n,e){const t=ml();Me(e,!0);const r=J(()=>e.value??{}),i=J(()=>qn({...c(r),[e.site]:""},e.site));function s(l){const a=l.currentTarget.value,d={...c(r),[e.site]:a};a||delete d[e.site],e.onchange(Object.keys(d).length?d:null)}var o=Nd();B(()=>{X(o,"id",t),Ws(o,c(r)[e.site]??""),X(o,"placeholder",c(i)),o.disabled=e.readonly}),Q("input",o,s),S(n,o),Ce()}an(["input"]);var Rd=I("<span>/</span>"),Fd=I("<a> </a>"),Hd=I('<span class="here truncate svelte-wekzd8"> </span>'),Vd=I("<!> <!>",1),Yd=I('<nav class="crumbs svelte-wekzd8"></nav>');const Ud={hash:"svelte-wekzd8",code:`.crumbs.svelte-wekzd8 {display:flex;align-items:center;gap:0.375rem;min-width:0;font-size:var(--fs-md);}a.svelte-wekzd8 {text-decoration:none;white-space:nowrap;}.sep.svelte-wekzd8 {color:var(--subtle);}.here.svelte-wekzd8 {color:var(--text);font-weight:500;}

@container page (width < 36rem) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}
@media (max-width: 799px) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}`};function Gd(n,e){Me(e,!0),Le(n,Ud);var t=Yd();Ge(t,21,()=>e.items,vr,(r,i,s)=>{const o=J(()=>s<e.items.length-2);var l=Vd(),a=ae(l);{var d=v=>{var p=Rd();let h;B(()=>h=Ke(p,1,"sep svelte-wekzd8",null,h,{far:s<e.items.length-1})),S(v,p)};W(a,v=>{s&&v(d)})}var u=D(a,2);{var g=v=>{var p=Fd();let h;var w=te(p,!0);B(()=>{X(p,"href",c(i).href),h=Ke(p,1,"svelte-wekzd8",null,h,{far:c(o)}),Y(w,c(i).label)}),S(v,p)},_=v=>{var p=Hd(),h=te(p,!0);B(()=>Y(h,c(i).label)),S(v,p)};W(u,v=>{c(i).href&&s<e.items.length-1?v(g):v(_,-1)})}S(r,l)}),B(()=>X(t,"aria-label",wn.texts.breadcrumbs)),S(n,t),Ce()}var Wd=I('<span class="note svelte-z3zeui"> </span>'),Xd=I('<span class="state svelte-z3zeui"><!></span>'),Kd=I('<!> <h1 class="svelte-z3zeui"><!></h1> <!> <!> <span class="actions svelte-z3zeui"><!></span>',1),Zd=I("<div><!></div>");const Jd={hash:"svelte-z3zeui",code:`.page-top.svelte-z3zeui {position:sticky;top:0;z-index:var(--z-bar);display:flex;align-items:center;gap:0.5rem;flex-wrap:wrap;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.bleed.svelte-z3zeui {margin:0 calc(-1 * var(--page-pad)) 1.25rem;padding:0.5rem var(--page-pad);}h1.svelte-z3zeui {min-width:0;font-size:var(--fs-lg);line-height:1.25rem;}.state.svelte-z3zeui {display:flex;align-items:center;gap:0.375rem;}.actions.svelte-z3zeui {display:flex;align-items:center;gap:0.5rem;margin-left:auto;}

@container page (width < 36rem) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}
@media (max-width: 799px) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}`};function Qd(n,e){Me(e,!0),Le(n,Jd);const t=d=>{var u=Kd(),g=ae(u);{var _=x=>{var T=Xe(),N=ae(T);Fe(N,()=>(s==null?void 0:s.content)??lt),S(x,T)};W(g,x=>{i||x(_)})}var v=D(g,2),p=L(v);{var h=x=>{Gd(x,{get items(){return e.crumbs}})},w=x=>{var T=Vs();B(()=>Y(T,e.title)),S(x,T)};W(p,x=>{e.crumbs?x(h):x(w,-1)})}var k=D(v,2);{var b=x=>{var T=Wd(),N=te(T,!0);B(()=>Y(N,e.note)),S(x,T)};W(k,x=>{e.note&&x(b)})}var M=D(k,2);{var E=x=>{var T=Xd(),N=L(T);Fe(N,()=>e.state),S(x,T)};W(M,x=>{e.state&&x(E)})}var y=D(M,2),A=L(y);Fe(A,()=>e.actions??lt),S(d,u)};let r=rn(e,"bleed",3,!0);const i=$s.get(),s=Nc.get();Yn(()=>{if(i)return i.content=t,()=>i.content=null});var o=Xe(),l=ae(o);{var a=d=>{var u=Zd();let g;var _=L(u);t(_),B(()=>g=Ke(u,1,"page-top svelte-z3zeui",null,g,{bleed:r()})),S(d,u)};W(l,d=>{i||d(a)})}S(n,o),Ce()}var $d=I("<col/>"),eu=I("<colgroup></colgroup>"),nu=I('<th class="svelte-migw9g"> </th>'),tu=I("<thead><tr></tr></thead>"),ru=I('<tr class="detail svelte-migw9g"><td><!></td></tr>'),iu=I('<tr class="svelte-migw9g"><!></tr> <!>',1),su=I('<tr class="svelte-migw9g"><td class="empty svelte-migw9g"> </td></tr>'),ou=I('<div class="scroll svelte-migw9g"><table><!><!><tbody class="svelte-migw9g"></tbody></table></div>');const lu={hash:"svelte-migw9g",code:`.scroll.svelte-migw9g {overflow-x:auto;}table.svelte-migw9g {width:100%;border-collapse:collapse;}table.fixed.svelte-migw9g {table-layout:fixed;}th.svelte-migw9g,
tbody.svelte-migw9g td {padding:0.375rem 0.5rem;text-align:left;vertical-align:top;}th.svelte-migw9g {font-weight:600;}tbody.svelte-migw9g td {border-top:1px solid var(--line);}tbody.svelte-migw9g code {font-size:var(--fs-md);color:var(--muted);}tbody.svelte-migw9g tr:where(.svelte-migw9g).target {background:var(--hover);transition:background 0.3s;}tbody.svelte-migw9g tr[aria-expanded=true]:where(.svelte-migw9g),
.detail.svelte-migw9g {background:var(--surface-2);}.detail.svelte-migw9g td {border-top:none;padding:0.25rem 0.5rem 0.75rem;}.empty.svelte-migw9g {color:var(--muted);}.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail) {cursor:pointer;}

@media (hover: hover) {.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail):hover {background:var(--hover);}
}`};function au(n,e){Me(e,!0),Le(n,lu);let t=rn(e,"columns",19,()=>[]);function r(_,v){if(_.target.closest("a, button, input, select"))return;if(e.ontoggle)return e.ontoggle(v);if(!e.href)return;const p=e.href(v);if(_.metaKey||_.ctrlKey)return void window.open(p,"_blank");const w=_.currentTarget.querySelector(`a[href="${CSS.escape(p)}"]`);w?w.click():location.assign(p)}var i=ou(),s=L(i);let o;var l=L(s);{var a=_=>{var v=eu();Ge(v,21,()=>e.widths,vr,(p,h)=>{var w=$d();let k;B(()=>k=wt(w,"",k,{width:c(h)})),S(p,w)}),S(_,v)};W(l,_=>{e.widths&&_(a)})}var d=D(l);{var u=_=>{var v=tu(),p=L(v);Ge(p,21,t,vr,(h,w)=>{var k=nu(),b=te(k,!0);B(()=>Y(b,c(w))),S(h,k)}),S(_,v)};W(d,_=>{t().length&&_(u)})}var g=D(d);Ge(g,23,()=>e.rows,_=>e.key(_),(_,v,p)=>{var h=iu(),w=ae(h),k=L(w);Fe(k,()=>e.row,()=>c(v),()=>c(p));var b=D(w,2);{var M=y=>{var A=ru(),x=L(A),T=L(x);Fe(T,()=>e.detail,()=>c(v)),B(()=>X(x,"colspan",t().length||1)),S(y,A)},E=J(()=>{var y;return e.detail&&((y=e.isOpen)==null?void 0:y.call(e,c(v)))});W(b,y=>{c(E)&&y(M)})}B((y,A)=>{X(w,"id",y),X(w,"aria-expanded",A),X(w,"tabindex",e.ontoggle?0:void 0)},[()=>{var y;return(y=e.id)==null?void 0:y.call(e,c(v))},()=>{var y;return e.ontoggle?!!((y=e.isOpen)!=null&&y.call(e,c(v))):void 0}]),Q("click",w,y=>r(y,c(v))),Q("keydown",w,y=>{!e.ontoggle||y.target!==y.currentTarget||y.key!=="Enter"&&y.key!==" "||(y.preventDefault(),e.ontoggle(c(v)))}),S(_,h)},_=>{var v=su(),p=L(v),h=te(p,!0);B(()=>{X(p,"colspan",t().length||1),Y(h,e.empty??wn.texts.nothingYet)}),S(_,v)}),B(()=>o=Ke(s,1,"svelte-migw9g",null,o,{links:!!e.href||!!e.ontoggle,fixed:!!e.widths})),S(n,i),Ce()}an(["click","keydown"]);var qt,Rt,Ft,Ht,Vt,Yt;class cu{constructor(e){q(this,qt,$(nn([])));q(this,Rt,$(null));q(this,Ft,$(nn([])));q(this,Ht,$(0));q(this,Vt,$(!1));q(this,Yt,$(""));this.host=e}get forms(){return c(f(this,qt))}set forms(e){O(f(this,qt),e,!0)}get form(){return c(f(this,Rt))}set form(e){O(f(this,Rt),e,!0)}get submissions(){return c(f(this,Ft))}set submissions(e){O(f(this,Ft),e,!0)}get total(){return c(f(this,Ht))}set total(e){O(f(this,Ht),e,!0)}get loading(){return c(f(this,Vt))}set loading(e){O(f(this,Vt),e,!0)}get error(){return c(f(this,Yt))}set error(e){O(f(this,Yt),e,!0)}async init(){await this.run(async()=>{const e=await this.host.request("GET","/entries?section=forms&limit=500&order=title");this.forms=e.entries.map(t=>({id:t.id,title:t.title}))}),await this.load()}async load(){await this.run(async()=>{const e=await this.request(0);this.submissions=e.submissions,this.total=e.total})}async more(){await this.run(async()=>{const e=await this.request(this.submissions.length);this.submissions=[...this.submissions,...e.submissions],this.total=e.total})}async delete(e){await this.run(async()=>{await this.host.request("DELETE",`/forms/submissions/${e}`),this.submissions=this.submissions.filter(t=>t.id!==e),this.total-=1})}request(e){const t=this.form===null?"":`&form=${this.form}`;return this.host.request("GET",`/forms/submissions?offset=${e}${t}`)}async run(e){this.loading=!0,this.error="";try{await e()}catch(t){this.error=t.message}finally{this.loading=!1}}}qt=new WeakMap,Rt=new WeakMap,Ft=new WeakMap,Ht=new WeakMap,Vt=new WeakMap,Yt=new WeakMap;var du=I("<option> </option>"),uu=I("<select><option> </option><!></select>"),vu=I('<p class="error"> </p>'),fu=I('<td class="nowrap svelte-1tixh7s"> </td> <td> </td> <td> </td> <td class="truncate"> </td>',1),hu=I('<dt class="svelte-1tixh7s"> </dt> <dd class="svelte-1tixh7s"> </dd>',1),gu=I('<dl class="svelte-1tixh7s"></dl> <button class="danger"><!> </button>',1),wu=I('<p class="more svelte-1tixh7s"><button class="light"> </button></p>'),pu=I('<!> <!> <div class="card"><!></div> <!>',1);const _u={hash:"svelte-1tixh7s",code:`.nowrap.svelte-1tixh7s {white-space:nowrap;}dl.svelte-1tixh7s {display:grid;grid-template-columns:minmax(8rem, max-content) 1fr;gap:0.375rem 1.5rem;margin:0 0 1rem;}dt.svelte-1tixh7s {color:var(--muted);}dd.svelte-1tixh7s {margin:0;white-space:pre-wrap;overflow-wrap:anywhere;}.more.svelte-1tixh7s {text-align:center;}

@media (max-width: 799px) {dl.svelte-1tixh7s {grid-template-columns:1fr;}dd.svelte-1tixh7s {margin-bottom:0.5rem;}
}`};function mu(n,e){Me(e,!0),Le(n,_u);const t=pi(),r=new cu(e.host);r.init();let i=$(null);const s=b=>{var M;return((M=e.schema.sites.find(E=>E.handle===b))==null?void 0:M.name)??b},o=b=>new Date(b).toLocaleString(document.documentElement.lang,{dateStyle:"medium",timeStyle:"short"}),l=b=>b.answers.map(M=>M.value).filter(Boolean).slice(0,3).join(" · ")||t.empty;function a(b){const M=b.currentTarget.value;r.form=M?Number(M):null,O(i,null),r.load()}function d(b){confirm(t.confirmDelete)&&r.delete(b.id)}var u=pu(),g=ae(u);Qd(g,{get title(){return t.title},get note(){return t.note},actions:M=>{var E=Xe(),y=ae(E);{var A=x=>{var T=uu(),N=L(T),G=te(N,!0);N.value=N.__value="";var ne=D(N);Ge(ne,17,()=>r.forms,le=>le.id,(le,he)=>{var ge=du(),F=te(ge,!0),re={};B(()=>{Y(F,c(he).title),re!==(re=c(he).id)&&(ge.value=(ge.__value=re)??"")}),S(le,ge)});var oe;Rl(T),B(()=>{Y(G,t.allForms),oe!==(oe=r.form??"")&&(T.value=(T.__value=oe)??"",Gs(T,oe))}),Q("change",T,a),S(x,T)};W(y,x=>{r.forms.length>1&&x(A)})}S(M,E)},$$slots:{actions:!0}});var _=D(g,2);{var v=b=>{var M=vu(),E=te(M,!0);B(()=>Y(E,r.error)),S(b,M)};W(_,b=>{r.error&&b(v)})}var p=D(_,2),h=L(p);{const b=(A,x=lt)=>{var T=fu(),N=ae(T),G=te(N,!0),ne=D(N,2),oe=te(ne,!0),le=D(ne,2),he=te(le,!0),ge=D(le,2),F=te(ge,!0);B((re,Ie,pe)=>{Y(G,re),Y(oe,x().form),Y(he,Ie),Y(F,pe)},[()=>o(x().createdAt),()=>s(x().site),()=>l(x())]),S(A,T)},M=(A,x=lt)=>{var T=gu(),N=ae(T);Ge(N,21,()=>x().answers,le=>le.handle,(le,he)=>{var ge=hu(),F=ae(ge),re=te(F,!0),Ie=D(F,2),pe=te(Ie,!0);B(()=>{Y(re,c(he).label),Y(pe,c(he).value||t.empty)}),S(le,ge)});var G=D(N,2),ne=L(G);on(ne,{name:"delete"});var oe=D(ne);B(()=>Y(oe,` ${t.delete??""}`)),Q("click",G,()=>d(x())),S(A,T)};let E=J(()=>[t.date,t.form,t.site,t.content]),y=J(()=>r.loading?"…":t.none);au(h,{get rows(){return r.submissions},key:A=>A.id,get columns(){return c(E)},get empty(){return c(y)},isOpen:A=>A.id===c(i),ontoggle:A=>O(i,c(i)===A.id?null:A.id,!0),row:b,detail:M,$$slots:{row:!0,detail:!0}})}var w=D(p,2);{var k=b=>{var M=wu(),E=L(M),y=te(E,!0);B(()=>{E.disabled=r.loading,Y(y,t.more)}),Q("click",E,()=>r.more()),S(b,M)};W(w,b=>{r.submissions.length<r.total&&b(k)})}S(n,u),Ce()}an(["change","click"]);const bu=Ks(Od),xu=Ks(qd);function Mu(n,e){return(e.field.type==="formSiteText"?xu:bu)(n,e)}function Cu(n,e){const t=Ys(mu,{target:n,props:e});return{destroy(){Us(t)}}}export{Mu as mount,Cu as page};
