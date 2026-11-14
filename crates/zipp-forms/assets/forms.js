var Qs=Object.defineProperty;var wi=n=>{throw TypeError(n)};var $s=(n,e,t)=>e in n?Qs(n,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):n[e]=t;var we=(n,e,t)=>$s(n,typeof e!="symbol"?e+"":e,t),wr=(n,e,t)=>e.has(n)||wi("Cannot "+t);var f=(n,e,t)=>(wr(n,e,"read from private field"),t?t.call(n):e.get(n)),N=(n,e,t)=>e.has(n)?wi("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,t),F=(n,e,t,r)=>(wr(n,e,"write to private field"),r?r.call(n,t):e.set(n,t),t),j=(n,e,t)=>(wr(n,e,"access private method"),t);const Ni={},he=Symbol("uninitialized"),Ri="http://www.w3.org/1999/xhtml",eo="http://www.w3.org/2000/svg",no="http://www.w3.org/1998/Math/MathML",Fi=!1;var vr=Array.isArray,to=Array.prototype.indexOf,At=Array.prototype.includes,fr=Array.from,Hi=Object.defineProperty,Kn=Object.getOwnPropertyDescriptor,Vi=Object.getOwnPropertyDescriptors,ro=Object.prototype,io=Array.prototype,Xr=Object.getPrototypeOf,pi=Object.isExtensible;const st=()=>{};function so(n){for(var e=0;e<n.length;e++)n[e]()}function Yi(){var n,e,t=new Promise((r,i)=>{n=r,e=i});return{promise:t,resolve:n,reject:e}}function oo(n,e){if(Array.isArray(n))return n;if(!(Symbol.iterator in n))return Array.from(n);const t=[];for(const r of n)if(t.push(r),t.length===e)break;return t}const xe=2,ot=4,hr=8,Ui=1<<24,Je=16,tn=32,Ln=64,Ar=128,Kr=256,nn=512,ge=1024,re=2048,De=4096,Me=8192,_e=16384,vt=32768,It=1<<25,Yn=65536,tr=1<<17,lo=1<<18,ft=1<<19,ao=1<<20,un=1<<25,rr=1<<21,Zn=1<<22,Tn=1<<23,Mn=Symbol("$state"),Gi=Symbol("component"),co=Symbol("legacy props"),uo=Symbol(""),$t=Symbol("attributes"),Ir=Symbol("class"),zr=Symbol("style"),_t=Symbol("text"),Ut=new class extends Error{constructor(){super(...arguments);we(this,"name","StaleReactionError");we(this,"message","The reaction that called `getAbortSignal()` was re-run or destroyed")}};var Di;const vo=!!((Di=globalThis.document)!=null&&Di.contentType)&&globalThis.document.contentType.includes("xml");function fo(){console.warn("https://svelte.dev/e/derived_inert")}function ho(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function go(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}function Wi(n){return n===this.v}function Xi(n,e){return n!=n?e==e:n!==e||n!==null&&typeof n=="object"||typeof n=="function"}function Ki(n){return!Xi(n,this.v)}function wo(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function po(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function _o(n,e,t){throw new Error("https://svelte.dev/e/each_key_duplicate")}function mo(n){throw new Error("https://svelte.dev/e/effect_in_teardown")}function bo(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function xo(n){throw new Error("https://svelte.dev/e/effect_orphan")}function ko(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function yo(n){throw new Error("https://svelte.dev/e/props_invalid_value")}function Mo(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function Co(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function So(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function Eo(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let Ao=!1;function Io(n){let e=n.p;for(;e!==null&&e.c===null;)e=e.p;return(e==null?void 0:e.c)??null}function Zi(n,e){return n===null&&wo(),n.c??(n.c=new Map(Io(n)||void 0))}let me=null;function lt(n){me=n}function zo(n){return Zi(me).get(n)}function To(n,e){return Zi(me).set(n,e),e}function Ce(n,e=!1,t){me={p:me,i:!1,c:null,e:null,s:n,x:null,r:V,l:null}}function Se(n){var e=me,t=e.e;if(t!==null){e.e=null;for(var r of t)ms(r)}return e.i=!0,me=e.p,Zr(n)}function Zr(n={}){return Hi(n,Gi,{value:!0}),n}function Ji(){return!0}let jn=[];function Qi(){var n=jn;jn=[],so(n)}function kn(n){if(jn.length===0&&!Mt){var e=jn;queueMicrotask(()=>{e===jn&&Qi()})}jn.push(n)}function Bo(){for(;jn.length>0;)Qi()}const Lo=-7169;function de(n,e){n.f=n.f&Lo|e}function Jr(n){(n.f&nn)!==0||n.deps===null?de(n,ge):de(n,De)}function $i(n,e){(n.f&re)!==0?e.set(n,re):(n.f&De)!==0&&e.get(n)!==re&&e.set(n,De),de(n,ge)}function Gt(n){var e=H,t=V;Ge(null),gn(null);try{return n()}finally{Ge(e),gn(t)}}function Po(n,e,t,r){const i=zt;var s=n.filter(v=>!v.settled),o=e.map(i);if(t.length===0&&s.length===0){r(o);return}var l=V,c=jo(),d=s.length===1?s[0].promise:s.length>1?Promise.all(s.map(v=>v.promise)):null;function u(v){if((l.f&_e)===0){c();try{r([...o,...v])}catch(_){dn(_,l)}ir()}}var g=es();if(t.length===0){d.then(()=>u([])).finally(g);return}function p(){Promise.all(t.map(v=>Do(v))).then(u).catch(v=>dn(v,l)).finally(g)}d?d.then(()=>{if((l.f&_e)!==0){g();return}c(),p(),ir()}):p()}function jo(){var n=V,e=H,t=me,r=D;return function(s=!0){gn(n),Ge(e),lt(t),s&&(n.f&_e)===0&&(r==null||r.activate(),r==null||r.apply())}}function ir(n=!0){gn(null),Ge(null),lt(null),n&&(D==null||D.deactivate())}function es(){var n=V,e=n.b,t=D,r=!!(e!=null&&e.is_rendered());return e==null||e.update_pending_count(1,t),t.increment(r,n),()=>{e==null||e.update_pending_count(-1,t),t.decrement(r,n)}}function zt(n){var e=xe|re;return V!==null&&(V.f|=ft),{ctx:me,deps:null,effects:null,equals:Wi,f:e,fn:n,reactions:null,rv:0,v:he,wv:0,parent:V,ac:null}}const mt=Symbol("obsolete");function Do(n,e,t){let r=V;r===null&&po();var i=void 0,s=Gn(he),o=!H,l=new Set;return Jo(()=>{var v,_;var c=V,d=Yi();i=d.promise;try{Promise.resolve(n()).then(d.resolve,h=>{h!==Ut&&d.reject(h)}).finally(ir)}catch(h){d.reject(h),ir()}var u=D;if(o){if((c.f&vt)!==0)var g=es();if((v=r.b)!=null&&v.is_rendered())(_=u.async_deriveds.get(c))==null||_.reject(mt);else for(const h of l.values())h.reject(mt);l.add(d),u.async_deriveds.set(c,d)}const p=(h,w=void 0)=>{g==null||g(),l.delete(d),w!==mt&&(u.activate(),w?(s.f|=Tn,at(s,w)):((s.f&Tn)!==0&&(s.f^=Tn),at(s,h)),u.deactivate())};d.promise.then(p,h=>p(null,h||"unknown"))}),_s(()=>{for(const c of l)c.reject(mt)}),new Promise(c=>{function d(u){function g(){u===i?c(s):d(i)}u.then(g,g)}d(i)})}function ne(n){const e=zt(n);return Cs(e),e}function ns(n){const e=zt(n);return e.equals=Ki,e}function Oo(n){var e=n.effects;if(e!==null){n.effects=null;for(var t=0;t<e.length;t+=1)ze(e[t])}}function Qr(n){var e,t=V,r=n.parent;if(!Sn&&r!==null&&n.v!==he&&(r.f&(_e|Me))!==0)return fo(),n.v;gn(r);try{Oo(n),e=Is(n)}finally{gn(t)}return e}function ts(n){var e=Qr(n);if(!n.equals(e)&&(n.wv=Es(),(!(D!=null&&D.is_fork)||n.deps===null)&&(D!==null?(D.capture(n,e,!0),Bn==null||Bn.capture(n,e,!0)):n.v=e,n.deps===null))){de(n,ge);return}Sn||(pe!==null?(ti()||D!=null&&D.is_fork)&&pe.set(n,e):Jr(n))}function qo(n){var e;if(n.effects!==null)for(const t of n.effects)(t.teardown||t.ac)&&((e=t.teardown)==null||e.call(t),t.ac!==null&&Gt(()=>{t.ac.abort(Ut),t.ac=null}),t.fn!==null&&(t.teardown=st),Tt(t,0),si(t))}function rs(n){if(n.effects!==null)for(const e of n.effects)e.teardown&&e.fn!==null&&ct(e)}let pr=null,Wn=null,D=null,Bn=null,pe=null,Tr=null,Mt=!1,_r=!1,Ct=null,er=null;var _i=0;let No=1;var $n,et,nt,mn,tt,bn,Lt,on,ln,rt,qn,G,Br,is,Lr,bt,Pr,ss,os,ls,Xn,Ro,xt;const cr=class cr{constructor(){N(this,G);we(this,"id",No++);we(this,"started",!1);we(this,"linked",!0);we(this,"prev",null);we(this,"next",null);we(this,"async_deriveds",new Map);we(this,"current",new Map);we(this,"previous",new Map);N(this,$n,new Set);N(this,et,new Set);N(this,nt,0);N(this,mn,new Map);N(this,tt,null);N(this,bn,[]);N(this,Lt,[]);N(this,on,new Map);N(this,ln,new Map);N(this,rt,new Set);we(this,"is_fork",!1);N(this,qn,!1);Wn===null?pr=Wn=this:(Wn.next=this,this.prev=Wn),Wn=this}skip_effect(e){f(this,ln).has(e)||f(this,ln).set(e,{d:[],m:[]}),f(this,rt).delete(e)}unskip_effect(e,t=r=>this.schedule(r)){var r=f(this,ln).get(e);if(r){f(this,ln).delete(e);for(var i of r.d)de(i,re),t(i);for(i of r.m)de(i,De),t(i)}f(this,rt).add(e)}capture(e,t,r=!1){e.v!==he&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&Tn)===0&&(this.current.set(e,[t,r]),pe==null||pe.set(e,t)),this.is_fork||(e.v=t)}activate(){D=this}deactivate(){D=null,pe=null}flush(){try{_r=!0,D=this,j(this,G,bt).call(this)}finally{_i=0,Tr=null,Ct=null,er=null,_r=!1,D=null,pe=null,fn.clear()}}discard(){var e;for(const t of f(this,et))t(this);f(this,et).clear();for(const t of this.async_deriveds.values())t.reject(mt);j(this,G,xt).call(this),(e=f(this,tt))==null||e.resolve()}register_created_effect(e){f(this,Lt).push(e)}increment(e,t){if(F(this,nt,f(this,nt)+1),e){let r=f(this,mn).get(t)??0;f(this,mn).set(t,r+1)}}decrement(e,t){if(F(this,nt,f(this,nt)-1),e){let r=f(this,mn).get(t)??0;r===1?f(this,mn).delete(t):f(this,mn).set(t,r-1)}f(this,qn)||(F(this,qn,!0),kn(()=>{F(this,qn,!1),this.linked&&this.flush()}))}transfer_reactions(e){for(const[t,r]of e)this.add_dirty_reaction(t,r);e.clear()}add_dirty_reaction(e,t){(t===re||f(this,on).get(e)!==re)&&f(this,on).set(e,t)}oncommit(e){f(this,$n).add(e)}ondiscard(e){f(this,et).add(e)}settled(){return(f(this,tt)??F(this,tt,Yi())).promise}static ensure(){if(D===null){const e=D=new cr;!_r&&!Mt&&kn(()=>{e.started||e.flush()})}return D}apply(){{pe=null;return}}schedule(e){var t;if(Tr=e,(t=e.b)!=null&&t.is_pending&&(e.f&(ot|hr|Ui))!==0&&(e.f&vt)===0){e.b.defer_effect(e);return}f(this,bn).push(e)}};$n=new WeakMap,et=new WeakMap,nt=new WeakMap,mn=new WeakMap,tt=new WeakMap,bn=new WeakMap,Lt=new WeakMap,on=new WeakMap,ln=new WeakMap,rt=new WeakMap,qn=new WeakMap,G=new WeakSet,Br=function(){if(this.is_fork)return!0;for(const e of f(this,mn).keys())if(!j(this,G,is).call(this,e))return!0;return!1},is=function(e){for(var t=e;t!==null;t=t.parent)if(f(this,ln).has(t))return!0;return!1},Lr=function(){var e=[];for(const s of f(this,bn))if(!((s.f&_e)!==0||(s.f&(re|De))===0)){for(var t=s,r=!1;t.parent!==null;){t=t.parent;var i=t.f;if((i&(Ln|tn))!==0){if((i&ge)===0){r=!0;break}t.f^=ge}}r||e.push(t)}return F(this,bn,[]),e},bt=function(){var l,c,d,u;this.started=!0;for(const[g,p]of f(this,on))de(g,p),this.schedule(g);this.apply();for(var e=Ct=[],t=[],r=er=[];f(this,bn).length>0;){_i++>1e3&&(j(this,G,xt).call(this),Ho());for(const g of j(this,G,Lr).call(this))try{j(this,G,Pr).call(this,g,e,t)}catch(p){throw ds(g),j(this,G,Br).call(this)||this.discard(),p}}if(D=null,r.length>0){var i=cr.ensure();for(const g of r)i.schedule(g)}if(Ct=null,er=null,j(this,G,Br).call(this)){j(this,G,Xn).call(this,t),j(this,G,Xn).call(this,e);for(const[g,p]of f(this,ln))cs(g,p);r.length>0&&j(l=D,G,bt).call(l);return}const s=j(this,G,ss).call(this);if(s){j(this,G,Xn).call(this,t),j(this,G,Xn).call(this,e),j(c=s,G,ls).call(c,this);return}f(this,on).clear();for(const g of f(this,$n))g(this);f(this,$n).clear(),Bn=this,mi(t),mi(e),Bn=null,(d=f(this,tt))==null||d.resolve();var o=D;f(this,nt)===0&&(f(this,bn).length===0||o!==null)&&j(this,G,xt).call(this),o!==null&&(fn.clear(),j(u=o,G,bt).call(u))},Pr=function(e,t,r){e.f^=ge;for(var i=e.first;i!==null;){var s=i.f,o=(s&(tn|Ln))!==0,l=o&&(s&ge)!==0,c=l||(s&Me)!==0||f(this,ln).has(i);if(!c&&i.fn!==null){o?i.f^=ge:(s&ot)!==0?t.push(i):Kt(i)&&((s&Je)!==0&&this.add_dirty_reaction(i,De),ct(i));var d=i.first;if(d!==null){i=d;continue}}for(;i!==null;){var u=i.next;if(u!==null){i=u;break}i=i.parent}}},ss=function(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&j(this,G,os).call(this,e))return e;return null},os=function(e){for(const[t,[,r]]of this.current)if(!r&&e.current.has(t))return!0;return!1},ls=function(e){var r;for(const[i,s]of e.current)!this.previous.has(i)&&e.previous.has(i)&&this.previous.set(i,e.previous.get(i)),this.current.set(i,s);for(const[i,s]of e.async_deriveds){const o=this.async_deriveds.get(i);o&&s.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(f(e,on));const t=i=>{var s=i.reactions;if(s!==null&&!((i.f&xe)!==0&&(i.f&(re|De))===0))for(const c of s){var o=c.f;if((o&xe)!==0)t(c);else{var l=c;o&(Zn|Je)&&!this.async_deriveds.has(l)&&(f(this,on).delete(l),de(l,re),this.schedule(l))}}};for(const i of this.current.keys())t(i);this.oncommit(()=>e.discard()),j(r=e,G,xt).call(r),D=this,j(this,G,bt).call(this)},Xn=function(e){for(var t=0;t<e.length;t+=1)$i(e[t],f(this,on))},Ro=function(){var g,p;for(let v=pr;v!==null;v=v.next){var e=v.id<this.id,t=[];for(const[_,[h,w]]of this.current){if(v.current.has(_)){var r=v.current.get(_)[0];if(e&&h!==r)v.current.set(_,[h,w]);else continue}t.push(_)}if(e)for(const[_,h]of this.async_deriveds){const w=v.async_deriveds.get(_);w&&h.promise.then(w.resolve).catch(w.reject)}var i=[...v.current.keys()].filter(_=>!v.current.get(_)[1]);if(!(!v.started||i.length===0)){var s=i.filter(_=>!this.current.has(_));if(s.length===0)e&&v.discard();else if(t.length>0){if(e)for(const _ of f(this,rt))v.unskip_effect(_,h=>{var w;(h.f&(Je|Zn))!==0?v.schedule(h):j(w=v,G,Xn).call(w,[h])});v.activate();var o=new Set,l=new Map;for(var c of t)as(c,s,o,l);l=new Map;var d=[...v.current].filter(([_,h])=>{const w=this.current.get(_);return w?w[0]!==h[0]||w[1]!==h[1]:!0}).map(([_])=>_);if(d.length>0)for(const _ of f(this,Lt))(_.f&(_e|Me|tr))===0&&$r(_,d,l)&&((_.f&(Zn|Je))!==0?(de(_,re),v.schedule(_)):v.add_dirty_reaction(_,re));if(f(v,bn).length>0&&!f(v,qn)){v.apply();for(var u of j(g=v,G,Lr).call(g))j(p=v,G,Pr).call(p,u,[],[])}v.deactivate()}}}},xt=function(){if(this.linked){var e=this.prev,t=this.next;e===null?pr=t:e.next=t,t===null?Wn=e:t.prev=e,this.linked=!1}};let Un=cr;function Fo(n){var e=Mt,t=Bn;Bn=null,Mt=!0;try{for(var r;;){if(Bo(),D===null)return r;D.flush()}}finally{Mt=e,Bn=t}}function Ho(){try{ko()}catch(n){dn(n,Tr)}}let Ke=null;function mi(n){var e=n.length;if(e!==0){for(var t=0;t<e;){var r=n[t++];if((r.f&(_e|Me))===0&&Kt(r)&&(Ke=new Set,ct(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&ks(r),(Ke==null?void 0:Ke.size)>0)){fn.clear();for(const i of Ke){if((i.f&(_e|Me))!==0)continue;const s=[i];let o=i.parent;for(;o!==null;)Ke.has(o)&&(Ke.delete(o),s.push(o)),o=o.parent;for(let l=s.length-1;l>=0;l--){const c=s[l];(c.f&(_e|Me))===0&&ct(c)}}Ke.clear()}}Ke=null}}function as(n,e,t,r){if(!t.has(n)&&(t.add(n),n.reactions!==null))for(const i of n.reactions){const s=i.f;(s&xe)!==0?as(i,e,t,r):(s&(Zn|Je))!==0&&(s&re)===0&&$r(i,e,r)&&(de(i,re),ei(i))}}function $r(n,e,t){const r=t.get(n);if(r!==void 0)return r;if(n.deps!==null)for(const i of n.deps){if(At.call(e,i))return!0;if((i.f&xe)!==0&&$r(i,e,t))return t.set(i,!0),!0}return t.set(n,!1),!1}function ei(n){D.schedule(n)}function cs(n,e){if(!((n.f&tn)!==0&&(n.f&ge)!==0)){(n.f&re)!==0?e.d.push(n):(n.f&De)!==0&&e.m.push(n),de(n,ge);for(var t=n.first;t!==null;)cs(t,e),t=t.next}}function ds(n){de(n,ge);for(var e=n.first;e!==null;)ds(e),e=e.next}let sr=new Set;const fn=new Map;let us=!1;function Gn(n,e){var t={f:0,v:n,reactions:null,equals:Wi,rv:0,wv:0};return t}function J(n,e){const t=Gn(n);return Cs(t),t}function Vo(n,e=!1,t=!0){const r=Gn(n);return e||(r.equals=Ki),r}function q(n,e,t=!1){H!==null&&(!$e||(H.f&tr)!==0)&&Ji()&&(H.f&(xe|Je|Zn|tr))!==0&&(hn===null||!hn.has(n))&&So();let r=t?Qe(e):e;return at(n,r,er)}var Pn=null,jr=0;function at(n,e,t=null){if(!n.equals(e)){Sn?fn.set(n,e):fn.has(n)||fn.set(n,n.v);var r=Un.ensure();if(r.capture(n,e),(n.f&xe)!==0){const i=n;(n.f&re)!==0&&Qr(i),pe===null&&Jr(i)}n.wv=Es(),Pn=null,jr=0,vs(n,re,t),Pn=null,V!==null&&(V.f&ge)!==0&&(V.f&(tn|Ln))===0&&(Fe===null?$o([n]):Fe.push(n)),!r.is_fork&&sr.size>0&&!us&&Yo()}return e}function Yo(){us=!1;for(const n of sr){(n.f&ge)!==0&&de(n,De);let e;try{e=Kt(n)}catch{e=!0}e&&ct(n)}sr.clear()}function St(n){q(n,n.v+1)}function vs(n,e,t){var r=n.reactions;if(r!==null){var i=r.length;if(jr+=i,jr>1e5&&Pn===null&&(Pn=new Set),Pn!==null){if(Pn.has(n))return;Pn.add(n)}for(var s=0;s<i;s++){var o=r[s],l=o.f,c=(l&re)===0;if(c&&de(o,e),(l&tr)!==0)sr.add(o);else if((l&xe)!==0){var d=o;pe==null||pe.delete(d),vs(d,De,t)}else if(c){var u=o;(l&Je)!==0&&Ke!==null&&Ke.add(u),t!==null?t.push(u):ei(u)}}}}function Qe(n){if(typeof n!="object"||n===null||Mn in n||Gi in n)return n;const e=Xr(n);if(e!==ro&&e!==io)return n;var t=new Map,r=vr(n),i=J(0),s=Vn,o=l=>{if(Vn===s)return l();var c=H,d=Vn;Ge(null),yi(s);var u=l();return Ge(c),yi(d),u};return r&&t.set("length",J(n.length)),new Proxy(n,{defineProperty(l,c,d){(!("value"in d)||d.configurable===!1||d.enumerable===!1||d.writable===!1)&&Mo();var u=t.get(c);return u===void 0?o(()=>{var g=J(d.value);return t.set(c,g),g}):q(u,d.value,!0),!0},deleteProperty(l,c){var d=t.get(c);if(d===void 0){if(c in l){const u=o(()=>J(he));t.set(c,u),St(i)}}else q(d,he),St(i);return!0},get(l,c,d){var v;if(c===Mn)return n;var u=t.get(c),g=c in l;if(u===void 0&&(!g||(v=Kn(l,c))!=null&&v.writable)&&(u=o(()=>{var _=Qe(g?l[c]:he),h=J(_);return h}),t.set(c,u)),u!==void 0){var p=a(u);return p===he?void 0:p}return Reflect.get(l,c,d)},getOwnPropertyDescriptor(l,c){var p;(p=this.has)==null||p.call(this,l,c);var d=Reflect.getOwnPropertyDescriptor(l,c),u=t.get(c);if(u!==void 0){var g=a(u);if(g===he)return;if(d&&"value"in d)d.value=g;else return{enumerable:!0,configurable:!0,value:g,writable:!0}}return d},has(l,c){var p;if(c===Mn)return!0;var d=t.get(c),u=d!==void 0&&d.v!==he||Reflect.has(l,c);if(d!==void 0||(V!==null||H!==null)&&(!u||(p=Kn(l,c))!=null&&p.writable)){d===void 0&&(d=o(()=>{var v=u?Qe(l[c]):he,_=J(v);return _}),t.set(c,d));var g=a(d);if(g===he)return!1}return u},set(l,c,d,u){var M;var g=t.get(c),p=c in l;if(r&&c==="length")for(var v=d;v<g.v;v+=1){var _=t.get(v+"");_!==void 0?q(_,he):v in l&&(_=o(()=>J(he)),t.set(v+"",_))}if(g===void 0)(!p||(M=Kn(l,c))!=null&&M.writable)&&(g=o(()=>J(void 0)),q(g,Qe(d)),t.set(c,g));else{p=g.v!==he;var h=o(()=>Qe(d));q(g,h)}var w=Reflect.getOwnPropertyDescriptor(l,c);if(w!=null&&w.set&&w.set.call(u,d),!p){if(r&&typeof c=="string"){var x=t.get("length"),m=Number(c);Number.isInteger(m)&&m>=x.v&&q(x,m+1)}St(i)}return!0},ownKeys(l){a(i);var c=Reflect.ownKeys(l).filter(g=>{var p=t.get(g);return p===void 0||p.v!==he});for(var[d,u]of t)u.v!==he&&!(d in l)&&c.push(d);return c},setPrototypeOf(){Co()}})}function bi(n){try{if(n!==null&&typeof n=="object"&&Mn in n)return n[Mn]}catch{}return n}function fs(n,e){return Object.is(bi(n),bi(e))}var xi,hs,gs,ws;function Uo(){if(xi===void 0){xi=window,hs=/Firefox/.test(navigator.userAgent);var n=Element.prototype,e=Node.prototype,t=Text.prototype;gs=Kn(e,"firstChild").get,ws=Kn(e,"nextSibling").get,pi(n)&&(n[Ir]=void 0,n[$t]=null,n[zr]=void 0,n.__e=void 0),pi(t)&&(t[_t]=void 0)}}function Cn(n=""){return document.createTextNode(n)}function yn(n){return gs.call(n)}function Wt(n){return ws.call(n)}function L(n,e){return yn(n)}function le(n,e=!1){{var t=yn(n);return t instanceof Comment&&t.data===""?Wt(t):t}}function te(n,e=!1){return yn(n)}function P(n,e=1,t=!1){let r=n;for(;e--;)r=Wt(r);return r}function Go(n){n.textContent=""}function ps(){return!1}function ni(n,e,t){return e==null||e===Ri?t?document.createElement(n,{is:t}):document.createElement(n):t?document.createElementNS(e,n,{is:t}):document.createElementNS(e,n)}function Wo(n){var e=V;if(e===null)return H.f|=Tn,n;if((e.f&vt)===0&&(e.f&ot)===0)throw n;dn(n,e)}function dn(n,e){if(n===Ni)throw n;if(!(e!==null&&(e.f&_e)!==0)){for(;e!==null;){if((e.f&Ar)!==0&&(e.f&(_e|It))===0){if((e.f&vt)===0)throw n;try{e.b.error(n);return}catch(t){n=t}}e=e.parent}throw n}}function Xo(n){V===null&&(H===null&&xo(),bo()),Sn&&mo()}function Ko(n,e){var t=e.last;t===null?e.last=e.first=n:(t.next=n,n.prev=t,e.last=n)}function En(n,e){var t=V;t!==null&&(t.f&Me)!==0&&(n|=Me);var r={ctx:me,deps:null,nodes:null,f:n|re|nn,first:null,fn:e,last:null,next:null,parent:t,b:t&&t.b,prev:null,teardown:null,wv:0,ac:null};D==null||D.register_created_effect(r);var i=r;if((n&ot)!==0)Ct!==null?Ct.push(r):Un.ensure().schedule(r);else if(e!==null){try{ct(r)}catch(o){throw ze(r),o}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&ft)===0&&(i=i.first,(n&Je)!==0&&(n&Yn)!==0&&i!==null&&(i.f|=Yn))}if(i!==null&&(i.parent=t,t!==null&&Ko(i,t),H!==null&&(H.f&xe)!==0&&(n&Ln)===0)){var s=H;(s.effects??(s.effects=[])).push(i)}return r}function ti(){return H!==null&&!$e}function _s(n){const e=En(hr,null);return de(e,ge),e.teardown=n,e}function Jn(n){Xo();var e=V.f,t=!H&&(e&tn)!==0&&me!==null&&!me.i;if(t){var r=me;(r.e??(r.e=[])).push(n)}else return ms(n)}function ms(n){return En(ot|ao,n)}function Zo(n){Un.ensure();const e=En(Ln|ft,n);return(t={})=>new Promise(r=>{t.outro?Hn(e,()=>{ze(e),r(void 0)}):(ze(e),r(void 0))})}function ri(n){return En(ot,n)}function Jo(n){return En(Zn|ft,n)}function ii(n,e=0){return En(hr|e,n)}function B(n,e=[],t=[],r=[]){Po(r,e,t,i=>{En(hr,()=>{n(...i.map(a))})})}function Xt(n,e=0){var t=En(Je|e,n);return t}function Ye(n){return En(tn|ft,n)}function bs(n){var e=n.teardown;if(e!==null){const t=Sn,r=H;ki(!0),Ge(null);try{e.call(null)}catch(i){dn(i,n.parent)}finally{ki(t),Ge(r)}}}function si(n,e=!1){var t=n.first;for(n.first=n.last=null;t!==null;){const i=t.ac;i!==null&&Gt(()=>{i.abort(Ut)});var r=t.next;(t.f&Ln)!==0?t.parent=null:ze(t,e),t=r}}function Qo(n){for(var e=n.first;e!==null;){var t=e.next;(e.f&tn)===0&&ze(e),e=t}}function ze(n,e=!0){var t=!1;(e||(n.f&lo)!==0)&&n.nodes!==null&&n.nodes.end!==null&&(xs(n.nodes.start,n.nodes.end),t=!0),n.f|=It,si(n,e&&!t),Tt(n,0);var r=n.nodes&&n.nodes.t;if(r!==null)for(const s of r)s.stop();bs(n),n.f^=It,n.f|=_e;var i=n.parent;i!==null&&i.first!==null&&ks(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.fn=n.nodes=n.ac=n.b=null}function xs(n,e){for(;n!==null;){var t=n===e?null:Wt(n);n.remove(),n=t}}function ks(n){var e=n.parent,t=n.prev,r=n.next;t!==null&&(t.next=r),r!==null&&(r.prev=t),e!==null&&(e.first===n&&(e.first=r),e.last===n&&(e.last=t))}function Hn(n,e,t=!0){var r=[];n.f|=Kr,ys(n,r,!0);var i=()=>{t&&ze(n),e&&e()},s=r.length;if(s>0){var o=()=>--s||i();for(var l of r)l.out(o)}else i()}function ys(n,e,t){if((n.f&Me)===0){n.f^=Me;var r=n.nodes&&n.nodes.t;if(r!==null)for(const l of r)(l.is_global||t)&&e.push(l);for(var i=n.first;i!==null;){var s=i.next;if((i.f&Ln)===0){var o=(i.f&Yn)!==0||(i.f&tn)!==0&&(n.f&Je)!==0;ys(i,e,o?t:!1)}i=s}}}function or(n){n.f&=~Kr,Ms(n,!0)}function Ms(n,e){if((n.f&Kr)===0&&(n.f&Me)!==0){n.f^=Me,(n.f&ge)===0&&(de(n,re),Un.ensure().schedule(n));for(var t=n.first;t!==null;){var r=t.next,i=(t.f&Yn)!==0||(t.f&tn)!==0;Ms(t,i?e:!1),t=r}var s=n.nodes&&n.nodes.t;if(s!==null)for(const o of s)(o.is_global||e)&&o.in()}}function oi(n,e){if(n.nodes)for(var t=n.nodes.start,r=n.nodes.end;t!==null;){var i=t===r?null:Wt(t);e.append(t),t=i}}let Sn=!1;function ki(n){Sn=n}let H=null,$e=!1;function Ge(n){H=n}let V=null;function gn(n){V=n}let hn=null;function Cs(n){H!==null&&((H.f&rr)!==0||(H.f&xe)!==0)&&(hn??(hn=new Set)).add(n)}let Ie=null,je=0,Fe=null;function $o(n){Fe=n}let Ss=1,Dn=0,Vn=Dn;function yi(n){Vn=n}function Es(){return++Ss}function Kt(n){var e=n.f;if((e&re)!==0)return!0;if((e&De)!==0){for(var t=n.deps,r=t.length,i=0;i<r;i++){var s=t[i];if(Kt(s)&&ts(s),s.wv>n.wv)return!0}(e&nn)!==0&&pe===null&&de(n,ge)}return!1}function As(n,e,t=!0){var r=n.reactions;if(r!==null&&!(hn!==null&&hn.has(n)))for(var i=0;i<r.length;i++){var s=r[i];(s.f&xe)!==0?As(s,e,!1):e===s&&(t?de(s,re):(s.f&ge)!==0&&de(s,De),ei(s))}}function Is(n){var e=Ie,t=je,r=Fe,i=H,s=hn,o=me,l=$e,c=Vn,d=n.f;Ie=null,je=0,Fe=null,H=(d&(tn|Ln))===0?n:null,hn=null,lt(n.ctx),$e=!1,Vn=++Dn,n.ac!==null&&(Gt(()=>{n.ac.abort(Ut)}),n.ac=null);try{n.f|=rr;var u=n.fn,g=u();n.f|=vt;var p=Mi(n);if(Ji()&&Fe!==null&&!$e&&p!==null&&(n.f&(xe|De|re))===0)for(var v=0;v<Fe.length;v++)As(Fe[v],n);if(i!==null&&i!==n){if(Dn++,i.deps!==null)for(let _=0;_<t;_+=1)i.deps[_].rv=Dn;if(e!==null)for(const _ of e)_.rv=Dn;Fe!==null&&(r===null?r=Fe:r.push(...Fe))}return(n.f&Tn)!==0&&(n.f^=Tn),g}catch(_){return Mi(n),Wo(_)}finally{n.f^=rr,Ie=e,je=t,Fe=r,H=i,hn=s,lt(o),$e=l,Vn=c}}function Mi(n){var i;var e=n.deps,t=D==null?void 0:D.is_fork;if(Ie!==null){var r;if(t||Tt(n,je),e!==null&&je>0)for(e.length=je+Ie.length,r=0;r<Ie.length;r++)e[je+r]=Ie[r];else n.deps=e=Ie;if(ti()&&(n.f&nn)!==0)for(r=je;r<e.length;r++)((i=e[r]).reactions??(i.reactions=[])).push(n)}else!t&&e!==null&&je<e.length&&(Tt(n,je),e.length=je);return e}function el(n,e){let t=e.reactions;if(t!==null){var r=to.call(t,n);if(r!==-1){var i=t.length-1;i===0?t=e.reactions=null:(t[r]=t[i],t.pop())}}if(t===null&&(e.f&xe)!==0&&(Ie===null||!At.call(Ie,e))){var s=e;(s.f&nn)!==0&&(s.f^=nn),s.v!==he&&Jr(s),s.ac!==null&&Gt(()=>{s.ac.abort(Ut),s.ac=null,de(s,re)}),qo(s),Tt(s,0)}}function Tt(n,e){var t=n.deps;if(t!==null)for(var r=e;r<t.length;r++)el(n,t[r])}function ct(n){var e=n.f;if((e&_e)===0){de(n,ge);var t=V;V=n;try{(e&(Je|Ui))!==0?Qo(n):si(n),bs(n);var r=Is(n);n.teardown=typeof r=="function"?r:null,n.wv=Ss;var i;Fi&&Ao&&(n.f&re)!==0&&n.deps}finally{V=t}}}async function zs(){await Promise.resolve(),Fo()}function a(n){var e=n.f,t=(e&xe)!==0;if(H!==null&&!$e){var r=V!==null&&(V.f&_e)!==0;if(!r&&(hn===null||!hn.has(n))){var i=H.deps;if((H.f&rr)!==0)n.rv<Dn&&(n.rv=Dn,Ie===null&&i!==null&&i[je]===n?je++:Ie===null?Ie=[n]:Ie.push(n));else{H.deps??(H.deps=[]),At.call(H.deps,n)||H.deps.push(n);var s=n.reactions;s===null?n.reactions=[H]:At.call(s,H)||s.push(H)}}}if(Sn&&fn.has(n))return fn.get(n);if(t){var o=n;if(Sn){var l=o.v;return((o.f&ge)===0&&o.reactions!==null||Bs(o))&&(l=Qr(o)),fn.set(o,l),l}var c=(o.f&nn)===0&&!$e&&H!==null&&(H.f&nn)!==0,d=(o.f&vt)===0;Kt(o)&&(c&&(o.f|=nn),ts(o)),c&&!d&&(rs(o),Ts(o))}if(pe!=null&&pe.has(n))return pe.get(n);if((n.f&Tn)!==0)throw n.v;return n.v}function Ts(n){if(n.f|=nn,n.deps!==null)for(const t of n.deps){var e=t.reactions;e===null?t.reactions=[n]:At.call(e,n)||e.push(n),(t.f&xe)!==0&&(t.f&nn)===0&&(rs(t),Ts(t))}}function Bs(n){if(n.v===he)return!0;if(n.deps===null)return!1;for(const e of n.deps)if(fn.has(e)||(e.f&xe)!==0&&Bs(e))return!0;return!1}function gr(n){var e=$e;try{return $e=!0,n()}finally{$e=e}}function nl(n){if(!(typeof n!="object"||!n||n instanceof EventTarget)){if(Mn in n)Dr(n);else if(!Array.isArray(n))for(let e in n){const t=n[e];typeof t=="object"&&t&&Mn in t&&Dr(t)}}}function Dr(n,e=new Set){if(typeof n=="object"&&n!==null&&!(n instanceof EventTarget)&&!e.has(n)){e.add(n),n instanceof Date&&n.getTime();for(let r in n)try{Dr(n[r],e)}catch{}const t=Xr(n);if(t!==Object.prototype&&t!==Array.prototype&&t!==Map.prototype&&t!==Set.prototype&&t!==Date.prototype){const r=Vi(t);for(let i in r){const s=r[i].get;if(s)try{s.call(n)}catch{}}}}}const On=Symbol("events"),Ls=new Set,Or=new Set;function tl(n,e,t,r={}){function i(s){if(r.capture||qr.call(e,s),!s.cancelBubble)return Gt(()=>t==null?void 0:t.call(this,s))}return n.startsWith("pointer")||n.startsWith("touch")||n==="wheel"?(i.__removed=!1,kn(()=>{i.__removed||e.addEventListener(n,i,r)})):e.addEventListener(n,i,r),i}function rl(n,e,t,r={}){var i=tl(e,n,t,r);return()=>{i.__removed=!0,n.removeEventListener(e,i,r)}}function Q(n,e,t){(e[On]??(e[On]={}))[n]=t}function pn(n){for(var e=0;e<n.length;e++)Ls.add(n[e]);for(var t of Or)t(n)}let mr=null,br=!1;function qr(n){var h,w;var e=this,t=e.ownerDocument,r=n.type,i=((h=n.composedPath)==null?void 0:h.call(n))||[],s=i[0]||n.target;mr=n,br||(br=!0,setTimeout(()=>{br=!1,mr=null}));var o=0,l=mr===n&&n[On];if(l){var c=i.indexOf(l);if(c!==-1&&(e===document||e===window)){n[On]=e;return}var d=i.indexOf(e);if(d===-1)return;c<=d&&(o=c)}if(s=i[o]||n.target,s!==e){Hi(n,"currentTarget",{configurable:!0,get(){return s||t}});var u=H,g=V;Ge(null),gn(null);try{for(var p,v=[];s!==null&&s!==e;){try{var _=(w=s[On])==null?void 0:w[r];_!=null&&(!s.disabled||n.target===s)&&_.call(s,n)}catch(x){p?v.push(x):p=x}if(n.cancelBubble)break;o++,s=o<i.length?i[o]:null}if(p){for(let x of v)queueMicrotask(()=>{throw x});throw p}}finally{n[On]=e,delete n.currentTarget,Ge(u),gn(g)}}}var Oi;const xr=((Oi=globalThis==null?void 0:globalThis.window)==null?void 0:Oi.trustedTypes)&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:n=>n});function il(n){return(xr==null?void 0:xr.createHTML(n))??n}function sl(n){var e=ni("template");return e.innerHTML=il(n.replaceAll("<!>","<!---->")),e.content}function dt(n,e){var t=V;t.nodes===null&&(t.nodes={start:n,end:e,a:null,t:null})}function I(n,e){var t=(e&1)!==0,r=(e&2)!==0,i,s=!n.startsWith("<!>");return()=>{i===void 0&&(i=sl(s?n:"<!>"+n),t||(i=yn(i)));var o=r||hs?document.importNode(i,!0):i.cloneNode(!0);if(t){var l=yn(o),c=o.lastChild;dt(l,c)}else dt(o,o);return o}}function Ps(n=""){{var e=Cn(n+"");return dt(e,e),e}}function We(){var n=document.createDocumentFragment(),e=document.createComment(""),t=Cn();return n.append(e,t),dt(e,t),n}function C(n,e){n!==null&&n.before(e)}const ol=["touchstart","touchmove"];function ll(n){return ol.includes(n)}function al(n){let e=0,t=Gn(0),r;return()=>{ti()&&(a(t),ii(()=>(e===0&&(r=gr(()=>n(()=>St(t)))),e+=1,()=>{kn(()=>{e-=1,e===0&&(r==null||r(),r=void 0,St(t))})})))}}var cl=Yn|ft;function dl(n,e,t,r){new ul(n,e,t,r)}var He,Wr,Ve,Nn,be,Be,ye,Le,an,Rn,zn,it,Pt,xn,dr,K,vl,fl,Nr,In,hl,Rr,kt,nr,Fr,Hr;class ul{constructor(e,t,r,i){N(this,K);we(this,"parent");we(this,"is_pending",!1);we(this,"transform_error");N(this,He);N(this,Wr,null);N(this,Ve);N(this,Nn);N(this,be);N(this,Be,null);N(this,ye,null);N(this,Le,null);N(this,an,null);N(this,Rn,0);N(this,zn,0);N(this,it,!1);N(this,Pt,new Map);N(this,xn,null);N(this,dr,al(()=>(F(this,xn,Gn(f(this,Rn))),()=>{F(this,xn,null)})));var s;F(this,He,e),F(this,Ve,t),F(this,Nn,o=>{var l=V;l.b=this,l.f|=Ar,r(o)}),this.parent=V.b,this.transform_error=i??((s=this.parent)==null?void 0:s.transform_error)??(o=>o),F(this,be,Xt(()=>{j(this,K,Rr).call(this)},cl))}defer_effect(e){$i(e,f(this,Pt))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!f(this,Ve).pending}update_pending_count(e,t){j(this,K,Fr).call(this,e,t),F(this,Rn,f(this,Rn)+e),!(!f(this,xn)||f(this,it))&&(F(this,it,!0),kn(()=>{F(this,it,!1),f(this,xn)&&at(f(this,xn),f(this,Rn))}))}get_effect_pending(){return f(this,dr).call(this),a(f(this,xn))}error(e){if(e===Ni||!f(this,Ve).onerror&&!f(this,Ve).failed)throw e;D!=null&&D.is_fork?(f(this,Be)&&D.skip_effect(f(this,Be)),f(this,ye)&&D.skip_effect(f(this,ye)),f(this,Le)&&D.skip_effect(f(this,Le)),D.oncommit(()=>{j(this,K,In).call(this)||j(this,K,Hr).call(this,e)})):j(this,K,Hr).call(this,e)}}He=new WeakMap,Wr=new WeakMap,Ve=new WeakMap,Nn=new WeakMap,be=new WeakMap,Be=new WeakMap,ye=new WeakMap,Le=new WeakMap,an=new WeakMap,Rn=new WeakMap,zn=new WeakMap,it=new WeakMap,Pt=new WeakMap,xn=new WeakMap,dr=new WeakMap,K=new WeakSet,vl=function(){try{F(this,Be,Ye(()=>f(this,Nn).call(this,f(this,He))))}catch(e){this.error(e)}},fl=function(e){const t=f(this,Ve).failed,{reset:r,invoke_onerror:i}=j(this,K,Nr).call(this,e);kn(i),t&&F(this,Le,Ye(()=>{t(f(this,He),()=>e,()=>r)}))},Nr=function(e){var t=!1,r=!1;const i=()=>{if(!j(this,K,In).call(this)){if(t){go();return}t=!0,r&&Eo(),f(this,Le)!==null&&Hn(f(this,Le),()=>{F(this,Le,null)}),j(this,K,nr).call(this,()=>{j(this,K,Rr).call(this)})}};return{reset:i,invoke_onerror:()=>{var o,l;if(!j(this,K,In).call(this))try{r=!0,(l=(o=f(this,Ve)).onerror)==null||l.call(o,e,i),r=!1}catch(c){dn(c,f(this,be)&&f(this,be).parent)}}}},In=function(){return(f(this,be).f&(_e|It))!==0},hl=function(){const e=f(this,Ve).pending;e&&(this.is_pending=!0,F(this,ye,Ye(()=>e(f(this,He)))),kn(()=>{if(!j(this,K,In).call(this)){var t=F(this,an,document.createDocumentFragment()),r=Cn(),i=!1;if(t.append(r),F(this,Be,j(this,K,nr).call(this,()=>{try{return Ye(()=>f(this,Nn).call(this,r))}catch(s){try{this.error(s),i=!0}catch(o){dn(o,f(this,be).parent)}return null}})),f(this,Be)===null){F(this,an,null),i&&j(this,K,kt).call(this,D);return}f(this,zn)===0&&(f(this,He).before(t),F(this,an,null),Hn(f(this,ye),()=>{F(this,ye,null)}),j(this,K,kt).call(this,D))}}))},Rr=function(){try{if(this.is_pending=this.has_pending_snippet(),F(this,zn,0),F(this,Rn,0),F(this,Be,Ye(()=>{f(this,Nn).call(this,f(this,He))})),f(this,zn)>0){var e=F(this,an,document.createDocumentFragment());oi(f(this,Be),e);const t=f(this,Ve).pending;F(this,ye,Ye(()=>t(f(this,He))))}else j(this,K,kt).call(this,D)}catch(t){this.error(t)}},kt=function(e){this.is_pending=!1,e.transfer_reactions(f(this,Pt))},nr=function(e){var t=V,r=H,i=me;gn(f(this,be)),Ge(f(this,be)),lt(f(this,be).ctx);try{return Un.ensure(),e()}finally{gn(t),Ge(r),lt(i)}},Fr=function(e,t){var r;if(!this.has_pending_snippet()){this.parent&&j(r=this.parent,K,Fr).call(r,e,t);return}F(this,zn,f(this,zn)+e),f(this,zn)===0&&(j(this,K,kt).call(this,t),f(this,ye)&&Hn(f(this,ye),()=>{F(this,ye,null)}),f(this,an)&&(f(this,He).before(f(this,an)),F(this,an,null)))},Hr=function(e){f(this,Be)&&(ze(f(this,Be)),F(this,Be,null)),f(this,ye)&&(ze(f(this,ye)),F(this,ye,null)),f(this,Le)&&(ze(f(this,Le)),F(this,Le,null));let t=f(this,Ve).failed;const r=i=>{if(j(this,K,In).call(this))return;const{reset:s,invoke_onerror:o}=j(this,K,Nr).call(this,i);o(),t&&!j(this,K,In).call(this)&&F(this,Le,j(this,K,nr).call(this,()=>{try{return Ye(()=>{var l=V;l.b=this,l.f|=Ar,t(f(this,He),()=>i,()=>s)})}catch(l){return dn(l,f(this,be).parent),null}}))};kn(()=>{if(!j(this,K,In).call(this)){var i;try{i=this.transform_error(e)}catch(s){dn(s,f(this,be)&&f(this,be).parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(r,s=>dn(s,f(this,be)&&f(this,be).parent)):r(i)}})};function Y(n,e){var t=e==null?"":typeof e=="object"?`${e}`:e;t!==(n[_t]??(n[_t]=n.nodeValue))&&(n[_t]=t,n.nodeValue=`${t}`)}function js(n,e){return gl(n,e)}const Jt=new Map;function gl(n,{target:e,anchor:t,props:r={},events:i,context:s,intro:o=!0,transformError:l}){Uo();var c=void 0,d=Zo(()=>{var u=t??e.appendChild(Cn());dl(u,{pending:()=>{}},v=>{Ce({});var _=me;s&&(_.c=s),i&&(r.$$events=i),c=n(v,r)||Zr(),Se()},l);var g=new Set,p=v=>{for(var _=0;_<v.length;_++){var h=v[_];if(!g.has(h)){g.add(h);var w=ll(h);for(const M of[e,document]){var x=Jt.get(M);x===void 0&&(x=new Map,Jt.set(M,x));var m=x.get(h);m===void 0?(M.addEventListener(h,qr,{passive:w}),x.set(h,1)):x.set(h,m+1)}}}};return p(fr(Ls)),Or.add(p),()=>{var w;for(var v of g)for(const x of[e,document]){var _=Jt.get(x),h=_.get(v);--h==0?(x.removeEventListener(v,qr),_.delete(v),_.size===0&&Jt.delete(x)):_.set(v,h)}Or.delete(p),u!==t&&((w=u.parentNode)==null||w.removeChild(u))}});return Vr.set(c,d),c}let Vr=new WeakMap;function Ds(n,e){const t=Vr.get(n);return t?(Vr.delete(n),t(e)):Promise.resolve()}var Ze,cn,Pe,Fn,jt,Dt,ur;class li{constructor(e,t=!0){we(this,"anchor");N(this,Ze,new Map);N(this,cn,new Map);N(this,Pe,new Map);N(this,Fn,new Set);N(this,jt,!0);N(this,Dt,e=>{if(f(this,Ze).has(e)){var t=f(this,Ze).get(e),r=f(this,cn).get(t);if(r)or(r),f(this,Fn).delete(t);else{var i=f(this,Pe).get(t);i&&(or(i.effect),f(this,cn).set(t,i.effect),f(this,Pe).delete(t),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),r=i.effect)}for(const[s,o]of f(this,Ze)){if(f(this,Ze).delete(s),s===e)break;const l=f(this,Pe).get(o);l&&(ze(l.effect),f(this,Pe).delete(o))}for(const[s,o]of f(this,cn)){if(s===t||f(this,Fn).has(s))continue;const l=()=>{if(Array.from(f(this,Ze).values()).includes(s)){var d=document.createDocumentFragment();oi(o,d),d.append(Cn()),f(this,Pe).set(s,{effect:o,fragment:d})}else ze(o);f(this,Fn).delete(s),f(this,cn).delete(s)};f(this,jt)||!r?(f(this,Fn).add(s),Hn(o,l,!1)):l()}}});N(this,ur,e=>{f(this,Ze).delete(e);const t=Array.from(f(this,Ze).values());for(const[r,i]of f(this,Pe))t.includes(r)||(ze(i.effect),f(this,Pe).delete(r))});this.anchor=e,F(this,jt,t)}ensure(e,t){var r=D,i=ps();if(t&&!f(this,cn).has(e)&&!f(this,Pe).has(e))if(i){var s=document.createDocumentFragment(),o=Cn();s.append(o),f(this,Pe).set(e,{effect:Ye(()=>t(o)),fragment:s})}else f(this,cn).set(e,Ye(()=>t(this.anchor)));if(f(this,Ze).set(r,e),i){for(const[l,c]of f(this,cn))l===e?r.unskip_effect(c):r.skip_effect(c);for(const[l,c]of f(this,Pe))l===e?r.unskip_effect(c.effect):r.skip_effect(c.effect);r.oncommit(f(this,Dt)),r.ondiscard(f(this,ur))}else f(this,Dt).call(this,r)}}Ze=new WeakMap,cn=new WeakMap,Pe=new WeakMap,Fn=new WeakMap,jt=new WeakMap,Dt=new WeakMap,ur=new WeakMap;function Oe(n,e,...t){var r=new li(n);Xt(()=>{const i=e()??null;r.ensure(i,i&&(s=>i(s,...t)))},Yn)}function U(n,e,t=!1){var r=new li(n),i=t?Yn:0;function s(o,l){r.ensure(o,l)}Xt(()=>{var o=!1;e((l,c=0)=>{o=!0,s(c,l)}),o||s(-1,null)},i)}const wl=Symbol("NaN");function pl(n,e,t){var r=new li(n);Xt(()=>{var i=e();i!==i&&(i=wl),r.ensure(i,t)})}function lr(n,e){return e}function _l(n,e,t){for(var r=[],i=e.length,s,o=e.length,l=0;l<i;l++){let g=e[l];Hn(g,()=>{if(s){if(s.pending.delete(g),s.done.add(g),s.pending.size===0){var p=n.outrogroups;Yr(n,fr(s.done)),p.delete(s),p.size===0&&(n.outrogroups=null)}}else o-=1},!1)}if(o===0){var c=r.length===0&&t!==null&&n.pending.size===0;if(c){var d=t,u=d.parentNode;Go(u),u.append(d),n.items.clear()}Yr(n,e,!c)}else s={pending:new Set(e),done:new Set},(n.outrogroups??(n.outrogroups=new Set)).add(s)}function Yr(n,e,t=!0){var r;if(n.pending.size>0){r=new Set;for(const o of n.pending.values())for(const l of o)r.add(n.items.get(l).e)}for(var i=0;i<e.length;i++){var s=e[i];if(r!=null&&r.has(s)){s.f|=un;const o=document.createDocumentFragment();oi(s,o)}else ze(e[i],t)}}var Ci;function Ue(n,e,t,r,i,s=null){var o=n,l=new Map,c=(e&4)!==0;if(c){var d=n;o=d.appendChild(Cn())}var u=null,g=ns(()=>{var m=t();return vr(m)?m:m==null?[]:fr(m)}),p=new Map,v=!0;function _(m){if((x.effect.f&_e)===0){x.pending.delete(m);var M=a(g);x.fallback=u,ml(x,M,o,e,r),u!==null&&(M.length===0?(u.f&un)===0?or(u):(u.f^=un,yt(u,null,o)):Hn(u,()=>{u=null}))}}function h(m){x.pending.delete(m)}var w=Xt(()=>{for(var m=a(g),M=m.length,A=new Set,y=D,S=ps(),k=0;k<M;k+=1){var z=m[k],O=r(z,k),W=v?null:l.get(O);W?(W.v&&at(W.v,z),W.i&&at(W.i,k),S&&y.unskip_effect(W.e)):(W=bl(l,v?o:Ci??(Ci=Cn()),z,O,k,i,e,t),v||(W.e.f|=un),l.set(O,W)),A.add(O)}if(M===0&&s&&!u&&(v?u=Ye(()=>s(o)):(u=Ye(()=>s(Ci??(Ci=Cn()))),u.f|=un)),M>A.size&&_o(),!v)if(p.set(y,A),S){for(const[$,oe]of l)A.has($)||y.skip_effect(oe.e);y.oncommit(_),y.ondiscard(h)}else _(y);a(g)}),x={effect:w,items:l,pending:p,outrogroups:null,fallback:u};v=!1}function pt(n){for(;n!==null&&(n.f&tn)===0;)n=n.next;return n}function ml(n,e,t,r,i){var W,$,oe,fe,Ee,R,ie,ue,rn;var s=(r&8)!==0,o=e.length,l=n.items,c=pt(n.effect.first),d,u=null,g,p=[],v=[],_,h,w,x;if(s)for(x=0;x<o;x+=1)_=e[x],h=i(_,x),w=l.get(h).e,(w.f&un)===0&&(($=(W=w.nodes)==null?void 0:W.a)==null||$.measure(),(g??(g=new Set)).add(w));for(x=0;x<o;x+=1){if(_=e[x],h=i(_,x),w=l.get(h).e,n.outrogroups!==null)for(const ke of n.outrogroups)ke.pending.delete(w),ke.done.delete(w);if((w.f&Me)!==0&&(or(w),s&&((fe=(oe=w.nodes)==null?void 0:oe.a)==null||fe.unfix(),(g??(g=new Set)).delete(w))),(w.f&un)!==0)if(w.f^=un,w===c)yt(w,null,t);else{var m=u?u.next:c;w===n.effect.last&&(n.effect.last=w.prev),w.prev&&(w.prev.next=w.next),w.next&&(w.next.prev=w.prev),An(n,u,w),An(n,w,m),yt(w,m,t),u=w,p=[],v=[],c=pt(u.next);continue}if(w!==c){if(d!==void 0&&d.has(w)){if(p.length<v.length){var M=v[0],A;u=M.prev;var y=p[0],S=p[p.length-1];for(A=0;A<p.length;A+=1)yt(p[A],M,t);for(A=0;A<v.length;A+=1)d.delete(v[A]);An(n,y.prev,S.next),An(n,u,y),An(n,S,M),c=M,u=S,x-=1,p=[],v=[]}else d.delete(w),yt(w,c,t),An(n,w.prev,w.next),An(n,w,u===null?n.effect.first:u.next),An(n,u,w),u=w;continue}for(p=[],v=[];c!==null&&c!==w;)(d??(d=new Set)).add(c),v.push(c),c=pt(c.next);if(c===null)continue}(w.f&un)===0&&p.push(w),u=w,c=pt(w.next)}if(n.outrogroups!==null){for(const ke of n.outrogroups)ke.pending.size===0&&(Yr(n,fr(ke.done)),(Ee=n.outrogroups)==null||Ee.delete(ke));n.outrogroups.size===0&&(n.outrogroups=null)}if(c!==null||d!==void 0){var k=[];if(d!==void 0)for(w of d)(w.f&Me)===0&&k.push(w);for(;c!==null;)(c.f&Me)===0&&c!==n.fallback&&k.push(c),c=pt(c.next);var z=k.length;if(z>0){var O=(r&4)!==0&&o===0?t:null;if(s){for(x=0;x<z;x+=1)(ie=(R=k[x].nodes)==null?void 0:R.a)==null||ie.measure();for(x=0;x<z;x+=1)(rn=(ue=k[x].nodes)==null?void 0:ue.a)==null||rn.fix()}_l(n,k,O)}}s&&kn(()=>{var ke,gt;if(g!==void 0)for(w of g)(gt=(ke=w.nodes)==null?void 0:ke.a)==null||gt.apply()})}function bl(n,e,t,r,i,s,o,l){var c=(o&1)!==0?(o&16)===0?Vo(t,!1,!1):Gn(t):null,d=(o&2)!==0?Gn(i):null;return{v:c,i:d,e:Ye(()=>(s(e,c??t,d??i,l),()=>{n.delete(r)}))}}function yt(n,e,t){if(n.nodes)for(var r=n.nodes.start,i=n.nodes.end,s=e&&(e.f&un)===0?e.nodes.start:t;r!==null;){var o=Wt(r);if(s.before(r),r===i)return;r=o}}function An(n,e,t){e===null?n.effect.first=t:e.next=t,t===null?n.effect.last=e:t.prev=e}function xl(n,e,t=!1,r=!1,i=!1,s=!1){var o=n,l="";if(t)var c=n;B(()=>{var d=V;if(l!==(l=e()??"")){if(t){d.nodes=null,c.innerHTML=l,l!==""&&dt(yn(c),c.lastChild);return}if(d.nodes!==null&&(xs(d.nodes.start,d.nodes.end),d.nodes=null),l!==""){var u=r?eo:i?no:void 0,g=ni(r?"svg":i?"math":"template",u);g.innerHTML=l;var p=r||i?g:g.content;if(dt(yn(p),p.lastChild),r||i)for(;yn(p);)o.before(yn(p));else o.before(p)}}})}function Te(n,e){ri(()=>{var i,s;n=((s=(i=V==null?void 0:V.parent)==null?void 0:i.nodes)==null?void 0:s.start)??n;var t=n.getRootNode(),r=t.host?t:t.head??t.ownerDocument.head;if(!r.querySelector("#"+e.hash)){const o=ni("style");o.id=e.hash,o.textContent=e.code,r.appendChild(o)}})}function Bt(n,e,t){ri(()=>{var r=gr(()=>e(n,t==null?void 0:t())||{});if(t&&(r!=null&&r.update)){var i=!1,s={};ii(()=>{var o=t();nl(o),i&&Xi(s,o)&&(s=o,r.update(o))}),i=!0}if(r!=null&&r.destroy)return()=>r.destroy()})}const Si=[...` 	
\r\f \v\uFEFF`];function kl(n,e,t){var r=n==null?"":""+n;if(t){for(var i of Object.keys(t))if(t[i])r=r?r+" "+i:i;else if(r.length)for(var s=i.length,o=0;(o=r.indexOf(i,o))>=0;){var l=o+s;(o===0||Si.includes(r[o-1]))&&(l===r.length||Si.includes(r[l]))?r=(o===0?"":r.substring(0,o))+r.substring(l+1):o=l}}return r===""?null:r}function Ei(n,e=!1){var t=e?" !important;":";",r="";for(var i of Object.keys(n)){var s=n[i];s!=null&&s!==""&&(r+=" "+i+": "+s+t)}return r}function yl(n,e){if(e){var t="",r,i;return Array.isArray(e)?(r=e[0],i=e[1]):r=e,r&&(t+=Ei(r)),i&&(t+=Ei(i,!0)),t=t.trim(),t===""?null:t}return String(n)}function Xe(n,e,t,r,i,s){var o=n[Ir];if(o!==t||o===void 0){var l=kl(t,r,s);l==null?n.removeAttribute("class"):n.className=l,n[Ir]=t}else if(s&&i!==s)for(var c in s){var d=!!s[c];(i==null||d!==!!i[c])&&n.classList.toggle(c,d)}return s}function kr(n,e={},t,r){for(var i in t){var s=t[i];e[i]!==s&&(t[i]==null?n.style.removeProperty(i):n.style.setProperty(i,s,r))}}function ht(n,e,t,r){var i=n[zr];if(i!==e){var s=yl(e,r);s==null?n.removeAttribute("style"):n.style.cssText=s,n[zr]=e}else r&&(Array.isArray(r)?(kr(n,t==null?void 0:t[0],r[0]),kr(n,t==null?void 0:t[1],r[1],"important")):kr(n,t,r));return r}function Ml(n,e){e?n.hasAttribute("selected")||n.setAttribute("selected",""):n.removeAttribute("selected")}function Cl(n,e){var t=n.__defaultValue,r=n.multiple,i=r?t??[]:null;if(!(r&&!vr(i))){n.selectedIndex;for(var s of n.options){var o=Ur(s);Ml(s,r?i.includes(o):fs(o,t))}}}function Os(n,e,t=!1){if(n.multiple){if(e==null)return;if(!vr(e))return ho();for(var r of n.options)r.selected=e.includes(Ur(r));return}for(r of n.options){var i=Ur(r);if(fs(i,e)){r.selected=!0;return}}(!t||e!==void 0)&&(n.selectedIndex=-1)}function Sl(n){var e=new MutationObserver(t=>{t.every(El)||("__defaultValue"in n&&Cl(n),"__value"in n&&Os(n,n.__value))});e.observe(n,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),_s(()=>{e.disconnect()})}function Ur(n){return"__value"in n?n.__value:n.value}function El(n){if(n.target.closest("selectedcontent")!==null)return!0;if(n.type==="childList"){var e=[...n.addedNodes,...n.removedNodes];return e.length>0&&e.every(t=>t.nodeName==="SELECTEDCONTENT")}return!1}const Al=Symbol("is custom element"),Il=Symbol("is html"),zl=vo?"progress":"PROGRESS";function Tl(n,e){var t=qs(n);t.value===(t.value=e??void 0)||n.value===e&&(e!==0||n.nodeName!==zl)||(n.value=e??"")}function Z(n,e,t,r){var i=qs(n),s=i[e];s!==(i[e]=t)&&(e==="loading"&&(n[uo]=t),t==null?n.removeAttribute(e):typeof t!="string"&&Bl(n).has(e)?n[e]=t:n.setAttribute(e,t))}function qs(n){return n[$t]??(n[$t]={[Al]:n.nodeName.includes("-"),[Il]:n.namespaceURI===Ri})}var Ai=new Map;function Bl(n){var e=n.getAttribute("is")||n.nodeName,t=Ai.get(e);if(t)return t;Ai.set(e,t=new Set);for(var r,i=n,s=Element.prototype;s!==i;){r=Vi(i);for(var o in r)r[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&t.add(o);i=Xr(i)}return t}function yr(n,e){return n===e||(n==null?void 0:n[Mn])===e}function ut(n=Zr(),e,t,r){var i=me.r,s=V;return ri(()=>{var o,l;return ii(()=>{o=l,l=[],gr(()=>{yr(t(...l),n)||(e(n,...l),o&&yr(t(...o),n)&&e(null,...o))})}),()=>{let c=s;for(;c!==i&&c.parent!==null&&c.parent.f&It;)c=c.parent;const d=()=>{l&&yr(t(...l),n)&&e(null,...l)},u=c.teardown;c.teardown=()=>{d(),u==null||u()}}}),n}let Qt=!1;function Ll(n){var e=Qt;try{return Qt=!1,[n(),Qt]}finally{Qt=e}}function en(n,e,t,r){var A;var i=!0,s=(t&8)!==0,o=(t&16)!==0,l=r,c=!0,d=void 0,u=()=>o&&i?(d??(d=zt(r)),a(d)):(c&&(c=!1,l=o?gr(r):r),l);let g;if(s){var p=Mn in n||co in n;g=((A=Kn(n,e))==null?void 0:A.set)??(p&&e in n?y=>n[e]=y:void 0)}var v,_=!1;s?[v,_]=Ll(()=>n[e]):v=n[e],v===void 0&&r!==void 0&&(v=u(),g&&(yo(),g(v)));var h;if(h=()=>{var y=n[e];return y===void 0?u():(c=!0,y)},(t&4)===0)return h;if(g){var w=n.$$legacy;return(function(y,S){return arguments.length>0?((!S||w||_)&&g(S?h():y),y):h()})}var x=!1,m=((t&1)!==0?zt:ns)(()=>(x=!1,h()));s&&a(m);var M=V;return(function(y,S){if(arguments.length>0){const k=S?a(m):s?Qe(y):y;return q(m,k),x=!0,l!==void 0&&(l=k),y}return Sn&&x||(M.f&_e)!==0?m.v:a(m)})}function Pl(n){return(e,t)=>{let r=t.onchange;const i=Qe({...t,onchange:o=>{i.value=o,r(o)}}),s=js(n,{target:e,props:i});return{update({onchange:o,...l}){o&&(r=o),Object.assign(i,l)},destroy(){Ds(s)}}}}const jl="5";var qi;typeof window<"u"&&((qi=window.__svelte??(window.__svelte={})).v??(qi.v=new Set)).add(jl);const Dl={actions:"Actions",back:"Back",breadcrumbs:"Breadcrumbs",cantMove:"It can't move",changed:"Changed",clear:"Clear",close:"Close",collapse:"Collapse",date:"Date",disabled:"Disabled",dragOrClick:"drag, or click for the moves",done:"Done",duplicate:"Duplicate",expand:"Expand",forward:"Forward",hasErrors:"Has errors",hide:"Hide it on the website",monthAfter:"Month after",monthBefore:"Month before",move:"Move",moveDown:"Move down",moveEarlier:"Move earlier",moveIn:"Move under the one above",moveLater:"Move later",moveOut:"Move out a level",moveUp:"Move up",noDate:"No date",nothingFound:"Nothing found",nothingYet:"Nothing yet",pickDay:"Pick a day",pickMonth:"Pick a month",pickYear:"Pick a year",remove:"Remove",resize:"Drag to change the width, double click for the default",search:"Search",show:"Show it on the website",time:"time",today:"Today"},Mr=Symbol.for("zipp-ui"),Ol=Qe({texts:Dl,locale:void 0}),Cr=globalThis,vn=Cr[Mr]??(Cr[Mr]=Ol),ql=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Nl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Rl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Fl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Hl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Vl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Yl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ul=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Gl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Xl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Kl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Zl=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Jl=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ql=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$l=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ea=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ta=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,oa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,la=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,va=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,_a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ya=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Ma=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ca=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Sa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ea=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Aa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ia=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,za=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ta=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Ba=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,La=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Pa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Da=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,qa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Na=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ra=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Fa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ha=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Va=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Za=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ec=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,nc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,tc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,rc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ic=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,oc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,lc=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`;var ac=I('<span class="icon svelte-oymxvd"></span>');const cc={hash:"svelte-oymxvd",code:".icon.svelte-oymxvd {display:inline-flex;flex-shrink:0;}.icon.svelte-oymxvd svg {width:100%;height:100%;stroke-width:1.75;}"};function wn(n,e){Ce(e,!0),Te(n,cc);const t=Object.assign({"./icons/add-image.svg":ql,"./icons/add-video.svg":Nl,"./icons/add.svg":Rl,"./icons/alert.svg":Fl,"./icons/all_pages.svg":Hl,"./icons/arrow-down.svg":Vl,"./icons/arrow-left.svg":Yl,"./icons/arrow-right.svg":Ul,"./icons/arrow-up.svg":Gl,"./icons/block.svg":Wl,"./icons/bold.svg":Xl,"./icons/button.svg":Kl,"./icons/calendar.svg":Zl,"./icons/channel.svg":Jl,"./icons/check.svg":Ql,"./icons/chevron-down.svg":$l,"./icons/chevron-left.svg":ea,"./icons/chevron-right.svg":na,"./icons/chevrons-up-down.svg":ta,"./icons/close.svg":ra,"./icons/copy-url.svg":ia,"./icons/copy.svg":sa,"./icons/crop.svg":oa,"./icons/dashboard.svg":la,"./icons/default.svg":aa,"./icons/delete.svg":ca,"./icons/desktop.svg":da,"./icons/doc.svg":ua,"./icons/download.svg":va,"./icons/edit.svg":fa,"./icons/entries.svg":ha,"./icons/external.svg":ga,"./icons/file.svg":wa,"./icons/flip-h.svg":pa,"./icons/flip-v.svg":_a,"./icons/focal.svg":ma,"./icons/folder-plus.svg":ba,"./icons/folder.svg":xa,"./icons/globe.svg":ka,"./icons/graphql.svg":ya,"./icons/grip.svg":Ma,"./icons/help.svg":Ca,"./icons/hide.svg":Sa,"./icons/indent.svg":Ea,"./icons/info.svg":Aa,"./icons/italic.svg":Ia,"./icons/link.svg":za,"./icons/live-preview.svg":Ta,"./icons/logout.svg":Ba,"./icons/mail.svg":La,"./icons/mobile.svg":Pa,"./icons/more.svg":ja,"./icons/move.svg":Da,"./icons/numbered.svg":Oa,"./icons/outdent.svg":qa,"./icons/page.svg":Na,"./icons/page_cat.svg":Ra,"./icons/phone.svg":Fa,"./icons/play.svg":Ha,"./icons/reload.svg":Va,"./icons/replace.svg":Ya,"./icons/reset.svg":Ua,"./icons/rotate-left.svg":Ga,"./icons/rotate-right.svg":Wa,"./icons/save.svg":Xa,"./icons/search.svg":Ka,"./icons/settings.svg":Za,"./icons/sidebar.svg":Ja,"./icons/single.svg":Qa,"./icons/size.svg":$a,"./icons/sparkles.svg":ec,"./icons/structure.svg":nc,"./icons/tablet.svg":tc,"./icons/translate.svg":rc,"./icons/unlink.svg":ic,"./icons/upload.svg":sc,"./icons/users.svg":oc,"./icons/view.svg":lc});let r=en(e,"size",3,16);var i=ac();let s;xl(i,()=>t[`./icons/${e.name}.svg`],!0),B(()=>s=ht(i,"",s,{width:`${r()/16}rem`,height:`${r()/16}rem`})),C(n,i),Se()}const Sr=Symbol.for("zipp-ui-layers"),Er=globalThis,Ns=Er[Sr]??(Er[Sr]={stack:[],onkeydown(n){const e=Ns.stack;n.key!=="Escape"||!e.length||document.querySelector("dialog:modal")||(n.preventDefault(),e[e.length-1]())}});function dc(n){const{stack:e,onkeydown:t}=Ns;return e.length||window.addEventListener("keydown",t),e.push(n),()=>{const r=e.indexOf(n);r>=0&&e.splice(r,1),e.length||window.removeEventListener("keydown",t)}}function Qn(n,e){const t=document.activeElement;let r=e;const i=dc(()=>r());return{update(s){r=s},destroy(){i(),t!=null&&t.isConnected&&t.focus()}}}const uc='a[href], button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]';function vc(n,e){if(n.key!=="Tab")return;const t=[...e.querySelectorAll(uc)].filter(o=>o.offsetParent!==null&&!o.matches('[tabindex="-1"]'));if(!t.length)return;const[r,i]=[t[0],t[t.length-1]],s=document.activeElement;n.shiftKey&&(s===r||s===e)?(n.preventDefault(),i.focus()):!n.shiftKey&&s===i&&(n.preventDefault(),r.focus())}function fc(n,e,t,r=!1){if(t)switch(n){case"ArrowDown":return r?(e+1)%t:Math.min(e+1,t-1);case"ArrowUp":return r?e<=0?t-1:e-1:Math.max(e-1,0);case"Home":return 0;case"End":return t-1}}var hc=I('<div class="popup svelte-1hgkc3f" tabindex="-1"><!></div>'),gc=I('<div class="popover svelte-1hgkc3f"><!> <!></div>');const wc={hash:"svelte-1hgkc3f",code:".popover.svelte-1hgkc3f {position:relative;}.popup.svelte-1hgkc3f {position:fixed;z-index:calc(var(--z-overlay) + 1);display:flex;flex-direction:column;gap:2px;max-width:calc(100vw - 1rem);}"};function ai(n,e){Ce(e,!0),Te(n,wc);let t=en(e,"open",15,!1),r=en(e,"at",15,null),i=en(e,"align",3,"start"),s=en(e,"menu",3,!1),o=J(void 0),l=J(void 0);const c=()=>t(!t()),d=()=>t(!1);Jn(()=>{t()||r(null)});const u=8,g=4;function p(m,M){let A=M;const y=()=>{const k=m.getBoundingClientRect();let z,O;if(A)z=A.x,O=A.y+k.height>innerHeight-u?A.y-k.height:A.y;else{const W=a(o).getBoundingClientRect();z=i()==="end"?W.right-k.width:i()==="center"?W.left+W.width/2-k.width/2:W.left;const $=W.bottom+g,oe=W.top-g-k.height;O=$+k.height>innerHeight-u&&oe>=u?oe:$}z=Math.min(z,innerWidth-k.width-u),m.style.left=`${Math.max(z,u)}px`,m.style.top=`${Math.max(O,u)}px`};y(),addEventListener("scroll",y,!0),addEventListener("resize",y);const S=new ResizeObserver(y);return S.observe(m),{update(k){A=k,y()},destroy(){removeEventListener("scroll",y,!0),removeEventListener("resize",y),S.disconnect()}}}Jn(()=>{if(!t())return;const m=M=>{var A;(A=a(o))!=null&&A.contains(M.target)||d()};return window.addEventListener("pointerdown",m),()=>window.removeEventListener("pointerdown",m)}),Jn(()=>{var M,A;if(!t())return;const m=s()?"[role=menuitem]":"input";(A=(M=a(l))==null?void 0:M.querySelector(m))==null||A.focus()});function v(m){if(!s()||!a(l))return;const M=[...a(l).querySelectorAll("[role=menuitem]")],A=M.indexOf(document.activeElement),y=fc(m.key,A,M.length,!0);y!==void 0&&(m.preventDefault(),M[y].focus())}var _=gc(),h=L(_);Oe(h,()=>e.trigger??st,()=>({toggle:c,open:t()}));var w=P(h,2);{var x=m=>{var M=hc();let A;var y=L(M);Oe(y,()=>e.children),ut(M,S=>q(l,S),()=>a(l)),Bt(M,(S,k)=>Qn==null?void 0:Qn(S,k),()=>d),Bt(M,(S,k)=>p==null?void 0:p(S,k),r),B(()=>{Z(M,"role",s()?"menu":"dialog"),Z(M,"aria-label",e.label),A=ht(M,"",A,{width:e.width})}),Q("keydown",M,v),C(m,M)};U(w,m=>{t()&&m(x)})}ut(_,m=>q(o,m),()=>a(o)),C(n,_),Se()}pn(["keydown"]);var pc=I('<button type="button" class="icon drag-handle svelte-nnt03w" data-drag-handle=""><!></button>'),_c=I('<button type="button" class="popup-item" role="menuitem"><!> </button>'),mc=I('<span class="popup-item note"> </span>');const bc={hash:"svelte-nnt03w",code:`.drag-handle.svelte-nnt03w {cursor:grab;color:var(--subtle);}

@media (hover: hover) {.drag-handle.svelte-nnt03w:hover {color:var(--text);}
}`};function Rs(n,e){Ce(e,!0),Te(n,bc);const t=ne(()=>e.label??vn.texts.move),r=()=>typeof e.index=="function"?e.index():e.index;let i=J(!1);const s=[["up","moveUp","arrow-up"],["down","moveDown","arrow-down"],["out","moveOut","outdent"],["in","moveIn","indent"]],o={up:["moveEarlier","arrow-left"],down:["moveLater","arrow-right"]},l={ArrowUp:"up",ArrowDown:"down",ArrowLeft:"out",ArrowRight:"in"},c=ne(()=>a(i)?s.filter(([h])=>e.sortable.stepDrop(r(),h)).map(([h,w,x])=>e.sortable.grid()&&o[h]?[h,...o[h]]:[h,w,x]):[]),d={ArrowLeft:"up",ArrowRight:"down"};function u(h){const w=e.sortable.grid()&&d[h.key]||l[h.key];w&&(h.preventDefault(),e.sortable.step(r(),w))}var g=We(),p=le(g);{var v=h=>{ai(h,{width:"13rem",get label(){return a(t)},menu:!0,get open(){return a(i)},set open(x){q(i,x,!0)},trigger:x=>{var m=pc();ht(m,"",{},{"touch-action":"none"});var M=L(m);wn(M,{name:"grip",size:15}),B(()=>{Z(m,"title",`${a(t)??""}: ${vn.texts.dragOrClick??""}`),Z(m,"aria-label",a(t)),Z(m,"aria-expanded",a(i))}),Q("pointerdown",m,A=>e.sortable.start(A,r())),Q("click",m,()=>!e.sortable.dragged&&q(i,!a(i))),Q("keydown",m,u),C(x,m)},children:(x,m)=>{var M=We(),A=le(M);Ue(A,17,()=>a(c),([y,S,k])=>y,(y,S)=>{var k=ne(()=>oo(a(S),3));let z=()=>a(k)[0],O=()=>a(k)[1],W=()=>a(k)[2];var $=_c(),oe=L($);wn(oe,{get name(){return W()},size:14});var fe=P(oe);B(()=>Y(fe,` ${vn.texts[O()]??""}`)),Q("click",$,()=>{q(i,!1),e.sortable.step(r(),z())}),C(y,$)},y=>{var S=mc(),k=te(S,!0);B(()=>Y(k,vn.texts.cantMove)),C(y,S)}),C(x,M)},$$slots:{trigger:!0,default:!0}})},_=ne(()=>e.sortable.movable());U(p,h=>{a(_)&&h(v)})}C(n,g),Se()}pn(["pointerdown","click","keydown"]);var xc=I('<span class="info svelte-l93724"><!></span>'),kc=I('<label class="svelte-l93724"><span> </span> <!></label>'),yc=I('<div class="field-row svelte-l93724"><!> <div><!></div></div>');const Mc={hash:"svelte-l93724",code:`.field-row.svelte-l93724 {display:grid;grid-template-columns:8.125rem minmax(0, 1fr);gap:1.25rem;align-items:center;min-width:0;}label.svelte-l93724 {align-self:start;display:flex;flex-direction:column;gap:1px;padding-top:0.4375rem;font-size:var(--fs-md);font-weight:600;color:var(--text);}.info.svelte-l93724 {color:var(--muted);font-size:var(--fs-sm);font-weight:400;}.value.svelte-l93724 {min-width:0;}.alone.svelte-l93724 {grid-column:2;}

@container (width < 26rem) {.field-row.svelte-l93724 {display:flex;flex-direction:column;align-items:stretch;gap:0.375rem;}label.svelte-l93724 {padding-top:0;}
}`};function Cc(n,e){Te(n,Mc);let t=en(e,"label",3,"");var r=yc(),i=L(r);{var s=d=>{var u=kc(),g=L(u),p=te(g,!0),v=P(g,2);{var _=h=>{var w=xc(),x=L(w);Oe(x,()=>e.info),C(h,w)};U(v,h=>{e.info&&h(_)})}B(()=>{Z(u,"for",e.for),Y(p,t())}),C(d,u)};U(i,d=>{t()&&d(s)})}var o=P(i,2);let l;var c=L(o);Oe(c,()=>e.children),B(()=>l=Xe(o,1,"value svelte-l93724",null,l,{alone:!t()})),C(n,r)}function Fs(n){const e=Symbol(n);return{provide:t=>void To(e,t),get:()=>zo(e)}}const Sc=Fs("page-lead"),Hs=Fs("flyout-bar");var Ec=I('<div class="page svelte-22zuw3"><!></div> <button type="button" class="icon"><!></button>',1),Ac=I('<div class="actions svelte-22zuw3"><!></div>'),Ic=I('<button type="button" class="primary"><!> </button>'),zc=I('<strong class="truncate"> </strong> <!>',1),Tc=I('<div class="backdrop" role="presentation"></div> <div class="flyout svelte-22zuw3" role="dialog" aria-modal="true" tabindex="-1"><div class="bar svelte-22zuw3"><!></div> <div><!></div></div>',1);const Bc={hash:"svelte-22zuw3",code:`.flyout.svelte-22zuw3 {position:fixed;top:0;right:0;bottom:0;z-index:var(--z-overlay);display:flex;flex-direction:column;width:min(65rem, 92vw);background:var(--bg);box-shadow:var(--shadow-flyout);outline:none;
  animation: svelte-22zuw3-slide 0.18s ease-out;}

@media (max-width: 799px) {.flyout.svelte-22zuw3 {width:100vw;}
}
@keyframes svelte-22zuw3-slide {
  from {
    transform: translateX(2.5rem);
    opacity: 0;
  }
}.bar.svelte-22zuw3 {position:relative;z-index:var(--z-bar);display:flex;align-items:center;justify-content:space-between;gap:0.75rem;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.actions.svelte-22zuw3 {display:flex;gap:0.5rem;}

/* the page inside scrolls, its top bar sticks to this */.body.svelte-22zuw3 {flex:1;min-height:0;padding:0 var(--page-pad) var(--page-pad);overflow-y:auto;}.page-in.svelte-22zuw3 {padding-top:1.25rem;}.page.svelte-22zuw3 {display:flex;align-items:center;gap:0.5rem;flex:1;flex-wrap:wrap;min-width:0;}`};function Lc(n,e){Ce(e,!0),Te(n,Bc);const t=Qe({content:null});Hs.provide(t);let r=J(void 0);Jn(()=>{var h;return(h=a(r))==null?void 0:h.focus()});function i(h){a(r)&&vc(h,a(r))}var s=Tc(),o=le(s),l=P(o,2),c=L(l),d=L(c);{var u=h=>{var w=Ec(),x=le(w),m=L(x);Oe(m,()=>t.content);var M=P(x,2),A=L(M);wn(A,{name:"close"}),B(()=>{Z(M,"title",vn.texts.close),Z(M,"aria-label",vn.texts.close)}),Q("click",M,function(...y){var S;(S=e.onclose)==null||S.apply(this,y)}),C(h,w)},g=h=>{var w=zc(),x=le(w),m=te(x,!0),M=P(x,2);{var A=S=>{var k=Ac(),z=L(k);Oe(z,()=>e.actions),C(S,k)},y=S=>{var k=Ic(),z=L(k);wn(z,{name:"close",size:14});var O=P(z);B(()=>Y(O,` ${vn.texts.done??""}`)),Q("click",k,function(...W){var $;($=e.onclose)==null||$.apply(this,W)}),C(S,k)};U(M,S=>{e.actions?S(A):S(y,-1)})}B(()=>Y(m,e.title)),C(h,w)};U(d,h=>{t.content?h(u):h(g,-1)})}var p=P(c,2);let v;var _=L(p);Oe(_,()=>e.children),ut(l,h=>q(r,h),()=>a(r)),Bt(l,(h,w)=>Qn==null?void 0:Qn(h,w),()=>e.onclose),B(()=>{Z(l,"aria-label",e.title),v=Xe(p,1,"body svelte-22zuw3",null,v,{"page-in":!!t.content})}),Q("click",o,function(...h){var w;(w=e.onclose)==null||w.apply(this,h)}),Q("keydown",l,i),C(n,s),Se()}pn(["click","keydown"]);function Pc(n){return n.defaultPrevented||n.target.closest("input, textarea, [contenteditable]")?!1:(n.preventDefault(),n.stopPropagation(),!0)}function jc(n){return n.button===2||n.clientX?{x:n.clientX,y:n.clientY}:null}var Dc=I('<a class="popup-item svelte-60nrgv" role="menuitem"><!> </a>'),Oc=I('<button type="button" role="menuitem"><!> </button>');const qc={hash:"svelte-60nrgv",code:".popup-item.svelte-60nrgv {min-width:8.625rem;white-space:nowrap;}"};function Nc(n,e){Ce(e,!0),Te(n,qc);var t=We(),r=le(t);Ue(r,17,()=>e.items,lr,(i,s)=>{var o=We(),l=le(o);{var c=u=>{var g=Dc(),p=L(g);wn(p,{get name(){return a(s).icon},size:14});var v=P(p);B(_=>{Z(g,"href",a(s).href),Z(g,"target",_),Z(g,"download",a(s).download),Y(v,` ${a(s).label??""}`)},[()=>a(s).href.startsWith("/admin")||a(s).download?void 0:"_blank"]),Q("click",g,function(..._){var h;(h=e.close)==null||h.apply(this,_)}),C(u,g)},d=u=>{var g=Oc();let p;var v=L(g);wn(v,{get name(){return a(s).icon},size:14});var _=P(v);B(()=>{p=Xe(g,1,"popup-item svelte-60nrgv",null,p,{danger:a(s).danger}),Y(_,` ${a(s).label??""}`)}),Q("click",g,()=>{var h,w;e.close(),(w=(h=a(s)).onclick)==null||w.call(h)}),C(u,g)};U(l,u=>{a(s).href?u(c):u(d,-1)})}C(i,o)}),C(n,t),Se()}pn(["click"]);var Rc=I('<button type="button" class="icon"><!></button>');function Vs(n,e){Ce(e,!0);const t=ne(()=>e.label??vn.texts.actions);let r=J(!1),i=J(null),s=J(void 0);Jn(()=>{var u;const d=(u=a(s))==null?void 0:u.closest("[data-menu-area]");if(d)return rl(d,"contextmenu",g=>{Pc(g)&&(q(i,jc(g),!0),q(r,!0))})});var o=We(),l=le(o);{var c=d=>{ai(d,{align:"end",get label(){return a(t)},menu:!0,get open(){return a(r)},set open(g){q(r,g,!0)},get at(){return a(i)},set at(g){q(i,g,!0)},trigger:(g,p)=>{let v=()=>(p==null?void 0:p()).toggle,_=()=>(p==null?void 0:p()).open;var h=Rc(),w=L(h);wn(w,{name:"more"}),ut(h,x=>q(s,x),()=>a(s)),B(()=>{Z(h,"title",a(t)),Z(h,"aria-label",a(t)),Z(h,"aria-expanded",_())}),Q("click",h,function(...x){var m;(m=v())==null||m.apply(this,x)}),C(g,h)},children:(g,p)=>{Nc(g,{get items(){return e.items},close:()=>q(r,!1)})},$$slots:{trigger:!0,default:!0}})};U(l,d=>{e.items.length&&d(c)})}C(n,o),Se()}pn(["click"]);var Fc=I('<button type="button"><!></button>'),Hc=I('<div class="segmented svelte-49ceuw"></div>');const Vc={hash:"svelte-49ceuw",code:`.segmented.svelte-49ceuw {display:inline-flex;flex-wrap:wrap;gap:2px;padding:3px;background:var(--hover);border:1px solid var(--line);border-radius:var(--radius-lg);}.segmented.svelte-49ceuw button:where(.svelte-49ceuw) {gap:0.375rem;padding:0.3125rem 0.75rem;border:none;border-radius:var(--radius);background:none;color:var(--muted);}.segmented.svelte-49ceuw button.on:where(.svelte-49ceuw) {background:var(--surface);color:var(--text);box-shadow:var(--shadow);}
@media (hover: hover) {.segmented.svelte-49ceuw button:where(.svelte-49ceuw):hover:not(.on):not(:disabled) {color:var(--text);background:none;}
}`};function Yc(n,e){Ce(e,!0),Te(n,Vc);let t=en(e,"tabs",3,!1);const r=ne(()=>e.options.some(o=>o.value===e.value));function i(o){const l={ArrowRight:1,ArrowDown:1,ArrowLeft:-1,ArrowUp:-1}[o.key];if(!l)return;o.preventDefault();const c=e.options.findIndex(g=>g.value===e.value),d=c<0?e.options[l>0?0:e.options.length-1]:e.options[(c+l+e.options.length)%e.options.length];e.onchange(d.value);const u=o.currentTarget.parentElement;zs().then(()=>{var g;return(g=u==null?void 0:u.querySelector('[tabindex="0"]'))==null?void 0:g.focus()})}var s=Hc();Ue(s,23,()=>e.options,o=>o.value,(o,l,c)=>{var d=Fc();let u;var g=L(d);{var p=_=>{var h=We(),w=le(h);Oe(w,()=>e.item,()=>a(l)),C(_,h)},v=_=>{var h=Ps();B(()=>Y(h,a(l).label)),C(_,h)};U(g,_=>{e.item?_(p):_(v,-1)})}B(()=>{Z(d,"role",t()?"tab":"radio"),Z(d,"aria-selected",t()?a(l).value===e.value:void 0),Z(d,"aria-checked",t()?void 0:a(l).value===e.value),Z(d,"tabindex",a(l).value===e.value||a(c)===0&&!a(r)?0:-1),u=Xe(d,1,"svelte-49ceuw",null,u,{on:a(l).value===e.value})}),Q("click",d,()=>e.onchange(a(l).value)),Q("keydown",d,i),C(o,d)}),B(()=>{Z(s,"role",t()?"tablist":"radiogroup"),Z(s,"aria-label",e.label)}),C(n,s),Se()}pn(["click","keydown"]);var Uc=I('<div class="sort-line svelte-183yeqa"></div>');const Gc={hash:"svelte-183yeqa",code:".sort-line.svelte-183yeqa {position:absolute;z-index:var(--z-raised);border-radius:2px;background:var(--accent);pointer-events:none;}"};function Ii(n,e){Ce(e,!0),Te(n,Gc);var t=We(),r=le(t);{var i=s=>{var o=Uc();let l;B(()=>l=ht(o,"",l,{top:`${e.sortable.line.top??""}px`,left:`${e.sortable.line.left??""}px`,width:`${e.sortable.line.width??""}px`,height:`${e.sortable.line.height??""}px`})),C(s,o)};U(r,s=>{e.sortable.line&&s(i)})}C(n,t),Se()}function Et(n,e){let t=1;for(;e+t<n.length&&n[e+t]>n[e];)t++;return t}function Wc(n,e,t,r,i){const s=n[e],o=Et(n,e),l=h=>h<e?h:h-o,c=h=>h+Et(n,h);let d=e-1;for(;d>=0&&n[d]>s;)d--;const u=d>=0&&n[d]===s?d:-1,g=e+o,p=g<n.length&&n[g]===s?g:-1;let v=e-1;for(;v>=0&&n[v]>=s;)v--;const _=Math.max(...n.slice(e,e+o))-s;switch(t){case"up":return u<0?null:{index:l(u),depth:s};case"down":{const h=n.length-o-i,w=p<0?-1:l(c(p));return w<0||w>h?null:{index:w,depth:s}}case"out":return s===0||v<0?null:{index:l(c(v)),depth:s-1};case"in":return u<0||s+1+_>r?null:{index:e,depth:s+1}}}var Ot,qt;class zi{constructor(e){N(this,Ot,J(null));N(this,qt,J(null));we(this,"drop",null);we(this,"dragged",!1);this.o=e}get dragging(){return a(f(this,Ot))}set dragging(e){q(f(this,Ot),e,!0)}get line(){return a(f(this,qt))}set line(e){q(f(this,qt),e,!0)}rows(){if(this.o.rows)return this.o.rows();const e=this.o.container();return e?[...e.querySelectorAll("[data-sort]")].filter(t=>{var i;const r=(i=t.parentElement)==null?void 0:i.closest("[data-sort]");return!r||!e.contains(r)}):[]}movable(){return this.o.depths().length>1}grid(){var e,t;return((t=(e=this.o).grid)==null?void 0:t.call(e))??!1}moving(e){const t=this.dragging;return t===null?!1:e>=t&&e<t+Et(this.o.depths(),t)}start(e,t){if(e.button!==0)return;const r=e.clientX,i=e.clientY;let s=!1;const o=d=>{if(!s){if(Math.hypot(d.clientX-r,d.clientY-i)<5)return;s=!0,this.dragging=t}d.preventDefault(),this.over(d,t,d.clientX-r),Xc(this.o.container(),d.clientY)},l=d=>{window.removeEventListener("pointermove",o),window.removeEventListener("pointerup",l),window.removeEventListener("pointercancel",l),window.removeEventListener("keydown",c,!0);const u=this.drop;this.dragging=null,this.line=null,this.drop=null,s&&(this.dragged=!0,setTimeout(()=>this.dragged=!1),u&&d.type==="pointerup"&&this.o.onmove(t,u))},c=d=>{d.key!=="Escape"||!s||(d.preventDefault(),d.stopImmediatePropagation(),this.drop=null,l(new PointerEvent("pointercancel")))};window.addEventListener("pointermove",o),window.addEventListener("pointerup",l),window.addEventListener("pointercancel",l),window.addEventListener("keydown",c,!0)}over(e,t,r){var d,u,g;const i=(d=this.o.container())==null?void 0:d.getBoundingClientRect(),s=this.rows(),o=this.o.depths();if(!i||s.length!==o.length)return;const l=Et(o,t),c=s.filter((p,v)=>v<t||v>=t+l);(g=(u=this.o).grid)!=null&&g.call(u)?this.overGrid(e,i,c):this.overList(e,i,c,o,t,r)}overGrid(e,t,r){var d,u;let i=-1,s=1/0;if(r.forEach((g,p)=>{const v=g.getBoundingClientRect(),_=Math.hypot(e.clientX-(v.left+v.width/2),e.clientY-(v.top+v.height/2));_<s&&([i,s]=[p,_])}),i<0)return;const o=r[i].getBoundingClientRect(),l=e.clientX>o.left+o.width/2,c=r.length-(((u=(d=this.o).keepLast)==null?void 0:u.call(d))??0);this.drop={index:Math.min(i+(l?1:0),c),depth:0},this.line={top:o.top-t.top,left:(l?o.right+3:o.left-5)-t.left,width:2,height:o.height}}overList(e,t,r,i,s,o){var A,y,S,k;const l=Et(i,s),c=i.filter((z,O)=>O<s||O>=s+l),d=z=>z.getBoundingClientRect();let u=r.findIndex(z=>{const O=d(z);return e.clientY<O.top+O.height/2});u<0&&(u=r.length),u=Math.min(u,r.length-(((y=(A=this.o).keepLast)==null?void 0:y.call(A))??0));const g=i.slice(s,s+l),p=Math.max(...g)-i[s],v=this.o.indent??24,_=i[s]+Math.round(o/v),h=Math.min(u>0?c[u-1]+1:0,(((k=(S=this.o).maxDepth)==null?void 0:k.call(S))??0)-p),w=u<r.length?c[u]:0;if(w>h){this.drop=null,this.line=null;return}const x=Math.max(w,Math.min(_,h));this.drop={index:u,depth:x};const m=u<r.length?d(r[u]).top-3:r.length?d(r[r.length-1]).bottom+1:t.top,M=(this.o.inset??0)+x*v;this.line={top:m-t.top,left:M,width:t.width-M-(this.o.inset??0),height:2}}stepDrop(e,t){var r,i,s,o;return Wc(this.o.depths(),e,t,((i=(r=this.o).maxDepth)==null?void 0:i.call(r))??0,((o=(s=this.o).keepLast)==null?void 0:o.call(s))??0)}async step(e,t){var s;const r=this.stepDrop(e,t);if(!r)return;await this.o.onmove(e,r),await zs();const i=this.rows()[r.index];(s=i==null?void 0:i.querySelector("[data-drag-handle]"))==null||s.focus()}}Ot=new WeakMap,qt=new WeakMap;const Ti=48,Bi=12;function Xc(n,e){const t=Kc(n),r=t?t.getBoundingClientRect().top:0,i=t?t.getBoundingClientRect().bottom:innerHeight,s=e<r+Ti?-Bi:e>i-Ti?Bi:0;s&&(t??window).scrollBy(0,s)}function Kc(n){for(let e=n==null?void 0:n.parentElement;e;e=e.parentElement){const t=getComputedStyle(e).overflowY;if((t==="auto"||t==="scroll")&&e.scrollHeight>e.clientHeight)return e}return null}const Li={en:{title:"Submissions",note:"What visitors sent with the forms, the newest first.",allForms:"All forms",form:"Form",date:"Date",site:"Site",content:"Content",none:"No submissions yet.",more:"Show more",delete:"Delete",confirmDelete:"Delete this submission for good?",empty:"–",page:"Page",pages:"Pages",addPage:"Add page",renamePage:"Rename",pageName:"Name of the page",deletePage:"Delete page",confirmDeletePage:"Delete this page and its fields?",moveLeft:"Move left",moveRight:"Move right",movePage:"Move page",addField:"Add field",edit:"Edit",duplicate:"Duplicate",switchOff:"Switch off",switchOn:"Switch on",off:"off",remove:"Delete",moveField:"Move field",toPage:"To {page}",width:"Width",full:"Full",noFields:"No fields on this page yet.",fieldsActions:"Field actions",pageActions:"Page actions"},de:{title:"Einsendungen",note:"Was Besucher mit den Formularen geschickt haben, die neusten zuerst.",allForms:"Alle Formulare",form:"Formular",date:"Datum",site:"Website",content:"Inhalt",none:"Noch keine Einsendungen.",more:"Mehr zeigen",delete:"Löschen",confirmDelete:"Diese Einsendung endgültig löschen?",empty:"–",page:"Seite",pages:"Seiten",addPage:"Seite hinzufügen",renamePage:"Umbenennen",pageName:"Name der Seite",deletePage:"Seite löschen",confirmDeletePage:"Diese Seite und ihre Felder löschen?",moveLeft:"Nach links",moveRight:"Nach rechts",movePage:"Seite verschieben",addField:"Feld hinzufügen",edit:"Bearbeiten",duplicate:"Duplizieren",switchOff:"Ausschalten",switchOn:"Einschalten",off:"aus",remove:"Löschen",moveField:"Feld verschieben",toPage:"Auf {page}",width:"Breite",full:"Ganz",noFields:"Noch keine Felder auf dieser Seite.",fieldsActions:"Aktionen des Felds",pageActions:"Aktionen der Seite"}};function ci(){return Li[document.documentElement.lang]??Li.en}var Zc=I('<span class="grip svelte-1rirqtx"><!></span>'),Jc=I('<span class="star svelte-1rirqtx">*</span>'),Qc=I('<span class="badge"> </span>'),$c=I('<p class="hint truncate svelte-1rirqtx"> </p>'),ed=I('<div role="button" tabindex="0" data-menu-area=""><div class="top svelte-1rirqtx"><!> <span class="label truncate svelte-1rirqtx"> <!></span> <span class="type svelte-1rirqtx"> </span> <!> <!></div> <div><span class="truncate"> </span></div> <!></div>');const nd={hash:"svelte-1rirqtx",code:`.field-card.svelte-1rirqtx {display:flex;flex-direction:column;gap:0.375rem;height:100%;padding:0.625rem 0.75rem 0.75rem;background:var(--surface);border:1px solid var(--border);border-radius:var(--radius-lg);cursor:pointer;transition:border-color 0.12s, box-shadow 0.12s;}
@media (hover: hover) {.field-card.svelte-1rirqtx {border-color:var(--accent);}
}.field-card.svelte-1rirqtx:focus-visible {outline:2px solid var(--accent);outline-offset:2px;}.field-card.selected.svelte-1rirqtx {border-color:var(--accent);box-shadow:0 0 0 1px var(--accent);}.field-card.off.svelte-1rirqtx {opacity:0.55;}.top.svelte-1rirqtx {display:flex;align-items:center;gap:0.375rem;min-height:1.75rem;}.grip.svelte-1rirqtx {display:flex;margin-left:-0.375rem;color:var(--muted);}.label.svelte-1rirqtx {flex:1;min-width:0;font-weight:600;font-size:var(--fs-base);}.star.svelte-1rirqtx {margin-left:0.125rem;color:var(--danger);}.type.svelte-1rirqtx {color:var(--muted);font-size:var(--fs-xs);white-space:nowrap;}.preview.svelte-1rirqtx {display:flex;align-items:center;height:2rem;padding:0 0.625rem;color:var(--muted);font-size:var(--fs-md);background:var(--hover);border:1px solid var(--line);border-radius:var(--radius);}.preview.textarea.svelte-1rirqtx {align-items:flex-start;height:4.5rem;padding-top:0.375rem;}.hint.svelte-1rirqtx {margin:0;color:var(--muted);font-size:var(--fs-xs);}`};function td(n,e){Ce(e,!0),Te(n,nd);let t=en(e,"selected",3,!1),r=en(e,"readonly",3,!1);const i=ci(),s=ne(()=>e.item.block.fields),o=ne(()=>e.item.block.enabled===!1),l=ne(()=>e.item.block.type==="formTextarea");function c(R){R.target.closest("button, a, input, select")||e.onedit()}function d(R){R.target===R.currentTarget&&(R.key==="Enter"||R.key===" ")&&(R.preventDefault(),e.onedit())}var u=ed();let g;var p=L(u),v=L(p);{var _=R=>{var ie=Zc(),ue=L(ie);Rs(ue,{get sortable(){return e.sortable},get index(){return e.index}}),C(R,ie)};U(v,R=>{r()||R(_)})}var h=P(v,2),w=L(h),x=P(w);{var m=R=>{var ie=Jc();C(R,ie)};U(x,R=>{a(s).required&&R(m)})}var M=P(h,2),A=te(M,!0),y=P(M,2);{var S=R=>{var ie=Qc(),ue=te(ie,!0);B(()=>Y(ue,i.off)),C(R,ie)};U(y,R=>{a(o)&&R(S)})}var k=P(y,2);{var z=R=>{Vs(R,{get items(){return e.menu},get label(){return i.fieldsActions}})};U(k,R=>{r()||R(z)})}var O=P(p,2);let W;var $=L(O),oe=te($,!0),fe=P(O,2);{var Ee=R=>{var ie=$c(),ue=te(ie,!0);B(()=>Y(ue,a(s).instructions)),C(R,ie)};U(fe,R=>{a(s).instructions&&R(Ee)})}B(()=>{g=Xe(u,1,"field-card svelte-1rirqtx",null,g,{selected:t(),off:a(o)}),Y(w,a(s).label||e.typeName),Y(A,e.typeName),W=Xe(O,1,"preview svelte-1rirqtx",null,W,{textarea:a(l)}),Y(oe,a(s).placeholder??"")}),Q("click",u,c),Q("keydown",u,d),C(n,u),Se()}pn(["click","keydown"]);const Gr="formPage",rd=[25,33,50,67,75,100];function id(n){const e=[];for(const t of n)t.block.type===Gr?e.push({page:t,fields:[]}):e.length?e.at(-1).fields.push(t):e.push({page:null,fields:[t]});return e.length||e.push({page:null,fields:[]}),e}function sd(n,e){return n.flatMap((t,r)=>{const i=t.page??(r>0||n.length>1?e():null);return i?[i,...t.fields]:t.fields})}const Pi=n=>{var e;return((e=n.page)==null?void 0:e.block.fields.label)??""};function od(n,e,t,r){return n.map((i,s)=>{if(s!==e)return i;const o=i.page??r(),l={...o.block,fields:{...o.block.fields,label:t}};return{...i,page:{...o,block:l}}})}function ji(n,e,t){const r=[...n],[i]=r.splice(e,1);return r.splice(t,0,i),r}function ar(n,e,t){return n.map((r,i)=>i===e?{...r,fields:t(r.fields)}:r)}function ld(n,e,t,r){const i=n[e].fields[t],s=ar(n,e,o=>o.filter((l,c)=>c!==t));return ar(s,r,o=>[...o,i])}const ad=n=>Math.round((n??100)/100*12);var cd=I('<span class="grip svelte-qbgxdw"><!></span>'),dd=I('<input class="rename svelte-qbgxdw"/>'),ud=I('<button type="button" role="tab" class="plain name svelte-qbgxdw"> <span class="count svelte-qbgxdw"> </span></button>'),vd=I('<div data-sort="" data-menu-area=""><!> <!> <!></div>'),fd=I('<button type="button" class="plain add-page svelte-qbgxdw"><!> <span> </span></button>'),hd=I('<div data-sort=""><!></div>'),gd=I('<p class="empty"> </p>'),wd=I('<button type="button" class="add-field svelte-qbgxdw"><!> </button>'),pd=I('<button type="button" class="popup-item" role="menuitem"> </button>'),_d=I('<div class="cell add svelte-qbgxdw"><!></div>'),md=I("<div></div>"),bd=I('<div class="settings svelte-qbgxdw"><!> <!></div>'),xd=I('<div class="builder svelte-qbgxdw"><div class="tabs svelte-qbgxdw" role="tablist"><!> <!> <!></div> <div class="grid svelte-qbgxdw"><!> <!> <!></div></div> <!>',1);const kd={hash:"svelte-qbgxdw",code:`.builder.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.75rem;}.tabs.svelte-qbgxdw {position:relative;display:flex;flex-wrap:wrap;align-items:center;gap:0.25rem;padding-bottom:0.5rem;border-bottom:1px solid var(--line);}.tab.svelte-qbgxdw {display:flex;align-items:center;gap:0.125rem;padding:0.125rem 0.25rem;border:1px solid transparent;border-radius:var(--radius);}.tab.on.svelte-qbgxdw {background:var(--accent-soft);border-color:var(--accent-ring);}.tab.svelte-qbgxdw .grip:where(.svelte-qbgxdw) {display:flex;color:var(--muted);}.name.svelte-qbgxdw {display:flex;align-items:center;gap:0.375rem;padding:0.25rem 0.375rem;font-weight:500;}.on.svelte-qbgxdw .name:where(.svelte-qbgxdw) {color:var(--accent);}.count.svelte-qbgxdw {min-width:1.25rem;padding:0 0.3125rem;color:var(--muted);font-size:var(--fs-xs);text-align:center;background:var(--hover);border-radius:999px;}.rename.svelte-qbgxdw {width:12rem;padding:0.25rem 0.5rem;}.add-page.svelte-qbgxdw {display:flex;align-items:center;gap:0.25rem;padding:0.375rem 0.5rem;color:var(--muted);}
@media (hover: hover) {.add-page.svelte-qbgxdw {color:var(--accent);}
}.grid.svelte-qbgxdw {position:relative;display:grid;grid-template-columns:repeat(12, minmax(0, 1fr));gap:0.625rem;}.cell.svelte-qbgxdw {grid-column:span var(--span, 12);min-width:0;}.cell.add.svelte-qbgxdw {grid-column:span 12;}.add-field.svelte-qbgxdw {display:flex;align-items:center;justify-content:center;gap:0.375rem;width:100%;padding:0.75rem;color:var(--muted);background:transparent;border:1px dashed var(--border-strong);border-radius:var(--radius-lg);}
@media (hover: hover) {.add-field.svelte-qbgxdw {color:var(--accent);border-color:var(--accent);}
}.settings.svelte-qbgxdw {display:flex;flex-direction:column;gap:0.625rem;}

@media (max-width: 799px) {.cell.svelte-qbgxdw {grid-column:span 12;}
}`};function yd(n,e){Ce(e,!0),Te(n,kd);const t=ci(),r=ne(()=>e.value??[]),i=ne(()=>id(a(r)));let s=J(0);const o=ne(()=>Math.min(a(s),a(i).length-1)),l=ne(()=>a(i)[a(o)]),c=ne(()=>e.blocks.types.filter(b=>b.handle!==Gr)),d=b=>{var E;return((E=e.blocks.types.find(T=>T.handle===b))==null?void 0:E.name)??b},u=(b,E)=>Pi(b)||`${t.page} ${E+1}`,g=()=>({block:e.blocks.create(Gr)});function p(b){const E=sd(b,g);e.onchange(E.length?E:null)}let v=J(null);function _(){const b=g();b.block.fields.label=`${t.page} ${a(i).length+1}`,p([...a(i),{page:b,fields:[]}]),q(s,a(i).length,!0)}function h(b,E){a(v)===b&&(q(v,null),p(od(a(i),b,E.trim(),g)))}function w(b,E){let T=!1;const X=setTimeout(()=>{b.focus(),b.select(),T=!0},50),ee=()=>T&&h(E,b.value);return b.addEventListener("blur",ee),{destroy(){clearTimeout(X),b.removeEventListener("blur",ee)}}}function x(b,E){p(ji(a(i),b,E)),q(s,E,!0)}function m(b){a(i)[b].fields.length&&!confirm(t.confirmDeletePage)||(p(a(i).filter((E,T)=>T!==b)),q(s,Math.max(0,b-1),!0))}const M=b=>[{label:t.renamePage,icon:"edit",onclick:()=>q(v,b,!0)},...b>0?[{label:t.moveLeft,icon:"arrow-left",onclick:()=>x(b,b-1)}]:[],...b<a(i).length-1?[{label:t.moveRight,icon:"arrow-right",onclick:()=>x(b,b+1)}]:[],...a(i).length>1?[{label:t.deletePage,icon:"delete",danger:!0,onclick:()=>m(b)}]:[]];let A=J(void 0);const y=new zi({container:()=>a(A),depths:()=>a(i).map(()=>0),grid:()=>!0,onmove:(b,E)=>x(b,E.index)});let S=J(null);const k=ne(()=>a(S)&&a(i).flatMap((b,E)=>b.fields.map((T,X)=>({item:T,gi:E,fi:X}))).find(b=>b.item.block.id===a(S))),z=b=>p(ar(a(i),a(o),b));let O=null;function W(b){const E=e.blocks.create(b);z(T=>[...T,{width:100,block:E}]),q(S,O=E.id,!0)}function $(b){const E=a(l).fields[b],T={...E,block:e.blocks.copy(E.block)};z(X=>[...X.slice(0,b+1),T,...X.slice(b+1)])}function oe(b){z(E=>E.map((T,X)=>{if(X!==b)return T;const{enabled:ee,...se}=T.block;return{...T,block:ee===!1?se:{...se,enabled:!1}}}))}function fe(b){a(S)===a(l).fields[b].block.id&&q(S,null),z(E=>E.filter((T,X)=>X!==b))}function Ee(b,E,T){p(ar(a(i),b,X=>X.map((ee,se)=>se===E?{...ee,width:T}:ee)))}const R=(b,E)=>[{label:t.edit,icon:"edit",onclick:()=>q(S,b.block.id,!0)},{label:t.duplicate,icon:"copy",onclick:()=>$(E)},...a(i).map((T,X)=>({g:T,gi:X})).filter(({gi:T})=>T!==a(o)).map(({g:T,gi:X})=>({label:t.toPage.replace("{page}",u(T,X)),icon:"move",onclick:()=>p(ld(a(i),a(o),E,X))})),{label:b.block.enabled===!1?t.switchOn:t.switchOff,icon:b.block.enabled===!1?"view":"hide",onclick:()=>oe(E)},{label:t.remove,icon:"delete",danger:!0,onclick:()=>fe(E)}];let ie=J(void 0);const ue=new zi({container:()=>a(ie),depths:()=>a(l).fields.map(()=>0),grid:()=>!0,onmove:(b,E)=>z(T=>ji(T,b,E.index))});function rn(b,E){const T=e.blocks.mount(b,E);return O===E&&(O=null,setTimeout(()=>{var X;return(X=b.querySelector("input"))==null?void 0:X.focus()},50)),{destroy:()=>T.destroy()}}const ke=b=>({25:"¼",33:"⅓",50:"½",67:"⅔",75:"¾",100:t.full})[b];var gt=xd(),di=le(gt),Zt=L(di),ui=L(Zt);Ue(ui,19,()=>a(i),b=>{var E;return((E=b.page)==null?void 0:E.block.id)??"first"},(b,E,T)=>{var X=vd();let ee;var se=L(X);{var qe=ce=>{var ae=cd(),Re=L(ae);Rs(Re,{get sortable(){return y},get index(){return a(T)}}),C(ce,ae)};U(se,ce=>{!e.readonly&&a(i).length>1&&ce(qe)})}var Ne=P(se,2);{var ve=ce=>{var ae=dd();Bt(ae,(Re,wt)=>w==null?void 0:w(Re,wt),()=>a(T)),B((Re,wt)=>{Z(ae,"aria-label",t.pageName),Tl(ae,Re),Z(ae,"placeholder",wt)},[()=>Pi(a(E)),()=>u(a(E),a(T))]),Q("keydown",ae,Re=>{Re.key==="Enter"&&h(a(T),Re.currentTarget.value),Re.key==="Escape"&&q(v,null)}),C(ce,ae)},Ae=ce=>{var ae=ud(),Re=L(ae),wt=P(Re),Zs=te(wt,!0);B(Js=>{Z(ae,"aria-selected",a(T)===a(o)),Y(Re,`${Js??""} `),Y(Zs,a(E).fields.length)},[()=>u(a(E),a(T))]),Q("click",ae,()=>q(s,a(T),!0)),Q("dblclick",ae,()=>!e.readonly&&q(v,a(T),!0)),C(ce,ae)};U(Ne,ce=>{a(v)===a(T)?ce(ve):ce(Ae,-1)})}var sn=P(Ne,2);{var _n=ce=>{{let ae=ne(()=>M(a(T)));Vs(ce,{get items(){return a(ae)},get label(){return t.pageActions}})}};U(sn,ce=>{!e.readonly&&a(T)===a(o)&&a(v)!==a(T)&&ce(_n)})}B(ce=>ee=Xe(X,1,"tab svelte-qbgxdw",null,ee,{on:a(T)===a(o),"sort-dragging":ce}),[()=>y.moving(a(T))]),C(b,X)});var vi=P(ui,2);{var Ys=b=>{var E=fd(),T=L(E);wn(T,{name:"add"});var X=P(T,2),ee=te(X,!0);B(()=>{Z(E,"title",t.addPage),Y(ee,t.addPage)}),Q("click",E,_),C(b,E)};U(vi,b=>{e.readonly||b(Ys)})}var Us=P(vi,2);Ii(Us,{get sortable(){return y}}),ut(Zt,b=>q(A,b),()=>a(A));var fi=P(Zt,2),hi=L(fi);Ue(hi,19,()=>a(l).fields,b=>b.block.id,(b,E,T)=>{var X=hd();let ee,se;var qe=L(X);{let Ne=ne(()=>d(a(E).block.type)),ve=ne(()=>a(S)===a(E).block.id),Ae=ne(()=>R(a(E),a(T)));td(qe,{get item(){return a(E)},get typeName(){return a(Ne)},get selected(){return a(ve)},get readonly(){return e.readonly},get sortable(){return ue},get index(){return a(T)},get menu(){return a(Ae)},onedit:()=>q(S,a(E).block.id,!0)})}B((Ne,ve)=>{ee=Xe(X,1,"cell svelte-qbgxdw",null,ee,{"sort-dragging":Ne}),se=ht(X,"",se,{"--span":ve})},[()=>ue.moving(a(T)),()=>ad(a(E).width)]),C(b,X)},b=>{var E=We(),T=le(E);{var X=ee=>{var se=gd(),qe=te(se,!0);B(()=>Y(qe,t.noFields)),C(ee,se)};U(T,ee=>{e.readonly&&ee(X)})}C(b,E)});var gi=P(hi,2);{var Gs=b=>{var E=_d(),T=L(E);ai(T,{menu:!0,get label(){return t.addField},trigger:(ee,se)=>{let qe=()=>(se==null?void 0:se()).toggle,Ne=()=>(se==null?void 0:se()).open;var ve=wd(),Ae=L(ve);wn(Ae,{name:"add"});var sn=P(Ae);B(()=>{Z(ve,"aria-expanded",Ne()),Y(sn,` ${t.addField??""}`)}),Q("click",ve,function(..._n){var ce;(ce=qe())==null||ce.apply(this,_n)}),C(ee,ve)},children:(ee,se)=>{var qe=We(),Ne=le(qe);Ue(Ne,17,()=>a(c),ve=>ve.handle,(ve,Ae)=>{var sn=pd(),_n=te(sn,!0);B(()=>Y(_n,a(Ae).name)),Q("click",sn,()=>W(a(Ae).handle)),C(ve,sn)}),C(ee,qe)},$$slots:{trigger:!0,default:!0}}),C(b,E)};U(gi,b=>{e.readonly||b(Gs)})}var Ws=P(gi,2);Ii(Ws,{get sortable(){return ue}}),ut(fi,b=>q(ie,b),()=>a(ie));var Xs=P(di,2);{var Ks=b=>{{let E=ne(()=>a(k).item.block.fields.label||d(a(k).item.block.type));Lc(b,{get title(){return a(E)},onclose:()=>q(S,null),children:(T,X)=>{var ee=bd(),se=L(ee);{var qe=ve=>{Cc(ve,{get label(){return t.width},children:(Ae,sn)=>{{let _n=ne(()=>rd.map(ae=>({value:ae,label:ke(ae)}))),ce=ne(()=>a(k).item.width??100);Yc(Ae,{get label(){return t.width},get options(){return a(_n)},get value(){return a(ce)},onchange:ae=>Ee(a(k).gi,a(k).fi,ae)})}},$$slots:{default:!0}})};U(se,ve=>{e.readonly||ve(qe)})}var Ne=P(se,2);pl(Ne,()=>a(S),ve=>{var Ae=md();Bt(Ae,(sn,_n)=>rn==null?void 0:rn(sn,_n),()=>a(S)),C(ve,Ae)}),C(T,ee)},$$slots:{default:!0}})}};U(Xs,b=>{a(k)&&b(Ks)})}B(()=>Z(Zt,"aria-label",t.pages)),C(n,gt),Se()}pn(["keydown","click","dblclick"]);var Md=I("<span>/</span>"),Cd=I("<a> </a>"),Sd=I('<span class="here truncate svelte-wekzd8"> </span>'),Ed=I("<!> <!>",1),Ad=I('<nav class="crumbs svelte-wekzd8"></nav>');const Id={hash:"svelte-wekzd8",code:`.crumbs.svelte-wekzd8 {display:flex;align-items:center;gap:0.375rem;min-width:0;font-size:var(--fs-md);}a.svelte-wekzd8 {text-decoration:none;white-space:nowrap;}.sep.svelte-wekzd8 {color:var(--subtle);}.here.svelte-wekzd8 {color:var(--text);font-weight:500;}

@container page (width < 36rem) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}
@media (max-width: 799px) {.far.svelte-wekzd8 {display:none;}a.svelte-wekzd8 {flex-shrink:4;min-width:2.5rem;overflow:hidden;text-overflow:ellipsis;}
}`};function zd(n,e){Ce(e,!0),Te(n,Id);var t=Ad();Ue(t,21,()=>e.items,lr,(r,i,s)=>{const o=ne(()=>s<e.items.length-2);var l=Ed(),c=le(l);{var d=v=>{var _=Md();let h;B(()=>h=Xe(_,1,"sep svelte-wekzd8",null,h,{far:s<e.items.length-1})),C(v,_)};U(c,v=>{s&&v(d)})}var u=P(c,2);{var g=v=>{var _=Cd();let h;var w=te(_,!0);B(()=>{Z(_,"href",a(i).href),h=Xe(_,1,"svelte-wekzd8",null,h,{far:a(o)}),Y(w,a(i).label)}),C(v,_)},p=v=>{var _=Sd(),h=te(_,!0);B(()=>Y(h,a(i).label)),C(v,_)};U(u,v=>{a(i).href&&s<e.items.length-1?v(g):v(p,-1)})}C(r,l)}),B(()=>Z(t,"aria-label",vn.texts.breadcrumbs)),C(n,t),Se()}var Td=I('<span class="note svelte-z3zeui"> </span>'),Bd=I('<span class="state svelte-z3zeui"><!></span>'),Ld=I('<!> <h1 class="svelte-z3zeui"><!></h1> <!> <!> <span class="actions svelte-z3zeui"><!></span>',1),Pd=I("<div><!></div>");const jd={hash:"svelte-z3zeui",code:`.page-top.svelte-z3zeui {position:sticky;top:0;z-index:var(--z-bar);display:flex;align-items:center;gap:0.5rem;flex-wrap:wrap;min-height:3rem;padding:0.5rem 1rem;background:var(--surface);border-bottom:1px solid var(--line);}.bleed.svelte-z3zeui {margin:0 calc(-1 * var(--page-pad)) 1.25rem;padding:0.5rem var(--page-pad);}h1.svelte-z3zeui {min-width:0;font-size:var(--fs-lg);line-height:1.25rem;}.state.svelte-z3zeui {display:flex;align-items:center;gap:0.375rem;}.actions.svelte-z3zeui {display:flex;align-items:center;gap:0.5rem;margin-left:auto;}

@container page (width < 36rem) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}
@media (max-width: 799px) {h1.svelte-z3zeui {flex:1 1 0;min-width:4rem;}.state.svelte-z3zeui {flex-shrink:0;}.note.svelte-z3zeui {flex-basis:100%;order:1;}
}`};function Dd(n,e){Ce(e,!0),Te(n,jd);const t=d=>{var u=Ld(),g=le(u);{var p=k=>{var z=We(),O=le(z);Oe(O,()=>(s==null?void 0:s.content)??st),C(k,z)};U(g,k=>{i||k(p)})}var v=P(g,2),_=L(v);{var h=k=>{zd(k,{get items(){return e.crumbs}})},w=k=>{var z=Ps();B(()=>Y(z,e.title)),C(k,z)};U(_,k=>{e.crumbs?k(h):k(w,-1)})}var x=P(v,2);{var m=k=>{var z=Td(),O=te(z,!0);B(()=>Y(O,e.note)),C(k,z)};U(x,k=>{e.note&&k(m)})}var M=P(x,2);{var A=k=>{var z=Bd(),O=L(z);Oe(O,()=>e.state),C(k,z)};U(M,k=>{e.state&&k(A)})}var y=P(M,2),S=L(y);Oe(S,()=>e.actions??st),C(d,u)};let r=en(e,"bleed",3,!0);const i=Hs.get(),s=Sc.get();Jn(()=>{if(i)return i.content=t,()=>i.content=null});var o=We(),l=le(o);{var c=d=>{var u=Pd();let g;var p=L(u);t(p),B(()=>g=Xe(u,1,"page-top svelte-z3zeui",null,g,{bleed:r()})),C(d,u)};U(l,d=>{i||d(c)})}C(n,o),Se()}var Od=I("<col/>"),qd=I("<colgroup></colgroup>"),Nd=I('<th class="svelte-migw9g"> </th>'),Rd=I("<thead><tr></tr></thead>"),Fd=I('<tr class="detail svelte-migw9g"><td><!></td></tr>'),Hd=I('<tr class="svelte-migw9g"><!></tr> <!>',1),Vd=I('<tr class="svelte-migw9g"><td class="empty svelte-migw9g"> </td></tr>'),Yd=I('<div class="scroll svelte-migw9g"><table><!><!><tbody class="svelte-migw9g"></tbody></table></div>');const Ud={hash:"svelte-migw9g",code:`.scroll.svelte-migw9g {overflow-x:auto;}table.svelte-migw9g {width:100%;border-collapse:collapse;}table.fixed.svelte-migw9g {table-layout:fixed;}th.svelte-migw9g,
tbody.svelte-migw9g td {padding:0.375rem 0.5rem;text-align:left;vertical-align:top;}th.svelte-migw9g {font-weight:600;}tbody.svelte-migw9g td {border-top:1px solid var(--line);}tbody.svelte-migw9g code {font-size:var(--fs-md);color:var(--muted);}tbody.svelte-migw9g tr:where(.svelte-migw9g).target {background:var(--hover);transition:background 0.3s;}tbody.svelte-migw9g tr[aria-expanded=true]:where(.svelte-migw9g),
.detail.svelte-migw9g {background:var(--surface-2);}.detail.svelte-migw9g td {border-top:none;padding:0.25rem 0.5rem 0.75rem;}.empty.svelte-migw9g {color:var(--muted);}.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail) {cursor:pointer;}

@media (hover: hover) {.links.svelte-migw9g tbody:where(.svelte-migw9g) tr:where(.svelte-migw9g):not(.detail):hover {background:var(--hover);}
}`};function Gd(n,e){Ce(e,!0),Te(n,Ud);let t=en(e,"columns",19,()=>[]);function r(p,v){if(p.target.closest("a, button, input, select"))return;if(e.ontoggle)return e.ontoggle(v);if(!e.href)return;const _=e.href(v);if(p.metaKey||p.ctrlKey)return void window.open(_,"_blank");const w=p.currentTarget.querySelector(`a[href="${CSS.escape(_)}"]`);w?w.click():location.assign(_)}var i=Yd(),s=L(i);let o;var l=L(s);{var c=p=>{var v=qd();Ue(v,21,()=>e.widths,lr,(_,h)=>{var w=Od();let x;B(()=>x=ht(w,"",x,{width:a(h)})),C(_,w)}),C(p,v)};U(l,p=>{e.widths&&p(c)})}var d=P(l);{var u=p=>{var v=Rd(),_=L(v);Ue(_,21,t,lr,(h,w)=>{var x=Nd(),m=te(x,!0);B(()=>Y(m,a(w))),C(h,x)}),C(p,v)};U(d,p=>{t().length&&p(u)})}var g=P(d);Ue(g,23,()=>e.rows,p=>e.key(p),(p,v,_)=>{var h=Hd(),w=le(h),x=L(w);Oe(x,()=>e.row,()=>a(v),()=>a(_));var m=P(w,2);{var M=y=>{var S=Fd(),k=L(S),z=L(k);Oe(z,()=>e.detail,()=>a(v)),B(()=>Z(k,"colspan",t().length||1)),C(y,S)},A=ne(()=>{var y;return e.detail&&((y=e.isOpen)==null?void 0:y.call(e,a(v)))});U(m,y=>{a(A)&&y(M)})}B((y,S)=>{Z(w,"id",y),Z(w,"aria-expanded",S),Z(w,"tabindex",e.ontoggle?0:void 0)},[()=>{var y;return(y=e.id)==null?void 0:y.call(e,a(v))},()=>{var y;return e.ontoggle?!!((y=e.isOpen)!=null&&y.call(e,a(v))):void 0}]),Q("click",w,y=>r(y,a(v))),Q("keydown",w,y=>{!e.ontoggle||y.target!==y.currentTarget||y.key!=="Enter"&&y.key!==" "||(y.preventDefault(),e.ontoggle(a(v)))}),C(p,h)},p=>{var v=Vd(),_=L(v),h=te(_,!0);B(()=>{Z(_,"colspan",t().length||1),Y(h,e.empty??vn.texts.nothingYet)}),C(p,v)}),B(()=>o=Xe(s,1,"svelte-migw9g",null,o,{links:!!e.href||!!e.ontoggle,fixed:!!e.widths})),C(n,i),Se()}pn(["click","keydown"]);var Nt,Rt,Ft,Ht,Vt,Yt;class Wd{constructor(e){N(this,Nt,J(Qe([])));N(this,Rt,J(null));N(this,Ft,J(Qe([])));N(this,Ht,J(0));N(this,Vt,J(!1));N(this,Yt,J(""));this.host=e}get forms(){return a(f(this,Nt))}set forms(e){q(f(this,Nt),e,!0)}get form(){return a(f(this,Rt))}set form(e){q(f(this,Rt),e,!0)}get submissions(){return a(f(this,Ft))}set submissions(e){q(f(this,Ft),e,!0)}get total(){return a(f(this,Ht))}set total(e){q(f(this,Ht),e,!0)}get loading(){return a(f(this,Vt))}set loading(e){q(f(this,Vt),e,!0)}get error(){return a(f(this,Yt))}set error(e){q(f(this,Yt),e,!0)}async init(){await this.run(async()=>{const e=await this.host.request("GET","/entries?section=forms&limit=500&order=title");this.forms=e.entries.map(t=>({id:t.id,title:t.title}))}),await this.load()}async load(){await this.run(async()=>{const e=await this.request(0);this.submissions=e.submissions,this.total=e.total})}async more(){await this.run(async()=>{const e=await this.request(this.submissions.length);this.submissions=[...this.submissions,...e.submissions],this.total=e.total})}async delete(e){await this.run(async()=>{await this.host.request("DELETE",`/forms/submissions/${e}`),this.submissions=this.submissions.filter(t=>t.id!==e),this.total-=1})}request(e){const t=this.form===null?"":`&form=${this.form}`;return this.host.request("GET",`/forms/submissions?offset=${e}${t}`)}async run(e){this.loading=!0,this.error="";try{await e()}catch(t){this.error=t.message}finally{this.loading=!1}}}Nt=new WeakMap,Rt=new WeakMap,Ft=new WeakMap,Ht=new WeakMap,Vt=new WeakMap,Yt=new WeakMap;var Xd=I("<option> </option>"),Kd=I("<select><option> </option><!></select>"),Zd=I('<p class="error"> </p>'),Jd=I('<td class="nowrap svelte-1tixh7s"> </td> <td> </td> <td> </td> <td class="truncate"> </td>',1),Qd=I('<dt class="svelte-1tixh7s"> </dt> <dd class="svelte-1tixh7s"> </dd>',1),$d=I('<dl class="svelte-1tixh7s"></dl> <button class="danger"><!> </button>',1),eu=I('<p class="more svelte-1tixh7s"><button class="light"> </button></p>'),nu=I('<!> <!> <div class="card"><!></div> <!>',1);const tu={hash:"svelte-1tixh7s",code:`.nowrap.svelte-1tixh7s {white-space:nowrap;}dl.svelte-1tixh7s {display:grid;grid-template-columns:minmax(8rem, max-content) 1fr;gap:0.375rem 1.5rem;margin:0 0 1rem;}dt.svelte-1tixh7s {color:var(--muted);}dd.svelte-1tixh7s {margin:0;white-space:pre-wrap;overflow-wrap:anywhere;}.more.svelte-1tixh7s {text-align:center;}

@media (max-width: 799px) {dl.svelte-1tixh7s {grid-template-columns:1fr;}dd.svelte-1tixh7s {margin-bottom:0.5rem;}
}`};function ru(n,e){Ce(e,!0),Te(n,tu);const t=ci(),r=new Wd(e.host);r.init();let i=J(null);const s=m=>{var M;return((M=e.schema.sites.find(A=>A.handle===m))==null?void 0:M.name)??m},o=m=>new Date(m).toLocaleString(document.documentElement.lang,{dateStyle:"medium",timeStyle:"short"}),l=m=>m.answers.map(M=>M.value).filter(Boolean).slice(0,3).join(" · ")||t.empty;function c(m){const M=m.currentTarget.value;r.form=M?Number(M):null,q(i,null),r.load()}function d(m){confirm(t.confirmDelete)&&r.delete(m.id)}var u=nu(),g=le(u);Dd(g,{get title(){return t.title},get note(){return t.note},actions:M=>{var A=We(),y=le(A);{var S=k=>{var z=Kd(),O=L(z),W=te(O,!0);O.value=O.__value="";var $=P(O);Ue($,17,()=>r.forms,fe=>fe.id,(fe,Ee)=>{var R=Xd(),ie=te(R,!0),ue={};B(()=>{Y(ie,a(Ee).title),ue!==(ue=a(Ee).id)&&(R.value=(R.__value=ue)??"")}),C(fe,R)});var oe;Sl(z),B(()=>{Y(W,t.allForms),oe!==(oe=r.form??"")&&(z.value=(z.__value=oe)??"",Os(z,oe))}),Q("change",z,c),C(k,z)};U(y,k=>{r.forms.length>1&&k(S)})}C(M,A)},$$slots:{actions:!0}});var p=P(g,2);{var v=m=>{var M=Zd(),A=te(M,!0);B(()=>Y(A,r.error)),C(m,M)};U(p,m=>{r.error&&m(v)})}var _=P(p,2),h=L(_);{const m=(S,k=st)=>{var z=Jd(),O=le(z),W=te(O,!0),$=P(O,2),oe=te($,!0),fe=P($,2),Ee=te(fe,!0),R=P(fe,2),ie=te(R,!0);B((ue,rn,ke)=>{Y(W,ue),Y(oe,k().form),Y(Ee,rn),Y(ie,ke)},[()=>o(k().createdAt),()=>s(k().site),()=>l(k())]),C(S,z)},M=(S,k=st)=>{var z=$d(),O=le(z);Ue(O,21,()=>k().answers,fe=>fe.handle,(fe,Ee)=>{var R=Qd(),ie=le(R),ue=te(ie,!0),rn=P(ie,2),ke=te(rn,!0);B(()=>{Y(ue,a(Ee).label),Y(ke,a(Ee).value||t.empty)}),C(fe,R)});var W=P(O,2),$=L(W);wn($,{name:"delete"});var oe=P($);B(()=>Y(oe,` ${t.delete??""}`)),Q("click",W,()=>d(k())),C(S,z)};let A=ne(()=>[t.date,t.form,t.site,t.content]),y=ne(()=>r.loading?"…":t.none);Gd(h,{get rows(){return r.submissions},key:S=>S.id,get columns(){return a(A)},get empty(){return a(y)},isOpen:S=>S.id===a(i),ontoggle:S=>q(i,a(i)===S.id?null:S.id,!0),row:m,detail:M,$$slots:{row:!0,detail:!0}})}var w=P(_,2);{var x=m=>{var M=eu(),A=L(M),y=te(A,!0);B(()=>{A.disabled=r.loading,Y(y,t.more)}),Q("click",A,()=>r.more()),C(m,M)};U(w,m=>{r.submissions.length<r.total&&m(x)})}C(n,u),Se()}pn(["change","click"]);const ou=Pl(yd);function lu(n,e){const t=js(ru,{target:n,props:e});return{destroy(){Ds(t)}}}export{ou as mount,lu as page};
