const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["_studio/Shell-DRkWzPNg.js","_studio/snippet-Dg_kry06.js","_studio/actions-BgeP228g.js","_studio/api-b77vxwot.js","_studio/AssetDetail-Cr7a2wGD.js","_studio/index-client-DYMvGiys.js","_studio/PageTop-Dq94GNix.js","_studio/PageTop-2HABWXZF.css","_studio/Fields-BvjpQJmS.js","_studio/markdown-W5giuhZg.js","_studio/format-B2TmwA2x.js","_studio/SchemaIcon-DBNYkfSt.js","_studio/Popover-DzxqEC7f.js","_studio/Popover-17_6FwQX.css","_studio/SchemaIcon-C2qbtct_.css","_studio/Segmented-BZNmcoRL.js","_studio/Segmented-CDCZlR9s.css","_studio/Switch-CYUvU2Qa.js","_studio/Switch-BTxz58ZF.css","_studio/Fields-DqHE8QVH.css","_studio/busy.svelte-DhqvKr3z.js","_studio/size-DNrmHlAi.js","_studio/AssetDetail-B-rNJIIB.css","_studio/Edit-8T4f_l-l.js","_studio/api-CkbVCY-M.js","_studio/preview.svelte-CdhUflYZ.js","_studio/preview-CmmVDi64.css","_studio/SiteBadges-BfmtIe3i.js","_studio/SiteBadges-2JyNTGtB.css","_studio/paths-zqspf6Oj.js","_studio/Edit-Ga1HzMXv.css","_studio/legacy-0lUMxcBn.js","_studio/Shell-SgZ8vMAv.css","_studio/Ai-D7Rop8uJ.js","_studio/Ai-QR1XEdSo.css","_studio/AssetsPage-DQJ6XYfb.js","_studio/AssetsPage-CTCYJkFL.css","_studio/Login-ByL9RlSl.js","_studio/AuthLayout-CdTTu5gy.js","_studio/AuthLayout-DrugWPL5.css","_studio/Forgot-DbMkqs3X.js","_studio/SetPassword-C-7AEfvI.js","_studio/LoginAs-C2twZsdk.js","_studio/List-CuZer0zm.js","_studio/List-CAE2UMzP.css","_studio/Tokens-UZ_dxJlK.js","_studio/api-0h0MC-ZZ.js","_studio/Table-BLLYzdw3.js","_studio/Table-MkMZOomE.css","_studio/Tokens-D3ILBhRw.css","_studio/NewToken-CwxjL8m2.js","_studio/NewToken-DUJ0BEpO.css","_studio/Setup-Dw1WFRqR.js","_studio/setup-DLSMuLl3.js","_studio/Setup-GuAVfguJ.css","_studio/Trash-ChYOTPVI.js","_studio/Trash-C2QsZMQE.css","_studio/Users-BowhHXlU.js","_studio/Users-D3kO8cuw.css","_studio/User-DX43jiea.js","_studio/User-D28InCH6.css","_studio/Start-BJEUtIyk.js","_studio/Doc-BUceaenD.js","_studio/Doc-BE2vUwqc.css","_studio/NotFound-DEA14OEf.js","_studio/NotFound-BsjKVbCC.css","_studio/ErrorPage-AnHt3dXP.js","_studio/ErrorPage-_uv0f8-W.css"])))=>i.map(i=>d[i]);
var Ur=Object.defineProperty;var Js=n=>{throw TypeError(n)};var Wr=(n,e,t)=>e in n?Ur(n,e,{enumerable:!0,configurable:!0,writable:!0,value:t}):n[e]=t;var y=(n,e,t)=>Wr(n,typeof e!="symbol"?e+"":e,t),Xt=(n,e,t)=>e.has(n)||Js("Cannot "+t);var a=(n,e,t)=>(Xt(n,e,"read from private field"),t?t.call(n):e.get(n)),k=(n,e,t)=>e.has(n)?Js("Cannot add the same private member more than once"):e instanceof WeakSet?e.add(n):e.set(n,t),x=(n,e,t,s)=>(Xt(n,e,"write to private field"),s?s.call(n,t):e.set(n,t),t),w=(n,e,t)=>(Xt(n,e,"access private method"),t);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))s(i);new MutationObserver(i=>{for(const r of i)if(r.type==="childList")for(const o of r.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&s(o)}).observe(document,{childList:!0,subtree:!0});function t(i){const r={};return i.integrity&&(r.integrity=i.integrity),i.referrerPolicy&&(r.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?r.credentials="include":i.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(i){if(i.ep)return;i.ep=!0;const r=t(i);fetch(i.href,r)}})();const Yr="modulepreload",Gr=function(n){return"/"+n},Qs={},O=function(e,t,s){let i=Promise.resolve();if(t&&t.length>0){let o=function(d){return Promise.all(d.map(c=>Promise.resolve(c).then(h=>({status:"fulfilled",value:h}),h=>({status:"rejected",reason:h}))))};document.getElementsByTagName("link");const l=document.querySelector("meta[property=csp-nonce]"),u=(l==null?void 0:l.nonce)||(l==null?void 0:l.getAttribute("nonce"));i=o(t.map(d=>{if(d=Gr(d),d in Qs)return;Qs[d]=!0;const c=d.endsWith(".css"),h=c?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${d}"]${h}`))return;const g=document.createElement("link");if(g.rel=c?"stylesheet":Yr,c||(g.as="script"),g.crossOrigin="",g.href=d,u&&g.setAttribute("nonce",u),document.head.appendChild(g),c)return new Promise((f,p)=>{g.addEventListener("load",f),g.addEventListener("error",()=>p(new Error(`Unable to preload CSS for ${d}`)))})}))}function r(o){const l=new Event("vite:preloadError",{cancelable:!0});if(l.payload=o,window.dispatchEvent(l),!l.defaultPrevented)throw o}return i.then(o=>{for(const l of o||[])l.status==="rejected"&&r(l.reason);return e().catch(r)})},Ei=!1;var Kr=Array.isArray,$r=Array.prototype.indexOf,$n=Array.prototype.includes,Zr=Array.from,Ti=Object.defineProperty,He=Object.getOwnPropertyDescriptor,Ai=Object.getOwnPropertyDescriptors,Jr=Object.prototype,Qr=Array.prototype,Ds=Object.getPrototypeOf,Xs=Object.isExtensible;function Fn(n){return typeof n=="function"}const Xr=()=>{};function Cd(n){return n()}function eo(n){for(var e=0;e<n.length;e++)n[e]()}function Ii(){var n,e,t=new Promise((s,i)=>{n=s,e=i});return{promise:t,resolve:n,reject:e}}function Md(n,e){if(Array.isArray(n))return n;if(e===void 0||!(Symbol.iterator in n))return Array.from(n);const t=[];for(const s of n)if(t.push(s),t.length===e)break;return t}const W=2,Tn=4,dt=8,Li=1<<24,ve=16,Ce=32,Ye=64,ps=128,Ps=256,ke=512,q=1024,C=2048,ne=4096,le=8192,U=16384,Pn=32768,Zn=1<<25,on=65536,Mt=1<<17,no=1<<18,Bn=1<<19,Ci=1<<20,Dd=1<<25,Dt=1<<21,mn=1<<22,Ue=1<<23,Ae=Symbol("$state"),Mi=Symbol("component"),Di=Symbol("legacy props"),to=Symbol(""),Tt=Symbol("attributes"),gs=Symbol("class"),vs=Symbol("style"),qn=Symbol("text"),At=Symbol("form reset"),ct=new class extends Error{constructor(){super(...arguments);y(this,"name","StaleReactionError");y(this,"message","The reaction that called `getAbortSignal()` was re-run or destroyed")}};var yi;const so=!!((yi=globalThis.document)!=null&&yi.contentType)&&globalThis.document.contentType.includes("xml"),Bd=1,Rd=2,Nd=4,Od=8,Fd=16,io=1,ro=2,oo=4,lo=8,ao=16,uo=1,co=2,Pi={},F=Symbol("uninitialized"),Bi="http://www.w3.org/1999/xhtml",ho="http://www.w3.org/2000/svg",fo="http://www.w3.org/1998/Math/MathML";function po(){console.warn("https://svelte.dev/e/derived_inert")}function zd(){console.warn("https://svelte.dev/e/select_multiple_invalid_value")}function go(){console.warn("https://svelte.dev/e/svelte_boundary_reset_noop")}function Ri(n){return n===this.v}function vo(n,e){return n!=n?e==e:n!==e||n!==null&&typeof n=="object"||typeof n=="function"}function Ni(n){return!vo(n,this.v)}function mo(n){throw new Error("https://svelte.dev/e/lifecycle_outside_component")}function wo(){throw new Error("https://svelte.dev/e/async_derived_orphan")}function qd(n,e,t){throw new Error("https://svelte.dev/e/each_key_duplicate")}function _o(n){throw new Error("https://svelte.dev/e/effect_in_teardown")}function ko(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function bo(n){throw new Error("https://svelte.dev/e/effect_orphan")}function yo(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function xo(n){throw new Error("https://svelte.dev/e/props_invalid_value")}function So(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function Eo(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function To(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}function Ao(){throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror")}let ht=!1,Io=!1;function jd(){ht=!0}function Lo(n){let e=n.p;for(;e!==null&&e.c===null;)e=e.p;return(e==null?void 0:e.c)??null}function Oi(n,e){return n===null&&mo(),n.c??(n.c=new Map(Lo(n)||void 0))}let M=null;function An(n){M=n}function Vd(n){return Oi(M).get(n)}function Hd(n,e){return Oi(M).set(n,e),e}function ft(n,e=!1,t){M={p:M,i:!1,c:null,e:null,s:n,x:null,r:T,l:ht&&!e?{s:null,u:null,$:[]}:null}}function pt(n){var e=M,t=e.e;if(t!==null){e.e=null;for(var s of t)lr(s)}return e.i=!0,M=e.p,Bs(n)}function Bs(n={}){return Ti(n,Mi,{value:!0}),n}function Rn(){return!ht||M!==null&&M.l===null}let Ze=[];function Fi(){var n=Ze;Ze=[],eo(n)}function ze(n){if(Ze.length===0&&!Wn){var e=Ze;queueMicrotask(()=>{e===Ze&&Fi()})}Ze.push(n)}function Co(){for(;Ze.length>0;)Fi()}const Mo=-7169;function B(n,e){n.f=n.f&Mo|e}function Rs(n){(n.f&ke)!==0||n.deps===null?B(n,q):B(n,ne)}function zi(n,e){(n.f&C)!==0?e.set(n,C):(n.f&ne)!==0&&e.get(n)!==C&&e.set(n,ne),B(n,q)}function Do(n,e){{const t=document.body;n.autofocus=!0,ze(()=>{document.activeElement===t&&n.focus()})}}let ei=!1;function Po(){ei||(ei=!0,document.addEventListener("reset",n=>{Promise.resolve().then(()=>{var e;if(!n.defaultPrevented)for(const t of n.target.elements)(e=t[At])==null||e.call(t)})},{capture:!0}))}function Nn(n){var e=S,t=T;ae(null),Me(null);try{return n()}finally{ae(e),Me(t)}}function qi(n,e,t,s=t){n.addEventListener(e,()=>Nn(t));const i=n[At];i?n[At]=()=>{i(),s(!0)}:n[At]=()=>s(!0),Po()}function Bo(n,e,t,s){const i=Rn()?Jn:Vi;var r=n.filter(f=>!f.settled),o=e.map(i);if(t.length===0&&r.length===0){s(o);return}var l=T,u=Ro(),d=r.length===1?r[0].promise:r.length>1?Promise.all(r.map(f=>f.promise)):null;function c(f){if((l.f&U)===0){u();try{s([...o,...f])}catch(p){Ee(p,l)}Pt()}}var h=ji();if(t.length===0){d.then(()=>c([])).finally(h);return}function g(){Promise.all(t.map(f=>No(f))).then(c).catch(f=>Ee(f,l)).finally(h)}d?d.then(()=>{if((l.f&U)!==0){h();return}u(),g(),Pt()}):g()}function Ro(){var n=T,e=S,t=M,s=m;return function(r=!0){Me(n),ae(e),An(t),r&&(n.f&U)===0&&(s==null||s.activate(),s==null||s.apply())}}function Pt(n=!0){Me(null),ae(null),An(null),n&&(m==null||m.deactivate())}function ji(){var n=T,e=n.b,t=m,s=!!(e!=null&&e.is_rendered());return e==null||e.update_pending_count(1,t),t.increment(s,n),()=>{e==null||e.update_pending_count(-1,t),t.decrement(s,n)}}function Jn(n){var e=W|C;return T!==null&&(T.f|=Bn),{ctx:M,deps:null,effects:null,equals:Ri,f:e,fn:n,reactions:null,rv:0,v:F,wv:0,parent:T,ac:null}}const jn=Symbol("obsolete");function No(n,e,t){let s=T;s===null&&wo();var i=void 0,r=gt(F),o=!S,l=new Set;return Jo(()=>{var f,p;var u=T,d=Ii();i=d.promise;try{Promise.resolve(n()).then(d.resolve,v=>{v!==ct&&d.reject(v)}).finally(Pt)}catch(v){d.reject(v),Pt()}var c=m;if(o){if((u.f&Pn)!==0)var h=ji();if((f=s.b)!=null&&f.is_rendered())(p=c.async_deriveds.get(u))==null||p.reject(jn);else for(const v of l.values())v.reject(jn);l.add(d),c.async_deriveds.set(u,d)}const g=(v,_=void 0)=>{h==null||h(),l.delete(d),_!==jn&&(c.activate(),_?(r.f|=Ue,Rt(r,_)):((r.f&Ue)!==0&&(r.f^=Ue),Rt(r,v)),c.deactivate())};d.promise.then(g,v=>g(null,v||"unknown"))}),rr(()=>{for(const u of l)u.reject(jn)}),new Promise(u=>{function d(c){function h(){c===i?u(r):d(i)}c.then(h,h)}d(i)})}function Ns(n){const e=Jn(n);return pr(e),e}function Vi(n){const e=Jn(n);return e.equals=Ni,e}function Oo(n){var e=n.effects;if(e!==null){n.effects=null;for(var t=0;t<e.length;t+=1)te(e[t])}}function Os(n){var e,t=T,s=n.parent;if(!qe&&s!==null&&n.v!==F&&(s.f&(U|le))!==0)return po(),n.v;Me(s);try{Oo(n),e=wr(n)}finally{Me(t)}return e}function Hi(n){var e=Os(n);if(!n.equals(e)&&(n.wv=vr(),(!(m!=null&&m.is_fork)||n.deps===null)&&(m!==null?(m.capture(n,e,!0),We==null||We.capture(n,e,!0)):n.v=e,n.deps===null))){B(n,q);return}qe||(V!==null?(qs()||m!=null&&m.is_fork)&&V.set(n,e):Rs(n))}function Fo(n){var e;if(n.effects!==null)for(const t of n.effects)(t.teardown||t.ac)&&((e=t.teardown)==null||e.call(t),t.ac!==null&&Nn(()=>{t.ac.abort(ct),t.ac=null}),t.fn!==null&&(t.teardown=Xr),Qn(t,0),js(t))}function Ui(n){if(n.effects!==null)for(const e of n.effects)e.teardown&&e.fn!==null&&Ln(e)}let es=null,fn=null,m=null,We=null,V=null,ms=null,Wn=!1,ns=!1,Yn=null,It=null;var ni=0;let zo=1;var kn,bn,yn,Ne,xn,Oe,Xn,be,ye,Sn,en,E,ws,Wi,_s,Vn,ks,Yi,Gi,Ki,gn,qo,Hn;const qt=class qt{constructor(){k(this,E);y(this,"id",zo++);y(this,"started",!1);y(this,"linked",!0);y(this,"prev",null);y(this,"next",null);y(this,"async_deriveds",new Map);y(this,"current",new Map);y(this,"previous",new Map);k(this,kn,new Set);k(this,bn,new Set);k(this,yn,0);k(this,Ne,new Map);k(this,xn,null);k(this,Oe,[]);k(this,Xn,[]);k(this,be,new Map);k(this,ye,new Map);k(this,Sn,new Set);y(this,"is_fork",!1);k(this,en,!1);fn===null?es=fn=this:(fn.next=this,this.prev=fn),fn=this}skip_effect(e){a(this,ye).has(e)||a(this,ye).set(e,{d:[],m:[]}),a(this,Sn).delete(e)}unskip_effect(e,t=s=>this.schedule(s)){var s=a(this,ye).get(e);if(s){a(this,ye).delete(e);for(var i of s.d)B(i,C),t(i);for(i of s.m)B(i,ne),t(i)}a(this,Sn).add(e)}capture(e,t,s=!1){e.v!==F&&!this.previous.has(e)&&this.previous.set(e,e.v),(e.f&Ue)===0&&(this.current.set(e,[t,s]),V==null||V.set(e,t)),this.is_fork||(e.v=t)}activate(){m=this}deactivate(){m=null,V=null}flush(){try{ns=!0,m=this,w(this,E,Vn).call(this)}finally{ni=0,ms=null,Yn=null,It=null,ns=!1,m=null,V=null,Ie.clear()}}discard(){var e;for(const t of a(this,bn))t(this);a(this,bn).clear();for(const t of this.async_deriveds.values())t.reject(jn);w(this,E,Hn).call(this),(e=a(this,xn))==null||e.resolve()}register_created_effect(e){a(this,Xn).push(e)}increment(e,t){if(x(this,yn,a(this,yn)+1),e){let s=a(this,Ne).get(t)??0;a(this,Ne).set(t,s+1)}}decrement(e,t){if(x(this,yn,a(this,yn)-1),e){let s=a(this,Ne).get(t)??0;s===1?a(this,Ne).delete(t):a(this,Ne).set(t,s-1)}a(this,en)||(x(this,en,!0),ze(()=>{x(this,en,!1),this.linked&&this.flush()}))}transfer_reactions(e){for(const[t,s]of e)this.add_dirty_reaction(t,s);e.clear()}add_dirty_reaction(e,t){(t===C||a(this,be).get(e)!==C)&&a(this,be).set(e,t)}oncommit(e){a(this,kn).add(e)}ondiscard(e){a(this,bn).add(e)}settled(){return(a(this,xn)??x(this,xn,Ii())).promise}static ensure(){if(m===null){const e=m=new qt;!ns&&!Wn&&ze(()=>{e.started||e.flush()})}return m}apply(){{V=null;return}}schedule(e){var t;if(ms=e,(t=e.b)!=null&&t.is_pending&&(e.f&(Tn|dt|Li))!==0&&(e.f&Pn)===0){e.b.defer_effect(e);return}a(this,Oe).push(e)}};kn=new WeakMap,bn=new WeakMap,yn=new WeakMap,Ne=new WeakMap,xn=new WeakMap,Oe=new WeakMap,Xn=new WeakMap,be=new WeakMap,ye=new WeakMap,Sn=new WeakMap,en=new WeakMap,E=new WeakSet,ws=function(){if(this.is_fork)return!0;for(const e of a(this,Ne).keys())if(!w(this,E,Wi).call(this,e))return!0;return!1},Wi=function(e){for(var t=e;t!==null;t=t.parent)if(a(this,ye).has(t))return!0;return!1},_s=function(){var e=[];for(const r of a(this,Oe))if(!((r.f&U)!==0||(r.f&(C|ne))===0)){for(var t=r,s=!1;t.parent!==null;){t=t.parent;var i=t.f;if((i&(Ye|Ce))!==0){if((i&q)===0){s=!0;break}t.f^=q}}s||e.push(t)}return x(this,Oe,[]),e},Vn=function(){var l,u,d,c;this.started=!0;for(const[h,g]of a(this,be))B(h,g),this.schedule(h);this.apply();for(var e=Yn=[],t=[],s=It=[];a(this,Oe).length>0;){ni++>1e3&&(w(this,E,Hn).call(this),Vo());for(const h of w(this,E,_s).call(this))try{w(this,E,ks).call(this,h,e,t)}catch(g){throw Ji(h),w(this,E,ws).call(this)||this.discard(),g}}if(m=null,s.length>0){var i=qt.ensure();for(const h of s)i.schedule(h)}if(Yn=null,It=null,w(this,E,ws).call(this)){w(this,E,gn).call(this,t),w(this,E,gn).call(this,e);for(const[h,g]of a(this,ye))Zi(h,g);s.length>0&&w(l=m,E,Vn).call(l);return}const r=w(this,E,Yi).call(this);if(r){w(this,E,gn).call(this,t),w(this,E,gn).call(this,e),w(u=r,E,Ki).call(u,this);return}a(this,be).clear();for(const h of a(this,kn))h(this);a(this,kn).clear(),We=this,ti(t),ti(e),We=null,(d=a(this,xn))==null||d.resolve();var o=m;a(this,yn)===0&&(a(this,Oe).length===0||o!==null)&&w(this,E,Hn).call(this),o!==null&&(Ie.clear(),w(c=o,E,Vn).call(c))},ks=function(e,t,s){e.f^=q;for(var i=e.first;i!==null;){var r=i.f,o=(r&(Ce|Ye))!==0,l=o&&(r&q)!==0,u=l||(r&le)!==0||a(this,ye).has(i);if(!u&&i.fn!==null){o?i.f^=q:(r&Tn)!==0?t.push(i):vt(i)&&((r&ve)!==0&&this.add_dirty_reaction(i,ne),Ln(i));var d=i.first;if(d!==null){i=d;continue}}for(;i!==null;){var c=i.next;if(c!==null){i=c;break}i=i.parent}}},Yi=function(){for(var e=this.prev;e!==null;e=e.prev)if(!e.is_fork&&w(this,E,Gi).call(this,e))return e;return null},Gi=function(e){for(const[t,[,s]]of this.current)if(!s&&e.current.has(t))return!0;return!1},Ki=function(e){var s;for(const[i,r]of e.current)!this.previous.has(i)&&e.previous.has(i)&&this.previous.set(i,e.previous.get(i)),this.current.set(i,r);for(const[i,r]of e.async_deriveds){const o=this.async_deriveds.get(i);o&&r.promise.then(o.resolve).catch(o.reject)}e.async_deriveds.clear(),this.transfer_reactions(a(e,be));const t=i=>{var r=i.reactions;if(r!==null&&!((i.f&W)!==0&&(i.f&(C|ne))===0))for(const u of r){var o=u.f;if((o&W)!==0)t(u);else{var l=u;o&(mn|ve)&&!this.async_deriveds.has(l)&&(a(this,be).delete(l),B(l,C),this.schedule(l))}}};for(const i of this.current.keys())t(i);this.oncommit(()=>e.discard()),w(s=e,E,Hn).call(s),m=this,w(this,E,Vn).call(this)},gn=function(e){for(var t=0;t<e.length;t+=1)zi(e[t],a(this,be))},qo=function(){var h,g;for(let f=es;f!==null;f=f.next){var e=f.id<this.id,t=[];for(const[p,[v,_]]of this.current){if(f.current.has(p)){var s=f.current.get(p)[0];if(e&&v!==s)f.current.set(p,[v,_]);else continue}t.push(p)}if(e)for(const[p,v]of this.async_deriveds){const _=f.async_deriveds.get(p);_&&v.promise.then(_.resolve).catch(_.reject)}var i=[...f.current.keys()].filter(p=>!f.current.get(p)[1]);if(!(!f.started||i.length===0)){var r=i.filter(p=>!this.current.has(p));if(r.length===0)e&&f.discard();else if(t.length>0){if(e)for(const p of a(this,Sn))f.unskip_effect(p,v=>{var _;(v.f&(ve|mn))!==0?f.schedule(v):w(_=f,E,gn).call(_,[v])});f.activate();var o=new Set,l=new Map;for(var u of t)$i(u,r,o,l);l=new Map;var d=[...f.current].filter(([p,v])=>{const _=this.current.get(p);return _?_[0]!==v[0]||_[1]!==v[1]:!0}).map(([p])=>p);if(d.length>0)for(const p of a(this,Xn))(p.f&(U|le|Mt))===0&&Fs(p,d,l)&&((p.f&(mn|ve))!==0?(B(p,C),f.schedule(p)):f.add_dirty_reaction(p,C));if(a(f,Oe).length>0&&!a(f,en)){f.apply();for(var c of w(h=f,E,_s).call(h))w(g=f,E,ks).call(g,c,[],[])}f.deactivate()}}}},Hn=function(){if(this.linked){var e=this.prev,t=this.next;e===null?es=t:e.next=t,t===null?fn=e:t.prev=e,this.linked=!1}};let ln=qt;function jo(n){var e=Wn,t=We;We=null,Wn=!0;try{for(var s;;){if(Co(),m===null)return s;m.flush()}}finally{Wn=e,We=t}}function Vo(){try{yo()}catch(n){Ee(n,ms)}}let pe=null;function ti(n){var e=n.length;if(e!==0){for(var t=0;t<e;){var s=n[t++];if((s.f&(U|le))===0&&vt(s)&&(pe=new Set,Ln(s),s.deps===null&&s.first===null&&s.nodes===null&&s.teardown===null&&s.ac===null&&dr(s),(pe==null?void 0:pe.size)>0)){Ie.clear();for(const i of pe){if((i.f&(U|le))!==0)continue;const r=[i];let o=i.parent;for(;o!==null;)pe.has(o)&&(pe.delete(o),r.push(o)),o=o.parent;for(let l=r.length-1;l>=0;l--){const u=r[l];(u.f&(U|le))===0&&Ln(u)}}pe.clear()}}pe=null}}function $i(n,e,t,s){if(!t.has(n)&&(t.add(n),n.reactions!==null))for(const i of n.reactions){const r=i.f;(r&W)!==0?$i(i,e,t,s):(r&(mn|ve))!==0&&(r&C)===0&&Fs(i,e,s)&&(B(i,C),zs(i))}}function Fs(n,e,t){const s=t.get(n);if(s!==void 0)return s;if(n.deps!==null)for(const i of n.deps){if($n.call(e,i))return!0;if((i.f&W)!==0&&Fs(i,e,t))return t.set(i,!0),!0}return t.set(n,!1),!1}function zs(n){m.schedule(n)}function Zi(n,e){if(!((n.f&Ce)!==0&&(n.f&q)!==0)){(n.f&C)!==0?e.d.push(n):(n.f&ne)!==0&&e.m.push(n),B(n,q);for(var t=n.first;t!==null;)Zi(t,e),t=t.next}}function Ji(n){B(n,q);for(var e=n.first;e!==null;)Ji(e),e=e.next}let Bt=new Set;const Ie=new Map;let Qi=!1;function gt(n,e){var t={f:0,v:n,reactions:null,equals:Ri,rv:0,wv:0};return t}function P(n,e){const t=gt(n);return pr(t),t}function Ud(n,e=!1,t=!0){var i;const s=gt(n);return e||(s.equals=Ni),ht&&t&&M!==null&&M.l!==null&&((i=M.l).s??(i.s=[])).push(s),s}function L(n,e,t=!1){S!==null&&(!_e||(S.f&Mt)!==0)&&Rn()&&(S.f&(W|ve|mn|Mt))!==0&&(Le===null||!Le.has(n))&&To();let s=t?me(e):e;return Rt(n,s,It)}var $e=null,bs=0;function Rt(n,e,t=null){if(!n.equals(e)){qe?Ie.set(n,e):Ie.has(n)||Ie.set(n,n.v);var s=ln.ensure();if(s.capture(n,e),(n.f&W)!==0){const i=n;(n.f&C)!==0&&Os(i),V===null&&Rs(i)}n.wv=vr(),$e=null,bs=0,Xi(n,C,t),$e=null,Rn()&&T!==null&&(T.f&q)!==0&&(T.f&(Ce|Ye))===0&&(se===null?Xo([n]):se.push(n)),!s.is_fork&&Bt.size>0&&!Qi&&Ho()}return e}function Ho(){Qi=!1;for(const n of Bt){(n.f&q)!==0&&B(n,ne);let e;try{e=vt(n)}catch{e=!0}e&&Ln(n)}Bt.clear()}function Wd(n,e=1){var t=b(n),s=e===1?t++:t--;return L(n,t),s}function Gn(n){L(n,n.v+1)}function Xi(n,e,t){var s=n.reactions;if(s!==null){var i=Rn(),r=s.length;if(bs+=r,bs>1e5&&$e===null&&($e=new Set),$e!==null){if($e.has(n))return;$e.add(n)}for(var o=0;o<r;o++){var l=s[o],u=l.f;if(!(!i&&l===T)){var d=(u&C)===0;if(d&&B(l,e),(u&Mt)!==0)Bt.add(l);else if((u&W)!==0){var c=l;V==null||V.delete(c),Xi(c,ne,t)}else if(d){var h=l;(u&ve)!==0&&pe!==null&&pe.add(h),t!==null?t.push(h):zs(h)}}}}}function me(n){if(typeof n!="object"||n===null||Ae in n||Mi in n)return n;const e=Ds(n);if(e!==Jr&&e!==Qr)return n;var t=new Map,s=Kr(n),i=P(0),r=rn,o=l=>{if(rn===r)return l();var u=S,d=rn;ae(null),li(r);var c=l();return ae(u),li(d),c};return s&&t.set("length",P(n.length)),new Proxy(n,{defineProperty(l,u,d){(!("value"in d)||d.configurable===!1||d.enumerable===!1||d.writable===!1)&&So();var c=t.get(u);return c===void 0?o(()=>{var h=P(d.value);return t.set(u,h),h}):L(c,d.value,!0),!0},deleteProperty(l,u){var d=t.get(u);if(d===void 0){if(u in l){const c=o(()=>P(F));t.set(u,c),Gn(i)}}else L(d,F),Gn(i);return!0},get(l,u,d){var f;if(u===Ae)return n;var c=t.get(u),h=u in l;if(c===void 0&&(!h||(f=He(l,u))!=null&&f.writable)&&(c=o(()=>{var p=me(h?l[u]:F),v=P(p);return v}),t.set(u,c)),c!==void 0){var g=b(c);return g===F?void 0:g}return Reflect.get(l,u,d)},getOwnPropertyDescriptor(l,u){var g;(g=this.has)==null||g.call(this,l,u);var d=Reflect.getOwnPropertyDescriptor(l,u),c=t.get(u);if(c!==void 0){var h=b(c);if(h===F)return;if(d&&"value"in d)d.value=h;else return{enumerable:!0,configurable:!0,value:h,writable:!0}}return d},has(l,u){var g;if(u===Ae)return!0;var d=t.get(u),c=d!==void 0&&d.v!==F||Reflect.has(l,u);if(d!==void 0||(T!==null||S!==null)&&(!c||(g=He(l,u))!=null&&g.writable)){d===void 0&&(d=o(()=>{var f=c?me(l[u]):F,p=P(f);return p}),t.set(u,d));var h=b(d);if(h===F)return!1}return c},set(l,u,d,c){var K;var h=t.get(u),g=u in l;if(s&&u==="length")for(var f=d;f<h.v;f+=1){var p=t.get(f+"");p!==void 0?L(p,F):f in l&&(p=o(()=>P(F)),t.set(f+"",p))}if(h===void 0)(!g||(K=He(l,u))!=null&&K.writable)&&(h=o(()=>P(void 0)),L(h,me(d)),t.set(u,h));else{g=h.v!==F;var v=o(()=>me(d));L(h,v)}var _=Reflect.getOwnPropertyDescriptor(l,u);if(_!=null&&_.set&&_.set.call(c,d),!g){if(s&&typeof u=="string"){var I=t.get("length"),R=Number(u);Number.isInteger(R)&&R>=I.v&&L(I,R+1)}Gn(i)}return!0},ownKeys(l){b(i);var u=Reflect.ownKeys(l).filter(h=>{var g=t.get(h);return g===void 0||g.v!==F});for(var[d,c]of t)c.v!==F&&!(d in l)&&u.push(d);return u},setPrototypeOf(){Eo()}})}function si(n){try{if(n!==null&&typeof n=="object"&&Ae in n)return n[Ae]}catch{}return n}function Yd(n,e){return Object.is(si(n),si(e))}var ii,er,nr,tr;function Uo(){if(ii===void 0){ii=window,er=/Firefox/.test(navigator.userAgent);var n=Element.prototype,e=Node.prototype,t=Text.prototype;nr=He(e,"firstChild").get,tr=He(e,"nextSibling").get,Xs(n)&&(n[gs]=void 0,n[Tt]=null,n[vs]=void 0,n.__e=void 0),Xs(t)&&(t[qn]=void 0)}}function In(n=""){return document.createTextNode(n)}function we(n){return nr.call(n)}function Ht(n){return tr.call(n)}function he(n,e){return we(n)}function pn(n,e=!1){{var t=we(n);return t instanceof Comment&&t.data===""?Ht(t):t}}function Be(n,e=!1){return we(n)}function $(n,e=1,t=!1){let s=n;for(;e--;)s=Ht(s);return s}function Gd(n){n.textContent=""}function Wo(){return!1}function sr(n,e,t){return e==null||e===Bi?t?document.createElement(n,{is:t}):document.createElement(n):t?document.createElementNS(e,n,{is:t}):document.createElementNS(e,n)}function Yo(n){var e=T;if(e===null)return S.f|=Ue,n;if((e.f&Pn)===0&&(e.f&Tn)===0)throw n;Ee(n,e)}function Ee(n,e){if(n===Pi)throw n;if(!(e!==null&&(e.f&U)!==0)){for(;e!==null;){if((e.f&ps)!==0&&(e.f&(U|Zn))===0){if((e.f&Pn)===0)throw n;try{e.b.error(n);return}catch(t){n=t}}e=e.parent}throw n}}function ir(n){T===null&&(S===null&&bo(),ko()),qe&&_o()}function Go(n,e){var t=e.last;t===null?e.last=e.first=n:(t.next=n,n.prev=t,e.last=n)}function De(n,e){var t=T;t!==null&&(t.f&le)!==0&&(n|=le);var s={ctx:M,deps:null,nodes:null,f:n|C|ke,first:null,fn:e,last:null,next:null,parent:t,b:t&&t.b,prev:null,teardown:null,wv:0,ac:null};m==null||m.register_created_effect(s);var i=s;if((n&Tn)!==0)Yn!==null?Yn.push(s):ln.ensure().schedule(s);else if(e!==null){try{Ln(s)}catch(o){throw te(s),o}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&(i.f&Bn)===0&&(i=i.first,(n&ve)!==0&&(n&on)!==0&&i!==null&&(i.f|=on))}if(i!==null&&(i.parent=t,t!==null&&Go(i,t),S!==null&&(S.f&W)!==0&&(n&Ye)===0)){var r=S;(r.effects??(r.effects=[])).push(i)}return s}function qs(){return S!==null&&!_e}function rr(n){const e=De(dt,null);return B(e,q),e.teardown=n,e}function or(n){ir();var e=T.f,t=!S&&(e&Ce)!==0&&M!==null&&!M.i;if(t){var s=M;(s.e??(s.e=[])).push(n)}else return lr(n)}function lr(n){return De(Tn|Ci,n)}function Ko(n){return ir(),De(dt|Ci,n)}function $o(n){ln.ensure();const e=De(Ye|Bn,n);return(t={})=>new Promise(s=>{t.outro?Kn(e,()=>{te(e),s(void 0)}):(te(e),s(void 0))})}function Zo(n){return De(Tn,n)}function Jo(n){return De(mn|Bn,n)}function Ut(n,e=0){return De(dt|e,n)}function wn(n,e=[],t=[],s=[]){Bo(s,e,t,i=>{De(dt,()=>{n(...i.map(b))})})}function Wt(n,e=0){var t=De(ve|e,n);return t}function Re(n){return De(Ce|Bn,n)}function ar(n){var e=n.teardown;if(e!==null){const t=qe,s=S;oi(!0),ae(null);try{e.call(null)}catch(i){Ee(i,n.parent)}finally{oi(t),ae(s)}}}function js(n,e=!1){var t=n.first;for(n.first=n.last=null;t!==null;){const i=t.ac;i!==null&&Nn(()=>{i.abort(ct)});var s=t.next;(t.f&Ye)!==0?t.parent=null:te(t,e),t=s}}function Qo(n){for(var e=n.first;e!==null;){var t=e.next;(e.f&Ce)===0&&te(e),e=t}}function te(n,e=!0){var t=!1;(e||(n.f&no)!==0)&&n.nodes!==null&&n.nodes.end!==null&&(ur(n.nodes.start,n.nodes.end),t=!0),n.f|=Zn,js(n,e&&!t),Qn(n,0);var s=n.nodes&&n.nodes.t;if(s!==null)for(const r of s)r.stop();ar(n),n.f^=Zn,n.f|=U;var i=n.parent;i!==null&&i.first!==null&&dr(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.fn=n.nodes=n.ac=n.b=null}function ur(n,e){for(;n!==null;){var t=n===e?null:Ht(n);n.remove(),n=t}}function dr(n){var e=n.parent,t=n.prev,s=n.next;t!==null&&(t.next=s),s!==null&&(s.prev=t),e!==null&&(e.first===n&&(e.first=s),e.last===n&&(e.last=t))}function Kn(n,e,t=!0){var s=[];n.f|=Ps,cr(n,s,!0);var i=()=>{t&&te(n),e&&e()},r=s.length;if(r>0){var o=()=>--r||i();for(var l of s)l.out(o)}else i()}function cr(n,e,t){if((n.f&le)===0){n.f^=le;var s=n.nodes&&n.nodes.t;if(s!==null)for(const l of s)(l.is_global||t)&&e.push(l);for(var i=n.first;i!==null;){var r=i.next;if((i.f&Ye)===0){var o=(i.f&on)!==0||(i.f&Ce)!==0&&(n.f&ve)!==0;cr(i,e,o?t:!1)}i=r}}}function ri(n){n.f&=~Ps,hr(n,!0)}function hr(n,e){if((n.f&Ps)===0&&(n.f&le)!==0){n.f^=le,(n.f&q)===0&&(B(n,C),ln.ensure().schedule(n));for(var t=n.first;t!==null;){var s=t.next,i=(t.f&on)!==0||(t.f&Ce)!==0;hr(t,i?e:!1),t=s}var r=n.nodes&&n.nodes.t;if(r!==null)for(const o of r)(o.is_global||e)&&o.in()}}function fr(n,e){if(n.nodes)for(var t=n.nodes.start,s=n.nodes.end;t!==null;){var i=t===s?null:Ht(t);e.append(t),t=i}}let qe=!1;function oi(n){qe=n}let S=null,_e=!1;function ae(n){S=n}let T=null;function Me(n){T=n}let Le=null;function pr(n){S!==null&&((S.f&Dt)!==0||(S.f&W)!==0)&&(Le??(Le=new Set)).add(n)}let Z=null,ee=0,se=null;function Xo(n){se=n}let gr=1,Je=0,rn=Je;function li(n){rn=n}function vr(){return++gr}function vt(n){var e=n.f;if((e&C)!==0)return!0;if((e&ne)!==0){for(var t=n.deps,s=t.length,i=0;i<s;i++){var r=t[i];if(vt(r)&&Hi(r),r.wv>n.wv)return!0}(e&ke)!==0&&V===null&&B(n,q)}return!1}function mr(n,e,t=!0){var s=n.reactions;if(s!==null&&!(Le!==null&&Le.has(n)))for(var i=0;i<s.length;i++){var r=s[i];(r.f&W)!==0?mr(r,e,!1):e===r&&(t?B(r,C):(r.f&q)!==0&&B(r,ne),zs(r))}}function wr(n){var e=Z,t=ee,s=se,i=S,r=Le,o=M,l=_e,u=rn,d=n.f;Z=null,ee=0,se=null,S=(d&(Ce|Ye))===0?n:null,Le=null,An(n.ctx),_e=!1,rn=++Je,n.ac!==null&&(Nn(()=>{n.ac.abort(ct)}),n.ac=null);try{n.f|=Dt;var c=n.fn,h=c();n.f|=Pn;var g=ai(n);if(Rn()&&se!==null&&!_e&&g!==null&&(n.f&(W|ne|C))===0)for(var f=0;f<se.length;f++)mr(se[f],n);if(i!==null&&i!==n){if(Je++,i.deps!==null)for(let p=0;p<t;p+=1)i.deps[p].rv=Je;if(e!==null)for(const p of e)p.rv=Je;se!==null&&(s===null?s=se:s.push(...se))}return(n.f&Ue)!==0&&(n.f^=Ue),h}catch(p){return ai(n),Yo(p)}finally{n.f^=Dt,Z=e,ee=t,se=s,S=i,Le=r,An(o),_e=l,rn=u}}function ai(n){var i;var e=n.deps,t=m==null?void 0:m.is_fork;if(Z!==null){var s;if(t||Qn(n,ee),e!==null&&ee>0)for(e.length=ee+Z.length,s=0;s<Z.length;s++)e[ee+s]=Z[s];else n.deps=e=Z;if(qs()&&(n.f&ke)!==0)for(s=ee;s<e.length;s++)((i=e[s]).reactions??(i.reactions=[])).push(n)}else!t&&e!==null&&ee<e.length&&(Qn(n,ee),e.length=ee);return e}function el(n,e){let t=e.reactions;if(t!==null){var s=$r.call(t,n);if(s!==-1){var i=t.length-1;i===0?t=e.reactions=null:(t[s]=t[i],t.pop())}}if(t===null&&(e.f&W)!==0&&(Z===null||!$n.call(Z,e))){var r=e;(r.f&ke)!==0&&(r.f^=ke),r.v!==F&&Rs(r),r.ac!==null&&Nn(()=>{r.ac.abort(ct),r.ac=null,B(r,C)}),Fo(r),Qn(r,0)}}function Qn(n,e){var t=n.deps;if(t!==null)for(var s=e;s<t.length;s++)el(n,t[s])}function Ln(n){var e=n.f;if((e&U)===0){B(n,q);var t=T;T=n;try{(e&(ve|Li))!==0?Qo(n):js(n),ar(n);var s=wr(n);n.teardown=typeof s=="function"?s:null,n.wv=gr;var i;Ei&&Io&&(n.f&C)!==0&&n.deps}finally{T=t}}}async function nl(){await Promise.resolve(),jo()}function b(n){var e=n.f,t=(e&W)!==0;if(S!==null&&!_e){var s=T!==null&&(T.f&U)!==0;if(!s&&(Le===null||!Le.has(n))){var i=S.deps;if((S.f&Dt)!==0)n.rv<Je&&(n.rv=Je,Z===null&&i!==null&&i[ee]===n?ee++:Z===null?Z=[n]:Z.push(n));else{S.deps??(S.deps=[]),$n.call(S.deps,n)||S.deps.push(n);var r=n.reactions;r===null?n.reactions=[S]:$n.call(r,S)||r.push(S)}}}if(qe&&Ie.has(n))return Ie.get(n);if(t){var o=n;if(qe){var l=o.v;return((o.f&q)===0&&o.reactions!==null||kr(o))&&(l=Os(o)),Ie.set(o,l),l}var u=(o.f&ke)===0&&!_e&&S!==null&&(S.f&ke)!==0,d=(o.f&Pn)===0;vt(o)&&(u&&(o.f|=ke),Hi(o)),u&&!d&&(Ui(o),_r(o))}if(V!=null&&V.has(n))return V.get(n);if((n.f&Ue)!==0)throw n.v;return n.v}function _r(n){if(n.f|=ke,n.deps!==null)for(const t of n.deps){var e=t.reactions;e===null?t.reactions=[n]:$n.call(e,n)||e.push(n),(t.f&W)!==0&&(t.f&ke)===0&&(Ui(t),_r(t))}}function kr(n){if(n.v===F)return!0;if(n.deps===null)return!1;for(const e of n.deps)if(Ie.has(e)||(e.f&W)!==0&&kr(e))return!0;return!1}function mt(n){var e=_e;try{return _e=!0,n()}finally{_e=e}}function Kd(n){if(!(typeof n!="object"||!n||n instanceof EventTarget)){if(Ae in n)ys(n);else if(!Array.isArray(n))for(let e in n){const t=n[e];typeof t=="object"&&t&&Ae in t&&ys(t)}}}function ys(n,e=new Set){if(typeof n=="object"&&n!==null&&!(n instanceof EventTarget)&&!e.has(n)){e.add(n),n instanceof Date&&n.getTime();for(let s in n)try{ys(n[s],e)}catch{}const t=Ds(n);if(t!==Object.prototype&&t!==Array.prototype&&t!==Map.prototype&&t!==Set.prototype&&t!==Date.prototype){const s=Ai(t);for(let i in s){const r=s[i].get;if(r)try{r.call(n)}catch{}}}}}const tl=["touchstart","touchmove"];function sl(n){return tl.includes(n)}const Qe=Symbol("events"),br=new Set,xs=new Set;function il(n,e,t,s={}){function i(r){if(s.capture||Ss.call(e,r),!r.cancelBubble)return Nn(()=>t==null?void 0:t.call(this,r))}return n.startsWith("pointer")||n.startsWith("touch")||n==="wheel"?(i.__removed=!1,ze(()=>{i.__removed||e.addEventListener(n,i,s)})):e.addEventListener(n,i,s),i}function ts(n,e,t,s,i){var r={capture:s,passive:i},o=il(n,e,t,r);(e===document.body||e===window||e===document||e instanceof HTMLMediaElement)&&rr(()=>{o.__removed=!0,e.removeEventListener(n,o,r)})}function bt(n,e,t){(e[Qe]??(e[Qe]={}))[n]=t}function rl(n){for(var e=0;e<n.length;e++)br.add(n[e]);for(var t of xs)t(n)}let ss=null,is=!1;function Ss(n){var v,_;var e=this,t=e.ownerDocument,s=n.type,i=((v=n.composedPath)==null?void 0:v.call(n))||[],r=i[0]||n.target;ss=n,is||(is=!0,setTimeout(()=>{is=!1,ss=null}));var o=0,l=ss===n&&n[Qe];if(l){var u=i.indexOf(l);if(u!==-1&&(e===document||e===window)){n[Qe]=e;return}var d=i.indexOf(e);if(d===-1)return;u<=d&&(o=u)}if(r=i[o]||n.target,r!==e){Ti(n,"currentTarget",{configurable:!0,get(){return r||t}});var c=S,h=T;ae(null),Me(null);try{for(var g,f=[];r!==null&&r!==e;){try{var p=(_=r[Qe])==null?void 0:_[s];p!=null&&(!r.disabled||n.target===r)&&p.call(r,n)}catch(I){g?f.push(I):g=I}if(n.cancelBubble)break;o++,r=o<i.length?i[o]:null}if(g){for(let I of f)queueMicrotask(()=>{throw I});throw g}}finally{n[Qe]=e,delete n.currentTarget,ae(c),Me(h)}}}var xi;const rs=((xi=globalThis==null?void 0:globalThis.window)==null?void 0:xi.trustedTypes)&&globalThis.window.trustedTypes.createPolicy("svelte-trusted-html",{createHTML:n=>n});function ol(n){return(rs==null?void 0:rs.createHTML(n))??n}function yr(n){var e=sr("template");return e.innerHTML=ol(n.replaceAll("<!>","<!---->")),e.content}function an(n,e){var t=T;t.nodes===null&&(t.nodes={start:n,end:e,a:null,t:null})}function un(n,e){var t=(e&uo)!==0,s=(e&co)!==0,i,r=!n.startsWith("<!>");return()=>{i===void 0&&(i=yr(r?n:"<!>"+n),t||(i=we(i)));var o=s||er?document.importNode(i,!0):i.cloneNode(!0);if(t){var l=we(o),u=o.lastChild;an(l,u)}else an(o,o);return o}}function ll(n,e,t="svg"){var s=!n.startsWith("<!>"),i=`<${t}>${s?n:"<!>"+n}</${t}>`,r;return()=>{if(!r){var o=yr(i),l=we(o);r=we(l)}var u=r.cloneNode(!0);return an(u,u),u}}function $d(n,e){return ll(n,e,"svg")}function Zd(n=""){{var e=In(n+"");return an(e,e),e}}function zn(){var n=document.createDocumentFragment(),e=document.createComment(""),t=In();return n.append(e,t),an(e,t),n}function oe(n,e){n!==null&&n.before(e)}function Jd(){var n;return(n=window.__svelte??(window.__svelte={})).uid??(n.uid=1),`c${window.__svelte.uid++}`}function al(n){let e=0,t=gt(0),s;return()=>{qs()&&(b(t),Ut(()=>(e===0&&(s=mt(()=>n(()=>Gn(t)))),e+=1,()=>{ze(()=>{e-=1,e===0&&(s==null||s(),s=void 0,Gn(t))})})))}}var ul=on|Bn;function dl(n,e,t,s){new cl(n,e,t,s)}var ie,Ms,re,nn,H,J,Y,Q,xe,tn,Ve,En,et,Fe,jt,A,hl,fl,Es,je,pl,Ts,Un,Lt,As,Is;class cl{constructor(e,t,s,i){k(this,A);y(this,"parent");y(this,"is_pending",!1);y(this,"transform_error");k(this,ie);k(this,Ms,null);k(this,re);k(this,nn);k(this,H);k(this,J,null);k(this,Y,null);k(this,Q,null);k(this,xe,null);k(this,tn,0);k(this,Ve,0);k(this,En,!1);k(this,et,new Map);k(this,Fe,null);k(this,jt,al(()=>(x(this,Fe,gt(a(this,tn))),()=>{x(this,Fe,null)})));var r;x(this,ie,e),x(this,re,t),x(this,nn,o=>{var l=T;l.b=this,l.f|=ps,s(o)}),this.parent=T.b,this.transform_error=i??((r=this.parent)==null?void 0:r.transform_error)??(o=>o),x(this,H,Wt(()=>{w(this,A,Ts).call(this)},ul))}defer_effect(e){zi(e,a(this,et))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!a(this,re).pending}update_pending_count(e,t){w(this,A,As).call(this,e,t),x(this,tn,a(this,tn)+e),!(!a(this,Fe)||a(this,En))&&(x(this,En,!0),ze(()=>{x(this,En,!1),a(this,Fe)&&Rt(a(this,Fe),a(this,tn))}))}get_effect_pending(){return a(this,jt).call(this),b(a(this,Fe))}error(e){if(e===Pi||!a(this,re).onerror&&!a(this,re).failed)throw e;m!=null&&m.is_fork?(a(this,J)&&m.skip_effect(a(this,J)),a(this,Y)&&m.skip_effect(a(this,Y)),a(this,Q)&&m.skip_effect(a(this,Q)),m.oncommit(()=>{w(this,A,je).call(this)||w(this,A,Is).call(this,e)})):w(this,A,Is).call(this,e)}}ie=new WeakMap,Ms=new WeakMap,re=new WeakMap,nn=new WeakMap,H=new WeakMap,J=new WeakMap,Y=new WeakMap,Q=new WeakMap,xe=new WeakMap,tn=new WeakMap,Ve=new WeakMap,En=new WeakMap,et=new WeakMap,Fe=new WeakMap,jt=new WeakMap,A=new WeakSet,hl=function(){try{x(this,J,Re(()=>a(this,nn).call(this,a(this,ie))))}catch(e){this.error(e)}},fl=function(e){const t=a(this,re).failed,{reset:s,invoke_onerror:i}=w(this,A,Es).call(this,e);ze(i),t&&x(this,Q,Re(()=>{t(a(this,ie),()=>e,()=>s)}))},Es=function(e){var t=!1,s=!1;const i=()=>{if(!w(this,A,je).call(this)){if(t){go();return}t=!0,s&&Ao(),a(this,Q)!==null&&Kn(a(this,Q),()=>{x(this,Q,null)}),w(this,A,Lt).call(this,()=>{w(this,A,Ts).call(this)})}};return{reset:i,invoke_onerror:()=>{var o,l;if(!w(this,A,je).call(this))try{s=!0,(l=(o=a(this,re)).onerror)==null||l.call(o,e,i),s=!1}catch(u){Ee(u,a(this,H)&&a(this,H).parent)}}}},je=function(){return(a(this,H).f&(U|Zn))!==0},pl=function(){const e=a(this,re).pending;e&&(this.is_pending=!0,x(this,Y,Re(()=>e(a(this,ie)))),ze(()=>{if(!w(this,A,je).call(this)){var t=x(this,xe,document.createDocumentFragment()),s=In(),i=!1;if(t.append(s),x(this,J,w(this,A,Lt).call(this,()=>{try{return Re(()=>a(this,nn).call(this,s))}catch(r){try{this.error(r),i=!0}catch(o){Ee(o,a(this,H).parent)}return null}})),a(this,J)===null){x(this,xe,null),i&&w(this,A,Un).call(this,m);return}a(this,Ve)===0&&(a(this,ie).before(t),x(this,xe,null),Kn(a(this,Y),()=>{x(this,Y,null)}),w(this,A,Un).call(this,m))}}))},Ts=function(){try{if(this.is_pending=this.has_pending_snippet(),x(this,Ve,0),x(this,tn,0),x(this,J,Re(()=>{a(this,nn).call(this,a(this,ie))})),a(this,Ve)>0){var e=x(this,xe,document.createDocumentFragment());fr(a(this,J),e);const t=a(this,re).pending;x(this,Y,Re(()=>t(a(this,ie))))}else w(this,A,Un).call(this,m)}catch(t){this.error(t)}},Un=function(e){this.is_pending=!1,e.transfer_reactions(a(this,et))},Lt=function(e){var t=T,s=S,i=M;Me(a(this,H)),ae(a(this,H)),An(a(this,H).ctx);try{return ln.ensure(),e()}finally{Me(t),ae(s),An(i)}},As=function(e,t){var s;if(!this.has_pending_snippet()){this.parent&&w(s=this.parent,A,As).call(s,e,t);return}x(this,Ve,a(this,Ve)+e),a(this,Ve)===0&&(w(this,A,Un).call(this,t),a(this,Y)&&Kn(a(this,Y),()=>{x(this,Y,null)}),a(this,xe)&&(a(this,ie).before(a(this,xe)),x(this,xe,null)))},Is=function(e){a(this,J)&&(te(a(this,J)),x(this,J,null)),a(this,Y)&&(te(a(this,Y)),x(this,Y,null)),a(this,Q)&&(te(a(this,Q)),x(this,Q,null));let t=a(this,re).failed;const s=i=>{if(w(this,A,je).call(this))return;const{reset:r,invoke_onerror:o}=w(this,A,Es).call(this,i);o(),t&&!w(this,A,je).call(this)&&x(this,Q,w(this,A,Lt).call(this,()=>{try{return Re(()=>{var l=T;l.b=this,l.f|=ps,t(a(this,ie),()=>i,()=>r)})}catch(l){return Ee(l,a(this,H).parent),null}}))};ze(()=>{if(!w(this,A,je).call(this)){var i;try{i=this.transform_error(e)}catch(r){Ee(r,a(this,H)&&a(this,H).parent);return}i!==null&&typeof i=="object"&&typeof i.then=="function"?i.then(s,r=>Ee(r,a(this,H)&&a(this,H).parent)):s(i)}})};function fe(n,e){var t=e==null?"":typeof e=="object"?`${e}`:e;t!==(n[qn]??(n[qn]=n.nodeValue))&&(n[qn]=t,n.nodeValue=`${t}`)}function gl(n,e){return vl(n,e)}const yt=new Map;function vl(n,{target:e,anchor:t,props:s={},events:i,context:r,intro:o=!0,transformError:l}){Uo();var u=void 0,d=$o(()=>{var c=t??e.appendChild(In());dl(c,{pending:()=>{}},f=>{ft({});var p=M;r&&(p.c=r),i&&(s.$$events=i),u=n(f,s)||Bs(),pt()},l);var h=new Set,g=f=>{for(var p=0;p<f.length;p++){var v=f[p];if(!h.has(v)){h.add(v);var _=sl(v);for(const K of[e,document]){var I=yt.get(K);I===void 0&&(I=new Map,yt.set(K,I));var R=I.get(v);R===void 0?(K.addEventListener(v,Ss,{passive:_}),I.set(v,1)):I.set(v,R+1)}}}};return g(Zr(br)),xs.add(g),()=>{var _;for(var f of h)for(const I of[e,document]){var p=yt.get(I),v=p.get(f);--v==0?(I.removeEventListener(f,Ss),p.delete(f),p.size===0&&yt.delete(I)):p.set(f,v)}xs.delete(g),c!==t&&((_=c.parentNode)==null||_.removeChild(c))}});return Ls.set(u,d),u}let Ls=new WeakMap;function Qd(n,e){const t=Ls.get(n);return t?(Ls.delete(n),t(e)):Promise.resolve()}var ge,Se,X,sn,nt,tt,Vt;class Vs{constructor(e,t=!0){y(this,"anchor");k(this,ge,new Map);k(this,Se,new Map);k(this,X,new Map);k(this,sn,new Set);k(this,nt,!0);k(this,tt,e=>{if(a(this,ge).has(e)){var t=a(this,ge).get(e),s=a(this,Se).get(t);if(s)ri(s),a(this,sn).delete(t);else{var i=a(this,X).get(t);i&&(ri(i.effect),a(this,Se).set(t,i.effect),a(this,X).delete(t),i.fragment.lastChild.remove(),this.anchor.before(i.fragment),s=i.effect)}for(const[r,o]of a(this,ge)){if(a(this,ge).delete(r),r===e)break;const l=a(this,X).get(o);l&&(te(l.effect),a(this,X).delete(o))}for(const[r,o]of a(this,Se)){if(r===t||a(this,sn).has(r))continue;const l=()=>{if(Array.from(a(this,ge).values()).includes(r)){var d=document.createDocumentFragment();fr(o,d),d.append(In()),a(this,X).set(r,{effect:o,fragment:d})}else te(o);a(this,sn).delete(r),a(this,Se).delete(r)};a(this,nt)||!s?(a(this,sn).add(r),Kn(o,l,!1)):l()}}});k(this,Vt,e=>{a(this,ge).delete(e);const t=Array.from(a(this,ge).values());for(const[s,i]of a(this,X))t.includes(s)||(te(i.effect),a(this,X).delete(s))});this.anchor=e,x(this,nt,t)}ensure(e,t){var s=m,i=Wo();if(t&&!a(this,Se).has(e)&&!a(this,X).has(e))if(i){var r=document.createDocumentFragment(),o=In();r.append(o),a(this,X).set(e,{effect:Re(()=>t(o)),fragment:r})}else a(this,Se).set(e,Re(()=>t(this.anchor)));if(a(this,ge).set(s,e),i){for(const[l,u]of a(this,Se))l===e?s.unskip_effect(u):s.skip_effect(u);for(const[l,u]of a(this,X))l===e?s.unskip_effect(u.effect):s.skip_effect(u.effect);s.oncommit(a(this,tt)),s.ondiscard(a(this,Vt))}else a(this,tt).call(this,s)}}ge=new WeakMap,Se=new WeakMap,X=new WeakMap,sn=new WeakMap,nt=new WeakMap,tt=new WeakMap,Vt=new WeakMap;function Nt(n,e,t=!1){var s=new Vs(n),i=t?on:0;function r(o,l){s.ensure(o,l)}Wt(()=>{var o=!1;e((l,u=0)=>{o=!0,r(u,l)}),o||r(-1,null)},i)}const ml=Symbol("NaN");function ui(n,e,t){var s=new Vs(n),i=!Rn();Wt(()=>{var r=e();r!==r&&(r=ml),i&&r!==null&&typeof r=="object"&&(r={}),s.ensure(r,t)})}function wl(n,e,t=!1,s=!1,i=!1,r=!1){var o=n,l="";if(t)var u=n;wn(()=>{var d=T;if(l!==(l=e()??"")){if(t){d.nodes=null,u.innerHTML=l,l!==""&&an(we(u),u.lastChild);return}if(d.nodes!==null&&(ur(d.nodes.start,d.nodes.end),d.nodes=null),l!==""){var c=s?ho:i?fo:void 0,h=sr(s?"svg":i?"math":"template",c);h.innerHTML=l;var g=s||i?h:h.content;if(an(we(g),g.lastChild),s||i)for(;we(g);)o.before(we(g));else o.before(g)}}})}function os(n,e,t){var s=new Vs(n);Wt(()=>{var i=e()??null;s.ensure(i,i&&(r=>t(r,i)))},on)}function xr(n){var e,t,s="";if(typeof n=="string"||typeof n=="number")s+=n;else if(typeof n=="object")if(Array.isArray(n)){var i=n.length;for(e=0;e<i;e++)n[e]&&(t=xr(n[e]))&&(s&&(s+=" "),s+=t)}else for(t in n)n[t]&&(s&&(s+=" "),s+=t);return s}function _l(){for(var n,e,t=0,s="",i=arguments.length;t<i;t++)(n=arguments[t])&&(e=xr(n))&&(s&&(s+=" "),s+=e);return s}function Xd(n){return typeof n=="object"?_l(n):n??""}const di=[...` 	
\r\f \v\uFEFF`];function kl(n,e,t){var s=n==null?"":""+n;if(e&&(s=s?s+" "+e:e),t){for(var i of Object.keys(t))if(t[i])s=s?s+" "+i:i;else if(s.length)for(var r=i.length,o=0;(o=s.indexOf(i,o))>=0;){var l=o+r;(o===0||di.includes(s[o-1]))&&(l===s.length||di.includes(s[l]))?s=(o===0?"":s.substring(0,o))+s.substring(l+1):o=l}}return s===""?null:s}function ci(n,e=!1){var t=e?" !important;":";",s="";for(var i of Object.keys(n)){var r=n[i];r!=null&&r!==""&&(s+=" "+i+": "+r+t)}return s}function bl(n,e){if(e){var t="",s,i;return Array.isArray(e)?(s=e[0],i=e[1]):s=e,s&&(t+=ci(s)),i&&(t+=ci(i,!0)),t=t.trim(),t===""?null:t}return String(n)}function yl(n,e,t,s,i,r){var o=n[gs];if(o!==t||o===void 0){var l=kl(t,s,r);l==null?n.removeAttribute("class"):n.className=l,n[gs]=t}else if(r&&i!==r)for(var u in r){var d=!!r[u];(i==null||d!==!!i[u])&&n.classList.toggle(u,d)}return r}function ls(n,e={},t,s){for(var i in t){var r=t[i];e[i]!==r&&(t[i]==null?n.style.removeProperty(i):n.style.setProperty(i,r,s))}}function Sr(n,e,t,s){var i=n[vs];if(i!==e){var r=bl(e,s);r==null?n.removeAttribute("style"):n.style.cssText=r,n[vs]=e}else s&&(Array.isArray(s)?(ls(n,t==null?void 0:t[0],s[0]),ls(n,t==null?void 0:t[1],s[1],"important")):ls(n,t,s));return s}const xl=Symbol("is custom element"),Sl=Symbol("is html"),El=so?"progress":"PROGRESS";function ec(n,e){var t=Hs(n);t.value===(t.value=e??void 0)||n.value===e&&(e!==0||n.nodeName!==El)||(n.value=e??"")}function nc(n,e){var t=Hs(n);t.checked!==(t.checked=e??void 0)&&(n.checked=e)}function hi(n,e,t,s){var i=Hs(n),r=i[e];r!==(i[e]=t)&&(e==="loading"&&(n[to]=t),t==null?n.removeAttribute(e):typeof t!="string"&&Tl(n).has(e)?n[e]=t:n.setAttribute(e,t))}function Hs(n){return n[Tt]??(n[Tt]={[xl]:n.nodeName.includes("-"),[Sl]:n.namespaceURI===Bi})}var fi=new Map;function Tl(n){var e=n.getAttribute("is")||n.nodeName,t=fi.get(e);if(t)return t;fi.set(e,t=new Set);for(var s,i=n,r=Element.prototype;r!==i;){s=Ai(i);for(var o in s)s[o].set&&o!=="innerHTML"&&o!=="textContent"&&o!=="innerText"&&t.add(o);i=Ds(i)}return t}function pi(n,e,t=e){var s=new WeakSet;qi(n,"input",async i=>{var r=i?n.defaultValue:n.value;if(r=as(n)?us(r):r,t(r),m!==null&&s.add(m),await nl(),r!==(r=e())){var o=n.selectionStart,l=n.selectionEnd,u=n.value.length;if(n.value=r??"",l!==null){var d=n.value.length;o===l&&l===u&&d>u?(n.selectionStart=d,n.selectionEnd=d):(n.selectionStart=o,n.selectionEnd=Math.min(l,d))}}}),mt(e)==null&&n.value&&(t(as(n)?us(n.value):n.value),m!==null&&s.add(m)),Ut(()=>{var i=e();if(n===document.activeElement){var r=m;if(s.has(r))return}as(n)&&i===us(n.value)||n.type==="date"&&!i&&!n.value||i!==n.value&&(n.value=i??"")})}function tc(n,e,t=e){qi(n,"change",s=>{var i=s?n.defaultChecked:n.checked;t(i)}),mt(e)==null&&t(n.checked),Ut(()=>{var s=e();n.checked=!!s})}function as(n){var e=n.type;return e==="number"||e==="range"}function us(n){return n===""?null:+n}function ds(n,e){return n===e||(n==null?void 0:n[Ae])===e}function Al(n=Bs(),e,t,s){var i=M.r,r=T;return Zo(()=>{var o,l;return Ut(()=>{o=l,l=[],mt(()=>{ds(t(...l),n)||(e(n,...l),o&&ds(t(...o),n)&&e(null,...o))})}),()=>{let u=r;for(;u!==i&&u.parent!==null&&u.parent.f&Zn;)u=u.parent;const d=()=>{l&&ds(t(...l),n)&&e(null,...l)},c=u.teardown;u.teardown=()=>{d(),c==null||c()}}}),n}let xt=!1;function Il(n){var e=xt;try{return xt=!1,[n(),xt]}finally{xt=e}}const Ll={get(n,e){let t=n.props.length;for(;t--;){let s=n.props[t];if(Fn(s)&&(s=s()),typeof s=="object"&&s!==null&&e in s)return s[e]}},set(n,e,t){let s=n.props.length;for(;s--;){let i=n.props[s];Fn(i)&&(i=i());const r=He(i,e);if(r&&r.set)return r.set(t),!0}return!1},getOwnPropertyDescriptor(n,e){let t=n.props.length;for(;t--;){let s=n.props[t];if(Fn(s)&&(s=s()),typeof s=="object"&&s!==null&&e in s){const i=He(s,e);return i&&!i.configurable&&(i.configurable=!0),i}}},has(n,e){if(e===Ae||e===Di)return!1;for(let t of n.props)if(Fn(t)&&(t=t()),t!=null&&e in t)return!0;return!1},ownKeys(n){const e=[];for(let t of n.props)if(Fn(t)&&(t=t()),!!t){for(const s in t)e.includes(s)||e.push(s);for(const s of Object.getOwnPropertySymbols(t))e.includes(s)||e.push(s)}return e}};function cs(...n){return new Proxy({props:n},Ll)}function Cl(n,e,t,s){var Pe;var i=!ht||(t&ro)!==0,r=(t&lo)!==0,o=(t&ao)!==0,l=s,u=!0,d=void 0,c=()=>o&&i?(d??(d=Jn(s)),b(d)):(u&&(u=!1,l=o?mt(s):s),l);let h;if(r){var g=Ae in n||Di in n;h=((Pe=He(n,e))==null?void 0:Pe.set)??(g&&e in n?D=>n[e]=D:void 0)}var f,p=!1;r?[f,p]=Il(()=>n[e]):f=n[e],f===void 0&&s!==void 0&&(f=c(),h&&(i&&xo(),h(f)));var v;if(i?v=()=>{var D=n[e];return D===void 0?c():(u=!0,D)}:v=()=>{var D=n[e];return D!==void 0&&(l=void 0),D===void 0?l:D},i&&(t&oo)===0)return v;if(h){var _=n.$$legacy;return(function(D,ue){return arguments.length>0?((!i||!ue||_||p)&&h(ue?v():D),D):v()})}var I=!1,R=((t&io)!==0?Jn:Vi)(()=>(I=!1,v()));r&&b(R);var K=T;return(function(D,ue){if(arguments.length>0){const de=ue?b(R):i&&r?me(D):D;return L(R,de),I=!0,l!==void 0&&(l=de),D}return qe&&I||(K.f&U)!==0?R.v:b(R)})}const Ml="5";var Si;typeof window<"u"&&((Si=window.__svelte??(window.__svelte={})).v??(Si.v=new Set)).add(Ml);class Xe{constructor(e,t={},s={}){y(this,"url");y(this,"state");y(this,"scrollY");y(this,"index");y(this,"origin");this.url=e,this.state=t,this.origin=(s==null?void 0:s.origin)??"manual",this.scrollY=(s==null?void 0:s.scrollY)??0,this.index=(s==null?void 0:s.index)??0}static fromCurrent(){const e=window.history.state;return new Xe(new URL(window.location.href),(e==null?void 0:e.state)??null,{origin:"manual",scrollY:(e==null?void 0:e.scrollY)??window.scrollY,index:(e==null?void 0:e.index)??0})}get pathname(){const e=this.url.pathname;return e.endsWith("/")&&e.length>1?e.substring(0,e.length-1):e}get uri(){return this.pathname+this.url.search+this.url.hash}get search(){return this.url.searchParams}toHistoryState(){return{state:this.state,scrollY:this.scrollY,index:this.index}}clone(){let e=null;return this.state!==null&&(typeof this.state.clone=="function"?e=this.state.clone():typeof this.state=="object"?e={...this.state}:e=this.state),new Xe(new URL(this.url.toString()),e,{origin:this.origin,scrollY:this.scrollY,index:this.index})}}class Dl{constructor(e,t){y(this,"uri");y(this,"isRegex");y(this,"loadComp");if(this.uri=e,this.isRegex=typeof this.uri!="string",this.loadComp=t,typeof this.uri=="string"&&this.uri.length>1&&this.uri.endsWith("/")&&(this.uri=this.uri.substring(0,this.uri.length-1)),this.isRegex&&!(e instanceof RegExp))throw new Error("expected a regex as uri")}check(e){const t=e.pathname;if(!this.isRegex)return this.uri===t;const s=t.match(this.uri);return!!s&&s[0]===t}toRegexProps(e){if(!this.isRegex)return{};const t=e.pathname.match(this.uri);return(t==null?void 0:t.groups)??{}}toSearchProps(e){return Object.fromEntries(e.url.searchParams.entries())}toProps(e){return{...this.toRegexProps(e),...this.toSearchProps(e)}}async load(e){return await this.loadComp(e)}}class Pl{constructor(){y(this,"inner");this.inner=new Set}add(e){const t=this.inner;return t.add(e),()=>{t.delete(e)}}trigger(...e){this.inner.forEach(t=>t(...e))}clearAndTrigger(...e){const t=this.inner;this.clear(),t.forEach(s=>s(...e))}clear(){this.inner=new Set}}class gi{constructor(e){y(this,"inner");y(this,"listeners");this.inner=e,this.listeners=new Pl}subscribe(e){return e(this.inner),this.listeners.add(e)}get(){return this.inner}set(e){this.inner=e,this.listeners.trigger(e)}setSilent(e){this.inner=e}notify(){this.listeners.trigger(this.inner)}}class vi{constructor(){y(this,"listeners");y(this,"lastValue");y(this,"open");this.listeners=[],this.lastValue=null,this.open=!1}isOpen(){return this.open}add(){if(this.open)throw new Error("Barrier is already open");const e=this.listeners.length,t={ready:!1,resolve:()=>{}},s=new Promise(i=>{t.resolve=i});return this.listeners[e]=t,{ready:async i=>{if(this.open)throw new Error("Barrier is already open");return this.lastValue=i,t.ready=!0,this._maybeTrigger(),await s},remove:()=>{if(this.open)throw new Error("Barrier is already open");this.listeners[e]=null,this._maybeTrigger()}}}_maybeTrigger(){this.listeners.every(t=>t===null||t.ready)&&(this.listeners.forEach(t=>t==null?void 0:t.resolve(this.lastValue)),this.listeners=[],this.open=!0)}}async function mi(n,e){await e.dataReady()||setTimeout(()=>{e.domReady()},100)}class Bl{constructor(){y(this,"routes");y(this,"newRequest");y(this,"requestListener");y(this,"newRequestVersion");y(this,"scrollDebounceTimeout");y(this,"currentRequest");this.routes=[],this.newRequest=new gi(null),this.requestListener=mi,this.newRequestVersion=0,this.scrollDebounceTimeout=null,this.currentRequest=new gi(null)}register(e,t){return this.registerRoute(new Dl(e,t))}registerRoute(e){return this.routes.push(e),e}onRequest(e){return this.requestListener=e,()=>{this.requestListener=mi}}onRoute(e){return this.onRequest((t,s)=>{const i=this.route(t);e(t,i,s)})}initClient(){const e=Xe.fromCurrent();return this.openReq(e,{history:"none",checkCurrent:!1}),this._listen(),window.history.scrollRestoration="manual",e}initServer(e){const t=new Xe(new URL(e));return this.newRequest.set(t),this.currentRequest.set(t),t}replaceState(e={}){this.currentRequest.get().state=e,window.history.replaceState(this.currentRequest.get().toHistoryState(),""),this.currentRequest.notify()}pushReq(e){this.currentRequest.setSilent(e),window.history.pushState(e.toHistoryState(),"",St(e)),this.currentRequest.notify()}replaceReq(e){this.currentRequest.setSilent(e),window.history.replaceState(e.toHistoryState(),"",St(e)),this.currentRequest.notify()}canGoBack(){const e=this.currentRequest.get();return e?e.index>0:!1}back(){return window.history.back()}reload(){const e=this.currentRequest.get();if(!e)throw new Error("router does not have a current request");this.openReq(e.clone(),{history:"replace",checkCurrent:!1})}route(e){return this.routes.find(t=>t.check(e))??null}async openReq(e,t={}){const s=(t==null?void 0:t.history)??"push",i=(t==null?void 0:t.checkCurrent)??!0,r=e.uri,o=this.currentRequest.get();if(i&&(o==null?void 0:o.uri)===r)return;s==="push"&&o&&(e.index=o.index+1);const l=++this.newRequestVersion,u=()=>this.newRequestVersion!==l,d=new vi,c=d.add(),h=d.add(),g=new vi,f=g.add(),p=g.add();if(this.requestListener(e,{dataReady:async()=>await c.ready(u()),domReady:async()=>await f.ready(u())}),this.newRequest.set(e),await h.ready(u())||(this.scrollDebounceTimeout&&(clearTimeout(this.scrollDebounceTimeout),this.scrollDebounceTimeout=null),o&&e.origin!=="pop"&&(o.scrollY=window.scrollY,window.history.replaceState(o.toHistoryState(),"")),s==="replace"?window.history.replaceState(e.toHistoryState(),"",St(e)):s==="push"&&window.history.pushState(e.toHistoryState(),"",St(e)),this.currentRequest.set(e),await p.ready(u())))return console.log("request overriden, version changed");window.scrollTo({top:e.scrollY})}open(e,t,s={}){s!=null&&s.origin||(s.origin="manual");const i=this._urlToRequest(e,t,s);i&&this.openReq(i,s)}_listen(){window.addEventListener("click",e=>{const t=e.target.closest("a");if(!Rl(e,t))return;const s=this._urlToRequest(t.href,{},{origin:"click"});if(!s)return;e.preventDefault();const i=this.currentRequest.get();if(i&&s.url.hash&&i.pathname===s.pathname&&i.url.search===s.url.search){this._openHash(s,i);return}this.openReq(s)}),window.addEventListener("popstate",e=>{e.preventDefault();const t=Xe.fromCurrent();t.origin="pop",this.openReq(t,{history:"none",checkCurrent:!1})}),window.addEventListener("scroll",()=>{const e=this.currentRequest.get();e&&(e.scrollY=window.scrollY,!this.scrollDebounceTimeout&&(this.scrollDebounceTimeout=setTimeout(()=>{this.scrollDebounceTimeout=null,e.uri===this.currentRequest.get().uri&&window.history.replaceState(e.toHistoryState(),"")},200)))})}_openHash(e,t){e.uri!==t.uri&&(this.scrollDebounceTimeout&&(clearTimeout(this.scrollDebounceTimeout),this.scrollDebounceTimeout=null),t.scrollY=window.scrollY,window.history.replaceState(t.toHistoryState(),""),e.index=t.index+1,e.scrollY=window.scrollY,this.pushReq(e));const s=document.getElementById(e.url.hash.substring(1));s==null||s.scrollIntoView({behavior:"smooth"})}_urlToRequest(e,t={},s={}){const i=window.location;e.startsWith("/")&&(e=i.protocol+"//"+i.host+e);let r;try{r=new URL(e)}catch(o){return console.log("invalid url",o),null}return r.protocol!==i.protocol||r.host!==i.host?null:new Xe(r,t,s)}}function St(n){return window.location.origin+n.uri}function Rl(n,e){const t=n.metaKey||n.ctrlKey||n.shiftKey,s=n.altKey;return!(!e||!e.href||t||s||e.target.toLowerCase()==="_blank"||e.hasAttribute("download")||!e.href.startsWith("http"))}const Cn="zipp-token-before";function Nl(){return localStorage.getItem(Cn)??sessionStorage.getItem(Cn)}function Ol(){localStorage.removeItem(Cn),sessionStorage.removeItem(Cn)}var st,it,rt,ot,lt;class Fl{constructor(){k(this,st,P(null));k(this,it,P(null));k(this,rt,P(null));k(this,ot,P(null));k(this,lt,P(!1));y(this,"remember",!0);y(this,"renewed",new Set)}get token(){return b(a(this,st))}set token(e){L(a(this,st),e,!0)}get expiresAt(){return b(a(this,it))}set expiresAt(e){L(a(this,it),e,!0)}get user(){return b(a(this,rt))}set user(e){L(a(this,rt),e,!0)}get impersonatedBy(){return b(a(this,ot))}set impersonatedBy(e){L(a(this,ot),e,!0)}get expired(){return b(a(this,lt))}set expired(e){L(a(this,lt),e,!0)}get loggedIn(){return this.user!==null}async init(){if(Ot())try{this.set(await G("GET","/auth/session"))}catch(e){if(!(e instanceof Yt)||!Mr(e))throw e;this.invalidate()}}async login(e,t,s=!0){const i=await G("POST","/auth/login",{email:e,password:t});this.use(i,s)}use(e,t=!0){this.set(e),Et(e.session.token,t)}async loginAs(e){const t=await G("POST",`/auth/links/${e}/login-as`),s=Ot(),i=s!==null&&_i();s&&!this.impersonatedBy&&(i?localStorage:sessionStorage).setItem(Cn,s),this.use(t,i)}expire(){this.expired||(this.remember=_i(),this.expired=!0,Et(null))}async renew(e,t){var r;const s=(r=this.user)==null?void 0:r.id,i=await G("POST","/auth/login",{email:e,password:t});if(this.use(i,this.remember),this.expired=!1,i.user.id!==s)return!1;for(const o of this.renewed)o();return!0}onRenewed(e){return this.renewed.add(e),()=>this.renewed.delete(e)}async endLoginAs(){await G("POST","/auth/logout").catch(()=>{});const e=Nl(),t=localStorage.getItem(Cn)!==null;return this.invalidate(),e?(Et(e,t),await this.init(),this.loggedIn):!1}async logout(){await G("POST","/auth/logout").catch(()=>{}),this.invalidate()}invalidate(){this.token=null,this.expiresAt=null,this.user=null,this.impersonatedBy=null,this.expired=!1,Et(null),Ol()}set(e){this.token=e.session.token,this.expiresAt=new Date(e.session.expiresAt),this.user=e.user,this.impersonatedBy=e.session.impersonatedBy}}st=new WeakMap,it=new WeakMap,rt=new WeakMap,ot=new WeakMap,lt=new WeakMap;const z=new Fl,zl={"common.add":"Add","common.admin":"Admin","common.apply":"Apply","common.back":"Back","common.cancel":"Cancel","common.change":"Change","common.choose":"Choose","common.close":"Close","common.confirmLeave":"Leave without saving the changes?","common.copied":"Copied","common.copy":"Copy","common.create":"Create","common.delete":"Delete","common.discard":"Discard","common.done":"Done","common.download":"Download","common.draft":"Draft","common.duplicate":"Duplicate","common.edit":"Edit","common.email":"Email","common.loading":"Loading…","common.name":"Name","common.never":"Never","common.new":"New","common.no":"No","common.none":"None","common.nothingFound":"Nothing found","common.open":"Open","common.password":"Password","common.preview":"Preview","common.publish":"Publish","common.published":"Published","common.remove":"Remove","common.rename":"Rename","common.restore":"Restore","common.retry":"Try again","common.save":"Save","common.saved":"Saved","common.saving":"Saving…","common.search":"Search","common.title":"Title","common.upload":"Upload","common.view":"View","common.yes":"Yes","common.entries":{one:"{n} entry",other:"{n} entries"},"common.files":{one:"{n} file",other:"{n} files"},"common.kb":"{n} KB","common.mb":"{n} MB","common.unreachable":"zipp can't be reached ({error}), reload the page to try again."},Er={"ui.actions":"Actions","ui.back":"Back","ui.breadcrumbs":"Breadcrumbs","ui.cantMove":"It can't move","ui.changed":"Changed","ui.clear":"Clear","ui.close":"Close","ui.collapse":"Collapse","ui.date":"Date","ui.disabled":"Disabled","ui.dragOrClick":"drag, or click for the moves","ui.done":"Done","ui.duplicate":"Duplicate","ui.expand":"Expand","ui.forward":"Forward","ui.hasErrors":"Has errors","ui.hide":"Hide it on the website","ui.loading":"Loading…","ui.monthAfter":"Month after","ui.monthBefore":"Month before","ui.move":"Move","ui.moveDown":"Move down","ui.moveIn":"Move under the one above","ui.moveOut":"Move out a level","ui.moveUp":"Move up","ui.noDate":"No date","ui.nothingFound":"Nothing found","ui.nothingYet":"Nothing yet","ui.pickDay":"Pick a day","ui.pickMonth":"Pick a month","ui.pickYear":"Pick a year","ui.remove":"Remove","ui.resize":"Drag to change the width, double click for the default","ui.search":"Search","ui.show":"Show it on the website","ui.time":"time","ui.today":"Today"},ql={"shell.entries":"Entries","shell.assets":"Assets","shell.ai":"AI","shell.setup":"Setup","shell.users":"Users","shell.trash":"Trash","shell.setupOverview":"Overview","shell.setupFields":"Fields","shell.setupRequests":"Requests","shell.setupSections":"Sections","shell.setupTypes":"Entry types","shell.setupBlocks":"Blocks","shell.setupSites":"Sites, transforms and roles","shell.menu":"Menu","shell.site":"Site","shell.development":"Development","shell.production":"Production","shell.offline":"offline","shell.yourAccount":"Your account","shell.logOut":"Log out","shell.loggedInAs":"Logged in as {name} by {by}","shell.backToAccount":"Back to your account","shell.entry":"Entry","shell.file":"File","shell.fileDeleted":"This file was deleted.","shell.readOnly":"Read only, from the code","shell.noDoc":"There is no doc {handle}","shell.pageFailed":"This page couldn't load","shell.toStart":"To the start","shell.notFound":"Not found","shell.noPage":"This page doesn't exist.","shell.noJson":"{path}: the answer is no JSON","shell.uploadFailed":"{path}: the upload failed"},jl={"auth.logIn":"Log in","auth.welcomeBack":"Welcome back. Log in with your account.","auth.emailPlaceholder":"name@example.com","auth.showPassword":"Show the password","auth.hidePassword":"Hide the password","auth.stayLoggedIn":"Stay logged in","auth.forgot":"Forgot your password?","auth.forgotIntro":"Your email gets a link to choose a new one.","auth.forgotSent":"If {email} has an account, a mail with a link is on its way. The link works for an hour.","auth.sendLink":"Send the link","auth.backToLogin":"Back to the login","auth.linkBroken":"This link doesn't work","auth.loggingIn":"Logging in…","auth.toStudio":"To the studio","auth.sessionEnded":"The session ended","auth.wereLoggedInAs":"You were logged in as {name}. Go back to your account or log in.","auth.backToAccount":"Back to your account","auth.logInAgain":"Log in again to go on, what you changed stays.","auth.logOut":"Log out","auth.welcome":"Welcome","auth.newPasswordTitle":"A new password","auth.inviteIntro":"Your account is {email}. Choose a password to log in.","auth.resetIntro":"Choose a new password for {email}. You are logged out everywhere else.","auth.yourName":"Your name","auth.namePlaceholder":"Anna Muster","auth.passwordPlaceholder":"At least 8 characters","auth.setAndLogIn":"Set the password and log in","auth.saveAndLogIn":"Save and log in","auth.linkProblem":"{problem}. Ask for a new one, or an admin for an invite.","auth.newPassword":"New password"},Vl={"users.title":"Users","users.new":"New user","users.user":"User","users.role":"Role","users.lastLogin":"Last login","users.you":"(you)","users.invited":"Invited","users.account":"Account","users.yourAccount":"Your account","users.thatsYou":"That's you","users.delete":"Delete the user","users.confirmDelete":"Delete {name}? They can't log in any more.","users.confirmEndOwn":"Log out in all other browsers?","users.confirmEnd":"Log {name} out everywhere?","users.namePlaceholder":"Anna Muster","users.language":"Language of the studio","users.browserLanguage":"The browser's ({language})","users.passwordNew":"Without one the user gets an invite link to choose it.","users.passwordKeepOwn":"Leave it empty to keep the current one. A new password logs you out in all other browsers.","users.passwordKeep":"Leave it empty to keep the current one. A new password logs them out in all browsers.","users.newPassword":"New password","users.passwordPlaceholder":"At least 8 characters","users.sendInvite":"Send the invite by mail","users.currentPassword":"Your current password","users.created":"Created","users.loggedIn":"Logged in","users.notLoggedIn":"Not logged in","users.browsers":{one:"in {n} browser",other:"in {n} browsers"},"users.endOwn":"Log out everywhere else","users.end":"Log out everywhere","users.access":"Access","users.invitedNote":"Invited, no password yet.","users.sendInviteAgain":"Send the invite again","users.sendReset":"Send a password reset","users.copyInvite":"Copy the invite link","users.copyReset":"Copy a reset link","users.loginAs":"Log in as {name}","users.copyLogin":"Copy a login link","users.copyLoginTitle":"To open in a private window, your login stays","users.inviteFailed":"The invite mail failed: {error}","users.inviteSent":"The invite is sent to {email}, it works until {until}.","users.resetSent":"The reset link is sent to {email}, it works until {until}.","users.inviteCopied":"The invite is copied, it works once until {until}.","users.resetCopied":"The reset link is copied, it works once until {until}.","users.loginCopied":"The login link is copied: open it once, in the next 10 minutes.","users.adminMay":"An admin may do everything: all sections, assets, AI, users and Setup.","users.assets":"Assets","users.ai":"AI","users.levels":"View sees entries, Edit changes them as drafts, Publish makes them live, Delete deletes them. The roles are code, {setup} shows all of them.","users.setup":"Setup"},Hl={"entries.noSection":"There is no section {section}","entries.viewOnWebsite":"View on the website","entries.confirmTrash":"Move {title} to the trash?","entries.confirmTrashBelow":{one:"Move {title} to the trash? The entry below it too.",other:"Move {title} to the trash? The {n} entries below it too."},"entries.new":"New entry","entries.search":"Search entries","entries.status":"Status","entries.type":"Type","entries.updated":"Updated","entries.move":"Move {title}","entries.unpublishedChanges":"Unpublished changes","entries.liveFrom":"Live from {time}","entries.pending":"Pending","entries.expiredAt":"Expired {time}","entries.expired":"Expired","entries.actionsFor":"Actions for {title}","entries.noneYet":"No entries yet","entries.shownOf":"{shown} of {total}","entries.chooseType":"Choose {type}","entries.chooseEntry":"Choose entry","entries.showBelow":"Show the entries below","entries.hideBelow":"Hide the entries below","entries.cantChoose":"A {type} can't be chosen here","entries.alreadyChosen":"Already chosen","entries.searchTypes":"Search {types}","entries.didntLoad":"The entries didn't load.","entries.openInFlyout":"Open in the flyout","entries.sitePublished":"{site}: published","entries.siteNotPublished":"{site}: not published","entries.siteOpen":"{title}, open","entries.desktop":"Desktop","entries.tablet":"Tablet","entries.mobile":"Mobile","entries.reload":"Reload","entries.websiteWidth":"Width of the website","entries.openInNewTab":"Open in a new tab","entries.website":"Website","entries.confirmDiscard":"Throw away the unpublished changes?","entries.confirmCopy":"Replace the content with the one of {site}?","entries.fieldLayout":"Field layout «{name}»","entries.draftBy":"by {user}, {time}","entries.live":"Live","entries.notPublished":"Not published","entries.notSaved":"Not saved","entries.savedAsDraft":"Saved as draft, not live yet","entries.moveToTrash":"Move to the trash","entries.discardTitle":"Throw away the unpublished changes","entries.liveTitle":"The website next to the editor, with the draft","entries.livePreview":"Live preview","entries.previewTitle":"Open the draft on the website","entries.viewTitle":"Open the page on the website","entries.mayNotPublish":"Your role can edit drafts, someone else publishes them","entries.conflict":"while you were editing. Your last changes are not saved yet.","entries.loadTheirs":"Load their changes","entries.keepMine":"Keep mine","entries.confirmType":"Change the type to {type}? It has no field like {fields}: their content is removed in every site.","entries.publishedIn":"Published in {site}","entries.notLiveYet":"Not live yet, from {time}","entries.notLiveAnymore":"Not live anymore, expired {time}","entries.postDate":"Post date","entries.whenPublished":"When published","entries.expiryDate":"Expiry date","entries.fieldsInSetup":"Its fields in Setup","entries.parent":"Parent","entries.noParent":"None, on the top","entries.slug":"Slug","entries.fromTitle":"from the title","entries.outlineOf":"Outline of {field}","entries.outline":"Outline · {field}","entries.disabled":"Disabled","entries.missing":"Missing","entries.changed":"Changed","entries.errors":{one:"{n} error",other:"{n} errors"},"entries.copyContent":"Copy content","entries.fromOtherSite":"From another site…","entries.revisions":"Revisions","entries.revisionsCount":"Revisions · {n}","entries.revisionOf":"Revision of {time}, read only","entries.revisionOfBy":"Revision of {time} by {user}, read only","entries.changedList":"Changed: {changes}","entries.backToEditing":"Back to editing","entries.noContentHere":"This revision has no content for this site.","entries.revisionLive":"live","entries.created":"Created","entries.confirmRestore":"Restore this revision?","entries.confirmRestoreDraft":"Restore this revision? The draft is replaced."},Ul={"fields.notHosted":"{kind} fields can't be mounted by plugins","fields.changed":"Changed","fields.translated":"Translated: each site has its own","fields.inSetup":"The field in Setup","fields.chooseOption":"Choose…","fields.write":"Write","fields.notTranslated":"Not translated: {text}","fields.moveEntry":"Move the entry","fields.removeFromField":"Remove from the field","fields.moveRow":"Move the row","fields.removeRow":"Remove the row","fields.addRow":"Add a row","fields.moveFile":"Move the file","fields.deletedFile":"A file that was deleted","fields.detailsTitle":"Details: title, alt text, focal point","fields.details":"Details","fields.dropToAdd":"Drop to add","fields.dragVideos":"Drag videos here","fields.dragImages":"Drag images here","fields.dragFiles":"Drag files here","fields.dragTypes":"Drag {types} files here","fields.blocks":{one:"{n} block",other:"{n} blocks"},"fields.images":{one:"{n} image",other:"{n} images"},"fields.collapseAll":"Collapse all","fields.expandAll":"Expand all","fields.moveBlock":"Move the block","fields.searchBlocks":"Search blocks","fields.addBlock":"Add a block","fields.addBlockHere":"Add a block here","fields.moreBlocks":"More blocks","fields.more":"More","fields.noLinkYet":"No link yet","fields.isMissing":"{label} is missing","fields.empty":"Empty","fields.format":"Format","fields.paragraph":"Paragraph","fields.heading":"Heading {level}","fields.bold":"Bold","fields.italic":"Italic","fields.list":"List","fields.numberedList":"Numbered list","fields.link":"Link","fields.insert":"Insert","fields.button":"Button","fields.writePlaceholder":"Write …","fields.continueWriting":"Continue writing …","fields.addLineAbove":"Add a line above","fields.addLineBelow":"Add a line below","fields.line":"Line","fields.deleteButton":"Delete button","fields.moveButton":"Move the button","fields.openTarget":"Open the target","fields.styles":"Styles","fields.text":"Text","fields.removeLink":"Remove link","fields.linkEntry":"Entry","fields.linkUrl":"URL","fields.linkPhone":"Phone","fields.linkFile":"File","fields.noTargetYet":"No target yet","fields.aFile":"A file","fields.target":"Target","fields.chooseAnotherEntry":"Choose another entry","fields.notPublished":"Not published","fields.removeEntry":"Remove the entry","fields.anotherFile":"Another file","fields.chooseFile":"Choose file","fields.label":"Label","fields.fromTarget":"From the target","fields.anchor":"Anchor","fields.anchorPlaceholder":"optional, e.g. contact","fields.anchorNote":"Jumps to a section of the target page.","fields.openInNewTab":"Open in a new tab","fields.downloadInstead":"Download instead of opening","fields.requiredNoLink":"Required – no link yet.","fields.noLink":"No link yet.","fields.actions":"Actions","fields.closeSettings":"Close its settings","fields.editLink":"Edit the link","fields.newTab":"New tab","fields.linkActions":"Actions for the link","fields.moveLink":"Move the link","fields.confirmRemoveLink":"Remove the link?","fields.confirmRemoveLinkChildren":"Remove the link with the links under it?","fields.upToLevels":"Up to {n} levels","fields.oneLevel":"One level","fields.isOn":"{field} is on","fields.isOff":"{field} is off","fields.isOneOf":"{field} is {values}","fields.or":"or","fields.isFilled":"{field} is filled","fields.isEmpty":"{field} is empty","fields.kindText":"Text","fields.kindTextLines":"Text, several lines","fields.max":"max {n}","fields.min":"min {n}","fields.kindNumber":"Number","fields.kindNumberRange":"Number {min}–{max}","fields.whole":"whole","fields.decimals":"{n} decimals","fields.kindBoolean":"Yes / no","fields.kindEmail":"Email address","fields.kindRelation":"Entries: {types}","fields.kindImages":"Images","fields.kindVideos":"Videos","fields.kindFiles":"Files","fields.kindLink":"Link to {to}","fields.sameTab":"same tab","fields.lists":"lists","fields.links":"links","fields.buttons":"buttons","fields.blocksOf":"blocks: {types}","fields.kindRichText":"Rich text","fields.kindRichTextWith":"Rich text: {parts}","fields.kindGroup":"Group of {component}","fields.kindDate":"Date","fields.kindDateTime":"Date and time","fields.kindCheckboxes":"Checkboxes: {options}","fields.kindTable":"Table: {columns}","fields.kindNavigation":"Navigation to {to}, {n} levels","fields.kindDropdown":"Dropdown: {options}","fields.kindTexts":"Texts: {n} keys","fields.kindBlocks":"Blocks: {types}","fields.kindCustom":"{type} (plugin)","fields.kindNote":"Note for editors","fields.optWidth":"Width","fields.widthOfRow":"{n} % of a row","fields.optShownWhen":"Shown when","fields.optText":"Text","fields.optInstructions":"Instructions","fields.optRequired":"Required","fields.optSites":"Sites","fields.perSite":"Per site, translated","fields.sameInAllSites":"The same in all sites","fields.optIndexed":"Indexed","fields.indexedYes":"Yes, lists and GraphQL filter and sort by it","fields.optDefault":"Default","fields.optLines":"Lines","fields.several":"Several","fields.one":"One","fields.optMaxLength":"Max length","fields.optMin":"Min","fields.optMax":"Max","fields.optDecimals":"Decimals","fields.any":"Any","fields.optUnit":"Unit","fields.optValue":"Value","fields.dayAndTime":"A day and time, 2026-11-14T20:00","fields.day":"A day, 2026-11-14","fields.optEntryTypes":"Entry types","fields.optMaxEntries":"Max entries","fields.optFiles":"Files","fields.imagesOnly":"Images only","fields.anyFile":"Any file","fields.optMaxFiles":"Max files","fields.optUploadTo":"Upload to","fields.library":"Library","fields.optLinksTo":"Links to","fields.optExternalLinks":"External links","fields.inNewTab":"In a new tab","fields.inSameTab":"In the same tab","fields.optLevels":"Levels","fields.optHeadings":"Headings","fields.optLists":"Lists","fields.optLinks":"Links","fields.optButtons":"Buttons","fields.optBlocksInText":"Blocks in the text","fields.styleText":"text","fields.styleParagraph":"paragraph","fields.optBlockTypes":"Block types","fields.optMinBlocks":"Min blocks","fields.optMaxBlocks":"Max blocks","fields.optFieldsOf":"Fields of","fields.optOptions":"Options","fields.optColumns":"Columns","fields.optKeys":"Keys","fields.optFieldType":"Field type","fields.ofPlugin":"{type}, of a plugin"},Wl={"assets.title":"Assets","assets.library":"Library","assets.files":"Files","assets.folders":"Folders","assets.folder":"Folder","assets.noFolder":"{root} (no folder)","assets.all":"All","assets.images":"Images","assets.videos":"Videos","assets.documents":"Documents","assets.noAssets":"No assets yet","assets.noFilesInFolder":"No files in this folder","assets.inAllFolders":"In all folders","assets.searchFiles":"Search files","assets.type":"Type","assets.size":"Size","assets.dimensions":"Dimensions","assets.uploaded":"Uploaded","assets.uploadedSize":"uploaded {width} × {height}","assets.uploadedBy":"by {email}","assets.deliveredAsPng":"delivered as PNG","assets.altText":"Alt text","assets.altTextOf":"Alt text ({site})","assets.altPlaceholder":"What the image shows","assets.titlePlaceholder":"From the filename","assets.filename":"Filename","assets.confirmDiscard":"Discard the changes?","assets.copyUrl":"Copy URL","assets.replaceFile":"Replace file","assets.usedIn":"Used in","assets.notUsed":"Not used in any entry","assets.deleteFile":"Delete the file","assets.details":"Details: title, alt text, focal point","assets.fileActions":"File actions","assets.confirmTrash":"Move {filename} to the trash?","assets.confirmTrashUsed":{one:"Move {filename} to the trash? It is used in {n} entry, they lose it until it is restored.",other:"Move {filename} to the trash? It is used in {n} entries, they lose it until it is restored."},"assets.confirmUnzip":`Unpack {name} into folders?

OK unpacks, Cancel uploads it as one zip file.`,"assets.confirmUnzipMany":`Unpack the {n} zip files into folders?

OK unpacks, Cancel uploads it as one zip file.`,"assets.moveTo":"Move to {folder}","assets.zipping":"Zipping {name}…","assets.uploadFolder":"Upload folder","assets.uploading":"Uploading…","assets.uploadingOf":"Uploading {n} of {total}","assets.uploadedFiles":{one:"Uploaded {n} file",other:"Uploaded {n} files"},"assets.sentOf":"{sent} of {total}","assets.dropFailed":"The drop: {error}","assets.dropToUpload":"Drop to upload","assets.dropInto":"Drop to upload into {path}","assets.chooseImage":"Choose an image","assets.chooseImages":"Choose images","assets.chooseFile":"Choose a file","assets.chooseFiles":"Choose files","assets.newFolder":"New folder","assets.folderName":"Folder name","assets.downloadZip":"Download as zip","assets.confirmDeleteFolder":"Delete the folder {name}?","assets.collapse":"Collapse","assets.expand":"Expand","assets.editImage":"Edit the image","assets.tool":"Tool","assets.focalPoint":"Focal point","assets.crop":"Crop","assets.rotateLeft":"Rotate left","assets.rotateRight":"Rotate right","assets.flipH":"Flip horizontally","assets.flipV":"Flip vertically","assets.resetTitle":"Back to the uploaded image","assets.reset":"Reset","assets.ratio":"Ratio","assets.free":"Free","assets.original":"Original","assets.square":"Square","assets.portrait":"Portrait","assets.cropNote":"Drag the crop past the image for room around it.","assets.transparentNote":"Transparent needs a PNG: a JPEG is delivered as PNG then.","assets.focalNote":"Click on the image to set the focal point, crops of the sizes keep it in view.","assets.width":"Width","assets.height":"Height","assets.originalSize":"Original size","assets.canvas":"Canvas around it","assets.canvasWidth":"Canvas width","assets.canvasHeight":"Canvas height","assets.background":"Background","assets.transparent":"Transparent"},Yl={"setup.setup":"Setup","setup.readOnly":"Read only, the content model is code","setup.noSectionThere":"There is no section {handle}","setup.noEntryType":"No entry type {handle}","setup.noBlock":"No block {handle}","setup.noSection":"No section {handle}","setup.everyAsset":"every asset","setup.sectionName":"section {name}","setup.handle":"Handle","setup.kind":"Kind","setup.fields":"Fields","setup.entryTypes":"Entry types","setup.entries":"Entries","setup.youMay":"You may","setup.usedIn":"Used in","setup.nothing":"nothing","setup.nothingHidden":"nothing, hidden","setup.admins":"Admins (no role)","setup.everything":"Everything","setup.single":"Single: exactly one entry","setup.channel":"Channel: a list, newest first","setup.structure":"Structure: a tree in a set order","setup.general":"General","setup.maxLevel":"Max level","setup.maxLevelOne":"1, only entries on the top","setup.maxLevelNone":"none, as deep as needed","setup.pages":"Pages","setup.pagesIf":"only entries with {field} on","setup.navigation":"Navigation","setup.noPages":"no pages","setup.home":"/ (home)","setup.sites":"Sites","setup.site":"Site","setup.language":"Language","setup.urls":"URLs","setup.status":"Status","setup.offline":"Offline","setup.online":"Online","setup.access":"Access","setup.role":"Role","setup.may":"May","setup.allRoles":"All roles","setup.unpublishedChanges":"Unpublished changes","setup.theFirst":"The first {n}.","setup.openInContent":"Open in Content","setup.other":"Other","setup.shownWhen":"shown when {condition}","setup.inACard":"in a card","setup.label":"Label","setup.perSite":"per site","setup.allSites":"all sites","setup.required":"required","setup.conditional":"conditional","setup.indexed":"indexed","setup.title":"title","setup.noFieldsHere":"No fields here.","setup.onlyTitle":"No fields, only a title.","setup.mail":"Mail","setup.mailsGoTo":"Mails go to {server} from {from}.","setup.sendTest":"Send a test mail to me","setup.sentTo":"Sent to {email}.","setup.noSmtp":"No SMTP server: mails are only logged. Start the cms with {smtp} and {from}.","setup.url":"URL","setup.primary":" (primary)","setup.offlineSite":" (offline, not on the website)","setup.transforms":"Image transforms","setup.auto":"auto","setup.cropped":", cropped around the focal point","setup.roles":"Roles","setup.youAre":"You are {who}. View sees entries, Edit changes them as drafts and creates new ones, Publish makes drafts live and moves entries, Delete deletes them. Users without a role are admins.","setup.anAdmin":"an admin and can do everything","setup.assets":"Assets","setup.ai":"AI","setup.notInNav":"Not in the navigation","setup.typesAndBlocks":"Types and blocks","setup.fieldTypes":"Field types","setup.transformsShort":"Transforms","setup.nFields":{one:"{n} field",other:"{n} fields"},"setup.noSectionsVisible":"No sections you may see.","setup.unusedBlocks":"Blocks used nowhere","setup.plugins":"Plugins","setup.nothingInModel":"Nothing in the content model.","setup.more":"More","setup.graphqlNote":"tokens of frontends, GraphiQL","setup.rolesNote":"who may do what","setup.sitesAndTransforms":"Sites and transforms","setup.mailNote":"how mails are sent","setup.nOpen":{one:"{n} open",other:"{n} open"},"setup.requestsNote":"changes editors asked for","setup.searchField":"Search a field or block","setup.nOfAll":"{n} of {all}","setup.holds":"Holds","setup.in":"In","setup.noFieldLike":"No field like that","setup.requestChange":"Request a change","setup.requestChangeTitle":"Ask the developers for a change here","setup.requestSent":"Sent, the developers see it under Requests.","setup.whatDoYouNeed":"What do you need on {context}?","setup.requestPlaceholder":"I need a subtitle field","setup.send":"Send","setup.requestsIntro":"Editors ask for changes with “Request a change” on the pages of Setup. Admins get a mail of each when mails are set up.","setup.request":"Request","setup.where":"Where","setup.who":"Who","setup.when":"When","setup.noRequests":"No requests yet"},Gl={"search.placeholder":"Search entries and files","search.searching":"Searching…","search.notPublished":"Not published","search.assets":"Assets"},Kl={"trash.title":"Trash","trash.note":"Deleted entries and assets, for {days} days","trash.empty":"Empty trash","trash.restored":"Restored:","trash.isEmpty":"The trash is empty.","trash.entries":"Entries","trash.entry":"Entry","trash.section":"Section","trash.deleted":"Deleted","trash.assets":"Assets","trash.asset":"Asset","trash.size":"Size","trash.below":{one:"and {n} entry below",other:"and {n} entries below"},"trash.by":"by {email}","trash.deleteForGood":"Delete for good","trash.deleteNameForGood":"Delete {name} for good","trash.confirmEntry":"Delete {name} for good? It can't be restored.","trash.confirmEntryChildren":{one:"Delete {name} for good? And {n} entry below it. It can't be restored.",other:"Delete {name} for good? And {n} entries below it. It can't be restored."},"trash.confirmAsset":"Delete {name} for good? It can't be restored.","trash.confirmAll":{one:"Delete {n} item for good? They can't be restored.",other:"Delete {n} items for good? They can't be restored."}},$l={"graphql.newToken":"New token","graphql.copyNow":"Copy the token now, it doesn't show again. A frontend sends it as {header} ; crelte reads it as {env} .","graphql.theToken":"The token","graphql.namePlaceholder":"Website","graphql.expires":"Expires","graphql.sections":"Sections","graphql.allSections":"All sections","graphql.sites":"Sites","graphql.allSites":"All sites","graphql.drafts":"Drafts","graphql.previewsShowDrafts":"Preview links show drafts","graphql.previewsNote":"For the website that shows the studio's previews; other frontends only read what is live.","graphql.all":"All","graphql.confirmRevoke":"Revoke {name}? Frontends that send it get no content any more.","graphql.graphiqlTitle":"GraphiQL with your session","graphql.tryIt":"Try it","graphql.intro":"Frontends send a token as {header} to {path} . Each reads only its sections and sites.","graphql.envToken":"The token of {env} reads everything.","graphql.token":"Token","graphql.lastUsed":"Last used","graphql.noTokens":"No tokens yet","graphql.expired":"Expired","graphql.revoke":"Revoke the token"},Zl={"ai.title":"AI","ai.width":"Width of the chat","ai.startOver":"Start over, this chat is only kept in this tab","ai.newChat":"New chat","ai.hintAgent":"Development: ask for anything, content, design or new fields, e.g. “Add a field for the opening hours” or “Make the buttons round”. Code changes show on the right right away, content changes are drafts you publish.","ai.hint":"Ask for anything on the website, e.g. “Write a better intro for Über uns”, “Add an event in Basel” or “Translate the footer into English”. Changes are drafts: you see them on the right and publish them in the editor.","ai.placeholder":"What should change?","ai.message":"Message","ai.keys":"Enter sends, Shift+Enter a new line","ai.send":"Send","ai.endedEarly":"The answer ended early"},Tr={...zl,...Er,...ql,...jl,...Vl,...Hl,...Ul,...Wl,...Yl,...Gl,...Kl,...$l,...Zl},Jl={"common.add":"Hinzufügen","common.admin":"Admin","common.apply":"Übernehmen","common.back":"Zurück","common.cancel":"Abbrechen","common.change":"Ändern","common.choose":"Auswählen","common.close":"Schliessen","common.confirmLeave":"Verlassen, ohne die Änderungen zu speichern?","common.copied":"Kopiert","common.copy":"Kopieren","common.create":"Erstellen","common.delete":"Löschen","common.discard":"Verwerfen","common.done":"Fertig","common.download":"Herunterladen","common.draft":"Entwurf","common.duplicate":"Duplizieren","common.edit":"Bearbeiten","common.email":"E-Mail","common.loading":"Lädt…","common.name":"Name","common.never":"Nie","common.new":"Neu","common.no":"Nein","common.none":"Keine","common.nothingFound":"Nichts gefunden","common.open":"Öffnen","common.password":"Passwort","common.preview":"Vorschau","common.publish":"Veröffentlichen","common.published":"Veröffentlicht","common.remove":"Entfernen","common.rename":"Umbenennen","common.restore":"Wiederherstellen","common.retry":"Nochmals versuchen","common.save":"Speichern","common.saved":"Gespeichert","common.saving":"Speichert…","common.search":"Suchen","common.title":"Titel","common.upload":"Hochladen","common.view":"Ansehen","common.yes":"Ja","common.entries":{one:"{n} Eintrag",other:"{n} Einträge"},"common.files":{one:"{n} Datei",other:"{n} Dateien"},"common.kb":"{n} KB","common.mb":"{n} MB","common.unreachable":"zipp ist nicht erreichbar ({error}). Laden Sie die Seite neu, um es nochmals zu versuchen."},Ql={"ui.actions":"Aktionen","ui.back":"Zurück","ui.breadcrumbs":"Navigationspfad","ui.cantMove":"Lässt sich nicht verschieben","ui.changed":"Geändert","ui.clear":"Leeren","ui.close":"Schliessen","ui.collapse":"Zuklappen","ui.date":"Datum","ui.disabled":"Ausgeschaltet","ui.dragOrClick":"ziehen oder klicken für die Schritte","ui.done":"Fertig","ui.duplicate":"Duplizieren","ui.expand":"Aufklappen","ui.forward":"Vorwärts","ui.hasErrors":"Hat Fehler","ui.hide":"Auf der Website ausblenden","ui.loading":"Lädt…","ui.monthAfter":"Nächster Monat","ui.monthBefore":"Vorheriger Monat","ui.move":"Verschieben","ui.moveDown":"Nach unten","ui.moveIn":"Unter den vorherigen verschieben","ui.moveOut":"Eine Ebene höher","ui.moveUp":"Nach oben","ui.noDate":"Kein Datum","ui.nothingFound":"Nichts gefunden","ui.nothingYet":"Noch nichts da","ui.pickDay":"Tag wählen","ui.pickMonth":"Monat wählen","ui.pickYear":"Jahr wählen","ui.remove":"Entfernen","ui.resize":"Ziehen ändert die Breite, Doppelklick setzt sie zurück","ui.search":"Suchen","ui.show":"Auf der Website zeigen","ui.time":"Uhrzeit","ui.today":"Heute"},Xl={"shell.entries":"Einträge","shell.assets":"Dateien","shell.ai":"KI","shell.setup":"Einrichtung","shell.users":"Benutzer","shell.trash":"Papierkorb","shell.setupOverview":"Übersicht","shell.setupFields":"Felder","shell.setupRequests":"Anfragen","shell.setupSections":"Abschnitte","shell.setupTypes":"Eintragstypen","shell.setupBlocks":"Blöcke","shell.setupSites":"Websites, Bildformate und Rollen","shell.menu":"Menü","shell.site":"Website","shell.development":"Entwicklung","shell.production":"Produktion","shell.offline":"offline","shell.yourAccount":"Ihr Konto","shell.logOut":"Abmelden","shell.loggedInAs":"Angemeldet als {name} durch {by}","shell.backToAccount":"Zurück zu Ihrem Konto","shell.entry":"Eintrag","shell.file":"Datei","shell.fileDeleted":"Diese Datei wurde gelöscht.","shell.readOnly":"Nur lesen, aus dem Code","shell.noDoc":"Es gibt keine Dokumentation {handle}","shell.pageFailed":"Diese Seite konnte nicht geladen werden","shell.toStart":"Zum Start","shell.notFound":"Nicht gefunden","shell.noPage":"Diese Seite gibt es nicht.","shell.noJson":"{path}: Die Antwort ist kein JSON","shell.uploadFailed":"{path}: Das Hochladen ist fehlgeschlagen"},ea={"auth.logIn":"Anmelden","auth.welcomeBack":"Willkommen zurück. Melden Sie sich mit Ihrem Konto an.","auth.emailPlaceholder":"name@beispiel.ch","auth.showPassword":"Passwort anzeigen","auth.hidePassword":"Passwort verbergen","auth.stayLoggedIn":"Angemeldet bleiben","auth.forgot":"Passwort vergessen?","auth.forgotIntro":"Sie erhalten per E-Mail einen Link, um ein neues zu wählen.","auth.forgotSent":"Wenn {email} ein Konto hat, ist eine E-Mail mit einem Link unterwegs. Der Link ist eine Stunde gültig.","auth.sendLink":"Link senden","auth.backToLogin":"Zurück zur Anmeldung","auth.linkBroken":"Dieser Link funktioniert nicht","auth.loggingIn":"Anmelden…","auth.toStudio":"Zum Studio","auth.sessionEnded":"Die Sitzung ist abgelaufen","auth.wereLoggedInAs":"Sie waren als {name} angemeldet. Gehen Sie zurück zu Ihrem Konto oder melden Sie sich an.","auth.backToAccount":"Zurück zu Ihrem Konto","auth.logInAgain":"Melden Sie sich erneut an, um weiterzuarbeiten. Ihre Änderungen bleiben erhalten.","auth.logOut":"Abmelden","auth.welcome":"Willkommen","auth.newPasswordTitle":"Ein neues Passwort","auth.inviteIntro":"Ihr Konto ist {email}. Wählen Sie ein Passwort, um sich anzumelden.","auth.resetIntro":"Wählen Sie ein neues Passwort für {email}. Sie werden überall sonst abgemeldet.","auth.yourName":"Ihr Name","auth.namePlaceholder":"Anna Muster","auth.passwordPlaceholder":"Mindestens 8 Zeichen","auth.setAndLogIn":"Passwort festlegen und anmelden","auth.saveAndLogIn":"Speichern und anmelden","auth.linkProblem":"{problem}. Fordern Sie einen neuen an oder bitten Sie einen Admin um eine Einladung.","auth.newPassword":"Neues Passwort"},na={"users.title":"Benutzer","users.new":"Neuer Benutzer","users.user":"Benutzer","users.role":"Rolle","users.lastLogin":"Letzte Anmeldung","users.you":"(Sie)","users.invited":"Eingeladen","users.account":"Konto","users.yourAccount":"Ihr Konto","users.thatsYou":"Das sind Sie","users.delete":"Benutzer löschen","users.confirmDelete":"{name} löschen? Die Person kann sich nicht mehr anmelden.","users.confirmEndOwn":"In allen anderen Browsern abmelden?","users.confirmEnd":"{name} überall abmelden?","users.namePlaceholder":"Anna Muster","users.language":"Sprache des Studios","users.browserLanguage":"Die des Browsers ({language})","users.passwordNew":"Ohne Passwort erhält die Person einen Einladungslink, um eines zu wählen.","users.passwordKeepOwn":"Leer lassen, um das bisherige zu behalten. Ein neues Passwort meldet Sie in allen anderen Browsern ab.","users.passwordKeep":"Leer lassen, um das bisherige zu behalten. Ein neues Passwort meldet die Person in allen Browsern ab.","users.newPassword":"Neues Passwort","users.passwordPlaceholder":"Mindestens 8 Zeichen","users.sendInvite":"Einladung per E-Mail senden","users.currentPassword":"Ihr aktuelles Passwort","users.created":"Erstellt","users.loggedIn":"Angemeldet","users.notLoggedIn":"Nicht angemeldet","users.browsers":{one:"in {n} Browser",other:"in {n} Browsern"},"users.endOwn":"Überall sonst abmelden","users.end":"Überall abmelden","users.access":"Zugang","users.invitedNote":"Eingeladen, noch kein Passwort.","users.sendInviteAgain":"Einladung nochmals senden","users.sendReset":"Passwort-Link senden","users.copyInvite":"Einladungslink kopieren","users.copyReset":"Passwort-Link kopieren","users.loginAs":"Als {name} anmelden","users.copyLogin":"Anmeldelink kopieren","users.copyLoginTitle":"Zum Öffnen in einem privaten Fenster, Ihre Anmeldung bleibt","users.inviteFailed":"Die Einladung konnte nicht gesendet werden: {error}","users.inviteSent":"Die Einladung ist an {email} gesendet, sie gilt bis {until}.","users.resetSent":"Der Passwort-Link ist an {email} gesendet, er gilt bis {until}.","users.inviteCopied":"Die Einladung ist kopiert, sie gilt einmal bis {until}.","users.resetCopied":"Der Passwort-Link ist kopiert, er gilt einmal bis {until}.","users.loginCopied":"Der Anmeldelink ist kopiert: einmal öffnen, in den nächsten 10 Minuten.","users.adminMay":"Admins dürfen alles: alle Abschnitte, Dateien, KI, Benutzer und Einrichtung.","users.assets":"Dateien","users.ai":"KI","users.levels":"Ansehen zeigt Einträge, Bearbeiten ändert sie als Entwurf, Veröffentlichen stellt sie online, Löschen löscht sie. Die Rollen sind Code, {setup} zeigt alle.","users.setup":"Einrichtung"},ta={"entries.noSection":"Es gibt keinen Abschnitt {section}","entries.viewOnWebsite":"Auf der Website ansehen","entries.confirmTrash":"{title} in den Papierkorb legen?","entries.confirmTrashBelow":{one:"{title} in den Papierkorb legen? Der untergeordnete Eintrag auch.",other:"{title} in den Papierkorb legen? Die {n} untergeordneten Einträge auch."},"entries.new":"Neuer Eintrag","entries.search":"Einträge suchen","entries.status":"Status","entries.type":"Typ","entries.updated":"Geändert","entries.move":"{title} verschieben","entries.unpublishedChanges":"Nicht veröffentlichte Änderungen","entries.liveFrom":"Live ab {time}","entries.pending":"Geplant","entries.expiredAt":"Abgelaufen am {time}","entries.expired":"Abgelaufen","entries.actionsFor":"Aktionen für {title}","entries.noneYet":"Noch keine Einträge","entries.shownOf":"{shown} von {total}","entries.chooseType":"{type} auswählen","entries.chooseEntry":"Eintrag auswählen","entries.showBelow":"Untergeordnete Einträge zeigen","entries.hideBelow":"Untergeordnete Einträge ausblenden","entries.cantChoose":"{type} kann hier nicht ausgewählt werden","entries.alreadyChosen":"Bereits ausgewählt","entries.searchTypes":"{types} suchen","entries.didntLoad":"Die Einträge konnten nicht geladen werden.","entries.openInFlyout":"Im Seitenfenster öffnen","entries.sitePublished":"{site}: veröffentlicht","entries.siteNotPublished":"{site}: nicht veröffentlicht","entries.siteOpen":"{title}, öffnen","entries.desktop":"Desktop","entries.tablet":"Tablet","entries.mobile":"Mobil","entries.reload":"Neu laden","entries.websiteWidth":"Breite der Website","entries.openInNewTab":"In einem neuen Tab öffnen","entries.website":"Website","entries.confirmDiscard":"Die nicht veröffentlichten Änderungen verwerfen?","entries.confirmCopy":"Den Inhalt durch den von {site} ersetzen?","entries.fieldLayout":"Feldlayout «{name}»","entries.draftBy":"von {user}, {time}","entries.live":"Live","entries.notPublished":"Nicht veröffentlicht","entries.notSaved":"Nicht gespeichert","entries.savedAsDraft":"Als Entwurf gespeichert, noch nicht live","entries.moveToTrash":"In den Papierkorb legen","entries.discardTitle":"Die nicht veröffentlichten Änderungen verwerfen","entries.liveTitle":"Die Website neben dem Editor, mit dem Entwurf","entries.livePreview":"Live-Vorschau","entries.previewTitle":"Den Entwurf auf der Website öffnen","entries.viewTitle":"Die Seite auf der Website öffnen","entries.mayNotPublish":"Ihre Rolle kann Entwürfe bearbeiten, veröffentlichen tut sie jemand anderes","entries.conflict":"während Sie bearbeitet haben. Ihre letzten Änderungen sind noch nicht gespeichert.","entries.loadTheirs":"Deren Änderungen laden","entries.keepMine":"Meine behalten","entries.confirmType":"Den Eintragstyp zu {type} ändern? Er hat kein Feld wie {fields}: deren Inhalt wird in jeder Website entfernt.","entries.publishedIn":"Veröffentlicht in {site}","entries.notLiveYet":"Noch nicht live, ab {time}","entries.notLiveAnymore":"Nicht mehr live, abgelaufen am {time}","entries.postDate":"Veröffentlichungsdatum","entries.whenPublished":"Beim Veröffentlichen","entries.expiryDate":"Ablaufdatum","entries.fieldsInSetup":"Seine Felder in der Einrichtung","entries.parent":"Übergeordneter Eintrag","entries.noParent":"Keiner, auf oberster Ebene","entries.slug":"Slug","entries.fromTitle":"aus dem Titel","entries.outlineOf":"Übersicht von {field}","entries.outline":"Übersicht · {field}","entries.disabled":"Deaktiviert","entries.missing":"Fehlt","entries.changed":"Geändert","entries.errors":{one:"{n} Fehler",other:"{n} Fehler"},"entries.copyContent":"Inhalt kopieren","entries.fromOtherSite":"Von einer anderen Website…","entries.revisions":"Versionen","entries.revisionsCount":"Versionen · {n}","entries.revisionOf":"Version vom {time}, nur lesen","entries.revisionOfBy":"Version vom {time} von {user}, nur lesen","entries.changedList":"Geändert: {changes}","entries.backToEditing":"Zurück zum Bearbeiten","entries.noContentHere":"Diese Version hat keinen Inhalt für diese Website.","entries.revisionLive":"live","entries.created":"Erstellt","entries.confirmRestore":"Diese Version wiederherstellen?","entries.confirmRestoreDraft":"Diese Version wiederherstellen? Der Entwurf wird ersetzt."},sa={"fields.notHosted":"{kind}-Felder können Plugins nicht einbinden","fields.changed":"Geändert","fields.translated":"Übersetzt: jede Website hat ihren eigenen Wert","fields.inSetup":"Das Feld in der Einrichtung","fields.chooseOption":"Auswählen…","fields.write":"Schreiben","fields.notTranslated":"Nicht übersetzt: {text}","fields.moveEntry":"Eintrag verschieben","fields.removeFromField":"Aus dem Feld entfernen","fields.moveRow":"Zeile verschieben","fields.removeRow":"Zeile entfernen","fields.addRow":"Zeile hinzufügen","fields.moveFile":"Datei verschieben","fields.deletedFile":"Eine gelöschte Datei","fields.detailsTitle":"Details: Titel, Alternativtext, Fokuspunkt","fields.details":"Details","fields.dropToAdd":"Loslassen zum Hinzufügen","fields.dragVideos":"Videos hierher ziehen","fields.dragImages":"Bilder hierher ziehen","fields.dragFiles":"Dateien hierher ziehen","fields.dragTypes":"{types}-Dateien hierher ziehen","fields.blocks":{one:"{n} Block",other:"{n} Blöcke"},"fields.images":{one:"{n} Bild",other:"{n} Bilder"},"fields.collapseAll":"Alle zuklappen","fields.expandAll":"Alle aufklappen","fields.moveBlock":"Block verschieben","fields.searchBlocks":"Blöcke suchen","fields.addBlock":"Block hinzufügen","fields.addBlockHere":"Hier einen Block hinzufügen","fields.moreBlocks":"Weitere Blöcke","fields.more":"Mehr","fields.noLinkYet":"Noch kein Link","fields.isMissing":"{label} fehlt","fields.empty":"Leer","fields.format":"Format","fields.paragraph":"Absatz","fields.heading":"Überschrift {level}","fields.bold":"Fett","fields.italic":"Kursiv","fields.list":"Liste","fields.numberedList":"Nummerierte Liste","fields.link":"Link","fields.insert":"Einfügen","fields.button":"Button","fields.writePlaceholder":"Schreiben …","fields.continueWriting":"Weiterschreiben …","fields.addLineAbove":"Zeile darüber einfügen","fields.addLineBelow":"Zeile darunter einfügen","fields.line":"Zeile","fields.deleteButton":"Button löschen","fields.moveButton":"Button verschieben","fields.openTarget":"Ziel öffnen","fields.styles":"Stile","fields.text":"Text","fields.removeLink":"Link entfernen","fields.linkEntry":"Eintrag","fields.linkUrl":"URL","fields.linkPhone":"Telefon","fields.linkFile":"Datei","fields.noTargetYet":"Noch kein Ziel","fields.aFile":"Eine Datei","fields.target":"Ziel","fields.chooseAnotherEntry":"Anderen Eintrag auswählen","fields.notPublished":"Nicht veröffentlicht","fields.removeEntry":"Eintrag entfernen","fields.anotherFile":"Andere Datei","fields.chooseFile":"Datei auswählen","fields.label":"Beschriftung","fields.fromTarget":"Vom Ziel","fields.anchor":"Anker","fields.anchorPlaceholder":"optional, z. B. kontakt","fields.anchorNote":"Springt zu einem Abschnitt der Zielseite.","fields.openInNewTab":"In einem neuen Tab öffnen","fields.downloadInstead":"Herunterladen statt öffnen","fields.requiredNoLink":"Pflichtfeld – noch kein Link.","fields.noLink":"Noch kein Link.","fields.actions":"Aktionen","fields.closeSettings":"Einstellungen schliessen","fields.editLink":"Link bearbeiten","fields.newTab":"Neuer Tab","fields.linkActions":"Aktionen für den Link","fields.moveLink":"Link verschieben","fields.confirmRemoveLink":"Den Link entfernen?","fields.confirmRemoveLinkChildren":"Den Link mit den Links darunter entfernen?","fields.upToLevels":"Bis zu {n} Ebenen","fields.oneLevel":"Eine Ebene","fields.isOn":"{field} ist an","fields.isOff":"{field} ist aus","fields.isOneOf":"{field} ist {values}","fields.or":"oder","fields.isFilled":"{field} ist ausgefüllt","fields.isEmpty":"{field} ist leer","fields.kindText":"Text","fields.kindTextLines":"Text, mehrzeilig","fields.max":"max. {n}","fields.min":"min. {n}","fields.kindNumber":"Zahl","fields.kindNumberRange":"Zahl {min}–{max}","fields.whole":"ganzzahlig","fields.decimals":"{n} Dezimalstellen","fields.kindBoolean":"Ja / Nein","fields.kindEmail":"E-Mail-Adresse","fields.kindRelation":"Einträge: {types}","fields.kindImages":"Bilder","fields.kindVideos":"Videos","fields.kindFiles":"Dateien","fields.kindLink":"Link zu {to}","fields.sameTab":"gleicher Tab","fields.lists":"Listen","fields.links":"Links","fields.buttons":"Buttons","fields.blocksOf":"Blöcke: {types}","fields.kindRichText":"Rich Text","fields.kindRichTextWith":"Rich Text: {parts}","fields.kindGroup":"Gruppe von {component}","fields.kindDate":"Datum","fields.kindDateTime":"Datum und Uhrzeit","fields.kindCheckboxes":"Checkboxen: {options}","fields.kindTable":"Tabelle: {columns}","fields.kindNavigation":"Navigation zu {to}, {n} Ebenen","fields.kindDropdown":"Auswahlliste: {options}","fields.kindTexts":"Texte: {n} Schlüssel","fields.kindBlocks":"Blöcke: {types}","fields.kindCustom":"{type} (Plugin)","fields.kindNote":"Hinweis für Redaktoren","fields.optWidth":"Breite","fields.widthOfRow":"{n} % einer Zeile","fields.optShownWhen":"Sichtbar, wenn","fields.optText":"Text","fields.optInstructions":"Anleitung","fields.optRequired":"Pflichtfeld","fields.optSites":"Websites","fields.perSite":"Pro Website, übersetzt","fields.sameInAllSites":"Auf allen Websites gleich","fields.optIndexed":"Indexiert","fields.indexedYes":"Ja, Listen und GraphQL filtern und sortieren danach","fields.optDefault":"Standardwert","fields.optLines":"Zeilen","fields.several":"Mehrere","fields.one":"Eine","fields.optMaxLength":"Maximale Länge","fields.optMin":"Min.","fields.optMax":"Max.","fields.optDecimals":"Dezimalstellen","fields.any":"Beliebig","fields.optUnit":"Einheit","fields.optValue":"Wert","fields.dayAndTime":"Ein Tag mit Uhrzeit, 2026-11-14T20:00","fields.day":"Ein Tag, 2026-11-14","fields.optEntryTypes":"Eintragstypen","fields.optMaxEntries":"Max. Einträge","fields.optFiles":"Dateien","fields.imagesOnly":"Nur Bilder","fields.anyFile":"Jede Datei","fields.optMaxFiles":"Max. Dateien","fields.optUploadTo":"Hochladen nach","fields.library":"Dateien","fields.optLinksTo":"Links zu","fields.optExternalLinks":"Externe Links","fields.inNewTab":"In einem neuen Tab","fields.inSameTab":"Im gleichen Tab","fields.optLevels":"Ebenen","fields.optHeadings":"Überschriften","fields.optLists":"Listen","fields.optLinks":"Links","fields.optButtons":"Buttons","fields.optBlocksInText":"Blöcke im Text","fields.styleText":"Text","fields.styleParagraph":"Absatz","fields.optBlockTypes":"Blocktypen","fields.optMinBlocks":"Min. Blöcke","fields.optMaxBlocks":"Max. Blöcke","fields.optFieldsOf":"Felder von","fields.optOptions":"Optionen","fields.optColumns":"Spalten","fields.optKeys":"Schlüssel","fields.optFieldType":"Feldtyp","fields.ofPlugin":"{type}, aus einem Plugin"},ia={"assets.title":"Dateien","assets.library":"Dateien","assets.files":"Dateien","assets.folders":"Ordner","assets.folder":"Ordner","assets.noFolder":"{root} (kein Ordner)","assets.all":"Alle","assets.images":"Bilder","assets.videos":"Videos","assets.documents":"Dokumente","assets.noAssets":"Noch keine Dateien","assets.noFilesInFolder":"Keine Dateien in diesem Ordner","assets.inAllFolders":"In allen Ordnern","assets.searchFiles":"Dateien suchen","assets.type":"Typ","assets.size":"Grösse","assets.dimensions":"Abmessungen","assets.uploaded":"Hochgeladen","assets.uploadedSize":"hochgeladen {width} × {height}","assets.uploadedBy":"von {email}","assets.deliveredAsPng":"als PNG ausgeliefert","assets.altText":"Alternativtext","assets.altTextOf":"Alternativtext ({site})","assets.altPlaceholder":"Was das Bild zeigt","assets.titlePlaceholder":"Aus dem Dateinamen","assets.filename":"Dateiname","assets.confirmDiscard":"Die Änderungen verwerfen?","assets.copyUrl":"URL kopieren","assets.replaceFile":"Datei ersetzen","assets.usedIn":"Verwendet in","assets.notUsed":"In keinem Eintrag verwendet","assets.deleteFile":"Datei löschen","assets.details":"Details: Titel, Alternativtext, Fokuspunkt","assets.fileActions":"Dateiaktionen","assets.confirmTrash":"{filename} in den Papierkorb legen?","assets.confirmTrashUsed":{one:"{filename} in den Papierkorb legen? Die Datei wird in {n} Eintrag verwendet, dort fehlt sie, bis sie wiederhergestellt wird.",other:"{filename} in den Papierkorb legen? Die Datei wird in {n} Einträgen verwendet, dort fehlt sie, bis sie wiederhergestellt wird."},"assets.confirmUnzip":`{name} in Ordner entpacken?

OK entpackt, Abbrechen lädt sie als eine ZIP-Datei hoch.`,"assets.confirmUnzipMany":`Die {n} ZIP-Dateien in Ordner entpacken?

OK entpackt, Abbrechen lädt sie als ZIP-Dateien hoch.`,"assets.moveTo":"Nach {folder} verschieben","assets.zipping":"{name} wird gezippt…","assets.uploadFolder":"Ordner hochladen","assets.uploading":"Lädt hoch…","assets.uploadingOf":"{n} von {total} wird hochgeladen","assets.uploadedFiles":{one:"{n} Datei hochgeladen",other:"{n} Dateien hochgeladen"},"assets.sentOf":"{sent} von {total}","assets.dropFailed":"Beim Ablegen: {error}","assets.dropToUpload":"Zum Hochladen ablegen","assets.dropInto":"Zum Hochladen nach {path} ablegen","assets.chooseImage":"Bild auswählen","assets.chooseImages":"Bilder auswählen","assets.chooseFile":"Datei auswählen","assets.chooseFiles":"Dateien auswählen","assets.newFolder":"Neuer Ordner","assets.folderName":"Ordnername","assets.downloadZip":"Als ZIP herunterladen","assets.confirmDeleteFolder":"Den Ordner {name} löschen?","assets.collapse":"Zuklappen","assets.expand":"Aufklappen","assets.editImage":"Bild bearbeiten","assets.tool":"Werkzeug","assets.focalPoint":"Fokuspunkt","assets.crop":"Zuschneiden","assets.rotateLeft":"Nach links drehen","assets.rotateRight":"Nach rechts drehen","assets.flipH":"Horizontal spiegeln","assets.flipV":"Vertikal spiegeln","assets.resetTitle":"Zurück zum hochgeladenen Bild","assets.reset":"Zurücksetzen","assets.ratio":"Seitenverhältnis","assets.free":"Frei","assets.original":"Original","assets.square":"Quadratisch","assets.portrait":"Hochformat","assets.cropNote":"Ziehen Sie den Ausschnitt über das Bild hinaus, um Platz darum herum zu schaffen.","assets.transparentNote":"Transparenz braucht ein PNG: Ein JPEG wird dann als PNG ausgeliefert.","assets.focalNote":"Klicken Sie auf das Bild, um den Fokuspunkt zu setzen, die Zuschnitte der Grössen behalten ihn im Bild.","assets.width":"Breite","assets.height":"Höhe","assets.originalSize":"Originalgrösse","assets.canvas":"Leinwand darum herum","assets.canvasWidth":"Leinwandbreite","assets.canvasHeight":"Leinwandhöhe","assets.background":"Hintergrund","assets.transparent":"Transparent"},ra={"setup.setup":"Einrichtung","setup.readOnly":"Nur lesen, das Inhaltsmodell ist Code","setup.noSectionThere":"Es gibt keinen Abschnitt {handle}","setup.noEntryType":"Kein Eintragstyp {handle}","setup.noBlock":"Kein Block {handle}","setup.noSection":"Kein Abschnitt {handle}","setup.everyAsset":"jede Datei","setup.sectionName":"Abschnitt {name}","setup.handle":"Handle","setup.kind":"Art","setup.fields":"Felder","setup.entryTypes":"Eintragstypen","setup.entries":"Einträge","setup.youMay":"Sie dürfen","setup.usedIn":"Verwendet in","setup.nothing":"nichts","setup.nothingHidden":"nichts, ausgeblendet","setup.admins":"Admins (ohne Rolle)","setup.everything":"Alles","setup.single":"Single: genau ein Eintrag","setup.channel":"Channel: eine Liste, neueste zuerst","setup.structure":"Structure: ein Baum in fester Reihenfolge","setup.general":"Allgemein","setup.maxLevel":"Maximale Ebene","setup.maxLevelOne":"1, nur Einträge auf oberster Ebene","setup.maxLevelNone":"keine, so tief wie nötig","setup.pages":"Seiten","setup.pagesIf":"nur Einträge mit {field} eingeschaltet","setup.navigation":"Navigation","setup.noPages":"keine Seiten","setup.home":"/ (Startseite)","setup.sites":"Websites","setup.site":"Website","setup.language":"Sprache","setup.urls":"URLs","setup.status":"Status","setup.offline":"Offline","setup.online":"Online","setup.access":"Zugriff","setup.role":"Rolle","setup.may":"Darf","setup.allRoles":"Alle Rollen","setup.unpublishedChanges":"Nicht veröffentlichte Änderungen","setup.theFirst":"Die ersten {n}.","setup.openInContent":"In Inhalte öffnen","setup.other":"Weitere","setup.shownWhen":"sichtbar, wenn {condition}","setup.inACard":"in einer Karte","setup.label":"Bezeichnung","setup.perSite":"pro Website","setup.allSites":"alle Websites","setup.required":"Pflichtfeld","setup.conditional":"bedingt","setup.indexed":"indexiert","setup.title":"Titel","setup.noFieldsHere":"Keine Felder hier.","setup.onlyTitle":"Keine Felder, nur ein Titel.","setup.mail":"E-Mail","setup.mailsGoTo":"E-Mails gehen an {server} mit dem Absender {from}.","setup.sendTest":"Test-E-Mail an mich senden","setup.sentTo":"An {email} gesendet.","setup.noSmtp":"Kein SMTP-Server: E-Mails werden nur protokolliert. Starten Sie das CMS mit {smtp} und {from}.","setup.url":"URL","setup.primary":" (primär)","setup.offlineSite":" (offline, nicht auf der Website)","setup.transforms":"Bildformate","setup.auto":"auto","setup.cropped":", um den Fokuspunkt zugeschnitten","setup.roles":"Rollen","setup.youAre":"Sie sind {who}. Ansehen zeigt Einträge, Bearbeiten ändert sie als Entwürfe und erstellt neue, Veröffentlichen macht Entwürfe live und verschiebt Einträge, Löschen löscht sie. Benutzer ohne Rolle sind Admins.","setup.anAdmin":"Admin und können alles","setup.assets":"Dateien","setup.ai":"KI","setup.notInNav":"Nicht in der Navigation","setup.typesAndBlocks":"Typen und Blöcke","setup.fieldTypes":"Feldtypen","setup.transformsShort":"Bildformate","setup.nFields":{one:"{n} Feld",other:"{n} Felder"},"setup.noSectionsVisible":"Keine Abschnitte, die Sie sehen dürfen.","setup.unusedBlocks":"Nirgends verwendete Blöcke","setup.plugins":"Plugins","setup.nothingInModel":"Nichts im Inhaltsmodell.","setup.more":"Weiteres","setup.graphqlNote":"Tokens der Frontends, GraphiQL","setup.rolesNote":"wer was darf","setup.sitesAndTransforms":"Websites und Bildformate","setup.mailNote":"wie E-Mails gesendet werden","setup.nOpen":{one:"{n} offen",other:"{n} offen"},"setup.requestsNote":"Änderungen, um die Redaktoren gebeten haben","setup.searchField":"Feld oder Block suchen","setup.nOfAll":"{n} von {all}","setup.holds":"Enthält","setup.in":"In","setup.noFieldLike":"Kein solches Feld","setup.requestChange":"Änderung anfragen","setup.requestChangeTitle":"Die Entwickler hier um eine Änderung bitten","setup.requestSent":"Gesendet, die Entwickler sehen es unter Anfragen.","setup.whatDoYouNeed":"Was brauchen Sie bei {context}?","setup.requestPlaceholder":"Ich brauche ein Feld für einen Untertitel","setup.send":"Senden","setup.requestsIntro":"Redaktoren bitten mit «Änderung anfragen» auf den Seiten der Einrichtung um Änderungen. Admins erhalten zu jeder eine E-Mail, wenn E-Mails eingerichtet sind.","setup.request":"Anfrage","setup.where":"Wo","setup.who":"Wer","setup.when":"Wann","setup.noRequests":"Noch keine Anfragen"},oa={"search.placeholder":"Einträge und Dateien suchen","search.searching":"Sucht…","search.notPublished":"Nicht veröffentlicht","search.assets":"Dateien"},la={"trash.title":"Papierkorb","trash.note":"Gelöschte Einträge und Dateien, für {days} Tage","trash.empty":"Papierkorb leeren","trash.restored":"Wiederhergestellt:","trash.isEmpty":"Der Papierkorb ist leer.","trash.entries":"Einträge","trash.entry":"Eintrag","trash.section":"Abschnitt","trash.deleted":"Gelöscht","trash.assets":"Dateien","trash.asset":"Datei","trash.size":"Grösse","trash.below":{one:"und {n} Eintrag darunter",other:"und {n} Einträge darunter"},"trash.by":"von {email}","trash.deleteForGood":"Endgültig löschen","trash.deleteNameForGood":"{name} endgültig löschen","trash.confirmEntry":"{name} endgültig löschen? Dies kann nicht wiederhergestellt werden.","trash.confirmEntryChildren":{one:"{name} endgültig löschen? Und {n} Eintrag darunter. Dies kann nicht wiederhergestellt werden.",other:"{name} endgültig löschen? Und {n} Einträge darunter. Dies kann nicht wiederhergestellt werden."},"trash.confirmAsset":"{name} endgültig löschen? Dies kann nicht wiederhergestellt werden.","trash.confirmAll":{one:"{n} Element endgültig löschen? Es kann nicht wiederhergestellt werden.",other:"{n} Elemente endgültig löschen? Sie können nicht wiederhergestellt werden."}},aa={"graphql.newToken":"Neuer Token","graphql.copyNow":"Kopieren Sie den Token jetzt, er wird nicht mehr angezeigt. Ein Frontend sendet ihn als {header} ; crelte liest ihn als {env} .","graphql.theToken":"Der Token","graphql.namePlaceholder":"Website","graphql.expires":"Läuft ab","graphql.sections":"Abschnitte","graphql.allSections":"Alle Abschnitte","graphql.sites":"Websites","graphql.allSites":"Alle Websites","graphql.drafts":"Entwürfe","graphql.previewsShowDrafts":"Vorschau-Links zeigen Entwürfe","graphql.previewsNote":"Für die Website, die die Vorschau des Studios zeigt; andere Frontends lesen nur, was live ist.","graphql.all":"Alle","graphql.confirmRevoke":"{name} widerrufen? Frontends, die ihn senden, erhalten keine Inhalte mehr.","graphql.graphiqlTitle":"GraphiQL mit Ihrer Sitzung","graphql.tryIt":"Ausprobieren","graphql.intro":"Frontends senden einen Token als {header} an {path} . Jeder liest nur seine Abschnitte und Websites.","graphql.envToken":"Der Token von {env} liest alles.","graphql.token":"Token","graphql.lastUsed":"Zuletzt verwendet","graphql.noTokens":"Noch keine Tokens","graphql.expired":"Abgelaufen","graphql.revoke":"Token widerrufen"},ua={"ai.title":"KI","ai.width":"Breite des Chats","ai.startOver":"Neu beginnen, dieser Chat bleibt nur in diesem Tab","ai.newChat":"Neuer Chat","ai.hintAgent":"Entwicklung: Fragen Sie nach allem, Inhalt, Design oder neuen Feldern, z. B. «Füge ein Feld für die Öffnungszeiten hinzu» oder «Mache die Buttons rund». Code-Änderungen erscheinen sofort rechts, Inhaltsänderungen sind Entwürfe, die Sie veröffentlichen.","ai.hint":"Fragen Sie nach allem auf der Website, z. B. «Schreibe eine bessere Einleitung für Über uns», «Füge einen Anlass in Basel hinzu» oder «Übersetze den Footer ins Englische». Änderungen sind Entwürfe: Sie sehen sie rechts und veröffentlichen sie im Editor.","ai.placeholder":"Was soll sich ändern?","ai.message":"Nachricht","ai.keys":"Enter sendet, Shift+Enter neue Zeile","ai.send":"Senden","ai.endedEarly":"Die Antwort endete zu früh"},da={...Jl,...Ql,...Xl,...ea,...na,...ta,...sa,...ia,...ra,...oa,...la,...aa,...ua},ca={actions:"Actions",back:"Back",breadcrumbs:"Breadcrumbs",cantMove:"It can't move",changed:"Changed",clear:"Clear",close:"Close",collapse:"Collapse",date:"Date",disabled:"Disabled",dragOrClick:"drag, or click for the moves",done:"Done",duplicate:"Duplicate",expand:"Expand",forward:"Forward",hasErrors:"Has errors",hide:"Hide it on the website",loading:"Loading…",monthAfter:"Month after",monthBefore:"Month before",move:"Move",moveDown:"Move down",moveIn:"Move under the one above",moveOut:"Move out a level",moveUp:"Move up",noDate:"No date",nothingFound:"Nothing found",nothingYet:"Nothing yet",pickDay:"Pick a day",pickMonth:"Pick a month",pickYear:"Pick a year",remove:"Remove",resize:"Drag to change the width, double click for the default",search:"Search",show:"Show it on the website",time:"time",today:"Today"},hs=Symbol.for("zipp-ui"),ha=me({texts:ca,locale:void 0}),fs=globalThis,wi=fs[hs]??(fs[hs]=ha),fa={en:"English",de:"Deutsch"},pa={en:Tr,de:da};function Ar(n){return!!n&&n in fa}function ga(){var e;const n=(((e=navigator.languages)==null?void 0:e[0])??navigator.language??"").slice(0,2).toLowerCase();return Ar(n)?n:"en"}function Us(){var e;const n=(e=z.user)==null?void 0:e.language;return Ar(n)?n:ga()}function va(){const n=Us(),e=navigator.language;return e.slice(0,2).toLowerCase()===n?e:n}function N(n,e={}){const t=Us();let s=pa[t][n]??Tr[n];if(typeof s!="string"){const i=Number(e.n);s=new Intl.PluralRules(t).select(i)==="one"?s.one:s.other}return s.replace(/\{(\w+)\}/g,(i,r)=>r in e?String(e[r]):i)}function ma(){const n=Object.keys(Er).map(e=>e.slice(3));return Object.fromEntries(n.map(e=>[e,N(`ui.${e}`)]))}function wa(){wi.texts=ma(),wi.locale=va(),document.documentElement.lang=Us()}class Yt extends Error{constructor(e,t,s=0){super(e),this.inner=t,this.status=s}get type(){return this.inner.type}get fields(){return this.inner.type==="INVALID"?this.inner.detail:[]}static fromJson(e,t,s){var r;const i=(r=t==null?void 0:t.error)!=null&&r.type?t.error:{type:"REQUEST",detail:s};return new Yt((t==null?void 0:t.message)??s,i,e)}}let Ir=()=>{};function _a(n){Ir=n}const _n="zipp-token";let Lr=localStorage.getItem(_n)??sessionStorage.getItem(_n),Cr=localStorage.getItem(_n)!==null;function Ot(){return Lr}function _i(){return Cr}function Et(n,e=!0){Lr=n,Cr=n!==null&&e,localStorage.removeItem(_n),sessionStorage.removeItem(_n),n&&(e?localStorage:sessionStorage).setItem(_n,n)}function Ws(n,e,t,s){const i={},r=Ot();r&&(i["session-token"]=r);const o=t instanceof FormData;return t!==void 0&&!o&&(i["content-type"]="application/json"),fetch("/api"+e,{method:n,headers:i,signal:s,body:t===void 0||o?t:JSON.stringify(t)})}function Mr(n){return n.type==="MISSING_SESSION_TOKEN"||n.type==="INVALID_SESSION_TOKEN"}function Gt(n,e){const t=Yt.fromJson(n.status,e,n.statusText);return Mr(t)&&Ir(),t}async function G(n,e,t){const s=await Ws(n,e,t),i=await s.text();let r=null;try{r=i?JSON.parse(i):null}catch{if(s.ok)throw new Error(N("shell.noJson",{path:e}))}if(!s.ok)throw Gt(s,r);return r}async function sc(n){const e=await Ws("GET",n);if(!e.ok)throw Gt(e,await e.json().catch(()=>null));return e.blob()}function ic(n,e,t){return new Promise((s,i)=>{const r=new XMLHttpRequest;r.open("POST","/api"+n);const o=Ot();o&&r.setRequestHeader("session-token",o),r.upload.onprogress=l=>t(l.loaded,l.total),r.onerror=()=>i(new Error(N("shell.uploadFailed",{path:n}))),r.onload=()=>{let l=null;try{l=r.responseText?JSON.parse(r.responseText):null}catch{if(r.status<300)return i(new Error(N("shell.noJson",{path:n})))}r.status>=200&&r.status<300?s(l):i(Gt(r,l))},r.send(e)})}async function*rc(n,e,t,s){const i=await Ws(n,e,t,s);if(!i.ok||!i.body)throw Gt(i,await i.json().catch(()=>null));const r=i.body.pipeThrough(new TextDecoderStream).getReader();try{yield*ka(r)}finally{r.cancel().catch(()=>{})}}async function*ka(n){let e="";for(;;){const{value:t,done:s}=await n.read();if(s)return;e+=t;let i;for(;(i=e.indexOf(`

`))>=0;){const r=e.slice(0,i).split(`
`);e=e.slice(i+2);const o=l=>r.filter(u=>u.startsWith(l+":")).map(u=>u.slice(l.length+1).trimStart()).join(`
`);yield{event:o("event"),id:o("id"),data:o("data")}}}}const Mn=me({open:null,saved:0});function oc(n){Mn.open={kind:"entry",id:n}}function lc(n){Mn.open={kind:"asset",id:n}}function ac(){Mn.open=null,Mn.saved++}const Ft=new Set;function uc(n){return Ft.add(n),()=>Ft.delete(n)}const Dr=()=>[...Ft].some(n=>n());function ba(){return Dr()?confirm(N("common.confirmLeave"))?(Ft.clear(),!0):!1:!0}typeof window<"u"&&window.addEventListener("beforeunload",n=>{Dr()&&n.preventDefault()});const ya="/_studio/lucide-1.65.0",Ys=new Map,ki=new Map;function xa(n){for(const[e,t]of Object.entries(n))Ys.set(e,t)}function dc(n){return Ys.get(n)}function cc(n){const e=Ys.get(n);if(e)return Promise.resolve(e);if(!/^[a-z0-9-]+$/.test(n))return Promise.resolve(void 0);let t=ki.get(n);return t||(t=fetch(`${ya}/${n}.svg`).then(s=>s.ok?s.text():void 0).then(s=>s!=null&&s.includes("<svg")?s:void 0).catch(()=>{}),ki.set(n,t)),t}const Sa=["handle","label","required","indexed","translatable","instructions","default","width","when"];function hc(n,e){const t=s=>JSON.stringify(Object.entries(s).filter(([i])=>!Sa.includes(i)));return t(n)===t(e)}function fc(n,e){return n[e.handle]}async function Ea(){const n=await G("GET","/schema");return xa(n.icons??{}),n}function Ta(n){return n.nav.length?n.nav:[{name:N("shell.entries"),sections:n.sections.map(e=>e.handle)}]}function pc(n,e,t){const s=n.siteUrls[e];return t===null||!s?null:`${s.replace(/\/$/,"")}/${t}`}function Pr(n,e){return n.components.find(t=>t.handle===e)}function Aa(n,e){var s;const t={};for(const i of((s=Pr(n,e))==null?void 0:s.fields)??[])if(i.default!==void 0)t[i.handle]=structuredClone(i.default);else if(i.kind==="group"){const r=Aa(n,i.component);Object.keys(r).length&&(t[i.handle]=r)}return t}function gc(n,e){var t;return((t=Pr(n,e))==null?void 0:t.name)??e}function Gs(n,e){return n.sections.find(t=>t.handle===e)}function vc(n,e){var t;return((t=Gs(n,e))==null?void 0:t.name)??e}class Ia{constructor(e){y(this,"redirect",null);this.req=e}get session(){return z}schema(){return Da()}setRedirect(e){this.redirect=e}}const Dn="/admin",j=new Bl;var at,ut;class La{constructor(){k(this,at,P(""));k(this,ut,Ns(()=>{var e;return((e=this.path.match(/^\/admin\/(?:entries|docs)\/([^/]+)/))==null?void 0:e[1])??""}))}get path(){return b(a(this,at))}set path(e){L(a(this,at),e,!0)}get section(){return b(a(this,ut))}set section(e){L(a(this,ut),e)}}at=new WeakMap,ut=new WeakMap;const Ca=new La;function mc(n,e){return`${Dn}/entries/${n}/${e}`}function wc(n,e=!1){j.open(n,{},{history:e?"replace":"push"})}const Ks=me({current:localStorage.getItem("zipp-site")??""});function Br(n){Ks.current=n,localStorage.setItem("zipp-site",n)}function _c(n){Br(n),j.currentRequest.get()&&j.reload()}function Ma(){if(location.pathname===Dn+"/login")return;const n=new URLSearchParams({url:location.pathname+location.search});j.open(`${Dn}/login?${n}`,{},{history:"replace"})}let vn=null;function Da(){if(vn)return vn;const n=Ea().then(e=>(e.sites.some(t=>t.handle===Ks.current)||Br(e.sites[0].handle),e));return n.catch(()=>{vn===n&&(vn=null)}),vn=n}function Pa(){vn=null}const Te=me({page:null,loading:0});function Ba(n,e){return async t=>{const[s,i]=await Promise.all([n(t),e(t)]);return{isPublic:i.isPublic,async loadProps(r,o){var u,d;const l=await((u=s.loadProps)==null?void 0:u.call(s,r,o))??{};r={...r,...l};try{const c=await((d=i.loadProps)==null?void 0:d.call(i,r,o))??{};return{...r,...c}}catch(c){return{...r,loadError:c}}},layout:s.default,default:i.default}}}function Ge(n,e){for(const[t,s]of e)j.register(t,Ba(n,s))}let Ct;async function Ra(n,e){var s;const t=n.pathname+n.url.search+"|"+Ks.current;if(!e)return{page:{layout:null,component:(await Ct.notFound()).default,props:{},key:t,isPublic:!0}};try{const i=await e.load(n);if(!i.isPublic&&!z.loggedIn){const u=new URLSearchParams({url:n.uri});return{redirect:`${Dn}/login?${u}`}}const r=new Ia(n);let o=e.toProps(n);if(o={...o,...await((s=i.loadProps)==null?void 0:s.call(i,o,r))??{}},r.redirect)return{redirect:r.redirect};const l=o.loadError?(await Ct.error()).default:i.default;return{page:{layout:i.layout??null,component:l,props:o,key:t,isPublic:!!i.isPublic}}}catch(i){return{page:{layout:null,component:(await Ct.error()).default,props:{loadError:i},key:t,isPublic:!1}}}}function Cs(n=!0){z.invalidate(),Pa(),Mn.open=null,n&&Ma()}function Na(){z.loggedIn&&Te.page&&!Te.page.isPublic?z.expire():Cs()}function Oa(){var n;(n=Te.page)!=null&&n.props.loadError&&j.reload()}function Fa(n,e){Ct=n,_a(Na),z.onRenewed(Oa),j.currentRequest.subscribe(i=>{i&&(Ca.path=i.pathname)});let t=!1,s=0;j.onRoute(async(i,r,o)=>{if(!ba()){i.origin==="pop"&&j.pushReq(j.currentRequest.get());return}const l=++s;Te.loading++;try{const u=await Ra(i,r);if("redirect"in u){if(l!==s)return;const c=!j.currentRequest.get()||i.origin==="pop"?"replace":"push";j.open(u.redirect,{},{history:c}),await o.dataReady();return}if(await o.dataReady())return;Te.page&&Te.page.key!==u.page.key&&(Mn.open=null),Te.page=u.page,t||(t=!0,e())}finally{Te.loading--}await Promise.resolve(),await o.domReady(),za(i,()=>l===s)}),j.initClient()}function za(n,e){const t=decodeURIComponent(n.url.hash.slice(1));if(!t)return;let s=0;const i=()=>{if(!e())return;const r=document.getElementById(t);if(!r){++s<30&&setTimeout(i,100);return}r.scrollIntoView({block:"center"}),r.classList.add("target"),setTimeout(()=>r.classList.remove("target"),2e3)};setTimeout(i,100)}document.addEventListener("click",n=>{var s,i;const e=(i=(s=n.target).closest)==null?void 0:i.call(s,"a");if(!e)return;const t=e.pathname===Dn||e.pathname.startsWith(Dn+"/");(e.origin!==location.origin||!t||e.hasAttribute("download"))&&n.stopPropagation()});var qa=un("<div></div>");function ja(n,e){ft(e,!0);let t=P(0),s=P(!1),i=0,r;or(()=>{i<e.loading?(L(t,.8),L(s,!0)):i!==0&&e.loading===0&&(L(t,1),L(s,!1),clearTimeout(r),r=setTimeout(()=>L(t,0),500)),i=e.loading});var o=qa();let l,u;wn(()=>{l=yl(o,1,"loader svelte-5sz1ho",null,l,{visible:b(s)}),u=Sr(o,"",u,{"--progress":b(t)})}),oe(n,o),pt()}function kc(){return G("GET","/users")}function bc(n){return G("GET",`/users/${n}`)}function yc(n){return G("POST","/users",n)}function xc(n,e){return G("PUT",`/users/${n}`,e)}function Sc(n){return G("DELETE",`/users/${n}`)}function Ec(n){return G("DELETE",`/users/${n}/sessions`)}function Tc(n,e){return G("POST",`/users/${n}/password-link`,{send:e})}function Ac(n){return G("POST",`/users/${n}/login-as`)}function Va(n){return n.name||n.email}const Ha=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ua=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ya=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ga=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ka=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$a=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Za=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Ja=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Qa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xa=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,eu=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,nu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,tu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,su=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,iu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ru=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ou=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,lu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,au=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,uu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,du=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,cu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,hu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,fu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,pu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,gu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,vu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,mu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,wu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,_u=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ku=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,bu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,yu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,xu=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Su=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Eu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Tu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Au=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Iu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Lu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Cu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Mu=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Du=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Pu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Bu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ru=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Nu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ou=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Fu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,zu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,qu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ju=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Vu=`<!-- @license lucide-static v1.65.0 - ISC -->
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
`,Hu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Uu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Wu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Yu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Gu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ku=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,$u=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Zu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Ju=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Qu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,Xu=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ed=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,nd=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,td=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,sd=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,id=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,rd=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,od=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`,ld=`<!-- @license lucide-static v1.64.0 - ISC -->
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
`;var ad=un('<span class="icon svelte-119qjlr"></span>');function ud(n,e){ft(e,!0);const t=Object.assign({"./icons/add.svg":Ha,"./icons/alert.svg":Ua,"./icons/all_pages.svg":Wa,"./icons/arrow-down.svg":Ya,"./icons/arrow-right.svg":Ga,"./icons/arrow-up.svg":Ka,"./icons/bold.svg":$a,"./icons/calendar.svg":Za,"./icons/channel.svg":Ja,"./icons/check.svg":Qa,"./icons/chevron-down.svg":Xa,"./icons/chevron-left.svg":eu,"./icons/chevron-right.svg":nu,"./icons/chevrons-up-down.svg":tu,"./icons/close.svg":su,"./icons/copy-url.svg":iu,"./icons/copy.svg":ru,"./icons/crop.svg":ou,"./icons/dashboard.svg":lu,"./icons/default.svg":au,"./icons/delete.svg":uu,"./icons/desktop.svg":du,"./icons/doc.svg":cu,"./icons/download.svg":hu,"./icons/edit.svg":fu,"./icons/entries.svg":pu,"./icons/external.svg":gu,"./icons/file.svg":vu,"./icons/flip-h.svg":mu,"./icons/flip-v.svg":wu,"./icons/focal.svg":_u,"./icons/folder-plus.svg":ku,"./icons/folder.svg":bu,"./icons/globe.svg":yu,"./icons/graphql.svg":xu,"./icons/grip.svg":Su,"./icons/help.svg":Eu,"./icons/hide.svg":Tu,"./icons/indent.svg":Au,"./icons/info.svg":Iu,"./icons/italic.svg":Lu,"./icons/link.svg":Cu,"./icons/live-preview.svg":Mu,"./icons/logout.svg":Du,"./icons/mail.svg":Pu,"./icons/mobile.svg":Bu,"./icons/more.svg":Ru,"./icons/move.svg":Nu,"./icons/numbered.svg":Ou,"./icons/outdent.svg":Fu,"./icons/page.svg":zu,"./icons/page_cat.svg":qu,"./icons/phone.svg":ju,"./icons/play.svg":Vu,"./icons/reload.svg":Hu,"./icons/replace.svg":Uu,"./icons/reset.svg":Wu,"./icons/rotate-left.svg":Yu,"./icons/rotate-right.svg":Gu,"./icons/save.svg":Ku,"./icons/search.svg":$u,"./icons/settings.svg":Zu,"./icons/sidebar.svg":Ju,"./icons/single.svg":Qu,"./icons/size.svg":Xu,"./icons/sparkles.svg":ed,"./icons/structure.svg":nd,"./icons/tablet.svg":td,"./icons/translate.svg":sd,"./icons/unlink.svg":id,"./icons/upload.svg":rd,"./icons/users.svg":od,"./icons/view.svg":ld});let s=Cl(e,"size",3,16);var i=ad();let r;wl(i,()=>t[`./icons/${e.name}.svg`],!0),wn(()=>r=Sr(i,"",r,{width:`${s()/16}rem`,height:`${s()/16}rem`})),oe(n,i),pt()}var dd=un('<div class="form svelte-1p9d86a"><div class="intro svelte-1p9d86a"><h2 id="session-ended" class="svelte-1p9d86a"> </h2> <p class="muted svelte-1p9d86a"> </p></div> <div class="actions svelte-1p9d86a"><button type="button"> </button> <button class="primary"> </button></div></div>'),cd=un('<p class="error"> </p>'),hd=un('<form class="form svelte-1p9d86a"><div class="intro svelte-1p9d86a"><h2 id="session-ended" class="svelte-1p9d86a"> </h2> <p class="muted svelte-1p9d86a"> </p></div> <label class="stacked"> <input type="email" autocomplete="username" required=""/></label> <label class="stacked"> <span class="password svelte-1p9d86a"><input autocomplete="current-password" required="" class="svelte-1p9d86a"/> <button type="button" class="icon svelte-1p9d86a"><!></button></span></label> <!> <div class="actions svelte-1p9d86a"><button type="button"> </button> <button class="primary"> </button></div></form>'),fd=un('<dialog aria-labelledby="session-ended" closedby="none" class="svelte-1p9d86a"><!></dialog>');function pd(n,e){var p;ft(e,!0);let t=P(me(((p=z.user)==null?void 0:p.email)??"")),s=P(""),i=P(!1),r=P(""),o=P(!1),l;or(()=>l.showModal());async function u(v){v.preventDefault(),L(r,""),L(o,!0);try{await z.renew(b(t),b(s))||location.assign("/admin")}catch(_){L(r,_.message,!0)}finally{L(o,!1)}}async function d(){L(o,!0);const v=await z.endLoginAs();location.assign(v?"/admin/users":"/admin/login")}var c=fd(),h=he(c);{var g=v=>{var _=dd(),I=he(_),R=he(I),K=Be(R,!0),Pe=$(R,2),D=Be(Pe,!0),ue=$(I,2),de=he(ue),Kt=Be(de,!0),cn=$(de,2),_t=Be(cn,!0);wn(($t,hn,On,Zt)=>{fe(K,$t),fe(D,hn),fe(Kt,On),cn.disabled=b(o),fe(_t,Zt)},[()=>N("auth.sessionEnded"),()=>N("auth.wereLoggedInAs",{name:Va(z.user)}),()=>N("auth.logIn"),()=>N("auth.backToAccount")]),bt("click",de,()=>Cs()),bt("click",cn,d),oe(v,_)},f=v=>{var _=hd(),I=he(_),R=he(I),K=Be(R,!0),Pe=$(R,2),D=Be(Pe,!0),ue=$(I,2),de=he(ue),Kt=$(de),cn=$(ue,2),_t=he(cn),$t=$(_t),hn=he($t);Do(hn);var On=$(hn,2),Zt=he(On);{let ce=Ns(()=>b(i)?"hide":"view");ud(Zt,{get name(){return b(ce)}})}var $s=$(cn,2);{var Nr=ce=>{var kt=cd(),Qt=Be(kt,!0);wn(()=>fe(Qt,b(r))),oe(ce,kt)};Nt($s,ce=>{b(r)&&ce(Nr)})}var Or=$($s,2),Jt=he(Or),Fr=Be(Jt,!0),Zs=$(Jt,2),zr=Be(Zs,!0);wn((ce,kt,Qt,qr,jr,Vr,Hr)=>{fe(K,ce),fe(D,kt),fe(de,`${Qt??""} `),fe(_t,`${qr??""} `),hi(hn,"type",b(i)?"text":"password"),hi(On,"title",jr),fe(Fr,Vr),Zs.disabled=b(o),fe(zr,Hr)},[()=>N("auth.sessionEnded"),()=>N("auth.logInAgain"),()=>N("common.email"),()=>N("common.password"),()=>b(i)?N("auth.hidePassword"):N("auth.showPassword"),()=>N("auth.logOut"),()=>N("auth.logIn")]),ts("submit",_,u),pi(Kt,()=>b(t),ce=>L(t,ce)),pi(hn,()=>b(s),ce=>L(s,ce)),bt("click",On,()=>L(i,!b(i))),bt("click",Jt,()=>Cs()),oe(v,_)};Nt(h,v=>{z.impersonatedBy&&z.user?v(g):v(f,-1)})}Al(c,v=>l=v,()=>l),ts("cancel",c,v=>v.preventDefault()),ts("close",c,()=>l.isConnected&&l.showModal()),oe(n,c),pt()}rl(["click"]);var gd=un("<!> <!> <!>",1);function vd(n,e){ft(e,!0);const t=Ns(()=>Te.page);Ko(wa);var s=gd(),i=pn(s);ja(i,{get loading(){return Te.loading}});var r=$(i,2);{var o=c=>{var h=zn(),g=pn(h);os(g,()=>b(t).layout,(f,p)=>{p(f,cs(()=>b(t).props,{children:(v,_)=>{var I=zn(),R=pn(I);ui(R,()=>b(t).key,K=>{var Pe=zn(),D=pn(Pe);os(D,()=>b(t).component,(ue,de)=>{de(ue,cs(()=>b(t).props))}),oe(K,Pe)}),oe(v,I)},$$slots:{default:!0}}))}),oe(c,h)},l=c=>{var h=zn(),g=pn(h);ui(g,()=>b(t).key,f=>{var p=zn(),v=pn(p);os(v,()=>b(t).component,(_,I)=>{I(_,cs(()=>b(t).props))}),oe(f,p)}),oe(c,h)};Nt(r,c=>{b(t).layout?c(o):c(l,-1)})}var u=$(r,2);{var d=c=>{pd(c,{})};Nt(u,c=>{z.expired&&c(d)})}oe(n,s),pt()}const Ke=()=>O(()=>import("./Shell-DRkWzPNg.js"),__vite__mapDeps([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32]));function md(){Ge(Ke,[["/admin/ai",()=>O(()=>import("./Ai-D7Rop8uJ.js"),__vite__mapDeps([33,5,6,1,7,2,25,15,16,26,9,34]))]])}function wd(){Ge(Ke,[["/admin/assets",()=>O(()=>import("./AssetsPage-DQJ6XYfb.js"),__vite__mapDeps([35,3,6,1,7,8,9,10,2,11,12,13,14,15,16,17,18,5,19,4,20,21,22,36]))]])}function _d(){j.register("/admin/login",()=>O(()=>import("./Login-ByL9RlSl.js"),__vite__mapDeps([37,38,1,39]))),j.register("/admin/forgot",()=>O(()=>import("./Forgot-DbMkqs3X.js"),__vite__mapDeps([40,38,1,39]))),j.register(/\/admin\/password\/(?<token>[0-9a-f]+)/,()=>O(()=>import("./SetPassword-C-7AEfvI.js"),__vite__mapDeps([41,38,1,39]))),j.register(/\/admin\/login-as\/(?<token>[0-9a-f]+)/,()=>O(()=>import("./LoginAs-C2twZsdk.js"),__vite__mapDeps([42,5,38,1,39])))}function kd(){Ge(Ke,[[/\/admin\/entries\/(?<section>[^/]+)/,()=>O(()=>import("./List-CuZer0zm.js"),__vite__mapDeps([43,24,6,1,7,11,12,2,13,14,27,28,29,10,44]))],[/\/admin\/entries\/(?<section>[^/]+)\/(?<id>\d+|new)/,()=>O(()=>import("./Edit-8T4f_l-l.js").then(n=>n.b),__vite__mapDeps([23,9,20,24,25,15,1,6,7,16,26,10,27,28,5,21,8,3,2,11,12,13,14,17,18,19,29,30]))]])}function bd(){Ge(Ke,[["/admin/graphql",()=>O(()=>import("./Tokens-UZ_dxJlK.js"),__vite__mapDeps([45,46,6,1,7,47,48,20,10,49]))],["/admin/graphql/new",()=>O(()=>import("./NewToken-CwxjL8m2.js"),__vite__mapDeps([50,6,1,7,17,12,2,13,18,20,46,51]))]])}const bi=["view","edit","publish","delete"];function wt(n){return n.roles.find(e=>{var t;return e.handle===((t=z.user)==null?void 0:t.role)})}function zt(n,e){return!!n&&bi.indexOf(n)>=bi.indexOf(e)}function Ic(n){var e;return yd(n,((e=z.user)==null?void 0:e.role)??null)}function yd(n,e){var t;return e===null?N("common.admin"):((t=n.roles.find(s=>s.handle===e))==null?void 0:t.name)??e}function Lc(n,e){var t;return dn()?"delete":((t=wt(n))==null?void 0:t.sections[e])??null}function dn(n){return z.user!==null&&z.user.role===null}function xd(n,e,t){var s;return dn()||zt((s=wt(n))==null?void 0:s.sections[e],t)}function Cc(n,e){var t;return dn()||zt((t=wt(n))==null?void 0:t.assets,e)}function Mc(n){const e=wt(n);return dn()||zt(e==null?void 0:e.assets,"delete")||Object.values((e==null?void 0:e.sections)??{}).some(t=>zt(t,"delete"))}function Dc(n){var e;return dn()||!!((e=wt(n))!=null&&e.ai)}function Pc(n){return Ta(n).map(e=>({...e,sections:e.sections.filter(t=>n.docs.some(s=>s.handle===t)||xd(n,t,"view"))})).filter(e=>e.sections.length)}function Bc(n){return n.dev&&dn()&&n.plugins.some(e=>e.name==="agent")}const Rr=[["overview","shell.setupOverview"],["sections","shell.setupSections"],["types","shell.setupTypes"],["blocks","shell.setupBlocks"],["fields","shell.setupFields"],["sites","shell.setupSites"],["requests","shell.setupRequests"]];function Rc(n){return Rr.filter(([e])=>e!=="requests"||dn())}function Nc(n,e){return e.sections.map(s=>Gs(n,s)).every(s=>(s==null?void 0:s.kind)==="single")?"globe":"entries"}function Oc(n,e){return((e==null?void 0:e.sections)??[]).map(t=>{const s=Gs(n,t);if(s)return{handle:t,name:s.name};const i=n.docs.find(r=>r.handle===t);return{handle:t,name:(i==null?void 0:i.name)??t}})}const Sd=new RegExp(`/admin/setup/(?<tab>${Rr.map(([n])=>n).join("|")})(?:/(?<handle>[^/]+))?`);function Ed(){const n=()=>O(()=>import("./Setup-Dw1WFRqR.js"),__vite__mapDeps([52,24,53,3,6,1,7,8,9,10,2,11,12,13,14,15,16,17,18,5,19,47,48,20,27,28,54]));Ge(Ke,[["/admin/setup",n],[Sd,n]])}function Td(){Ge(Ke,[["/admin/trash",()=>O(()=>import("./Trash-ChYOTPVI.js"),__vite__mapDeps([55,6,1,7,47,48,20,10,56]))]])}function Ad(){Ge(Ke,[["/admin/users",()=>O(()=>import("./Users-BowhHXlU.js"),__vite__mapDeps([57,6,1,7,47,48,10,58]))],[/\/admin\/users\/(?<id>\d+|new)/,()=>O(()=>import("./User-DX43jiea.js"),__vite__mapDeps([59,5,6,1,7,3,20,53,24,15,16,10,60]))]])}function Id(){Ge(Ke,[["/admin",()=>O(()=>import("./Start-BJEUtIyk.js"),__vite__mapDeps([61,31,29]))],["/admin/entries",()=>O(()=>import("./Start-BJEUtIyk.js"),__vite__mapDeps([61,31,29]))],[/\/admin\/docs\/(?<handle>[^/]+)/,()=>O(()=>import("./Doc-BUceaenD.js"),__vite__mapDeps([62,2,6,1,7,63]))]]),_d(),kd(),wd(),Ed(),md(),Ad(),bd(),Td()}Id();z.init().then(()=>Fa({notFound:()=>O(()=>import("./NotFound-DEA14OEf.js"),__vite__mapDeps([64,31,6,1,7,65])),error:()=>O(()=>import("./ErrorPage-AnHt3dXP.js"),__vite__mapDeps([66,6,1,7,67]))},()=>gl(vd,{target:document.getElementById("app")}))).catch(n=>{const e=document.createElement("p");e.className="error",e.style.padding="1.5rem",e.textContent=N("common.unreachable",{error:n.message}),document.getElementById("app").append(e)});export{Xr as $,Yt as A,ts as B,bt as C,Bc as D,Ks as E,Sr as F,hi as G,rl as H,ud as I,zn as J,pi as K,mc as L,Zd as M,Cc as N,ui as O,tc as P,z as Q,j as R,Cl as S,$d as T,yl as U,Gs as V,gc as W,xd as X,pc as Y,Pr as Z,wc as _,N as a,cc as a$,ec as a0,dn as a1,nc as a2,Lc as a3,Md as a4,O as a5,Ta as a6,Ic as a7,Rr as a8,Ca as a9,lc as aA,Cs as aB,M as aC,Ko as aD,Cd as aE,eo as aF,Kd as aG,Jn as aH,jd as aI,Wd as aJ,ba as aK,Aa as aL,hc as aM,rr as aN,ic as aO,fc as aP,os as aQ,gl as aR,Qd as aS,Hd as aT,Vd as aU,oc as aV,Jd as aW,Xd as aX,Kr as aY,Ds as aZ,Jr as a_,vc as aa,kc as ab,Va as ac,yd as ad,wi as ae,yc as af,Tc as ag,xc as ah,Sc as ai,Ec as aj,bc as ak,va as al,Ac as am,uc as an,ii as ao,fa as ap,ga as aq,Mn as ar,ac as as,_c as at,Nc as au,Oc as av,nl as aw,Dc as ax,Rc as ay,Mc as az,fe as b,dc as b0,Ut as b1,vo as b2,zd as b3,Yd as b4,qi as b5,m as b6,sc as b7,mo as b8,ht as b9,Vs as bA,In as ba,Wt as bb,Nd as bc,Rt as bd,Dd as be,Re as bf,qd as bg,Wo as bh,Zr as bi,Bd as bj,Fd as bk,Ud as bl,gt as bm,Rd as bn,U as bo,ri as bp,Kn as bq,le as br,ze as bs,Ce as bt,Od as bu,Gd as bv,fr as bw,te as bx,Ht as by,on as bz,oe as c,pt as d,he as e,Vi as f,b as g,un as h,pn as i,wl as j,or as k,rc as l,P as m,me as n,Be as o,ft as p,L as q,G as r,$ as s,wn as t,Ns as u,Pc as v,mt as w,Nt as x,Al as y,Zo as z};
