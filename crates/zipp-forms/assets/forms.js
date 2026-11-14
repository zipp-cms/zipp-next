var so=Object.defineProperty;var xi=n=>{throw TypeError(n)};var oo=(n,e,t)=>e in n?so(n,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):n[e]=t;var ge=(n,e,t)=>oo(n,typeof e!="symbol"?e+"":e,t),br=(n,e,t)=>e.has(n)||xi("Cannot "+t);var f=(n,e,t)=>(br(n,e,"read from private field"),t?t.call(n):e.get(n)),N=(n,e,t)=>e.has(n)?xi("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,t),H=(n,e,t,r)=>(br(n,e,"write to private field"),r?r.call(n,t):e.set(n,t),t),j=(n,e,t)=>(br(n,e,"access private method"),t);const Yi={},ve=Symbol("uninitialized"),Ui="http://www.w3.org/1999/xhtml",lo="http://www.w3.org/2000/svg",ao="http://www.w3.org/1998/Math/MathML",Gi=!1;var fr=Array.isArray,co=Array.prototype.indexOf,Et=Array.prototype.includes,hr=Array.from,Wi=Object.defineProperty,Jn=Object.getOwnPropertyDescriptor,Xi=Object.getOwnPropertyDescriptors,uo=Object.prototype,vo=Array.prototype,Qr=Object.getPrototypeOf,ki=Object.isExtensible;const ot=()=>{};function fo(n){for(var e=0;e<n.length;e++)n[e]()}function Ki(){var n,e,t=new Promise((r,i)=>{n=r,e=i});return{promise:t,resolve:n,reject:e}}function ho(n,e){if(Array.isArray(n))return n;if(!(Symbol.iterator in n))return Array.from(n);const t=[];for(const r of n)if(t.push(r),t.length===e)break;return t}const ke=2,lt=4,gr=8,Zi=1<<24,Qe=16,sn=32,Pn=64,Br=128,$r=256,tn=512,fe=1024,se=2048,Ne=4096,Se=8192,pe=16384,ft=32768,At=1<<25,Gn=65536,rr=1<<17,go=1<<18,ht=1<<19,wo=1<<20,fn=1<<25,ir=1<<21,Qn=1<<22,Bn=1<<23,Mn=Symbol("$state"),Ji=Symbol("component"),po=Symbol("legacy props"),_o=Symbol(""),er=Symbol("attributes"),Lr=Symbol("class"),Pr=Symbol("style"),pt=Symbol("text"),Yt=new class extends Error{constructor(){super(...arguments);ge(this,"name","StaleReactionError");ge(this,"message","The reaction that called `getAbortSignal()` was re-run or destroyed")}};var Fi;const mo=!!((Fi=globalThis.document)!=null&&Fi.contentType)&&globalThis.document.contentType.includes("xml");function bo(){console.warn("https://svelte.dev/e/derived_inert")}function xo(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function ko(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}function Qi(n){return n===this.v}function $i(n,e){return n!=n?e==e:n!==e||n!==null&&typeof n=="object"||typeof n=="function"}function es(n){return!$i(n,this.v)}function yo(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function Mo(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function Co(n,e,t){throw new Error("https://svelte.dev/e/each_key_duplicate")}function So(n){throw new Error("https://svelte.dev/e/effect_in_teardown")}function Eo(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function Ao(n){throw new Error("https://svelte.dev/e/effect_orphan")}function Io(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function zo(n){throw new Error("https://svelte.dev/e/props_invalid_value")}function To(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function Bo(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function Lo(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function Po(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let jo=!1;function Do(n){let e=n.p;for(;e!==null&&e.c===null;)e=e.p;return(e==null?void 0:e.c)??null}function ns(n,e){return n===null&&yo(),n.c??(n.c=new Map(Do(n)||void 0))}let _e=null;function at(n){_e=n}function Oo(n){return ns(_e).get(n)}function qo(n,e){return ns(_e).set(n,e),e}function Ee(n,e=!1,t){_e={p:_e,i:!1,c:null,e:null,s:n,x:null,r:U,l:null}}function Ae(n){var e=_e,t=e.e;if(t!==null){e.e=null;for(var r of t)Ms(r)}return e.i=!0,_e=e.p,ei(n)}function ei(n={}){return Wi(n,Ji,{value:!0}),n}function ts(){return!0}let Dn=[];function rs(){var n=Dn;Dn=[],fo(n)}function kn(n){if(Dn.length===0&&!yt){var e=Dn;queueMicrotask(()=>{e===Dn&&rs()})}Dn.push(n)}function No(){for(;Dn.length>0;)rs()}const Ro=-7169;function ue(n,e){n.f=n.f&Ro|e}function ni(n){(n.f&tn)!==0||n.deps===null?ue(n,fe):ue(n,Ne)}function is(n,e){(n.f&se)!==0?e.set(n,se):(n.f&Ne)!==0&&e.get(n)!==se&&e.set(n,Ne),ue(n,fe)}function Ut(n){var e=V,t=U;Ge(null),pn(null);try{return n()}finally{Ge(e),pn(t)}}function Fo(n,e,t,r){const i=It;var s=n.filter(v=>!v.settled),o=e.map(i);if(t.length===0&&s.length===0){r(o);return}var l=U,c=Ho(),d=s.length===1?s[0].promise:s.length>1?Promise.all(s.map(v=>v.promise)):null;function u(v){if((l.f&pe)===0){c();try{r([...o,...v])}catch(p){vn(p,l)}sr()}}var g=ss();if(t.length===0){d.then(()=>u([])).finally(g);return}function _(){Promise.all(t.map(v=>Vo(v))).then(u).catch(v=>vn(v,l)).finally(g)}d?d.then(()=>{if((l.f&pe)!==0){g();return}c(),_(),sr()}):_()}function Ho(){var n=U,e=V,t=_e,r=D;return function(s=!0){pn(n),Ge(e),at(t),s&&(n.f&pe)===0&&(r==null||r.activate(),r==null||r.apply())}}function sr(n=!0){pn(null),Ge(null),at(null),n&&(D==null||D.deactivate())}function ss(){var n=U,e=n.b,t=D,r=!!(e!=null&&e.is_rendered());return e==null||e.update_pending_count(1,t),t.increment(r,n),()=>{e==null||e.update_pending_count(-1,t),t.decrement(r,n)}}function It(n){var e=ke|se;return U!==null&&(U.f|=ht),{ctx:_e,deps:null,effects:null,equals:Qi,f:e,fn:n,reactions:null,rv:0,v:ve,wv:0,parent:U,ac:null}}const _t=Symbol("obsolete");function Vo(n,e,t){let r=U;r===null&&Mo();var i=void 0,s=Xn(ve),o=!V,l=new Set;return il(()=>{var v,p;var c=U,d=Ki();i=d.promise;try{Promise.resolve(n()).then(d.resolve,h=>{h!==Yt&&d.reject(h)}).finally(sr)}catch(h){d.reject(h),sr()}var u=D;if(o){if((c.f&ft)!==0)var g=ss();if((v=r.b)!=null&&v.is_rendered())(p=u.async_deriveds.get(c))==null||p.reject(_t);else for(const h of l.values())h.reject(_t);l.add(d),u.async_deriveds.set(c,d)}const _=(h,w=void 0)=>{g==null||g(),l.delete(d),w!==_t&&(u.activate(),w?(s.f|=Bn,ct(s,w)):((s.f&Bn)!==0&&(s.f^=Bn),ct(s,h)),u.deactivate())};d.promise.then(_,h=>_(null,h||"unknown"))}),ys(()=>{for(const c of l)c.reject(_t)}),new Promise(c=>{function d(u){function g(){u===i?c(s):d(i)}u.then(g,g)}d(i)})}function ne(n){const e=It(n);return zs(e),e}function os(n){const e=It(n);return e.equals=es,e}function Yo(n){var e=n.effects;if(e!==null){n.effects=null;for(var t=0;t<e.length;t+=1)Te(e[t])}}function ti(n){var e,t=U,r=n.parent;if(!Sn&&r!==null&&n.v!==ve&&(r.f&(pe|Se))!==0)return bo(),n.v;pn(r);try{Yo(n),e=Ps(n)}finally{pn(t)}return e}function ls(n){var e=ti(n);if(!n.equals(e)&&(n.wv=Bs(),(!(D!=null&&D.is_fork)||n.deps===null)&&(D!==null?(D.capture(n,e,!0),Ln==null||Ln.capture(n,e,!0)):n.v=e,n.deps===null))){ue(n,fe);return}Sn||(we!==null?(oi()||D!=null&&D.is_fork)&&we.set(n,e):ni(n))}function Uo(n){var e;if(n.effects!==null)for(const t of n.effects)(t.teardown||t.ac)&&((e=t.teardown)==null||e.call(t),t.ac!==null&&Ut(()=>{t.ac.abort(Yt),t.ac=null}),t.fn!==null&&(t.teardown=ot),zt(t,0),ci(t))}function as(n){if(n.effects!==null)for(const e of n.effects)e.teardown&&e.fn!==null&&dt(e)}let xr=null,Kn=null,D=null,Ln=null,we=null,jr=null,yt=!1,kr=!1,Mt=null,nr=null;var yi=0;let Go=1;var et,nt,tt,mn,rt,bn,Bt,an,cn,it,Nn,X,Dr,cs,Or,mt,qr,ds,us,vs,Zn,Wo,bt;const dr=class dr{constructor(){N(this,X);ge(this,"id",Go++);ge(this,"started",!1);ge(this,"linked",!0);ge(this,"prev",null);ge(this,"next",null);ge(this,"async_deriveds",new Map);ge(this,"current",new Map);ge(this,"previous",new Map);N(this,et,new Set);N(this,nt,new Set);N(this,tt,0);N(this,mn,new Map);N(this,rt,null);N(this,bn,[]);N(this,Bt,[]);N(this,an,new Map);N(this,cn,new Map);N(this,it,new Set);ge(this,"is_fork",!1);N(this,Nn,!1);Kn===null?xr=Kn=this:(Kn.next=this,this.prev=Kn),Kn=this}skip_effect(e){f(this,cn).has(e)||f(this,cn).set(e,{d:[],m:[]}),f(this,it).delete(e)}unskip_effect(e,t=r=>this.schedule(r)){var r=f(this,cn).get(e);if(r){f(this,cn).delete(e);for(var i of r.d)ue(i,se),t(i);for(i of r.m)ue(i,Ne),t(i)}f(this,it).add(e)}capture(e,t,r=!1){e.v!==ve&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&Bn)===0&&(this.current.set(e,[t,r]),we==null||we.set(e,t)),this.is_fork||(e.v=t)}activate(){D=this}deactivate(){D=null,we=null}flush(){try{kr=!0,D=this,j(this,X,mt).call(this)}finally{yi=0,jr=null,Mt=null,nr=null,kr=!1,D=null,we=null,gn.clear()}}discard(){var e;for(const t of f(this,nt))t(this);f(this,nt).clear();for(const t of this.async_deriveds.values())t.reject(_t);j(this,X,bt).call(this),(e=f(this,rt))==null||e.resolve()}register_created_effect(e){f(this,Bt).push(e)}increment(e,t){if(H(this,tt,f(this,tt)+1),e){let r=f(this,mn).get(t)??0;f(this,mn).set(t,r+1)}}decrement(e,t){if(H(this,tt,f(this,tt)-1),e){let r=f(this,mn).get(t)??0;r===1?f(this,mn).delete(t):f(this,mn).set(t,r-1)}f(this,Nn)||(H(this,Nn,!0),kn(()=>{H(this,Nn,!1),this.linked&&this.flush()}))}transfer_reactions(e){for(const[t,r]of e)this.add_dirty_reaction(t,r);e.clear()}add_dirty_reaction(e,t){(t===se||f(this,an).get(e)!==se)&&f(this,an).set(e,t)}oncommit(e){f(this,et).add(e)}ondiscard(e){f(this,nt).add(e)}settled(){return(f(this,rt)??H(this,rt,Ki())).promise}static ensure(){if(D===null){const e=D=new dr;!kr&&!yt&&kn(()=>{e.started||e.flush()})}return D}apply(){{we=null;return}}schedule(e){var t;if(jr=e,(t=e.b)!=null&&t.is_pending&&(e.f&(lt|gr|Zi))!==0&&(e.f&ft)===0){e.b.defer_effect(e);return}f(this,bn).push(e)}};et=new WeakMap,nt=new WeakMap,tt=new WeakMap,mn=new WeakMap,rt=new WeakMap,bn=new WeakMap,Bt=new WeakMap,an=new WeakMap,cn=new WeakMap,it=new WeakMap,Nn=new WeakMap,X=new WeakSet,Dr=function(){if(this.is_fork)return!0;for(const e of f(this,mn).keys())if(!j(this,X,cs).call(this,e))return!0;return!1},cs=function(e){for(var t=e;t!==null;t=t.parent)if(f(this,cn).has(t))return!0;return!1},Or=function(){var e=[];for(const s of f(this,bn))if(!((s.f&pe)!==0||(s.f&(se|Ne))===0)){for(var t=s,r=!1;t.parent!==null;){t=t.parent;var i=t.f;if((i&(Pn|sn))!==0){if((i&fe)===0){r=!0;break}t.f^=fe}}r||e.push(t)}return H(this,bn,[]),e},mt=function(){var l,c,d,u;this.started=!0;for(const[g,_]of f(this,an))ue(g,_),this.schedule(g);this.apply();for(var e=Mt=[],t=[],r=nr=[];f(this,bn).length>0;){yi++>1e3&&(j(this,X,bt).call(this),Ko());for(const g of j(this,X,Or).call(this))try{j(this,X,qr).call(this,g,e,t)}catch(_){throw gs(g),j(this,X,Dr).call(this)||this.discard(),_}}if(D=null,r.length>0){var i=dr.ensure();for(const g of r)i.schedule(g)}if(Mt=null,nr=null,j(this,X,Dr).call(this)){j(this,X,Zn).call(this,t),j(this,X,Zn).call(this,e);for(const[g,_]of f(this,cn))hs(g,_);r.length>0&&j(l=D,X,mt).call(l);return}const s=j(this,X,ds).call(this);if(s){j(this,X,Zn).call(this,t),j(this,X,Zn).call(this,e),j(c=s,X,vs).call(c,this);return}f(this,an).clear();for(const g of f(this,et))g(this);f(this,et).clear(),Ln=this,Mi(t),Mi(e),Ln=null,(d=f(this,rt))==null||d.resolve();var o=D;f(this,tt)===0&&(f(this,bn).length===0||o!==null)&&j(this,X,bt).call(this),o!==null&&(gn.clear(),j(u=o,X,mt).call(u))},qr=function(e,t,r){e.f^=fe;for(var i=e.first;i!==null;){var s=i.f,o=(s&(sn|Pn))!==0,l=o&&(s&fe)!==0,c=l||(s&Se)!==0||f(this,cn).has(i);if(!c&&i.fn!==null){o?i.f^=fe:(s&lt)!==0?t.push(i):Xt(i)&&((s&Qe)!==0&&this.add_dirty_reaction(i,Ne),dt(i));var d=i.first;if(d!==null){i=d;continue}}for(;i!==null;){var u=i.next;if(u!==null){i=u;break}i=i.parent}}},ds=function(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&j(this,X,us).call(this,e))return e;return null},us=function(e){for(const[t,[,r]]of this.current)if(!r&&e.current.has(t))return!0;return!1},vs=function(e){var r;for(const[i,s]of e.current)!this.previous.has(i)&&e.previous.has(i)&&this.previous.set(i,e.previous.get(i)),this.current.set(i,s);for(const[i,s]of e.async_deriveds){const o=this.async_deriveds.get(i);o&&s.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(f(e,an));const t=i=>{var s=i.reactions;if(s!==null&&!((i.f&ke)!==0&&(i.f&(se|Ne))===0))for(const c of s){var o=c.f;if((o&ke)!==0)t(c);else{var l=c;o&(Qn|Qe)&&!this.async_deriveds.has(l)&&(f(this,an).delete(l),ue(l,se),this.schedule(l))}}};for(const i of this.current.keys())t(i);this.oncommit(()=>e.discard()),j(r=e,X,bt).call(r),D=this,j(this,X,mt).call(this)},Zn=function(e){for(var t=0;t<e.length;t+=1)is(e[t],f(this,an))},Wo=function(){var g,_;for(let v=xr;v!==null;v=v.next){var e=v.id<this.id,t=[];for(const[p,[h,w]]of this.current){if(v.current.has(p)){var r=v.current.get(p)[0];if(e&&h!==r)v.current.set(p,[h,w]);else continue}t.push(p)}if(e)for(const[p,h]of this.async_deriveds){const w=v.async_deriveds.get(p);w&&h.promise.then(w.resolve).catch(w.reject)}var i=[...v.current.keys()].filter(p=>!v.current.get(p)[1]);if(!(!v.started||i.length===0)){var s=i.filter(p=>!this.current.has(p));if(s.length===0)e&&v.discard();else if(t.length>0){if(e)for(const p of f(this,it))v.unskip_effect(p,h=>{var w;(h.f&(Qe|Qn))!==0?v.schedule(h):j(w=v,X,Zn).call(w,[h])});v.activate();var o=new Set,l=new Map;for(var c of t)fs(c,s,o,l);l=new Map;var d=[...v.current].filter(([p,h])=>{const w=this.current.get(p);return w?w[0]!==h[0]||w[1]!==h[1]:!0}).map(([p])=>p);if(d.length>0)for(const p of f(this,Bt))(p.f&(pe|Se|rr))===0&&ri(p,d,l)&&((p.f&(Qn|Qe))!==0?(ue(p,se),v.schedule(p)):v.add_dirty_reaction(p,se));if(f(v,bn).length>0&&!f(v,Nn)){v.apply();for(var u of j(g=v,X,Or).call(g))j(_=v,X,qr).call(_,u,[],[])}v.deactivate()}}}},bt=function(){if(this.linked){var e=this.prev,t=this.next;e===null?xr=t:e.next=t,t===null?Kn=e:t.prev=e,this.linked=!1}};let Wn=dr;function Xo(n){var e=yt,t=Ln;Ln=null,yt=!0;try{for(var r;;){if(No(),D===null)return r;D.flush()}}finally{yt=e,Ln=t}}function Ko(){try{Io()}catch(n){vn(n,jr)}}let Ze=null;function Mi(n){var e=n.length;if(e!==0){for(var t=0;t<e;){var r=n[t++];if((r.f&(pe|Se))===0&&Xt(r)&&(Ze=new Set,dt(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&Es(r),(Ze==null?void 0:Ze.size)>0)){gn.clear();for(const i of Ze){if((i.f&(pe|Se))!==0)continue;const s=[i];let o=i.parent;for(;o!==null;)Ze.has(o)&&(Ze.delete(o),s.push(o)),o=o.parent;for(let l=s.length-1;l>=0;l--){const c=s[l];(c.f&(pe|Se))===0&&dt(c)}}Ze.clear()}}Ze=null}}function fs(n,e,t,r){if(!t.has(n)&&(t.add(n),n.reactions!==null))for(const i of n.reactions){const s=i.f;(s&ke)!==0?fs(i,e,t,r):(s&(Qn|Qe))!==0&&(s&se)===0&&ri(i,e,r)&&(ue(i,se),ii(i))}}function ri(n,e,t){const r=t.get(n);if(r!==void 0)return r;if(n.deps!==null)for(const i of n.deps){if(Et.call(e,i))return!0;if((i.f&ke)!==0&&ri(i,e,t))return t.set(i,!0),!0}return t.set(n,!1),!1}function ii(n){D.schedule(n)}function hs(n,e){if(!((n.f&sn)!==0&&(n.f&fe)!==0)){(n.f&se)!==0?e.d.push(n):(n.f&Ne)!==0&&e.m.push(n),ue(n,fe);for(var t=n.first;t!==null;)hs(t,e),t=t.next}}function gs(n){ue(n,fe);for(var e=n.first;e!==null;)gs(e),e=e.next}let or=new Set;const gn=new Map;let ws=!1;function Xn(n,e){var t={f:0,v:n,reactions:null,equals:Qi,rv:0,wv:0};return t}function J(n,e){const t=Xn(n);return zs(t),t}function Zo(n,e=!1,t=!0){const r=Xn(n);return e||(r.equals=es),r}function O(n,e,t=!1){V!==null&&(!en||(V.f&rr)!==0)&&ts()&&(V.f&(ke|Qe|Qn|rr))!==0&&(wn===null||!wn.has(n))&&Lo();let r=t?$e(e):e;return ct(n,r,nr)}var jn=null,Nr=0;function ct(n,e,t=null){if(!n.equals(e)){Sn?gn.set(n,e):gn.has(n)||gn.set(n,n.v);var r=Wn.ensure();if(r.capture(n,e),(n.f&ke)!==0){const i=n;(n.f&se)!==0&&ti(i),we===null&&ni(i)}n.wv=Bs(),jn=null,Nr=0,ps(n,se,t),jn=null,U!==null&&(U.f&fe)!==0&&(U.f&(sn|Pn))===0&&(Fe===null?ol([n]):Fe.push(n)),!r.is_fork&&or.size>0&&!ws&&Jo()}return e}function Jo(){ws=!1;for(const n of or){(n.f&fe)!==0&&ue(n,Ne);let e;try{e=Xt(n)}catch{e=!0}e&&dt(n)}or.clear()}function Ct(n){O(n,n.v+1)}function ps(n,e,t){var r=n.reactions;if(r!==null){var i=r.length;if(Nr+=i,Nr>1e5&&jn===null&&(jn=new Set),jn!==null){if(jn.has(n))return;jn.add(n)}for(var s=0;s<i;s++){var o=r[s],l=o.f,c=(l&se)===0;if(c&&ue(o,e),(l&rr)!==0)or.add(o);else if((l&ke)!==0){var d=o;we==null||we.delete(d),ps(d,Ne,t)}else if(c){var u=o;(l&Qe)!==0&&Ze!==null&&Ze.add(u),t!==null?t.push(u):ii(u)}}}}function $e(n){if(typeof n!="object"||n===null||Mn in n||Ji in n)return n;const e=Qr(n);if(e!==uo&&e!==vo)return n;var t=new Map,r=fr(n),i=J(0),s=Un,o=l=>{if(Un===s)return l();var c=V,d=Un;Ge(null),Ai(s);var u=l();return Ge(c),Ai(d),u};return r&&t.set("length",J(n.length)),new Proxy(n,{defineProperty(l,c,d){(!("value"in d)||d.configurable===!1||d.enumerable===!1||d.writable===!1)&&To();var u=t.get(c);return u===void 0?o(()=>{var g=J(d.value);return t.set(c,g),g}):O(u,d.value,!0),!0},deleteProperty(l,c){var d=t.get(c);if(d===void 0){if(c in l){const u=o(()=>J(ve));t.set(c,u),Ct(i)}}else O(d,ve),Ct(i);return!0},get(l,c,d){var v;if(c===Mn)return n;var u=t.get(c),g=c in l;if(u===void 0&&(!g||(v=Jn(l,c))!=null&&v.writable)&&(u=o(()=>{var p=$e(g?l[c]:ve),h=J(p);return h}),t.set(c,u)),u!==void 0){var _=a(u);return _===ve?void 0:_}return Reflect.get(l,c,d)},getOwnPropertyDescriptor(l,c){var _;(_=this.has)==null||_.call(this,l,c);var d=Reflect.getOwnPropertyDescriptor(l,c),u=t.get(c);if(u!==void 0){var g=a(u);if(g===ve)return;if(d&&"value"in d)d.value=g;else return{enumerable:!0,configurable:!0,value:g,writable:!0}}return d},has(l,c){var _;if(c===Mn)return!0;var d=t.get(c),u=d!==void 0&&d.v!==ve||Reflect.has(l,c);if(d!==void 0||(U!==null||V!==null)&&(!u||(_=Jn(l,c))!=null&&_.writable)){d===void 0&&(d=o(()=>{var v=u?$e(l[c]):ve,p=J(v);return p}),t.set(c,d));var g=a(d);if(g===ve)return!1}return u},set(l,c,d,u){var M;var g=t.get(c),_=c in l;if(r&&c==="length")for(var v=d;v<g.v;v+=1){var p=t.get(v+"");p!==void 0?O(p,ve):v in l&&(p=o(()=>J(ve)),t.set(v+"",p))}if(g===void 0)(!_||(M=Jn(l,c))!=null&&M.writable)&&(g=o(()=>J(void 0)),O(g,$e(d)),t.set(c,g));else{_=g.v!==ve;var h=o(()=>$e(d));O(g,h)}var w=Reflect.getOwnPropertyDescriptor(l,c);if(w!=null&&w.set&&w.set.call(u,d),!_){if(r&&typeof c=="string"){var k=t.get("length"),b=Number(c);Number.isInteger(b)&&b>=k.v&&O(k,b+1)}Ct(i)}return!0},ownKeys(l){a(i);var c=Reflect.ownKeys(l).filter(g=>{var _=t.get(g);return _===void 0||_.v!==ve});for(var[d,u]of t)u.v!==ve&&!(d in l)&&c.push(d);return c},setPrototypeOf(){Bo()}})}function Ci(n){try{if(n!==null&&typeof n=="object"&&Mn in n)return n[Mn]}catch{}return n}function _s(n,e){return Object.is(Ci(n),Ci(e))}var Si,ms,bs,xs;function Qo(){if(Si===void 0){Si=window,ms=/Firefox/.test(navigator.userAgent);var n=Element.prototype,e=Node.prototype,t=Text.prototype;bs=Jn(e,"firstChild").get,xs=Jn(e,"nextSibling").get,ki(n)&&(n[Lr]=void 0,n[er]=null,n[Pr]=void 0,n.__e=void 0),ki(t)&&(t[pt]=void 0)}}function Cn(n=""){return document.createTextNode(n)}function yn(n){return bs.call(n)}function Gt(n){return xs.call(n)}function L(n,e){return yn(n)}function oe(n,e=!1){{var t=yn(n);return t instanceof Comment&&t.data===""?Gt(t):t}}function re(n,e=!1){return yn(n)}function P(n,e=1,t=!1){let r=n;for(;e--;)r=Gt(r);return r}function $o(n){n.textContent=""}function ks(){return!1}function si(n,e,t){return e==null||e===Ui?t?document.createElement(n,{is:t}):document.createElement(n):t?document.createElementNS(e,n,{is:t}):document.createElementNS(e,n)}function el(n){var e=U;if(e===null)return V.f|=Bn,n;if((e.f&ft)===0&&(e.f&lt)===0)throw n;vn(n,e)}function vn(n,e){if(n===Yi)throw n;if(!(e!==null&&(e.f&pe)!==0)){for(;e!==null;){if((e.f&Br)!==0&&(e.f&(pe|At))===0){if((e.f&ft)===0)throw n;try{e.b.error(n);return}catch(t){n=t}}e=e.parent}throw n}}function nl(n){U===null&&(V===null&&Ao(),Eo()),Sn&&So()}function tl(n,e){var t=e.last;t===null?e.last=e.first=n:(t.next=n,n.prev=t,e.last=n)}function En(n,e){var t=U;t!==null&&(t.f&Se)!==0&&(n|=Se);var r={ctx:_e,deps:null,nodes:null,f:n|se|tn,first:null,fn:e,last:null,next:null,parent:t,b:t&&t.b,prev:null,teardown:null,wv:0,ac:null};D==null||D.register_created_effect(r);var i=r;if((n&lt)!==0)Mt!==null?Mt.push(r):Wn.ensure().schedule(r);else if(e!==null){try{dt(r)}catch(o){throw Te(r),o}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&ht)===0&&(i=i.first,(n&Qe)!==0&&(n&Gn)!==0&&i!==null&&(i.f|=Gn))}if(i!==null&&(i.parent=t,t!==null&&tl(i,t),V!==null&&(V.f&ke)!==0&&(n&Pn)===0)){var s=V;(s.effects??(s.effects=[])).push(i)}return r}function oi(){return V!==null&&!en}function ys(n){const e=En(gr,null);return ue(e,fe),e.teardown=n,e}function Vn(n){nl();var e=U.f,t=!V&&(e&sn)!==0&&_e!==null&&!_e.i;if(t){var r=_e;(r.e??(r.e=[])).push(n)}else return Ms(n)}function Ms(n){return En(lt|wo,n)}function rl(n){Wn.ensure();const e=En(Pn|ht,n);return(t={})=>new Promise(r=>{t.outro?Yn(e,()=>{Te(e),r(void 0)}):(Te(e),r(void 0))})}function li(n){return En(lt,n)}function il(n){return En(Qn|ht,n)}function ai(n,e=0){return En(gr|e,n)}function B(n,e=[],t=[],r=[]){Fo(r,e,t,i=>{En(gr,()=>{n(...i.map(a))})})}function Wt(n,e=0){var t=En(Qe|e,n);return t}function Ye(n){return En(sn|ht,n)}function Cs(n){var e=n.teardown;if(e!==null){const t=Sn,r=V;Ei(!0),Ge(null);try{e.call(null)}catch(i){vn(i,n.parent)}finally{Ei(t),Ge(r)}}}function ci(n,e=!1){var t=n.first;for(n.first=n.last=null;t!==null;){const i=t.ac;i!==null&&Ut(()=>{i.abort(Yt)});var r=t.next;(t.f&Pn)!==0?t.parent=null:Te(t,e),t=r}}function sl(n){for(var e=n.first;e!==null;){var t=e.next;(e.f&sn)===0&&Te(e),e=t}}function Te(n,e=!0){var t=!1;(e||(n.f&go)!==0)&&n.nodes!==null&&n.nodes.end!==null&&(Ss(n.nodes.start,n.nodes.end),t=!0),n.f|=At,ci(n,e&&!t),zt(n,0);var r=n.nodes&&n.nodes.t;if(r!==null)for(const s of r)s.stop();Cs(n),n.f^=At,n.f|=pe;var i=n.parent;i!==null&&i.first!==null&&Es(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.fn=n.nodes=n.ac=n.b=null}function Ss(n,e){for(;n!==null;){var t=n===e?null:Gt(n);n.remove(),n=t}}function Es(n){var e=n.parent,t=n.prev,r=n.next;t!==null&&(t.next=r),r!==null&&(r.prev=t),e!==null&&(e.first===n&&(e.first=r),e.last===n&&(e.last=t))}function Yn(n,e,t=!0){var r=[];n.f|=$r,As(n,r,!0);var i=()=>{t&&Te(n),e&&e()},s=r.length;if(s>0){var o=()=>--s||i();for(var l of r)l.out(o)}else i()}function As(n,e,t){if((n.f&Se)===0){n.f^=Se;var r=n.nodes&&n.nodes.t;if(r!==null)for(const l of r)(l.is_global||t)&&e.push(l);for(var i=n.first;i!==null;){var s=i.next;if((i.f&Pn)===0){var o=(i.f&Gn)!==0||(i.f&sn)!==0&&(n.f&Qe)!==0;As(i,e,o?t:!1)}i=s}}}function lr(n){n.f&=~$r,Is(n,!0)}function Is(n,e){if((n.f&$r)===0&&(n.f&Se)!==0){n.f^=Se,(n.f&fe)===0&&(ue(n,se),Wn.ensure().schedule(n));for(var t=n.first;t!==null;){var r=t.next,i=(t.f&Gn)!==0||(t.f&sn)!==0;Is(t,i?e:!1),t=r}var s=n.nodes&&n.nodes.t;if(s!==null)for(const o of s)(o.is_global||e)&&o.in()}}function di(n,e){if(n.nodes)for(var t=n.nodes.start,r=n.nodes.end;t!==null;){var i=t===r?null:Gt(t);e.append(t),t=i}}let Sn=!1;function Ei(n){Sn=n}let V=null,en=!1;function Ge(n){V=n}let U=null;function pn(n){U=n}let wn=null;function zs(n){V!==null&&((V.f&ir)!==0||(V.f&ke)!==0)&&(wn??(wn=new Set)).add(n)}let ze=null,qe=0,Fe=null;function ol(n){Fe=n}let Ts=1,On=0,Un=On;function Ai(n){Un=n}function Bs(){return++Ts}function Xt(n){var e=n.f;if((e&se)!==0)return!0;if((e&Ne)!==0){for(var t=n.deps,r=t.length,i=0;i<r;i++){var s=t[i];if(Xt(s)&&ls(s),s.wv>n.wv)return!0}(e&tn)!==0&&we===null&&ue(n,fe)}return!1}function Ls(n,e,t=!0){var r=n.reactions;if(r!==null&&!(wn!==null&&wn.has(n)))for(var i=0;i<r.length;i++){var s=r[i];(s.f&ke)!==0?Ls(s,e,!1):e===s&&(t?ue(s,se):(s.f&fe)!==0&&ue(s,Ne),ii(s))}}function Ps(n){var e=ze,t=qe,r=Fe,i=V,s=wn,o=_e,l=en,c=Un,d=n.f;ze=null,qe=0,Fe=null,V=(d&(sn|Pn))===0?n:null,wn=null,at(n.ctx),en=!1,Un=++On,n.ac!==null&&(Ut(()=>{n.ac.abort(Yt)}),n.ac=null);try{n.f|=ir;var u=n.fn,g=u();n.f|=ft;var _=Ii(n);if(ts()&&Fe!==null&&!en&&_!==null&&(n.f&(ke|Ne|se))===0)for(var v=0;v<Fe.length;v++)Ls(Fe[v],n);if(i!==null&&i!==n){if(On++,i.deps!==null)for(let p=0;p<t;p+=1)i.deps[p].rv=On;if(e!==null)for(const p of e)p.rv=On;Fe!==null&&(r===null?r=Fe:r.push(...Fe))}return(n.f&Bn)!==0&&(n.f^=Bn),g}catch(p){return Ii(n),el(p)}finally{n.f^=ir,ze=e,qe=t,Fe=r,V=i,wn=s,at(o),en=l,Un=c}}function Ii(n){var i;var e=n.deps,t=D==null?void 0:D.is_fork;if(ze!==null){var r;if(t||zt(n,qe),e!==null&&qe>0)for(e.length=qe+ze.length,r=0;r<ze.length;r++)e[qe+r]=ze[r];else n.deps=e=ze;if(oi()&&(n.f&tn)!==0)for(r=qe;r<e.length;r++)((i=e[r]).reactions??(i.reactions=[])).push(n)}else!t&&e!==null&&qe<e.length&&(zt(n,qe),e.length=qe);return e}function ll(n,e){let t=e.reactions;if(t!==null){var r=co.call(t,n);if(r!==-1){var i=t.length-1;i===0?t=e.reactions=null:(t[r]=t[i],t.pop())}}if(t===null&&(e.f&ke)!==0&&(ze===null||!Et.call(ze,e))){var s=e;(s.f&tn)!==0&&(s.f^=tn),s.v!==ve&&ni(s),s.ac!==null&&Ut(()=>{s.ac.abort(Yt),s.ac=null,ue(s,se)}),Uo(s),zt(s,0)}}function zt(n,e){var t=n.deps;if(t!==null)for(var r=e;r<t.length;r++)ll(n,t[r])}function dt(n){var e=n.f;if((e&pe)===0){ue(n,fe);var t=U;U=n;try{(e&(Qe|Zi))!==0?sl(n):ci(n),Cs(n);var r=Ps(n);n.teardown=typeof r=="function"?r:null,n.wv=Ts;var i;Gi&&jo&&(n.f&se)!==0&&n.deps}finally{U=t}}}async function js(){await Promise.resolve(),Xo()}function a(n){var e=n.f,t=(e&ke)!==0;if(V!==null&&!en){var r=U!==null&&(U.f&pe)!==0;if(!r&&(wn===null||!wn.has(n))){var i=V.deps;if((V.f&ir)!==0)n.rv<On&&(n.rv=On,ze===null&&i!==null&&i[qe]===n?qe++:ze===null?ze=[n]:ze.push(n));else{V.deps??(V.deps=[]),Et.call(V.deps,n)||V.deps.push(n);var s=n.reactions;s===null?n.reactions=[V]:Et.call(s,V)||s.push(V)}}}if(Sn&&gn.has(n))return gn.get(n);if(t){var o=n;if(Sn){var l=o.v;return((o.f&fe)===0&&o.reactions!==null||Os(o))&&(l=ti(o)),gn.set(o,l),l}var c=(o.f&tn)===0&&!en&&V!==null&&(V.f&tn)!==0,d=(o.f&ft)===0;Xt(o)&&(c&&(o.f|=tn),ls(o)),c&&!d&&(as(o),Ds(o))}if(we!=null&&we.has(n))return we.get(n);if((n.f&Bn)!==0)throw n.v;return n.v}function Ds(n){if(n.f|=tn,n.deps!==null)for(const t of n.deps){var e=t.reactions;e===null?t.reactions=[n]:Et.call(e,n)||e.push(n),(t.f&ke)!==0&&(t.f&tn)===0&&(as(t),Ds(t))}}function Os(n){if(n.v===ve)return!0;if(n.deps===null)return!1;for(const e of n.deps)if(gn.has(e)||(e.f&ke)!==0&&Os(e))return!0;return!1}function wr(n){var e=en;try{return en=!0,n()}finally{en=e}}function al(n){if(!(typeof n!="object"||!n||n instanceof EventTarget)){if(Mn in n)Rr(n);else if(!Array.isArray(n))for(let e in n){const t=n[e];typeof t=="object"&&t&&Mn in t&&Rr(t)}}}function Rr(n,e=new Set){if(typeof n=="object"&&n!==null&&!(n instanceof EventTarget)&&!e.has(n)){e.add(n),n instanceof Date&&n.getTime();for(let r in n)try{Rr(n[r],e)}catch{}const t=Qr(n);if(t!==Object.prototype&&t!==Array.prototype&&t!==Map.prototype&&t!==Set.prototype&&t!==Date.prototype){const r=Xi(t);for(let i in r){const s=r[i].get;if(s)try{s.call(n)}catch{}}}}}const qn=Symbol("events"),qs=new Set,Fr=new Set;function cl(n,e,t,r={}){function i(s){if(r.capture||Hr.call(e,s),!s.cancelBubble)return Ut(()=>t==null?void 0:t.call(this,s))}return n.startsWith("pointer")||n.startsWith("touch")||n==="wheel"?(i.__removed=!1,kn(()=>{i.__removed||e.addEventListener(n,i,r)})):e.addEventListener(n,i,r),i}function dl(n,e,t,r={}){var i=cl(e,n,t,r);return()=>{i.__removed=!0,n.removeEventListener(e,i,r)}}function Q(n,e,t){(e[qn]??(e[qn]={}))[n]=t}function _n(n){for(var e=0;e<n.length;e++)qs.add(n[e]);for(var t of Fr)t(n)}let yr=null,Mr=!1;function Hr(n){var h,w;var e=this,t=e.ownerDocument,r=n.type,i=((h=n.composedPath)==null?void 0:h.call(n))||[],s=i[0]||n.target;yr=n,Mr||(Mr=!0,setTimeout(()=>{Mr=!1,yr=null}));var o=0,l=yr===n&&n[qn];if(l){var c=i.indexOf(l);if(c!==-1&&(e===document||e===window)){n[qn]=e;return}var d=i.indexOf(e);if(d===-1)return;c<=d&&(o=c)}if(s=i[o]||n.target,s!==e){Wi(n,"currentTarget",{configurable:!0,get(){return s||t}});var u=V,g=U;Ge(null),pn(null);try{for(var _,v=[];s!==null&&s!==e;){try{var p=(w=s[qn])==null?void 0:w[r];p!=null&&(!s.disabled||n.target===s)&&p.call(s,n)}catch(k){_?v.push(k):_=k}if(n.cancelBubble)break;o++,s=o<i.length?i[o]:null}if(_){for(let k of v)queueMicrotask(()=>{throw k});throw _}}finally{n[qn]=e,delete n.currentTarget,Ge(u),pn(g)}}}var Hi;const Cr=((Hi=globalThis==null?void 0:globalThis.window)==null?void 0:Hi.trustedTypes)&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:n=>n});function ul(n){return(Cr==null?void 0:Cr.createHTML(n))??n}function vl(n){var e=si("template");return e.innerHTML=ul(n.replaceAll("<!>","<!---->")),e.content}function ut(n,e){var t=U;t.nodes===null&&(t.nodes={start:n,end:e,a:null,t:null})}function z(n,e){var t=(e&1)!==0,r=(e&2)!==0,i,s=!n.startsWith("<!>");return()=>{i===void 0&&(i=vl(s?n:"<!>"+n),t||(i=yn(i)));var o=r||ms?document.importNode(i,!0):i.cloneNode(!0);if(t){var l=yn(o),c=o.lastChild;ut(l,c)}else ut(o,o);return o}}function Ns(n=""){{var e=Cn(n+"");return ut(e,e),e}}function We(){var n=document.createDocumentFragment(),e=document.createComment(""),t=Cn();return n.append(e,t),ut(e,t),n}function S(n,e){n!==null&&n.before(e)}const fl=["touchstart","touchmove"];function hl(n){return fl.includes(n)}function gl(n){let e=0,t=Xn(0),r;return()=>{oi()&&(a(t),ai(()=>(e===0&&(r=wr(()=>n(()=>Ct(t)))),e+=1,()=>{kn(()=>{e-=1,e===0&&(r==null||r(),r=void 0,Ct(t))})})))}}var wl=Gn|ht;function pl(n,e,t,r){new _l(n,e,t,r)}var He,Jr,Ve,Rn,xe,je,Ce,De,dn,Fn,Tn,st,Lt,xn,ur,K,ml,bl,Vr,zn,xl,Yr,xt,tr,Ur,Gr;class _l{constructor(e,t,r,i){N(this,K);ge(this,"parent");ge(this,"is_pending",!1);ge(this,"transform_error");N(this,He);N(this,Jr,null);N(this,Ve);N(this,Rn);N(this,xe);N(this,je,null);N(this,Ce,null);N(this,De,null);N(this,dn,null);N(this,Fn,0);N(this,Tn,0);N(this,st,!1);N(this,Lt,new Map);N(this,xn,null);N(this,ur,gl(()=>(H(this,xn,Xn(f(this,Fn))),()=>{H(this,xn,null)})));var s;H(this,He,e),H(this,Ve,t),H(this,Rn,o=>{var l=U;l.b=this,l.f|=Br,r(o)}),this.parent=U.b,this.transform_error=i??((s=this.parent)==null?void 0:s.transform_error)??(o=>o),H(this,xe,Wt(()=>{j(this,K,Yr).call(this)},wl))}defer_effect(e){is(e,f(this,Lt))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!f(this,Ve).pending}update_pending_count(e,t){j(this,K,Ur).call(this,e,t),H(this,Fn,f(this,Fn)+e),!(!f(this,xn)||f(this,st))&&(H(this,st,!0),kn(()=>{H(this,st,!1),f(this,xn)&&ct(f(this,xn),f(this,Fn))}))}get_effect_pending(){return f(this,ur).call(this),a(f(this,xn))}error(e){if(e===Yi||!f(this,Ve).onerror&&!f(this,Ve).failed)throw e;D!=null&&D.is_fork?(f(this,je)&&D.skip_effect(f(this,je)),f(this,Ce)&&D.skip_effect(f(this,Ce)),f(this,De)&&D.skip_effect(f(this,De)),D.oncommit(()=>{j(this,K,zn).call(this)||j(this,K,Gr).call(this,e)})):j(this,K,Gr).call(this,e)}}He=new WeakMap,Jr=new WeakMap,Ve=new WeakMap,Rn=new WeakMap,xe=new WeakMap,je=new WeakMap,Ce=new WeakMap,De=new WeakMap,dn=new WeakMap,Fn=new WeakMap,Tn=new WeakMap,st=new WeakMap,Lt=new WeakMap,xn=new WeakMap,ur=new WeakMap,K=new WeakSet,ml=function(){try{H(this,je,Ye(()=>f(this,Rn).call(this,f(this,He))))}catch(e){this.error(e)}},bl=function(e){const t=f(this,Ve).failed,{reset:r,invoke_onerror:i}=j(this,K,Vr).call(this,e);kn(i),t&&H(this,De,Ye(()=>{t(f(this,He),()=>e,()=>r)}))},Vr=function(e){var t=!1,r=!1;const i=()=>{if(!j(this,K,zn).call(this)){if(t){ko();return}t=!0,r&&Po(),f(this,De)!==null&&Yn(f(this,De),()=>{H(this,De,null)}),j(this,K,tr).call(this,()=>{j(this,K,Yr).call(this)})}};return{reset:i,invoke_onerror:()=>{var o,l;if(!j(this,K,zn).call(this))try{r=!0,(l=(o=f(this,Ve)).onerror)==null||l.call(o,e,i),r=!1}catch(c){vn(c,f(this,xe)&&f(this,xe).parent)}}}},zn=function(){return(f(this,xe).f&(pe|At))!==0},xl=function(){const e=f(this,Ve).pending;e&&(this.is_pending=!0,H(this,Ce,Ye(()=>e(f(this,He)))),kn(()=>{if(!j(this,K,zn).call(this)){var t=H(this,dn,document.createDocumentFragment()),r=Cn(),i=!1;if(t.append(r),H(this,je,j(this,K,tr).call(this,()=>{try{return Ye(()=>f(this,Rn).call(this,r))}catch(s){try{this.error(s),i=!0}catch(o){vn(o,f(this,xe).parent)}return null}})),f(this,je)===null){H(this,dn,null),i&&j(this,K,xt).call(this,D);return}f(this,Tn)===0&&(f(this,He).before(t),H(this,dn,null),Yn(f(this,Ce),()=>{H(this,Ce,null)}),j(this,K,xt).call(this,D))}}))},Yr=function(){try{if(this.is_pending=this.has_pending_snippet(),H(this,Tn,0),H(this,Fn,0),H(this,je,Ye(()=>{f(this,Rn).call(this,f(this,He))})),f(this,Tn)>0){var e=H(this,dn,document.createDocumentFragment());di(f(this,je),e);const t=f(this,Ve).pending;H(this,Ce,Ye(()=>t(f(this,He))))}else j(this,K,xt).call(this,D)}catch(t){this.error(t)}},xt=function(e){this.is_pending=!1,e.transfer_reactions(f(this,Lt))},tr=function(e){var t=U,r=V,i=_e;pn(f(this,xe)),Ge(f(this,xe)),at(f(this,xe).ctx);try{return Wn.ensure(),e()}finally{pn(t),Ge(r),at(i)}},Ur=function(e,t){var r;if(!this.has_pending_snippet()){this.parent&&j(r=this.parent,K,Ur).call(r,e,t);return}H(this,Tn,f(this,Tn)+e),f(this,Tn)===0&&(j(this,K,xt).call(this,t),f(this,Ce)&&Yn(f(this,Ce),()=>{H(this,Ce,null)}),f(this,dn)&&(f(this,He).before(f(this,dn)),H(this,dn,null)))},Gr=function(e){f(this,je)&&(Te(f(this,je)),H(this,je,null)),f(this,Ce)&&(Te(f(this,Ce)),H(this,Ce,null)),f(this,De)&&(Te(f(this,De)),H(this,De,null));let t=f(this,Ve).failed;const r=i=>{if(j(this,K,zn).call(this))return;const{reset:s,invoke_onerror:o}=j(this,K,Vr).call(this,i);o(),t&&!j(this,K,zn).call(this)&&H(this,De,j(this,K,tr).call(this,()=>{try{return Ye(()=>{var l=U;l.b=this,l.f|=Br,t(f(this,He),()=>i,()=>s)})}catch(l){return vn(l,f(this,xe).parent),null}}))};kn(()=>{if(!j(this,K,zn).call(this)){var i;try{i=this.transform_error(e)}catch(s){vn(s,f(this,xe)&&f(this,xe).parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(r,s=>vn(s,f(this,xe)&&f(this,xe).parent)):r(i)}})};function Y(n,e){var t=e==null?"":typeof e=="object"?`${e}`:e;t!==(n[pt]??(n[pt]=n.nodeValue))&&(n[pt]=t,n.nodeValue=`${t}`)}function Rs(n,e){return kl(n,e)}const Qt=new Map;function kl(n,{target:e,anchor:t,props:r={},events:i,context:s,intro:o=!0,transformError:l}){Qo();var c=void 0,d=rl(()=>{var u=t??e.appendChild(Cn());pl(u,{pending:()=>{}},v=>{Ee({});var p=_e;s&&(p.c=s),i&&(r.$$events=i),c=n(v,r)||ei(),Ae()},l);var g=new Set,_=v=>{for(var p=0;p<v.length;p++){var h=v[p];if(!g.has(h)){g.add(h);var w=hl(h);for(const M of[e,document]){var k=Qt.get(M);k===void 0&&(k=new Map,Qt.set(M,k));var b=k.get(h);b===void 0?(M.addEventListener(h,Hr,{passive:w}),k.set(h,1)):k.set(h,b+1)}}}};return _(hr(qs)),Fr.add(_),()=>{var w;for(var v of g)for(const k of[e,document]){var p=Qt.get(k),h=p.get(v);--h==0?(k.removeEventListener(v,Hr),p.delete(v),p.size===0&&Qt.delete(k)):p.set(v,h)}Fr.delete(_),u!==t&&((w=u.parentNode)==null||w.removeChild(u))}});return Wr.set(c,d),c}let Wr=new WeakMap;function Fs(n,e){const t=Wr.get(n);return t?(Wr.delete(n),t(e)):Promise.resolve()}var Je,un,Oe,Hn,Pt,jt,vr;class ui{constructor(e,t=!0){ge(this,"anchor");N(this,Je,new Map);N(this,un,new Map);N(this,Oe,new Map);N(this,Hn,new Set);N(this,Pt,!0);N(this,jt,e=>{if(f(this,Je).has(e)){var t=f(this,Je).get(e),r=f(this,un).get(t);if(r)lr(r),f(this,Hn).delete(t);else{var i=f(this,Oe).get(t);i&&(lr(i.effect),f(this,un).set(t,i.effect),f(this,Oe).delete(t),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),r=i.effect)}for(const[s,o]of f(this,Je)){if(f(this,Je).delete(s),s===e)break;const l=f(this,Oe).get(o);l&&(Te(l.effect),f(this,Oe).delete(o))}for(const[s,o]of f(this,un)){if(s===t||f(this,Hn).has(s))continue;const l=()=>{if(Array.from(f(this,Je).values()).includes(s)){var d=document.createDocumentFragment();di(o,d),d.append(Cn()),f(this,Oe).set(s,{effect:o,fragment:d})}else Te(o);f(this,Hn).delete(s),f(this,un).delete(s)};f(this,Pt)||!r?(f(this,Hn).add(s),Yn(o,l,!1)):l()}}});N(this,vr,e=>{f(this,Je).delete(e);const t=Array.from(f(this,Je).values());for(const[r,i]of f(this,Oe))t.includes(r)||(Te(i.effect),f(this,Oe).delete(r))});this.anchor=e,H(this,Pt,t)}ensure(e,t){var r=D,i=ks();if(t&&!f(this,un).has(e)&&!f(this,Oe).has(e))if(i){var s=document.createDocumentFragment(),o=Cn();s.append(o),f(this,Oe).set(e,{effect:Ye(()=>t(o)),fragment:s})}else f(this,un).set(e,Ye(()=>t(this.anchor)));if(f(this,Je).set(r,e),i){for(const[l,c]of f(this,un))l===e?r.unskip_effect(c):r.skip_effect(c);for(const[l,c]of f(this,Oe))l===e?r.unskip_effect(c.effect):r.skip_effect(c.effect);r.oncommit(f(this,jt)),r.ondiscard(f(this,vr))}else f(this,jt).call(this,r)}}Je=new WeakMap,un=new WeakMap,Oe=new WeakMap,Hn=new WeakMap,Pt=new WeakMap,jt=new WeakMap,vr=new WeakMap;function Re(n,e,...t){var r=new ui(n);Wt(()=>{const i=e()??null;r.ensure(i,i&&(s=>i(s,...t)))},Gn)}function W(n,e,t=!1){var r=new ui(n),i=t?Gn:0;function s(o,l){r.ensure(o,l)}Wt(()=>{var o=!1;e((l,c=0)=>{o=!0,s(c,l)}),o||s(-1,null)},i)}const yl=Symbol("NaN");function Ml(n,e,t){var r=new ui(n);Wt(()=>{var i=e();i!==i&&(i=yl),r.ensure(i,t)})}function ar(n,e){return e}function Cl(n,e,t){for(var r=[],i=e.length,s,o=e.length,l=0;l<i;l++){let g=e[l];Yn(g,()=>{if(s){if(s.pending.delete(g),s.done.add(g),s.pending.size===0){var _=n.outrogroups;Xr(n,hr(s.done)),_.delete(s),_.size===0&&(n.outrogroups=null)}}else o-=1},!1)}if(o===0){var c=r.length===0&&t!==null&&n.pending.size===0;if(c){var d=t,u=d.parentNode;$o(u),u.append(d),n.items.clear()}Xr(n,e,!c)}else s={pending:new Set(e),done:new Set},(n.outrogroups??(n.outrogroups=new Set)).add(s)}function Xr(n,e,t=!0){var r;if(n.pending.size>0){r=new Set;for(const o of n.pending.values())for(const l of o)r.add(n.items.get(l).e)}for(var i=0;i<e.length;i++){var s=e[i];if(r!=null&&r.has(s)){s.f|=fn;const o=document.createDocumentFragment();di(s,o)}else Te(e[i],t)}}var zi;function Ue(n,e,t,r,i,s=null){var o=n,l=new Map,c=(e&4)!==0;if(c){var d=n;o=d.appendChild(Cn())}var u=null,g=os(()=>{var b=t();return fr(b)?b:b==null?[]:hr(b)}),_=new Map,v=!0;function p(b){if((k.effect.f&pe)===0){k.pending.delete(b);var M=a(g);k.fallback=u,Sl(k,M,o,e,r),u!==null&&(M.length===0?(u.f&fn)===0?lr(u):(u.f^=fn,kt(u,null,o)):Yn(u,()=>{u=null}))}}function h(b){k.pending.delete(b)}var w=Wt(()=>{for(var b=a(g),M=b.length,E=new Set,y=D,A=ks(),x=0;x<M;x+=1){var T=b[x],q=r(T,x),G=v?null:l.get(q);G?(G.v&&ct(G.v,T),G.i&&ct(G.i,x),A&&y.unskip_effect(G.e)):(G=El(l,v?o:zi??(zi=Cn()),T,q,x,i,e,t),v||(G.e.f|=fn),l.set(q,G)),E.add(q)}if(M===0&&s&&!u&&(v?u=Ye(()=>s(o)):(u=Ye(()=>s(zi??(zi=Cn()))),u.f|=fn)),M>E.size&&Co(),!v)if(_.set(y,E),A){for(const[te,$]of l)E.has(te)||y.skip_effect($.e);y.oncommit(p),y.ondiscard(h)}else p(y);a(g)}),k={effect:w,items:l,pending:_,outrogroups:null,fallback:u};v=!1}function wt(n){for(;n!==null&&(n.f&sn)===0;)n=n.next;return n}function Sl(n,e,t,r,i){var G,te,$,le,me,F,ae,he,An;var s=(r&8)!==0,o=e.length,l=n.items,c=wt(n.effect.first),d,u=null,g,_=[],v=[],p,h,w,k;if(s)for(k=0;k<o;k+=1)p=e[k],h=i(p,k),w=l.get(h).e,(w.f&fn)===0&&((te=(G=w.nodes)==null?void 0:G.a)==null||te.measure(),(g??(g=new Set)).add(w));for(k=0;k<o;k+=1){if(p=e[k],h=i(p,k),w=l.get(h).e,n.outrogroups!==null)for(const ye of n.outrogroups)ye.pending.delete(w),ye.done.delete(w);if((w.f&Se)!==0&&(lr(w),s&&((le=($=w.nodes)==null?void 0:$.a)==null||le.unfix(),(g??(g=new Set)).delete(w))),(w.f&fn)!==0)if(w.f^=fn,w===c)kt(w,null,t);else{var b=u?u.next:c;w===n.effect.last&&(n.effect.last=w.prev),w.prev&&(w.prev.next=w.next),w.next&&(w.next.prev=w.prev),In(n,u,w),In(n,w,b),kt(w,b,t),u=w,_=[],v=[],c=wt(u.next);continue}if(w!==c){if(d!==void 0&&d.has(w)){if(_.length<v.length){var M=v[0],E;u=M.prev;var y=_[0],A=_[_.length-1];for(E=0;E<_.length;E+=1)kt(_[E],M,t);for(E=0;E<v.length;E+=1)d.delete(v[E]);In(n,y.prev,A.next),In(n,u,y),In(n,A,M),c=M,u=A,k-=1,_=[],v=[]}else d.delete(w),kt(w,c,t),In(n,w.prev,w.next),In(n,w,u===null?n.effect.first:u.next),In(n,u,w),u=w;continue}for(_=[],v=[];c!==null&&c!==w;)(d??(d=new Set)).add(c),v.push(c),c=wt(c.next);if(c===null)continue}(w.f&fn)===0&&_.push(w),u=w,c=wt(w.next)}if(n.outrogroups!==null){for(const ye of n.outrogroups)ye.pending.size===0&&(Xr(n,hr(ye.done)),(me=n.outrogroups)==null||me.delete(ye));n.outrogroups.size===0&&(n.outrogroups=null)}if(c!==null||d!==void 0){var x=[];if(d!==void 0)for(w of d)(w.f&Se)===0&&x.push(w);for(;c!==null;)(c.f&Se)===0&&c!==n.fallback&&x.push(c),c=wt(c.next);var T=x.length;if(T>0){var q=(r&4)!==0&&o===0?t:null;if(s){for(k=0;k<T;k+=1)(ae=(F=x[k].nodes)==null?void 0:F.a)==null||ae.measure();for(k=0;k<T;k+=1)(An=(he=x[k].nodes)==null?void 0:he.a)==null||An.fix()}Cl(n,x,q)}}s&&kn(()=>{var ye,Kt;if(g!==void 0)for(w of g)(Kt=(ye=w.nodes)==null?void 0:ye.a)==null||Kt.apply()})}function El(n,e,t,r,i,s,o,l){var c=(o&1)!==0?(o&16)===0?Zo(t,!1,!1):Xn(t):null,d=(o&2)!==0?Xn(i):null;return{v:c,i:d,e:Ye(()=>(s(e,c??t,d??i,l),()=>{n.delete(r)}))}}function kt(n,e,t){if(n.nodes)for(var r=n.nodes.start,i=n.nodes.end,s=e&&(e.f&fn)===0?e.nodes.start:t;r!==null;){var o=Gt(r);if(s.before(r),r===i)return;r=o}}function In(n,e,t){e===null?n.effect.first=t:e.next=t,t===null?n.effect.last=e:t.prev=e}function Al(n,e,t=!1,r=!1,i=!1,s=!1){var o=n,l="";if(t)var c=n;B(()=>{var d=U;if(l!==(l=e()??"")){if(t){d.nodes=null,c.innerHTML=l,l!==""&&ut(yn(c),c.lastChild);return}if(d.nodes!==null&&(Ss(d.nodes.start,d.nodes.end),d.nodes=null),l!==""){var u=r?lo:i?ao:void 0,g=si(r?"svg":i?"math":"template",u);g.innerHTML=l;var _=r||i?g:g.content;if(ut(yn(_),_.lastChild),r||i)for(;yn(_);)o.before(yn(_));else o.before(_)}}})}function Be(n,e){li(()=>{var i,s;n=((s=(i=U==null?void 0:U.parent)==null?void 0:i.nodes)==null?void 0:s.start)??n;var t=n.getRootNode(),r=t.host?t:t.head??t.ownerDocument.head;if(!r.querySelector("#"+e.hash)){const o=si("style");o.id=e.hash,o.textContent=e.code,r.appendChild(o)}})}function Tt(n,e,t){li(()=>{var r=wr(()=>e(n,t==null?void 0:t())||{});if(t&&(r!=null&&r.update)){var i=!1,s={};ai(()=>{var o=t();al(o),i&&$i(s,o)&&(s=o,r.update(o))}),i=!0}if(r!=null&&r.destroy)return()=>r.destroy()})}const Ti=[...` 	
\r\f \v\uFEFF`];function Il(n,e,t){var r=n==null?"":""+n;if(t){for(var i of Object.keys(t))if(t[i])r=r?r+" "+i:i;else if(r.length)for(var s=i.length,o=0;(o=r.indexOf(i,o))>=0;){var l=o+s;(o===0||Ti.includes(r[o-1]))&&(l===r.length||Ti.includes(r[l]))?r=(o===0?"":r.substring(0,o))+r.substring(l+1):o=l}}return r===""?null:r}function Bi(n,e=!1){var t=e?" !important;":";",r="";for(var i of Object.keys(n)){var s=n[i];s!=null&&s!==""&&(r+=" "+i+": "+s+t)}return r}function zl(n,e){if(e){var t="",r,i;return Array.isArray(e)?(r=e[0],i=e[1]):r=e,r&&(t+=Bi(r)),i&&(t+=Bi(i,!0)),t=t.trim(),t===""?null:t}return String(n)}function Xe(n,e,t,r,i,s){var o=n[Lr];if(o!==t||o===void 0){var l=Il(t,r,s);l==null?n.removeAttribute("class"):n.className=l,n[Lr]=t}else if(s&&i!==s)for(var c in s){var d=!!s[c];(i==null||d!==!!i[c])&&n.classList.toggle(c,d)}return s}function Sr(n,e={},t,r){for(var i in t){var s=t[i];e[i]!==s&&(t[i]==null?n.style.removeProperty(i):n.style.setProperty(i,s,r))}}function gt(n,e,t,r){var i=n[Pr];if(i!==e){var s=zl(e,r);s==null?n.removeAttribute("style"):n.style.cssText=s,n[Pr]=e}else r&&(Array.isArray(r)?(Sr(n,t==null?void 0:t[0],r[0]),Sr(n,t==null?void 0:t[1],r[1],"important")):Sr(n,t,r));return r}function Tl(n,e){e?n.hasAttribute("selected")||n.setAttribute("selected",""):n.removeAttribute("selected")}function Bl(n,e){var t=n.__defaultValue,r=n.multiple,i=r?t??[]:null;if(!(r&&!fr(i))){n.selectedIndex;for(var s of n.options){var o=Kr(s);Tl(s,r?i.includes(o):_s(o,t))}}}function Hs(n,e,t=!1){if(n.multiple){if(e==null)return;if(!fr(e))return xo();for(var r of n.options)r.selected=e.includes(Kr(r));return}for(r of n.options){var i=Kr(r);if(_s(i,e)){r.selected=!0;return}}(!t||e!==void 0)&&(n.selectedIndex=-1)}function Ll(n){var e=new MutationObserver(t=>{t.every(Pl)||("__defaultValue"in n&&Bl(n),"__value"in n&&Hs(n,n.__value))});e.observe(n,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),ys(()=>{e.disconnect()})}function Kr(n){return"__value"in n?n.__value:n.value}function Pl(n){if(n.target.closest("selectedcontent")!==null)return!0;if(n.type==="childList"){var e=[...n.addedNodes,...n.removedNodes];return e.length>0&&e.every(t=>t.nodeName==="SELECTEDCONTENT")}return!1}const jl=Symbol("is custom element"),Dl=Symbol("is html"),Ol=mo?"progress":"PROGRESS";function ql(n,e){var t=Vs(n);t.value===(t.value=e??void 0)||n.value===e&&(e!==0||n.nodeName!==Ol)||(n.value=e??"")}function Z(n,e,t,r){var i=Vs(n),s=i[e];s!==(i[e]=t)&&(e==="loading"&&(n[_o]=t),t==null?n.removeAttribute(e):typeof t!="string"&&Nl(n).has(e)?n[e]=t:n.setAttribute(e,t))}function Vs(n){return n[er]??(n[er]={[jl]:n.nodeName.includes("-"),[Dl]:n.namespaceURI===Ui})}var Li=new Map;function Nl(n){var e=n.getAttribute("is")||n.nodeName,t=Li.get(e);if(t)return t;Li.set(e,t=new Set);for(var r,i=n,s=Element.prototype;s!==i;){r=Xi(i);for(var o in r)r[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&t.add(o);i=Qr(i)}return t}function Er(n,e){return n===e||(n==null?void 0:n[Mn])===e}function vt(n=ei(),e,t,r){var i=_e.r,s=U;return li(()=>{var o,l;return ai(()=>{o=l,l=[],wr(()=>{Er(t(...l),n)||(e(n,...l),o&&Er(t(...o),n)&&e(null,...o))})}),()=>{let c=s;for(;c!==i&&c.parent!==null&&c.parent.f&At;)c=c.parent;const d=()=>{l&&Er(t(...l),n)&&e(null,...l)},u=c.teardown;c.teardown=()=>{d(),u==null||u()}}}),n}let $t=!1;function Rl(n){var e=$t;try{return $t=!1,[n(),$t]}finally{$t=e}}function nn(n,e,t,r){var E;var i=!0,s=(t&8)!==0,o=(t&16)!==0,l=r,c=!0,d=void 0,u=()=>o&&i?(d??(d=It(r)),a(d)):(c&&(c=!1,l=o?wr(r):r),l);let g;if(s){var _=Mn in n||po in n;g=((E=Jn(n,e))==null?void 0:E.set)??(_&&e in n?y=>n[e]=y:void 0)}var v,p=!1;s?[v,p]=Rl(()=>n[e]):v=n[e],v===void 0&&r!==void 0&&(v=u(),g&&(zo(),g(v)));var h;if(h=()=>{var y=n[e];return y===void 0?u():(c=!0,y)},(t&4)===0)return h;if(g){var w=n.$$legacy;return(function(y,A){return arguments.length>0?((!A||w||p)&&g(A?h():y),y):h()})}var k=!1,b=((t&1)!==0?It:os)(()=>(k=!1,h()));s&&a(b);var M=U;return(function(y,A){if(arguments.length>0){const x=A?a(b):s?$e(y):y;return O(b,x),k=!0,l!==void 0&&(l=x),y}return Sn&&k||(M.f&pe)!==0?b.v:a(b)})}function Fl(n){return(e,t)=>{let r=t.onchange;const i=$e({...t,onchange:o=>{i.value=o,r(o)}}),s=Rs(n,{target:e,props:i});return{update({onchange:o,...l}){o&&(r=o),Object.assign(i,l)},destroy(){Fs(s)}}}}const Hl="5";var Vi;typeof window<"u"&&((Vi=window.__svelte??(window.__svelte={})).v??(Vi.v=new Set)).add(Hl);const Vl={actions:"Actions",back:"Back",breadcrumbs:"Breadcrumbs",cantMove:"It can't move",changed:"Changed",clear:"Clear",close:"Close",collapse:"Collapse",date:"Date",disabled:"Disabled",dragOrClick:"drag, or click for the moves",done:"Done",duplicate:"Duplicate",expand:"Expand",forward:"Forward",hasErrors:"Has errors",hide:"Hide it on the website",monthAfter:"Month after",monthBefore:"Month before",move:"Move",moveDown:"Move down",moveEarlier:"Move earlier",moveIn:"Move under the one above",moveLater:"Move later",moveOut:"Move out a level",moveUp:"Move up",noDate:"No date",nothingFound:"Nothing found",nothingYet:"Nothing yet",pickDay:"Pick a day",pickMonth:"Pick a month",pickYear:"Pick a year",remove:"Remove",resize:"Drag to change the width, double click for the default",search:"Search",show:"Show it on the website",time:"time",today:"Today"},Ar=Symbol.for("zipp-ui"),Yl=$e({texts:Vl,locale:void 0}),Ir=globalThis,hn=Ir[Ar]??(Ir[Ar]=Yl),Ul=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Gl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Wl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Kl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Zl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Jl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ql=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$l=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ea=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ta=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,ra=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,oa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,la=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,va=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,_a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ea=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,za=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Ta=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,La=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Oa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,qa=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Va=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ka=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Za=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Qa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ec=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,nc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,tc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,rc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ic=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,oc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,lc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ac=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,cc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,dc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,uc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,vc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,fc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,hc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`;var gc=z('<span class="icon svelte-oymxvd"></span>');const wc={hash:"svelte-oymxvd",code:".icon.svelte-oymxvd {display:inline-flex;flex-shrink:0;}.icon.svelte-oymxvd svg {width:100%;height:100%;stroke-width:1.75;}"};function rn(n,e){Ee(e,!0),Be(n,wc);const t=Object.assign({"./icons/add-image.svg":Ul,"./icons/add-video.svg":Gl,"./icons/add.svg":Wl,"./icons/alert.svg":Xl,"./icons/all_pages.svg":Kl,"./icons/arrow-down.svg":Zl,"./icons/arrow-left.svg":Jl,"./icons/arrow-right.svg":Ql,"./icons/arrow-up.svg":$l,"./icons/block.svg":ea,"./icons/bold.svg":na,"./icons/button.svg":ta,"./icons/calendar.svg":ra,"./icons/channel.svg":ia,"./icons/check.svg":sa,"./icons/chevron-down.svg":oa,"./icons/chevron-left.svg":la,"./icons/chevron-right.svg":aa,"./icons/chevrons-up-down.svg":ca,"./icons/close.svg":da,"./icons/copy-url.svg":ua,"./icons/copy.svg":va,"./icons/crop.svg":fa,"./icons/dashboard.svg":ha,"./icons/default.svg":ga,"./icons/delete.svg":wa,"./icons/desktop.svg":pa,"./icons/doc.svg":_a,"./icons/download.svg":ma,"./icons/edit.svg":ba,"./icons/entries.svg":xa,"./icons/external.svg":ka,"./icons/file.svg":ya,"./icons/flip-h.svg":Ma,"./icons/flip-v.svg":Ca,"./icons/focal.svg":Sa,"./icons/folder-plus.svg":Ea,"./icons/folder.svg":Aa,"./icons/globe.svg":Ia,"./icons/graphql.svg":za,"./icons/grip.svg":Ta,"./icons/help.svg":Ba,"./icons/hide.svg":La,"./icons/indent.svg":Pa,"./icons/info.svg":ja,"./icons/italic.svg":Da,"./icons/link.svg":Oa,"./icons/live-preview.svg":qa,"./icons/logout.svg":Na,"./icons/mail.svg":Ra,"./icons/mobile.svg":Fa,"./icons/more.svg":Ha,"./icons/move.svg":Va,"./icons/numbered.svg":Ya,"./icons/outdent.svg":Ua,"./icons/page.svg":Ga,"./icons/page_cat.svg":Wa,"./icons/phone.svg":Xa,"./icons/play.svg":Ka,"./icons/reload.svg":Za,"./icons/replace.svg":Ja,"./icons/reset.svg":Qa,"./icons/rotate-left.svg":$a,"./icons/rotate-right.svg":ec,"./icons/save.svg":nc,"./icons/search.svg":tc,"./icons/settings.svg":rc,"./icons/sidebar.svg":ic,"./icons/single.svg":sc,"./icons/size.svg":oc,"./icons/sparkles.svg":lc,"./icons/structure.svg":ac,"./icons/tablet.svg":cc,"./icons/translate.svg":dc,"./icons/unlink.svg":uc,"./icons/upload.svg":vc,"./icons/users.svg":fc,"./icons/view.svg":hc});let r=nn(e,"size",3,16);var i=gc();let s;Al(i,()=>t[`./icons/${e.name}.svg`],!0),B(()=>s=gt(i,"",s,{width:`${r()/16}rem`,height:`${r()/16}rem`})),S(n,i),Ae()}const zr=Symbol.for("zipp-ui-layers"),Tr=globalThis,Ys=Tr[zr]??(Tr[zr]={stack:[],onkeydown(n){const e=Ys.stack;n.key!=="Escape"||!e.length||document.querySelector("dialog:modal")||(n.preventDefault(),e[e.length-1]())}});function pc(n){const{stack:e,onkeydown:t}=Ys;return e.length||window.addEventListener("keydown",t),e.push(n),()=>{const r=e.indexOf(n);r>=0&&e.splice(r,1),e.length||window.removeEventListener("keydown",t)}}function $n(n,e){const t=document.activeElement;let r=e;const i=pc(()=>r());return{update(s){r=s},destroy(){i(),t!=null&&t.isConnected&&t.focus()}}}const _c='a[href], button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]';function mc(n,e){if(n.key!=="Tab")return;const t=[...e.querySelectorAll(_c)].filter(o=>o.offsetParent!==null&&!o.matches('[tabindex="-1"]'));if(!t.length)return;const[r,i]=[t[0],t[t.length-1]],s=document.activeElement;n.shiftKey&&(s===r||s===e)?(n.preventDefault(),i.focus()):!n.shiftKey&&s===i&&(n.preventDefault(),r.focus())}function bc(n,e,t,r=!1){if(t)switch(n){case"ArrowDown":return r?(e+1)%t:Math.min(e+1,t-1);case"ArrowUp":return r?e<=0?t-1:e-1:Math.max(e-1,0);case"Home":return 0;case"End":return t-1}}var xc=z('<div class="popup svelte-1hgkc3f" tabindex="-1"><!></div>'),kc=z('<div class="popover svelte-1hgkc3f"><!> <!></div>');const yc={hash:"svelte-1hgkc3f",code:".popover.svelte-1hgkc3f {position:relative;}.popup.svelte-1hgkc3f {position:fixed;z-index:calc(var(--z-overlay) + 1);display:flex;flex-direction:column;gap:2px;max-width:calc(100vw - 1rem);}"};function vi(n,e){Ee(e,!0),Be(n,yc);let t=nn(e,"open",15,!1),r=nn(e,"at",15,null),i=nn(e,"align",3,"start"),s=nn(e,"menu",3,!1),o=J(void 0),l=J(void 0);const c=()=>t(!t()),d=()=>t(!1);Vn(()=>{t()||r(null)});const u=8,g=4;function _(b,M){let E=M;const y=()=>{const x=b.getBoundingClientRect();let T,q;if(E)T=E.x,q=E.y+x.height>innerHeight-u?E.y-x.height:E.y;else{const G=a(o).getBoundingClientRect();T=i()==="end"?G.right-x.width:i()==="center"?G.left+G.width/2-x.width/2:G.left;const te=G.bottom+g,$=G.top-g-x.height;q=te+x.height>innerHeight-u&&$>=u?$:te}T=Math.min(T,innerWidth-x.width-u),b.style.left=`${Math.max(T,u)}px`,b.style.top=`${Math.max(q,u)}px`};y(),addEventListener("scroll",y,!0),addEventListener("resize",y);const A=new ResizeObserver(y);return A.observe(b),{update(x){E=x,y()},destroy(){removeEventListener("scroll",y,!0),removeEventListener("resize",y),A.disconnect()}}}Vn(()=>{if(!t())return;const b=M=>{var E;(E=a(o))!=null&&E.contains(M.target)||d()};return window.addEventListener("pointerdown",b),()=>window.removeEventListener("pointerdown",b)}),Vn(()=>{var M,E;if(!t())return;const b=s()?"[role=menuitem]":"input";(E=(M=a(l))==null?void 0:M.querySelector(b))==null||E.focus()});function v(b){if(!s()||!a(l))return;const M=[...a(l).querySelectorAll("[role=menuitem]")],E=M.indexOf(document.activeElement),y=bc(b.key,E,M.length,!0);y!==void 0&&(b.preventDefault(),M[y].focus())}var p=kc(),h=L(p);Re(h,()=>e.trigger??ot,()=>({toggle:c,open:t()}));var w=P(h,2);{var k=b=>{var M=xc();let E;var y=L(M);Re(y,()=>e.children),vt(M,A=>O(l,A),()=>a(l)),Tt(M,(A,x)=>$n==null?void 0:$n(A,x),()=>d),Tt(M,(A,x)=>_==null?void 0:_(A,x),r),B(()=>{Z(M,"role",s()?"menu":"dialog"),Z(M,"aria-label",e.label),E=gt(M,"",E,{width:e.width})}),Q("keydown",M,v),S(b,M)};W(w,b=>{t()&&b(k)})}vt(p,b=>O(o,b),()=>a(o)),S(n,p),Ae()}_n(["keydown"]);var Mc=z('<button type="button" class="icon drag-handle svelte-nnt03w" data-drag-handle=""><!></button>'),Cc=z('<button type="button" class="popup-item" role="menuitem"><!> </button>'),Sc=z('<span class="popup-item note"> </span>');const Ec={hash:"svelte-nnt03w",code:`.drag-handle.svelte-nnt03w {cursor:grab;color:var(--subtle);}

@media (hover: hover) {.drag-handle.svelte-nnt03w:hover {color:var(--text);}
}`};function Us(n,e){Ee(e,!0),Be(n,Ec);const t=ne(()=>e.label??hn.texts.move),r=()=>typeof e.index=="function"?e.index():e.index;let i=J(!1);const s=[["up","moveUp","arrow-up"],["down","moveDown","arrow-down"],["out","moveOut","outdent"],["in","moveIn","indent"]],o={up:["moveEarlier","arrow-left"],down:["moveLater","arrow-right"]},l={ArrowUp:"up",ArrowDown:"down",ArrowLeft:"out",ArrowRight:"in"},c=ne(()=>a(i)?s.filter(([h])=>e.sortable.stepDrop(r(),h)).map(([h,w,k])=>e.sortable.grid()&&o[h]?[h,...o[h]]:[h,w,k]):[]),d={ArrowLeft:"up",ArrowRight:"down"};function u(h){const w=e.sortable.grid()&&d[h.key]||l[h.key];w&&(h.preventDefault(),e.sortable.step(r(),w))}var g=We(),_=oe(g);{var v=h=>{vi(h,{width:"13rem",get label(){return a(t)},menu:!0,get open(){return a(i)},set open(k){O(i,k,!0)},trigger:k=>{var b=Mc();gt(b,"",{},{"touch-action":"none"});var M=L(b);rn(M,{name:"grip",size:15}),B(()=>{Z(b,"title",`${a(t)??""}: ${hn.texts.dragOrClick??""}`),Z(b,"aria-label",a(t)),Z(b,"aria-expanded",a(i))}),Q("pointerdown",b,E=>e.sortable.start(E,r())),Q("click",b,()=>!e.sortable.dragged&&O(i,!a(i))),Q("keydown",b,u),S(k,b)},children:(k,b)=>{var M=We(),E=oe(M);Ue(E,17,()=>a(c),([y,A,x])=>y,(y,A)=>{var x=ne(()=>ho(a(A),3));let T=()=>a(x)[0],q=()=>a(x)[1],G=()=>a(x)[2];var te=Cc(),$=L(te);rn($,{get name(){return G()},size:14});var le=P($);B(()=>Y(le,` ${hn.texts[q()]??""}`)),Q("click",te,()=>{O(i,!1),e.sortable.step(r(),T())}),S(y,te)},y=>{var A=Sc(),x=re(A,!0);B(()=>Y(x,hn.texts.cantMove)),S(y,A)}),S(k,M)},$$slots:{trigger:!0,default:!0}})},p=ne(()=>e.sortable.movable());W(_,h=>{a(p)&&h(v)})}S(n,g),Ae()}_n(["pointerdown","click","keydown"]);var Ac=z('<span class="info svelte-l93724"><!></span>'),Ic=z('<label class="svelte-l93724"><span> </span> <!></label>'),zc=z('<div class="field-row svelte-l93724"><!> <div><!></div></div>');const Tc={hash:"svelte-l93724",code:`.field-row.svelte-l93724 {display:grid;grid-template-columns:8.125rem minmax(0, 1fr);gap:1.25rem;align-items:center;min-width:0;}label.svelte-l93724 {align-self:start;display:flex;flex-direction:column;gap:1px;padding-top:0.4375rem;font-size:var(--fs-md);font-weight:600;color:var(--text);}.info.svelte-l93724 {color:var(--muted);font-size:var(--fs-sm);font-weight:400;}.value.svelte-l93724 {min-width:0;}.alone.svelte-l93724 {grid-column:2;}

@container (width < 26rem) {.field-row.svelte-l93724 {display:flex;flex-direction:column;align-items:stretch;gap:0.375rem;}label.svelte-l93724 {padding-top:0;}
}`};function Bc(n,e){Be(n,Tc);let t=nn(e,"label",3,"");var r=zc(),i=L(r);{var s=d=>{var u=Ic(),g=L(u),_=re(g,!0),v=P(g,2);{var p=h=>{var w=Ac(),k=L(w);Re(k,()=>e.info),S(h,w)};W(v,h=>{e.info&&h(p)})}B(()=>{Z(u,"for",e.for),Y(_,t())}),S(d,u)};W(i,d=>{t()&&d(s)})}var o=P(i,2);let l;var c=L(o);Re(c,()=>e.children),B(()=>l=Xe(o,1,"value svelte-l93724",null,l,{alone:!t()})),S(n,r)}function Gs(n){const e=Symbol(n);return{provide:t=>void qo(e,t),get:()=>Oo(e)}}const Lc=Gs("page-lead"),Ws=Gs("flyout-bar");var Pc=z('<div class="page svelte-22zuw3"><!></div> <button type="button" class="icon"><!></button>',1),jc=z('<div class="actions svelte-22zuw3"><!></div>'),Dc=z('<button type="button" class="primary"><!> </button>'),Oc=z('<strong class="truncate"> </strong> <!>',1),qc=z('<div class="backdrop" role="presentation"></div> <div class="flyout svelte-22zuw3" role="dialog" aria-modal="true" tabindex="-1"><div class="bar svelte-22zuw3"><!></div> <div><!></div></div>',1);const Nc={hash:"svelte-22zuw3",code:`.flyout.svelte-22zuw3 {position:fixed;top:0;right:0;bottom:0;z-index:var(--z-overlay);display:flex;flex-direction:column;width:min(65rem, 92vw);background:var(--bg);box-shadow:var(--shadow-flyout);outline:none;
  animation: svelte-22zuw3-slide 0.18s ease-out;}

@media (max-width: 799px) {.flyout.svelte-22zuw3 {width:100vw;}
}
@keyframes svelte-22zuw3-slide {
  from {
    transform: translateX(2.5rem);
    opacity: 0;
  }
}.bar.svelte-22zuw3 {position:relative;z-index:var(--z-bar);display:flex;align-items:center;justify-content:space-between;gap:0.75rem;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.actions.svelte-22zuw3 {display:flex;gap:0.5rem;}

/* the page inside scrolls, its top bar sticks to this */.body.svelte-22zuw3 {flex:1;min-height:0;padding:0 var(--page-pad) var(--page-pad);overflow-y:auto;}.page-in.svelte-22zuw3 {padding-top:1.25rem;}.page.svelte-22zuw3 {display:flex;align-items:center;gap:0.5rem;flex:1;flex-wrap:wrap;min-width:0;}`};function Rc(n,e){Ee(e,!0),Be(n,Nc);const t=$e({content:null});Ws.provide(t);let r=J(void 0);Vn(()=>{var h;return(h=a(r))==null?void 0:h.focus()});function i(h){a(r)&&mc(h,a(r))}var s=qc(),o=oe(s),l=P(o,2),c=L(l),d=L(c);{var u=h=>{var w=Pc(),k=oe(w),b=L(k);Re(b,()=>t.content);var M=P(k,2),E=L(M);rn(E,{name:"close"}),B(()=>{Z(M,"title",hn.texts.close),Z(M,"aria-label",hn.texts.close)}),Q("click",M,function(...y){var A;(A=e.onclose)==null||A.apply(this,y)}),S(h,w)},g=h=>{var w=Oc(),k=oe(w),b=re(k,!0),M=P(k,2);{var E=A=>{var x=jc(),T=L(x);Re(T,()=>e.actions),S(A,x)},y=A=>{var x=Dc(),T=L(x);rn(T,{name:"close",size:14});var q=P(T);B(()=>Y(q,` ${hn.texts.done??""}`)),Q("click",x,function(...G){var te;(te=e.onclose)==null||te.apply(this,G)}),S(A,x)};W(M,A=>{e.actions?A(E):A(y,-1)})}B(()=>Y(b,e.title)),S(h,w)};W(d,h=>{t.content?h(u):h(g,-1)})}var _=P(c,2);let v;var p=L(_);Re(p,()=>e.children),vt(l,h=>O(r,h),()=>a(r)),Tt(l,(h,w)=>$n==null?void 0:$n(h,w),()=>e.onclose),B(()=>{Z(l,"aria-label",e.title),v=Xe(_,1,"body svelte-22zuw3",null,v,{"page-in":!!t.content})}),Q("click",o,function(...h){var w;(w=e.onclose)==null||w.apply(this,h)}),Q("keydown",l,i),S(n,s),Ae()}_n(["click","keydown"]);function Fc(n){return n.defaultPrevented||n.target.closest("input, textarea, [contenteditable]")?!1:(n.preventDefault(),n.stopPropagation(),!0)}function Hc(n){return n.button===2||n.clientX?{x:n.clientX,y:n.clientY}:null}var Vc=z('<a class="popup-item svelte-60nrgv" role="menuitem"><!> </a>'),Yc=z('<button type="button" role="menuitem"><!> </button>');const Uc={hash:"svelte-60nrgv",code:".popup-item.svelte-60nrgv {min-width:8.625rem;white-space:nowrap;}"};function Gc(n,e){Ee(e,!0),Be(n,Uc);var t=We(),r=oe(t);Ue(r,17,()=>e.items,ar,(i,s)=>{var o=We(),l=oe(o);{var c=u=>{var g=Vc(),_=L(g);rn(_,{get name(){return a(s).icon},size:14});var v=P(_);B(p=>{Z(g,"href",a(s).href),Z(g,"target",p),Z(g,"download",a(s).download),Y(v,` ${a(s).label??""}`)},[()=>a(s).href.startsWith("/admin")||a(s).download?void 0:"_blank"]),Q("click",g,function(...p){var h;(h=e.close)==null||h.apply(this,p)}),S(u,g)},d=u=>{var g=Yc();let _;var v=L(g);rn(v,{get name(){return a(s).icon},size:14});var p=P(v);B(()=>{_=Xe(g,1,"popup-item svelte-60nrgv",null,_,{danger:a(s).danger}),Y(p,` ${a(s).label??""}`)}),Q("click",g,()=>{var h,w;e.close(),(w=(h=a(s)).onclick)==null||w.call(h)}),S(u,g)};W(l,u=>{a(s).href?u(c):u(d,-1)})}S(i,o)}),S(n,t),Ae()}_n(["click"]);var Wc=z('<button type="button" class="icon"><!></button>');function Xs(n,e){Ee(e,!0);const t=ne(()=>e.label??hn.texts.actions);let r=J(!1),i=J(null),s=J(void 0);Vn(()=>{var u;const d=(u=a(s))==null?void 0:u.closest("[data-menu-area]");if(d)return dl(d,"contextmenu",g=>{Fc(g)&&(O(i,Hc(g),!0),O(r,!0))})});var o=We(),l=oe(o);{var c=d=>{vi(d,{align:"end",get label(){return a(t)},menu:!0,get open(){return a(r)},set open(g){O(r,g,!0)},get at(){return a(i)},set at(g){O(i,g,!0)},trigger:(g,_)=>{let v=()=>(_==null?void 0:_()).toggle,p=()=>(_==null?void 0:_()).open;var h=Wc(),w=L(h);rn(w,{name:"more"}),vt(h,k=>O(s,k),()=>a(s)),B(()=>{Z(h,"title",a(t)),Z(h,"aria-label",a(t)),Z(h,"aria-expanded",p())}),Q("click",h,function(...k){var b;(b=v())==null||b.apply(this,k)}),S(g,h)},children:(g,_)=>{Gc(g,{get items(){return e.items},close:()=>O(r,!1)})},$$slots:{trigger:!0,default:!0}})};W(l,d=>{e.items.length&&d(c)})}S(n,o),Ae()}_n(["click"]);var Xc=z('<button type="button"><!></button>'),Kc=z('<div class="segmented svelte-49ceuw"></div>');const Zc={hash:"svelte-49ceuw",code:`.segmented.svelte-49ceuw {display:inline-flex;flex-wrap:wrap;gap:2px;padding:3px;background:var(--hover);border:1px solid var(--line);border-radius:var(--radius-lg);}.segmented.svelte-49ceuw button:where(.svelte-49ceuw) {gap:0.375rem;padding:0.3125rem 0.75rem;border:none;border-radius:var(--radius);background:none;color:var(--muted);}.segmented.svelte-49ceuw button.on:where(.svelte-49ceuw) {background:var(--surface);color:var(--text);box-shadow:var(--shadow);}
@media (hover: hover) {.segmented.svelte-49ceuw button:where(.svelte-49ceuw):hover:not(.on):not(:disabled) {color:var(--text);background:none;}
}`};function Jc(n,e){Ee(e,!0),Be(n,Zc);let t=nn(e,"tabs",3,!1);const r=ne(()=>e.options.some(o=>o.value===e.value));function i(o){const l={ArrowRight:1,ArrowDown:1,ArrowLeft:-1,ArrowUp:-1}[o.key];if(!l)return;o.preventDefault();const c=e.options.findIndex(g=>g.value===e.value),d=c<0?e.options[l>0?0:e.options.length-1]:e.options[(c+l+e.options.length)%e.options.length];e.onchange(d.value);const u=o.currentTarget.parentElement;js().then(()=>{var g;return(g=u==null?void 0:u.querySelector('[tabindex="0"]'))==null?void 0:g.focus()})}var s=Kc();Ue(s,23,()=>e.options,o=>o.value,(o,l,c)=>{var d=Xc();let u;var g=L(d);{var _=p=>{var h=We(),w=oe(h);Re(w,()=>e.item,()=>a(l)),S(p,h)},v=p=>{var h=Ns();B(()=>Y(h,a(l).label)),S(p,h)};W(g,p=>{e.item?p(_):p(v,-1)})}B(()=>{Z(d,"role",t()?"tab":"radio"),Z(d,"aria-selected",t()?a(l).value===e.value:void 0),Z(d,"aria-checked",t()?void 0:a(l).value===e.value),Z(d,"tabindex",a(l).value===e.value||a(c)===0&&!a(r)?0:-1),u=Xe(d,1,"svelte-49ceuw",null,u,{on:a(l).value===e.value})}),Q("click",d,()=>e.onchange(a(l).value)),Q("keydown",d,i),S(o,d)}),B(()=>{Z(s,"role",t()?"tablist":"radiogroup"),Z(s,"aria-label",e.label)}),S(n,s),Ae()}_n(["click","keydown"]);var Qc=z('<div class="sort-line svelte-183yeqa"></div>');const $c={hash:"svelte-183yeqa",code:".sort-line.svelte-183yeqa {position:absolute;z-index:var(--z-raised);border-radius:2px;background:var(--accent);pointer-events:none;}"};function Pi(n,e){Ee(e,!0),Be(n,$c);var t=We(),r=oe(t);{var i=s=>{var o=Qc();let l;B(()=>l=gt(o,"",l,{top:`${e.sortable.line.top??""}px`,left:`${e.sortable.line.left??""}px`,width:`${e.sortable.line.width??""}px`,height:`${e.sortable.line.height??""}px`})),S(s,o)};W(r,s=>{e.sortable.line&&s(i)})}S(n,t),Ae()}function St(n,e){let t=1;for(;e+t<n.length&&n[e+t]>n[e];)t++;return t}function ed(n,e,t,r,i){const s=n[e],o=St(n,e),l=h=>h<e?h:h-o,c=h=>h+St(n,h);let d=e-1;for(;d>=0&&n[d]>s;)d--;const u=d>=0&&n[d]===s?d:-1,g=e+o,_=g<n.length&&n[g]===s?g:-1;let v=e-1;for(;v>=0&&n[v]>=s;)v--;const p=Math.max(...n.slice(e,e+o))-s;switch(t){case"up":return u<0?null:{index:l(u),depth:s};case"down":{const h=n.length-o-i,w=_<0?-1:l(c(_));return w<0||w>h?null:{index:w,depth:s}}case"out":return s===0||v<0?null:{index:l(c(v)),depth:s-1};case"in":return u<0||s+1+p>r?null:{index:e,depth:s+1}}}var Dt,Ot;class ji{constructor(e){N(this,Dt,J(null));N(this,Ot,J(null));ge(this,"drop",null);ge(this,"dragged",!1);this.o=e}get dragging(){return a(f(this,Dt))}set dragging(e){O(f(this,Dt),e,!0)}get line(){return a(f(this,Ot))}set line(e){O(f(this,Ot),e,!0)}rows(){if(this.o.rows)return this.o.rows();const e=this.o.container();return e?[...e.querySelectorAll("[data-sort]")].filter(t=>{var i;const r=(i=t.parentElement)==null?void 0:i.closest("[data-sort]");return!r||!e.contains(r)}):[]}movable(){return this.o.depths().length>1}grid(){var e,t;return((t=(e=this.o).grid)==null?void 0:t.call(e))??!1}moving(e){const t=this.dragging;return t===null?!1:e>=t&&e<t+St(this.o.depths(),t)}start(e,t){if(e.button!==0)return;const r=e.clientX,i=e.clientY;let s=!1;const o=d=>{if(!s){if(Math.hypot(d.clientX-r,d.clientY-i)<5)return;s=!0,this.dragging=t}d.preventDefault(),this.over(d,t,d.clientX-r),nd(this.o.container(),d.clientY)},l=d=>{window.removeEventListener("pointermove",o),window.removeEventListener("pointerup",l),window.removeEventListener("pointercancel",l),window.removeEventListener("keydown",c,!0);const u=this.drop;this.dragging=null,this.line=null,this.drop=null,s&&(this.dragged=!0,setTimeout(()=>this.dragged=!1),u&&d.type==="pointerup"&&this.o.onmove(t,u))},c=d=>{d.key!=="Escape"||!s||(d.preventDefault(),d.stopImmediatePropagation(),this.drop=null,l(new PointerEvent("pointercancel")))};window.addEventListener("pointermove",o),window.addEventListener("pointerup",l),window.addEventListener("pointercancel",l),window.addEventListener("keydown",c,!0)}over(e,t,r){var d,u,g;const i=(d=this.o.container())==null?void 0:d.getBoundingClientRect(),s=this.rows(),o=this.o.depths();if(!i||s.length!==o.length)return;const l=St(o,t),c=s.filter((_,v)=>v<t||v>=t+l);(g=(u=this.o).grid)!=null&&g.call(u)?this.overGrid(e,i,c):this.overList(e,i,c,o,t,r)}overGrid(e,t,r){var d,u;let i=-1,s=1/0;if(r.forEach((g,_)=>{const v=g.getBoundingClientRect(),p=Math.hypot(e.clientX-(v.left+v.width/2),e.clientY-(v.top+v.height/2));p<s&&([i,s]=[_,p])}),i<0)return;const o=r[i].getBoundingClientRect(),l=e.clientX>o.left+o.width/2,c=r.length-(((u=(d=this.o).keepLast)==null?void 0:u.call(d))??0);this.drop={index:Math.min(i+(l?1:0),c),depth:0},this.line={top:o.top-t.top,left:(l?o.right+3:o.left-5)-t.left,width:2,height:o.height}}overList(e,t,r,i,s,o){var E,y,A,x;const l=St(i,s),c=i.filter((T,q)=>q<s||q>=s+l),d=T=>T.getBoundingClientRect();let u=r.findIndex(T=>{const q=d(T);return e.clientY<q.top+q.height/2});u<0&&(u=r.length),u=Math.min(u,r.length-(((y=(E=this.o).keepLast)==null?void 0:y.call(E))??0));const g=i.slice(s,s+l),_=Math.max(...g)-i[s],v=this.o.indent??24,p=i[s]+Math.round(o/v),h=Math.min(u>0?c[u-1]+1:0,(((x=(A=this.o).maxDepth)==null?void 0:x.call(A))??0)-_),w=u<r.length?c[u]:0;if(w>h){this.drop=null,this.line=null;return}const k=Math.max(w,Math.min(p,h));this.drop={index:u,depth:k};const b=u<r.length?d(r[u]).top-3:r.length?d(r[r.length-1]).bottom+1:t.top,M=(this.o.inset??0)+k*v;this.line={top:b-t.top,left:M,width:t.width-M-(this.o.inset??0),height:2}}stepDrop(e,t){var r,i,s,o;return ed(this.o.depths(),e,t,((i=(r=this.o).maxDepth)==null?void 0:i.call(r))??0,((o=(s=this.o).keepLast)==null?void 0:o.call(s))??0)}async step(e,t){var s;const r=this.stepDrop(e,t);if(!r)return;await this.o.onmove(e,r),await js();const i=this.rows()[r.index];(s=i==null?void 0:i.querySelector("[data-drag-handle]"))==null||s.focus()}}Dt=new WeakMap,Ot=new WeakMap;const Di=48,Oi=12;function nd(n,e){const t=td(n),r=t?t.getBoundingClientRect().top:0,i=t?t.getBoundingClientRect().bottom:innerHeight,s=e<r+Di?-Oi:e>i-Di?Oi:0;s&&(t??window).scrollBy(0,s)}function td(n){for(let e=n==null?void 0:n.parentElement;e;e=e.parentElement){const t=getComputedStyle(e).overflowY;if((t==="auto"||t==="scroll")&&e.scrollHeight>e.clientHeight)return e}return null}const qi={en:{title:"Submissions",note:"What visitors sent with the forms, the newest first.",allForms:"All forms",form:"Form",date:"Date",site:"Site",content:"Content",none:"No submissions yet.",more:"Show more",delete:"Delete",confirmDelete:"Delete this submission for good?",empty:"–",page:"Page",pages:"Pages",addPage:"Add page",renamePage:"Rename",pageName:"Name of the page",deletePage:"Delete page",confirmDeletePage:"Delete this page and its fields?",moveLeft:"Move left",moveRight:"Move right",movePage:"Move page",addField:"Add field",edit:"Edit",duplicate:"Duplicate",switchOff:"Switch off",switchOn:"Switch on",off:"off",remove:"Delete",moveField:"Move field",toPage:"To {page}",width:"Width",full:"Full",noFields:"No fields on this page yet.",cancel:"Cancel",apply:"Apply",fieldsActions:"Field actions",pageActions:"Page actions"},de:{title:"Einsendungen",note:"Was Besucher mit den Formularen geschickt haben, die neusten zuerst.",allForms:"Alle Formulare",form:"Formular",date:"Datum",site:"Website",content:"Inhalt",none:"Noch keine Einsendungen.",more:"Mehr zeigen",delete:"Löschen",confirmDelete:"Diese Einsendung endgültig löschen?",empty:"–",page:"Seite",pages:"Seiten",addPage:"Seite hinzufügen",renamePage:"Umbenennen",pageName:"Name der Seite",deletePage:"Seite löschen",confirmDeletePage:"Diese Seite und ihre Felder löschen?",moveLeft:"Nach links",moveRight:"Nach rechts",movePage:"Seite verschieben",addField:"Feld hinzufügen",edit:"Bearbeiten",duplicate:"Duplizieren",switchOff:"Ausschalten",switchOn:"Einschalten",off:"aus",remove:"Löschen",moveField:"Feld verschieben",toPage:"Auf {page}",width:"Breite",full:"Ganz",noFields:"Noch keine Felder auf dieser Seite.",cancel:"Abbrechen",apply:"Übernehmen",fieldsActions:"Aktionen des Felds",pageActions:"Aktionen der Seite"}};function fi(){return qi[document.documentElement.lang]??qi.en}var rd=z('<span class="grip svelte-1rirqtx"><!></span>'),id=z('<span class="star svelte-1rirqtx">*</span>'),sd=z('<span class="badge"> </span>'),od=z('<p class="hint truncate svelte-1rirqtx"> </p>'),ld=z('<div role="button" tabindex="0" data-menu-area=""><div class="top svelte-1rirqtx"><!> <span class="label truncate svelte-1rirqtx"> <!></span> <span class="type svelte-1rirqtx"> </span> <!> <!></div> <div><span class="truncate"> </span></div> <!></div>');const ad={hash:"svelte-1rirqtx",code:`.field-card.svelte-1rirqtx {display:flex;flex-direction:column;gap:0.375rem;height:100%;padding:0.625rem 0.75rem 0.75rem;background:var(--surface);border:1px solid var(--border);border-radius:var(--radius-lg);cursor:pointer;transition:border-color 0.12s, box-shadow 0.12s;}
@media (hover: hover) {.field-card.svelte-1rirqtx {border-color:var(--accent);}
}.field-card.svelte-1rirqtx:focus-visible {outline:2px solid var(--accent);outline-offset:2px;}.field-card.selected.svelte-1rirqtx {border-color:var(--accent);box-shadow:0 0 0 1px var(--accent);}.field-card.off.svelte-1rirqtx {opacity:0.55;}.top.svelte-1rirqtx {display:flex;align-items:center;gap:0.375rem;min-height:1.75rem;}.grip.svelte-1rirqtx {display:flex;margin-left:-0.375rem;color:var(--muted);}.label.svelte-1rirqtx {flex:1;min-width:0;font-weight:600;font-size:var(--fs-base);}.star.svelte-1rirqtx {margin-left:0.125rem;color:var(--danger);}.type.svelte-1rirqtx {color:var(--muted);font-size:var(--fs-xs);white-space:nowrap;}.preview.svelte-1rirqtx {display:flex;align-items:center;height:2rem;padding:0 0.625rem;color:var(--muted);font-size:var(--fs-md);background:var(--hover);border:1px solid var(--line);border-radius:var(--radius);}.preview.textarea.svelte-1rirqtx {align-items:flex-start;height:4.5rem;padding-top:0.375rem;}.hint.svelte-1rirqtx {margin:0;color:var(--muted);font-size:var(--fs-xs);}`};function cd(n,e){Ee(e,!0),Be(n,ad);let t=nn(e,"selected",3,!1),r=nn(e,"readonly",3,!1);const i=fi(),s=ne(()=>e.item.block.fields),o=ne(()=>e.item.block.enabled===!1),l=ne(()=>e.item.block.type==="formTextarea");function c(F){F.target.closest("button, a, input, select")||e.onedit()}function d(F){F.target===F.currentTarget&&(F.key==="Enter"||F.key===" ")&&(F.preventDefault(),e.onedit())}var u=ld();let g;var _=L(u),v=L(_);{var p=F=>{var ae=rd(),he=L(ae);Us(he,{get sortable(){return e.sortable},get index(){return e.index}}),S(F,ae)};W(v,F=>{r()||F(p)})}var h=P(v,2),w=L(h),k=P(w);{var b=F=>{var ae=id();S(F,ae)};W(k,F=>{a(s).required&&F(b)})}var M=P(h,2),E=re(M,!0),y=P(M,2);{var A=F=>{var ae=sd(),he=re(ae,!0);B(()=>Y(he,i.off)),S(F,ae)};W(y,F=>{a(o)&&F(A)})}var x=P(y,2);{var T=F=>{Xs(F,{get items(){return e.menu},get label(){return i.fieldsActions}})};W(x,F=>{r()||F(T)})}var q=P(_,2);let G;var te=L(q),$=re(te,!0),le=P(q,2);{var me=F=>{var ae=od(),he=re(ae,!0);B(()=>Y(he,a(s).instructions)),S(F,ae)};W(le,F=>{a(s).instructions&&F(me)})}B(()=>{g=Xe(u,1,"field-card svelte-1rirqtx",null,g,{selected:t(),off:a(o)}),Y(w,a(s).label||e.typeName),Y(E,e.typeName),G=Xe(q,1,"preview svelte-1rirqtx",null,G,{textarea:a(l)}),Y($,a(s).placeholder??"")}),Q("click",u,c),Q("keydown",u,d),S(n,u),Ae()}_n(["click","keydown"]);const Zr="formPage",dd=[25,33,50,67,75,100];function ud(n){const e=[];for(const t of n)t.block.type===Zr?e.push({page:t,fields:[]}):e.length?e.at(-1).fields.push(t):e.push({page:null,fields:[t]});return e.length||e.push({page:null,fields:[]}),e}function vd(n,e){return n.flatMap((t,r)=>{const i=t.page??(r>0||n.length>1?e():null);return i?[i,...t.fields]:t.fields})}const Ni=n=>{var e;return((e=n.page)==null?void 0:e.block.fields.label)??""};function fd(n,e,t,r){return n.map((i,s)=>{if(s!==e)return i;const o=i.page??r(),l={...o.block,fields:{...o.block.fields,label:t}};return{...i,page:{...o,block:l}}})}function Ri(n,e,t){const r=[...n],[i]=r.splice(e,1);return r.splice(t,0,i),r}function cr(n,e,t){return n.map((r,i)=>i===e?{...r,fields:t(r.fields)}:r)}function hd(n,e,t,r){const i=n[e].fields[t],s=cr(n,e,o=>o.filter((l,c)=>c!==t));return cr(s,r,o=>[...o,i])}const gd=n=>Math.round((n??100)/100*12);var wd=z('<button type="button"> </button> <button type="button" class="primary"><!> </button>',1),pd=z('<span class="grip svelte-qbgxdw"><!></span>'),_d=z('<input class="rename svelte-qbgxdw"/>'),md=z('<button type="button" role="tab" class="plain name svelte-qbgxdw"> <span class="count svelte-qbgxdw"> </span></button>'),bd=z('<div data-sort="" data-menu-area=""><!> <!> <!></div>'),xd=z('<button type="button" class="plain add-page svelte-qbgxdw"><!> <span> </span></button>'),kd=z('<div data-sort=""><!></div>'),yd=z('<p class="empty"> </p>'),Md=z('<button type="button" class="add-field svelte-qbgxdw"><!> </button>'),Cd=z('<button type="button" class="popup-item" role="menuitem"> </button>'),Sd=z('<div class="cell add svelte-qbgxdw"><!></div>'),Ed=z("<div></div>"),Ad=z('<div class="settings svelte-qbgxdw"><!> <!></div>'),Id=z('<div class="builder svelte-qbgxdw"><div class="tabs svelte-qbgxdw" role="tablist"><!> <!> <!></div> <div class="grid svelte-qbgxdw"><!> <!> <!></div></div> <!>',1);const zd={hash:"svelte-qbgxdw",code:`.builder.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.75rem;}.tabs.svelte-qbgxdw {position:relative;display:flex;flex-wrap:wrap;align-items:center;gap:0.25rem;padding-bottom:0.5rem;border-bottom:1px solid var(--line);}.tab.svelte-qbgxdw {display:flex;align-items:center;gap:0.125rem;padding:0.125rem 0.25rem;border:1px solid transparent;border-radius:var(--radius);}.tab.on.svelte-qbgxdw {background:var(--accent-soft);border-color:var(--accent-ring);}.tab.svelte-qbgxdw .grip:where(.svelte-qbgxdw) {display:flex;color:var(--muted);}.name.svelte-qbgxdw {display:flex;align-items:center;gap:0.375rem;padding:0.25rem 0.375rem;font-weight:500;}.on.svelte-qbgxdw .name:where(.svelte-qbgxdw) {color:var(--accent);}.count.svelte-qbgxdw {min-width:1.25rem;padding:0 0.3125rem;color:var(--muted);font-size:var(--fs-xs);text-align:center;background:var(--hover);border-radius:999px;}.rename.svelte-qbgxdw {width:12rem;padding:0.25rem 0.5rem;}.add-page.svelte-qbgxdw {display:flex;align-items:center;gap:0.25rem;padding:0.375rem 0.5rem;color:var(--muted);}
@media (hover: hover) {.add-page.svelte-qbgxdw {color:var(--accent);}
}.grid.svelte-qbgxdw {position:relative;display:grid;grid-template-columns:repeat(12, minmax(0, 1fr));gap:0.625rem;}.cell.svelte-qbgxdw {grid-column:span var(--span, 12);min-width:0;}.cell.add.svelte-qbgxdw {grid-column:span 12;}.add-field.svelte-qbgxdw {display:flex;align-items:center;justify-content:center;gap:0.375rem;width:100%;padding:0.75rem;color:var(--muted);background:transparent;border:1px dashed var(--border-strong);border-radius:var(--radius-lg);}
@media (hover: hover) {.add-field.svelte-qbgxdw {color:var(--accent);border-color:var(--accent);}
}.settings.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.625rem;}

@media (max-width: 799px) {.cell.svelte-qbgxdw {grid-column:span 12;}
}`};function Td(n,e){Ee(e,!0),Be(n,zd);const t=m=>{var C=wd(),I=oe(C),R=re(I,!0),ee=P(I,2),ie=L(ee);rn(ie,{name:"check",size:14});var Me=P(ie);B(()=>{Y(R,r.cancel),Y(Me,` ${r.apply??""}`)}),Q("click",I,te),Q("click",ee,G),S(m,C)},r=fi(),i=ne(()=>e.value??[]),s=ne(()=>ud(a(i)));let o=J(0);const l=ne(()=>Math.min(a(o),a(s).length-1)),c=ne(()=>a(s)[a(l)]),d=ne(()=>e.blocks.types.filter(m=>m.handle!==Zr)),u=m=>{var C;return((C=e.blocks.types.find(I=>I.handle===m))==null?void 0:C.name)??m},g=(m,C)=>Ni(m)||`${r.page} ${C+1}`,_=()=>({block:e.blocks.create(Zr)});function v(m){const C=vd(m,_);e.onchange(C.length?C:null)}let p=J(null);function h(){const m=_();m.block.fields.label=`${r.page} ${a(s).length+1}`,v([...a(s),{page:m,fields:[]}]),O(o,a(s).length,!0)}function w(m,C){a(p)===m&&(O(p,null),v(fd(a(s),m,C.trim(),_)))}function k(m,C){let I=!1;const R=setTimeout(()=>{m.focus(),m.select(),I=!0},50),ee=()=>I&&w(C,m.value);return m.addEventListener("blur",ee),{destroy(){clearTimeout(R),m.removeEventListener("blur",ee)}}}function b(m,C){v(Ri(a(s),m,C)),O(o,C,!0)}function M(m){a(s)[m].fields.length&&!confirm(r.confirmDeletePage)||(v(a(s).filter((C,I)=>I!==m)),O(o,Math.max(0,m-1),!0))}const E=m=>[{label:r.renamePage,icon:"edit",onclick:()=>O(p,m,!0)},...m>0?[{label:r.moveLeft,icon:"arrow-left",onclick:()=>b(m,m-1)}]:[],...m<a(s).length-1?[{label:r.moveRight,icon:"arrow-right",onclick:()=>b(m,m+1)}]:[],...a(s).length>1?[{label:r.deletePage,icon:"delete",danger:!0,onclick:()=>M(m)}]:[]];let y=J(void 0);const A=new ji({container:()=>a(y),depths:()=>a(s).map(()=>0),grid:()=>!0,onmove:(m,C)=>b(m,C.index)});let x=J(null),T=null;function q(m){T=e.value,O(x,m,!0)}function G(){O(x,null)}function te(){O(x,null),e.onchange(T)}const $=ne(()=>a(x)&&a(s).flatMap((m,C)=>m.fields.map((I,R)=>({item:I,gi:C,fi:R}))).find(m=>m.item.block.id===a(x))),le=m=>v(cr(a(s),a(l),m));let me=null;function F(m){const C=e.blocks.create(m),I=e.value;le(R=>[...R,{width:100,block:C}]),me=C.id,q(C.id),T=I}function ae(m){const C=a(c).fields[m],I={...C,block:e.blocks.copy(C.block)};le(R=>[...R.slice(0,m+1),I,...R.slice(m+1)])}function he(m){le(C=>C.map((I,R)=>{if(R!==m)return I;const{enabled:ee,...ie}=I.block;return{...I,block:ee===!1?ie:{...ie,enabled:!1}}}))}function An(m){a(x)===a(c).fields[m].block.id&&O(x,null),le(C=>C.filter((I,R)=>R!==m))}function ye(m,C,I){v(cr(a(s),m,R=>R.map((ee,ie)=>ie===C?{...ee,width:I}:ee)))}const Kt=(m,C)=>[{label:r.edit,icon:"edit",onclick:()=>O(x,m.block.id,!0)},{label:r.duplicate,icon:"copy",onclick:()=>ae(C)},...a(s).map((I,R)=>({g:I,gi:R})).filter(({gi:I})=>I!==a(l)).map(({g:I,gi:R})=>({label:r.toPage.replace("{page}",g(I,R)),icon:"move",onclick:()=>v(hd(a(s),a(l),C,R))})),{label:m.block.enabled===!1?r.switchOn:r.switchOff,icon:m.block.enabled===!1?"view":"hide",onclick:()=>he(C)},{label:r.remove,icon:"delete",danger:!0,onclick:()=>An(C)}];let pr=J(void 0),Zt={x:0,y:0};Vn(()=>{const m=C=>Zt={x:C.clientX,y:C.clientY};return window.addEventListener("pointermove",m),()=>window.removeEventListener("pointermove",m)});function Ks(){var R;const m=(R=document.elementFromPoint(Zt.x,Zt.y))==null?void 0:R.closest(".cell[data-sort]");if(!m||m.classList.contains("sort-dragging")||getComputedStyle(m).getPropertyValue("--span")==="12")return!1;const C=m.getBoundingClientRect(),I=(Zt.y-C.top)/C.height;return I>.25&&I<.75}const _r=new ji({container:()=>a(pr),depths:()=>a(c).fields.map(()=>0),grid:Ks,onmove:(m,C)=>le(I=>Ri(I,m,C.index))});function mr(m,C){const I=e.blocks.mount(m,C);return me===C&&(me=null,setTimeout(()=>{var R;return(R=m.querySelector("input"))==null?void 0:R.focus()},50)),{destroy:()=>I.destroy()}}const Zs=m=>({25:"¼",33:"⅓",50:"½",67:"⅔",75:"¾",100:r.full})[m];var hi=Id(),gi=oe(hi),Jt=L(gi),wi=L(Jt);Ue(wi,19,()=>a(s),m=>{var C;return((C=m.page)==null?void 0:C.block.id)??"first"},(m,C,I)=>{var R=bd();let ee;var ie=L(R);{var Me=ce=>{var de=pd(),Ie=L(de);Us(Ie,{get sortable(){return A},get index(){return a(I)}}),S(ce,de)};W(ie,ce=>{!e.readonly&&a(s).length>1&&ce(Me)})}var Le=P(ie,2);{var be=ce=>{var de=_d();Tt(de,(Ie,ln)=>k==null?void 0:k(Ie,ln),()=>a(I)),B((Ie,ln)=>{Z(de,"aria-label",r.pageName),ql(de,Ie),Z(de,"placeholder",ln)},[()=>Ni(a(C)),()=>g(a(C),a(I))]),Q("keydown",de,Ie=>{Ie.key==="Enter"&&w(a(I),Ie.currentTarget.value),Ie.key==="Escape"&&O(p,null)}),S(ce,de)},Ke=ce=>{var de=md(),Ie=L(de),ln=P(Ie),ro=re(ln,!0);B(io=>{Z(de,"aria-selected",a(I)===a(l)),Y(Ie,`${io??""} `),Y(ro,a(C).fields.length)},[()=>g(a(C),a(I))]),Q("click",de,()=>O(o,a(I),!0)),Q("dblclick",de,()=>!e.readonly&&O(p,a(I),!0)),S(ce,de)};W(Le,ce=>{a(p)===a(I)?ce(be):ce(Ke,-1)})}var Pe=P(Le,2);{var on=ce=>{{let de=ne(()=>E(a(I)));Xs(ce,{get items(){return a(de)},get label(){return r.pageActions}})}};W(Pe,ce=>{!e.readonly&&a(I)===a(l)&&a(p)!==a(I)&&ce(on)})}B(ce=>ee=Xe(R,1,"tab svelte-qbgxdw",null,ee,{on:a(I)===a(l),"sort-dragging":ce}),[()=>A.moving(a(I))]),S(m,R)});var pi=P(wi,2);{var Js=m=>{var C=xd(),I=L(C);rn(I,{name:"add"});var R=P(I,2),ee=re(R,!0);B(()=>{Z(C,"title",r.addPage),Y(ee,r.addPage)}),Q("click",C,h),S(m,C)};W(pi,m=>{e.readonly||m(Js)})}var Qs=P(pi,2);Pi(Qs,{get sortable(){return A}}),vt(Jt,m=>O(y,m),()=>a(y));var _i=P(Jt,2),mi=L(_i);Ue(mi,19,()=>a(c).fields,m=>m.block.id,(m,C,I)=>{var R=kd();let ee,ie;var Me=L(R);{let Le=ne(()=>u(a(C).block.type)),be=ne(()=>a(x)===a(C).block.id),Ke=ne(()=>Kt(a(C),a(I)));cd(Me,{get item(){return a(C)},get typeName(){return a(Le)},get selected(){return a(be)},get readonly(){return e.readonly},get sortable(){return _r},get index(){return a(I)},get menu(){return a(Ke)},onedit:()=>q(a(C).block.id)})}B((Le,be)=>{ee=Xe(R,1,"cell svelte-qbgxdw",null,ee,{"sort-dragging":Le}),ie=gt(R,"",ie,{"--span":be})},[()=>_r.moving(a(I)),()=>gd(a(C).width)]),S(m,R)},m=>{var C=We(),I=oe(C);{var R=ee=>{var ie=yd(),Me=re(ie,!0);B(()=>Y(Me,r.noFields)),S(ee,ie)};W(I,ee=>{e.readonly&&ee(R)})}S(m,C)});var bi=P(mi,2);{var $s=m=>{var C=Sd(),I=L(C);vi(I,{menu:!0,get label(){return r.addField},trigger:(ee,ie)=>{let Me=()=>(ie==null?void 0:ie()).toggle,Le=()=>(ie==null?void 0:ie()).open;var be=Md(),Ke=L(be);rn(Ke,{name:"add"});var Pe=P(Ke);B(()=>{Z(be,"aria-expanded",Le()),Y(Pe,` ${r.addField??""}`)}),Q("click",be,function(...on){var ce;(ce=Me())==null||ce.apply(this,on)}),S(ee,be)},children:(ee,ie)=>{var Me=We(),Le=oe(Me);Ue(Le,17,()=>a(d),be=>be.handle,(be,Ke)=>{var Pe=Cd(),on=re(Pe,!0);B(()=>Y(on,a(Ke).name)),Q("click",Pe,()=>F(a(Ke).handle)),S(be,Pe)}),S(ee,Me)},$$slots:{trigger:!0,default:!0}}),S(m,C)};W(bi,m=>{e.readonly||m($s)})}var eo=P(bi,2);Pi(eo,{get sortable(){return _r}}),vt(_i,m=>O(pr,m),()=>a(pr));var no=P(gi,2);{var to=m=>{{let C=ne(()=>a($).item.block.fields.label||u(a($).item.block.type)),I=ne(()=>e.readonly?G:te),R=ne(()=>e.readonly?void 0:t);Rc(m,{get title(){return a(C)},get onclose(){return a(I)},get actions(){return a(R)},children:(ee,ie)=>{var Me=Ad(),Le=L(Me);{var be=Pe=>{Bc(Pe,{get label(){return r.width},children:(on,ce)=>{{let de=ne(()=>dd.map(ln=>({value:ln,label:Zs(ln)}))),Ie=ne(()=>a($).item.width??100);Jc(on,{get label(){return r.width},get options(){return a(de)},get value(){return a(Ie)},onchange:ln=>ye(a($).gi,a($).fi,ln)})}},$$slots:{default:!0}})};W(Le,Pe=>{e.readonly||Pe(be)})}var Ke=P(Le,2);Ml(Ke,()=>a(x),Pe=>{var on=Ed();Tt(on,(ce,de)=>mr==null?void 0:mr(ce,de),()=>a(x)),S(Pe,on)}),S(ee,Me)},$$slots:{default:!0}})}};W(no,m=>{a($)&&m(to)})}B(()=>Z(Jt,"aria-label",r.pages)),S(n,hi),Ae()}_n(["click","keydown","dblclick"]);var Bd=z("<span>/</span>"),Ld=z("<a> </a>"),Pd=z('<span class="here truncate svelte-wekzd8"> </span>'),jd=z("<!> <!>",1),Dd=z('<nav class="crumbs svelte-wekzd8"></nav>');const Od={hash:"svelte-wekzd8",code:`.crumbs.svelte-wekzd8 {display:flex;align-items:center;gap:0.375rem;min-width:0;font-size:var(--fs-md);}a.svelte-wekzd8 {text-decoration:none;white-space:nowrap;}.sep.svelte-wekzd8 {color:var(--subtle);}.here.svelte-wekzd8 {color:var(--text);font-weight:500;}

@container page (width < 36rem) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}
@media (max-width: 799px) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}`};function qd(n,e){Ee(e,!0),Be(n,Od);var t=Dd();Ue(t,21,()=>e.items,ar,(r,i,s)=>{const o=ne(()=>s<e.items.length-2);var l=jd(),c=oe(l);{var d=v=>{var p=Bd();let h;B(()=>h=Xe(p,1,"sep svelte-wekzd8",null,h,{far:s<e.items.length-1})),S(v,p)};W(c,v=>{s&&v(d)})}var u=P(c,2);{var g=v=>{var p=Ld();let h;var w=re(p,!0);B(()=>{Z(p,"href",a(i).href),h=Xe(p,1,"svelte-wekzd8",null,h,{far:a(o)}),Y(w,a(i).label)}),S(v,p)},_=v=>{var p=Pd(),h=re(p,!0);B(()=>Y(h,a(i).label)),S(v,p)};W(u,v=>{a(i).href&&s<e.items.length-1?v(g):v(_,-1)})}S(r,l)}),B(()=>Z(t,"aria-label",hn.texts.breadcrumbs)),S(n,t),Ae()}var Nd=z('<span class="note svelte-z3zeui"> </span>'),Rd=z('<span class="state svelte-z3zeui"><!></span>'),Fd=z('<!> <h1 class="svelte-z3zeui"><!></h1> <!> <!> <span class="actions svelte-z3zeui"><!></span>',1),Hd=z("<div><!></div>");const Vd={hash:"svelte-z3zeui",code:`.page-top.svelte-z3zeui {position:sticky;top:0;z-index:var(--z-bar);display:flex;align-items:center;gap:0.5rem;flex-wrap:wrap;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.bleed.svelte-z3zeui {margin:0 calc(-1 * var(--page-pad)) 1.25rem;padding:0.5rem var(--page-pad);}h1.svelte-z3zeui {min-width:0;font-size:var(--fs-lg);line-height:1.25rem;}.state.svelte-z3zeui {display:flex;align-items:center;gap:0.375rem;}.actions.svelte-z3zeui {display:flex;align-items:center;gap:0.5rem;margin-left:auto;}

@container page (width < 36rem) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}
@media (max-width: 799px) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}`};function Yd(n,e){Ee(e,!0),Be(n,Vd);const t=d=>{var u=Fd(),g=oe(u);{var _=x=>{var T=We(),q=oe(T);Re(q,()=>(s==null?void 0:s.content)??ot),S(x,T)};W(g,x=>{i||x(_)})}var v=P(g,2),p=L(v);{var h=x=>{qd(x,{get items(){return e.crumbs}})},w=x=>{var T=Ns();B(()=>Y(T,e.title)),S(x,T)};W(p,x=>{e.crumbs?x(h):x(w,-1)})}var k=P(v,2);{var b=x=>{var T=Nd(),q=re(T,!0);B(()=>Y(q,e.note)),S(x,T)};W(k,x=>{e.note&&x(b)})}var M=P(k,2);{var E=x=>{var T=Rd(),q=L(T);Re(q,()=>e.state),S(x,T)};W(M,x=>{e.state&&x(E)})}var y=P(M,2),A=L(y);Re(A,()=>e.actions??ot),S(d,u)};let r=nn(e,"bleed",3,!0);const i=Ws.get(),s=Lc.get();Vn(()=>{if(i)return i.content=t,()=>i.content=null});var o=We(),l=oe(o);{var c=d=>{var u=Hd();let g;var _=L(u);t(_),B(()=>g=Xe(u,1,"page-top svelte-z3zeui",null,g,{bleed:r()})),S(d,u)};W(l,d=>{i||d(c)})}S(n,o),Ae()}var Ud=z("<col/>"),Gd=z("<colgroup></colgroup>"),Wd=z('<th class="svelte-migw9g"> </th>'),Xd=z("<thead><tr></tr></thead>"),Kd=z('<tr class="detail svelte-migw9g"><td><!></td></tr>'),Zd=z('<tr class="svelte-migw9g"><!></tr> <!>',1),Jd=z('<tr class="svelte-migw9g"><td class="empty svelte-migw9g"> </td></tr>'),Qd=z('<div class="scroll svelte-migw9g"><table><!><!><tbody class="svelte-migw9g"></tbody></table></div>');const $d={hash:"svelte-migw9g",code:`.scroll.svelte-migw9g {overflow-x:auto;}table.svelte-migw9g {width:100%;border-collapse:collapse;}table.fixed.svelte-migw9g {table-layout:fixed;}th.svelte-migw9g,
tbody.svelte-migw9g td {padding:0.375rem 0.5rem;text-align:left;vertical-align:top;}th.svelte-migw9g {font-weight:600;}tbody.svelte-migw9g td {border-top:1px solid var(--line);}tbody.svelte-migw9g code {font-size:var(--fs-md);color:var(--muted);}tbody.svelte-migw9g tr:where(.svelte-migw9g).target {background:var(--hover);transition:background 0.3s;}tbody.svelte-migw9g tr[aria-expanded=true]:where(.svelte-migw9g),
.detail.svelte-migw9g {background:var(--surface-2);}.detail.svelte-migw9g td {border-top:none;padding:0.25rem 0.5rem 0.75rem;}.empty.svelte-migw9g {color:var(--muted);}.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail) {cursor:pointer;}

@media (hover: hover) {.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail):hover {background:var(--hover);}
}`};function eu(n,e){Ee(e,!0),Be(n,$d);let t=nn(e,"columns",19,()=>[]);function r(_,v){if(_.target.closest("a, button, input, select"))return;if(e.ontoggle)return e.ontoggle(v);if(!e.href)return;const p=e.href(v);if(_.metaKey||_.ctrlKey)return void window.open(p,"_blank");const w=_.currentTarget.querySelector(`a[href="${CSS.escape(p)}"]`);w?w.click():location.assign(p)}var i=Qd(),s=L(i);let o;var l=L(s);{var c=_=>{var v=Gd();Ue(v,21,()=>e.widths,ar,(p,h)=>{var w=Ud();let k;B(()=>k=gt(w,"",k,{width:a(h)})),S(p,w)}),S(_,v)};W(l,_=>{e.widths&&_(c)})}var d=P(l);{var u=_=>{var v=Xd(),p=L(v);Ue(p,21,t,ar,(h,w)=>{var k=Wd(),b=re(k,!0);B(()=>Y(b,a(w))),S(h,k)}),S(_,v)};W(d,_=>{t().length&&_(u)})}var g=P(d);Ue(g,23,()=>e.rows,_=>e.key(_),(_,v,p)=>{var h=Zd(),w=oe(h),k=L(w);Re(k,()=>e.row,()=>a(v),()=>a(p));var b=P(w,2);{var M=y=>{var A=Kd(),x=L(A),T=L(x);Re(T,()=>e.detail,()=>a(v)),B(()=>Z(x,"colspan",t().length||1)),S(y,A)},E=ne(()=>{var y;return e.detail&&((y=e.isOpen)==null?void 0:y.call(e,a(v)))});W(b,y=>{a(E)&&y(M)})}B((y,A)=>{Z(w,"id",y),Z(w,"aria-expanded",A),Z(w,"tabindex",e.ontoggle?0:void 0)},[()=>{var y;return(y=e.id)==null?void 0:y.call(e,a(v))},()=>{var y;return e.ontoggle?!!((y=e.isOpen)!=null&&y.call(e,a(v))):void 0}]),Q("click",w,y=>r(y,a(v))),Q("keydown",w,y=>{!e.ontoggle||y.target!==y.currentTarget||y.key!=="Enter"&&y.key!==" "||(y.preventDefault(),e.ontoggle(a(v)))}),S(_,h)},_=>{var v=Jd(),p=L(v),h=re(p,!0);B(()=>{Z(p,"colspan",t().length||1),Y(h,e.empty??hn.texts.nothingYet)}),S(_,v)}),B(()=>o=Xe(s,1,"svelte-migw9g",null,o,{links:!!e.href||!!e.ontoggle,fixed:!!e.widths})),S(n,i),Ae()}_n(["click","keydown"]);var qt,Nt,Rt,Ft,Ht,Vt;class nu{constructor(e){N(this,qt,J($e([])));N(this,Nt,J(null));N(this,Rt,J($e([])));N(this,Ft,J(0));N(this,Ht,J(!1));N(this,Vt,J(""));this.host=e}get forms(){return a(f(this,qt))}set forms(e){O(f(this,qt),e,!0)}get form(){return a(f(this,Nt))}set form(e){O(f(this,Nt),e,!0)}get submissions(){return a(f(this,Rt))}set submissions(e){O(f(this,Rt),e,!0)}get total(){return a(f(this,Ft))}set total(e){O(f(this,Ft),e,!0)}get loading(){return a(f(this,Ht))}set loading(e){O(f(this,Ht),e,!0)}get error(){return a(f(this,Vt))}set error(e){O(f(this,Vt),e,!0)}async init(){await this.run(async()=>{const e=await this.host.request("GET","/entries?section=forms&limit=500&order=title");this.forms=e.entries.map(t=>({id:t.id,title:t.title}))}),await this.load()}async load(){await this.run(async()=>{const e=await this.request(0);this.submissions=e.submissions,this.total=e.total})}async more(){await this.run(async()=>{const e=await this.request(this.submissions.length);this.submissions=[...this.submissions,...e.submissions],this.total=e.total})}async delete(e){await this.run(async()=>{await this.host.request("DELETE",`/forms/submissions/${e}`),this.submissions=this.submissions.filter(t=>t.id!==e),this.total-=1})}request(e){const t=this.form===null?"":`&form=${this.form}`;return this.host.request("GET",`/forms/submissions?offset=${e}${t}`)}async run(e){this.loading=!0,this.error="";try{await e()}catch(t){this.error=t.message}finally{this.loading=!1}}}qt=new WeakMap,Nt=new WeakMap,Rt=new WeakMap,Ft=new WeakMap,Ht=new WeakMap,Vt=new WeakMap;var tu=z("<option> </option>"),ru=z("<select><option> </option><!></select>"),iu=z('<p class="error"> </p>'),su=z('<td class="nowrap svelte-1tixh7s"> </td> <td> </td> <td> </td> <td class="truncate"> </td>',1),ou=z('<dt class="svelte-1tixh7s"> </dt> <dd class="svelte-1tixh7s"> </dd>',1),lu=z('<dl class="svelte-1tixh7s"></dl> <button class="danger"><!> </button>',1),au=z('<p class="more svelte-1tixh7s"><button class="light"> </button></p>'),cu=z('<!> <!> <div class="card"><!></div> <!>',1);const du={hash:"svelte-1tixh7s",code:`.nowrap.svelte-1tixh7s {white-space:nowrap;}dl.svelte-1tixh7s {display:grid;grid-template-columns:minmax(8rem, max-content) 1fr;gap:0.375rem 1.5rem;margin:0 0 1rem;}dt.svelte-1tixh7s {color:var(--muted);}dd.svelte-1tixh7s {margin:0;white-space:pre-wrap;overflow-wrap:anywhere;}.more.svelte-1tixh7s {text-align:center;}

@media (max-width: 799px) {dl.svelte-1tixh7s {grid-template-columns:1fr;}dd.svelte-1tixh7s {margin-bottom:0.5rem;}
}`};function uu(n,e){Ee(e,!0),Be(n,du);const t=fi(),r=new nu(e.host);r.init();let i=J(null);const s=b=>{var M;return((M=e.schema.sites.find(E=>E.handle===b))==null?void 0:M.name)??b},o=b=>new Date(b).toLocaleString(document.documentElement.lang,{dateStyle:"medium",timeStyle:"short"}),l=b=>b.answers.map(M=>M.value).filter(Boolean).slice(0,3).join(" · ")||t.empty;function c(b){const M=b.currentTarget.value;r.form=M?Number(M):null,O(i,null),r.load()}function d(b){confirm(t.confirmDelete)&&r.delete(b.id)}var u=cu(),g=oe(u);Yd(g,{get title(){return t.title},get note(){return t.note},actions:M=>{var E=We(),y=oe(E);{var A=x=>{var T=ru(),q=L(T),G=re(q,!0);q.value=q.__value="";var te=P(q);Ue(te,17,()=>r.forms,le=>le.id,(le,me)=>{var F=tu(),ae=re(F,!0),he={};B(()=>{Y(ae,a(me).title),he!==(he=a(me).id)&&(F.value=(F.__value=he)??"")}),S(le,F)});var $;Ll(T),B(()=>{Y(G,t.allForms),$!==($=r.form??"")&&(T.value=(T.__value=$)??"",Hs(T,$))}),Q("change",T,c),S(x,T)};W(y,x=>{r.forms.length>1&&x(A)})}S(M,E)},$$slots:{actions:!0}});var _=P(g,2);{var v=b=>{var M=iu(),E=re(M,!0);B(()=>Y(E,r.error)),S(b,M)};W(_,b=>{r.error&&b(v)})}var p=P(_,2),h=L(p);{const b=(A,x=ot)=>{var T=su(),q=oe(T),G=re(q,!0),te=P(q,2),$=re(te,!0),le=P(te,2),me=re(le,!0),F=P(le,2),ae=re(F,!0);B((he,An,ye)=>{Y(G,he),Y($,x().form),Y(me,An),Y(ae,ye)},[()=>o(x().createdAt),()=>s(x().site),()=>l(x())]),S(A,T)},M=(A,x=ot)=>{var T=lu(),q=oe(T);Ue(q,21,()=>x().answers,le=>le.handle,(le,me)=>{var F=ou(),ae=oe(F),he=re(ae,!0),An=P(ae,2),ye=re(An,!0);B(()=>{Y(he,a(me).label),Y(ye,a(me).value||t.empty)}),S(le,F)});var G=P(q,2),te=L(G);rn(te,{name:"delete"});var $=P(te);B(()=>Y($,` ${t.delete??""}`)),Q("click",G,()=>d(x())),S(A,T)};let E=ne(()=>[t.date,t.form,t.site,t.content]),y=ne(()=>r.loading?"…":t.none);eu(h,{get rows(){return r.submissions},key:A=>A.id,get columns(){return a(E)},get empty(){return a(y)},isOpen:A=>A.id===a(i),ontoggle:A=>O(i,a(i)===A.id?null:A.id,!0),row:b,detail:M,$$slots:{row:!0,detail:!0}})}var w=P(p,2);{var k=b=>{var M=au(),E=L(M),y=re(E,!0);B(()=>{E.disabled=r.loading,Y(y,t.more)}),Q("click",E,()=>r.more()),S(b,M)};W(w,b=>{r.submissions.length<r.total&&b(k)})}S(n,u),Ae()}_n(["change","click"]);const hu=Fl(Td);function gu(n,e){const t=Rs(uu,{target:n,props:e});return{destroy(){Fs(t)}}}export{hu as mount,gu as page};
