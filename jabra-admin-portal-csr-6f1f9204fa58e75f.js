let a4=`string`,X=`undefined`,a2=`number`,af=1409,W=null,a0=1,Y=`utf-8`,ag=2026,aa=4,a1=`function`,a3=`boolean`,a6=`Object`,a9=16,_=0,U=Array,a5=Array.isArray,ad=Date,Z=Error,a8=FinalizationRegistry,a7=JSON.stringify,ac=Object,ae=Promise,ab=Reflect,$=Uint8Array,V=undefined;var R=((b,c)=>{a=b.exports;T.__wbindgen_wasm_module=c;s=W;q=W;h=W;a.__wbindgen_start();return a});var r=(()=>{if(q===W||q.byteLength===_){q=new Int32Array(a.memory.buffer)};return q});var k=(a=>{if(d===b.length)b.push(b.length+ a0);const c=d;d=b[c];b[c]=a;return c});var i=(()=>{if(h===W||h.byteLength===_){h=new $(a.memory.buffer)};return h});var T=(async(b)=>{if(a!==V)return a;if(typeof b===X){b=new URL(`jabra-admin-portal-csr_bg.wasm`,import.meta.url)};const c=P();if(typeof b===a4||typeof Request===a1&&b instanceof Request||typeof URL===a1&&b instanceof URL){b=fetch(b)};Q(c);const {instance:d,module:e}=await O(await b,c);return R(d,e)});var u=(a=>{const b=typeof a;if(b==a2||b==a3||a==W){return `${a}`};if(b==a4){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==W){return `Symbol`}else{return `Symbol(${b})`}};if(b==a1){const b=a.name;if(typeof b==a4&&b.length>_){return `Function(${b})`}else{return `Function`}};if(a5(a)){const b=a.length;let c=`[`;if(b>_){c+=u(a[_])};for(let d=a0;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>a0){d=c[a0]}else{return toString.call(a)};if(d==a6){try{return `Object(`+ a7(a)+ `)`}catch(a){return a6}};if(a instanceof Z){return `${a.name}: ${a.message}\n${a.stack}`};return d});function G(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var O=(async(a,b)=>{if(typeof Response===a1&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===a1){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var E=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h85ce6203248b9483(b,c,k(d))});var P=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new Z();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{var d=F(b,c);if(b!==_){a.__wbindgen_free(b,c,a0)};console.error(d)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==a0){b.a=_;return !0};const c=!1;return c});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===a4?e:V;var g=p(f)?_:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/aa+ a0]=h;r()[b/aa+ _]=g});b.wbg.__wbindgen_number_new=(a=>{const b=a;return k(b)});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbg_fetch_bc7c8e27076a5c84=(a=>{const b=fetch(c(a));return k(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===a4;return b});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===a2?d:V;t()[a/8+ a0]=p(e)?_:e;r()[a/aa+ _]=!p(e)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===V;return b});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===W;return b});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===a3?(b?a0:_):2;return d});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return p(b)?_:k(b)});b.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=c(a).location;return k(b)});b.wbg.__wbg_history_bc4057de66a2015f=function(){return G((a=>{const b=c(a).history;return k(b)}),arguments)};b.wbg.__wbg_matchMedia_66bb21e3ef19270c=function(){return G(((a,b,d)=>{var e=F(b,d);const f=c(a).matchMedia(e);return p(f)?_:k(f)}),arguments)};b.wbg.__wbg_scrollTo_4d970c5e1c4b340b=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbg_requestAnimationFrame_549258cfa66011f0=function(){return G(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_clearTimeout_ba63ae54a36e111e=((a,b)=>{c(a).clearTimeout(b)});b.wbg.__wbg_fetch_c4b6afebdb1f918e=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_setTimeout_c172d5704ef82276=function(){return G(((a,b,d)=>{const e=c(a).setTimeout(c(b),d);return e}),arguments)};b.wbg.__wbg_documentElement_da9c841ddb352d95=(a=>{const b=c(a).documentElement;return p(b)?_:k(b)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return p(b)?_:k(b)});b.wbg.__wbg_head_d7a99d3f407e2291=(a=>{const b=c(a).head;return p(b)?_:k(b)});b.wbg.__wbg_createComment_354ccab4fdc521ee=((a,b,d)=>{var e=F(b,d);const f=c(a).createComment(e);return k(f)});b.wbg.__wbg_createDocumentFragment_8c86903bbb0a3c3c=(a=>{const b=c(a).createDocumentFragment();return k(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return G(((a,b,d)=>{var e=F(b,d);const f=c(a).createElement(e);return k(f)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);const i=c(a).createElementNS(g,h);return k(i)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{var e=F(b,d);const f=c(a).createTextNode(e);return k(f)});b.wbg.__wbg_getElementById_c369ff43f0db99cf=((a,b,d)=>{var e=F(b,d);const f=c(a).getElementById(e);return p(f)?_:k(f)});b.wbg.__wbg_querySelector_a5f74efc5fa193dd=function(){return G(((a,b,d)=>{var e=F(b,d);const f=c(a).querySelector(e);return p(f)?_:k(f)}),arguments)};b.wbg.__wbg_classList_1f0528ee002e56d4=(a=>{const b=c(a).classList;return k(b)});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{var e=F(b,d);c(a).innerHTML=e});b.wbg.__wbg_getAttribute_99bddb29274b29b9=((b,d,e,f)=>{var g=F(e,f);const h=c(d).getAttribute(g);var i=p(h)?_:o(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=l;r()[b/aa+ a0]=j;r()[b/aa+ _]=i});b.wbg.__wbg_hasAttribute_8340e1a2a46f10f3=((a,b,d)=>{var e=F(b,d);const f=c(a).hasAttribute(e);return f});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return G(((a,b,d)=>{var e=F(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_scrollIntoView_0c1a31f3d0dce6ae=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_210596e44d88649f=function(){return G(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_remove_49b0a5925a04b955=(a=>{c(a).remove()});b.wbg.__wbg_cookie_1ba54ab8c1bb184d=function(){return G(((b,d)=>{const e=c(d).cookie;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_setcookie_21ee60540965860c=function(){return G(((a,b,d)=>{var e=F(b,d);c(a).cookie=e}),arguments)};b.wbg.__wbg_length_d0a802565d17eec4=(a=>{const b=c(a).length;return b});b.wbg.__wbg_new_4c501d7c115d20a6=function(){return G((()=>{const a=new URLSearchParams();return k(a)}),arguments)};b.wbg.__wbg_newwithstrsequencesequence_25ea18ad2d9b3020=function(){return G((a=>{const b=new URLSearchParams(c(a));return k(b)}),arguments)};b.wbg.__wbg_instanceof_HtmlFormElement_ec8cd1ecba7bc422=(a=>{let b;try{b=c(a) instanceof HTMLFormElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return p(b)?_:k(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_previousSibling_9708a091a3e6e03b=(a=>{const b=c(a).previousSibling;return p(b)?_:k(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return p(b)?_:k(b)});b.wbg.__wbg_settextContent_d271bab459cbb1ba=((a,b,d)=>{var e=F(b,d);c(a).textContent=e});b.wbg.__wbg_appendChild_580ccb11a660db68=function(){return G(((a,b)=>{const d=c(a).appendChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return G((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return G(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_view_7f0ce470793a340f=(a=>{const b=c(a).view;return p(b)?_:k(b)});b.wbg.__wbg_respond_b1a43b2e3a06d525=function(){return G(((a,b)=>{c(a).respond(b>>>_)}),arguments)};b.wbg.__wbg_new_7053f98f3a2f6419=function(){return G(((a,b)=>{var c=F(a,b);const d=new BroadcastChannel(c);return k(d)}),arguments)};b.wbg.__wbg_close_10b1f19e83d1b320=(a=>{c(a).close()});b.wbg.__wbg_postMessage_318b80af3de6469e=function(){return G(((a,b)=>{c(a).postMessage(c(b))}),arguments)};b.wbg.__wbg_addEventListener_53b787075bd5e003=function(){return G(((a,b,d,e)=>{var f=F(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_92cb9b3943463338=function(){return G(((a,b,d,e)=>{var f=F(b,d);c(a).removeEventListener(f,c(e))}),arguments)};b.wbg.__wbg_removeEventListener_f3689e55cc5b09c4=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);c(a).removeEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_matches_e14ed9ff8291cf24=(a=>{const b=c(a).matches;return b});b.wbg.__wbg_debug_5fb96680aecf5dc8=(a=>{console.debug(c(a))});b.wbg.__wbg_error_8e3928cfb8a43e2b=(a=>{console.error(c(a))});b.wbg.__wbg_info_530a29cb2e4e3304=(a=>{console.info(c(a))});b.wbg.__wbg_log_5bb5f88f245d7762=(a=>{console.log(c(a))});b.wbg.__wbg_warn_63bbae1730aead09=(a=>{console.warn(c(a))});b.wbg.__wbg_instanceof_HtmlButtonElement_534f7aa847dae46f=(a=>{let b;try{b=c(a) instanceof HTMLButtonElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_HtmlInputElement_307512fe1252c849=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_checked_749a34774f2df2e3=(a=>{const b=c(a).checked;return b});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_submitter_e111819bd16549f1=(a=>{const b=c(a).submitter;return p(b)?_:k(b)});b.wbg.__wbg_href_7bfb3b2fdc0a6c3f=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_origin_ea68ac578fa8517a=((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_pathname_c5fe403ef9525ec6=((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_search_c68f506c44be6d1e=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_setsearch_fd62f4de409a2bb3=((a,b,d)=>{var e=F(b,d);c(a).search=e});b.wbg.__wbg_searchParams_bc5845fe67587f77=(a=>{const b=c(a).searchParams;return k(b)});b.wbg.__wbg_hash_cdea7a9b7e684a42=((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_new_67853c351755d2cf=function(){return G(((a,b)=>{var c=F(a,b);const d=new URL(c);return k(d)}),arguments)};b.wbg.__wbg_newwithbase_6aabbfb1b2e6a1cb=function(){return G(((a,b,c,d)=>{var e=F(a,b);var f=F(c,d);const g=new URL(e,f);return k(g)}),arguments)};b.wbg.__wbg_createObjectURL_ad8244759309f204=function(){return G(((b,d)=>{const e=URL.createObjectURL(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_click_897b305b2e10b9cf=(a=>{c(a).click()});b.wbg.__wbg_newwithform_c3097bb4f1b13572=function(){return G((a=>{const b=new FormData(c(a));return k(b)}),arguments)};b.wbg.__wbg_state_9cc3f933b7d50acb=function(){return G((a=>{const b=c(a).state;return k(b)}),arguments)};b.wbg.__wbg_pushState_b8e8d346f8bb33fd=function(){return G(((a,b,d,e,f,g)=>{var h=F(d,e);var i=F(f,g);c(a).pushState(c(b),h,i)}),arguments)};b.wbg.__wbg_replaceState_ec9431bea5108a50=function(){return G(((a,b,d,e,f,g)=>{var h=F(d,e);var i=F(f,g);c(a).replaceState(c(b),h,i)}),arguments)};b.wbg.__wbg_data_3ce7c145ca4fbcdc=(a=>{const b=c(a).data;return k(b)});b.wbg.__wbg_byobRequest_72fca99f9c32c193=(a=>{const b=c(a).byobRequest;return p(b)?_:k(b)});b.wbg.__wbg_close_184931724d961ccc=function(){return G((a=>{c(a).close()}),arguments)};b.wbg.__wbg_url_7807f6a1fddc3e23=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_newwithstr_36b0b3f97efe096f=function(){return G(((a,b)=>{var c=F(a,b);const d=new Request(c);return k(d)}),arguments)};b.wbg.__wbg_newwithstrandinit_3fd6fba4083ff2d0=function(){return G(((a,b,d)=>{var e=F(a,b);const f=new Request(e,c(d));return k(f)}),arguments)};b.wbg.__wbg_instanceof_Response_849eb93e75734b6e=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_url_5f6dc4009ac5f99d=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_redirected_1a9130cafa803002=(a=>{const b=c(a).redirected;return b});b.wbg.__wbg_status_61a01141acd3cf74=(a=>{const b=c(a).status;return b});b.wbg.__wbg_headers_9620bfada380764a=(a=>{const b=c(a).headers;return k(b)});b.wbg.__wbg_text_450a059667fd91fd=function(){return G((a=>{const b=c(a).text();return k(b)}),arguments)};b.wbg.__wbg_add_7a8b240850cb3c95=function(){return G(((a,b)=>{c(a).add(...c(b))}),arguments)};b.wbg.__wbg_add_dcb05a8ba423bdac=function(){return G(((a,b,d)=>{var e=F(b,d);c(a).add(e)}),arguments)};b.wbg.__wbg_remove_3305bb0551072040=function(){return G(((a,b)=>{c(a).remove(...c(b))}),arguments)};b.wbg.__wbg_remove_698118fb25ab8150=function(){return G(((a,b,d)=>{var e=F(b,d);c(a).remove(e)}),arguments)};b.wbg.__wbg_sethref_b94692d1a9f05b53=function(){return G(((a,b,d)=>{var e=F(b,d);c(a).href=e}),arguments)};b.wbg.__wbg_origin_ee93e29ace71f568=function(){return G(((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_protocol_b7292c581cfe1e5c=function(){return G(((b,d)=>{const e=c(d).protocol;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_hostname_3d9f22c60dc5bec6=function(){return G(((b,d)=>{const e=c(d).hostname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_port_b8d9a9c4e2b26efa=function(){return G(((b,d)=>{const e=c(d).port;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_pathname_5449afe3829f96a1=function(){return G(((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_search_489f12953342ec1f=function(){return G(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_hash_553098e838e06c1d=function(){return G(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f}),arguments)};b.wbg.__wbg_setdata_8c2b43af041cc1b3=((a,b,d)=>{var e=F(b,d);c(a).data=e});b.wbg.__wbg_newwithu8arraysequenceandoptions_366f462e1b363808=function(){return G(((a,b)=>{const d=new Blob(c(a),c(b));return k(d)}),arguments)};b.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=c(a).target;return p(b)?_:k(b)});b.wbg.__wbg_defaultPrevented_cc14a1dd3dd69c38=(a=>{const b=c(a).defaultPrevented;return b});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_stopPropagation_fa5b666049c9fd02=(a=>{c(a).stopPropagation()});b.wbg.__wbg_ctrlKey_008695ce60a588f5=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_1e76dbfcdd36a4b4=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_07da841b54bd3ed6=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_86bfd3b0d3a8083f=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_button_367cdc7303e3cf9b=(a=>{const b=c(a).button;return b});b.wbg.__wbg_close_a994f9425dab445c=function(){return G((a=>{c(a).close()}),arguments)};b.wbg.__wbg_enqueue_ea194723156c0cc2=function(){return G(((a,b)=>{c(a).enqueue(c(b))}),arguments)};b.wbg.__wbg_new_ab6fd82b10560829=function(){return G((()=>{const a=new Headers();return k(a)}),arguments)};b.wbg.__wbg_append_7bfcb4937d1d5e29=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);c(a).append(g,h)}),arguments)};b.wbg.__wbg_set_cb0e7a5c2dd66afd=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);c(a).set(g,h)}),arguments)};b.wbg.__wbg_instanceof_WorkerGlobalScope_46b577f151fad960=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_fetch_921fad6ef9e883dd=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_signal_a61f78a3478fd9bc=(a=>{const b=c(a).signal;return k(b)});b.wbg.__wbg_new_0d76b0581eca6298=function(){return G((()=>{const a=new AbortController();return k(a)}),arguments)};b.wbg.__wbg_abort_2aa7521d5690750e=(a=>{c(a).abort()});b.wbg.__wbg_append_7ba9d5c2eb183eea=function(){return G(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_instanceof_HtmlAnchorElement_5fc0eb2fbc8672d8=(a=>{let b;try{b=c(a) instanceof HTMLAnchorElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_target_f0876f510847bc60=((b,d)=>{const e=c(d).target;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_setdownload_65ac7e7c800d764e=((a,b,d)=>{var e=F(b,d);c(a).download=e});b.wbg.__wbg_href_40fd5bca11c13133=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbg_sethref_b0712139dd35e2fd=((a,b,d)=>{var e=F(b,d);c(a).href=e});b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===a1;return b});b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>_];return k(d)});b.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_new_16b304a2cfa7ff4a=(()=>{const a=new U();return k(a)});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{var c=F(a,b);const d=new Function(c);return k(d)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==W;return d});b.wbg.__wbg_next_40fc327bfc8770e6=(a=>{const b=c(a).next;return k(b)});b.wbg.__wbg_next_196c84450b364254=function(){return G((a=>{const b=c(a).next();return k(b)}),arguments)};b.wbg.__wbg_done_298b57d23c0fc80c=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_d93c65011f51a456=(a=>{const b=c(a).value;return k(b)});b.wbg.__wbg_iterator_2cee6dadfd956dfa=(()=>{const a=Symbol.iterator;return k(a)});b.wbg.__wbg_get_e3c254076557e348=function(){return G(((a,b)=>{const d=ab.get(c(a),c(b));return k(d)}),arguments)};b.wbg.__wbg_call_27c0f87801dedf93=function(){return G(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new ac();return k(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return G((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return G((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return G((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return G((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbg_decodeURI_34e1afc7326c927c=function(){return G(((a,b)=>{var c=F(a,b);const d=decodeURI(c);return k(d)}),arguments)};b.wbg.__wbg_encodeURI_a0b23d39c4951f9a=((a,b)=>{var c=F(a,b);const d=encodeURI(c);return k(d)});b.wbg.__wbg_isArray_2ab64d95e09ea0ae=(a=>{const b=a5(c(a));return b});b.wbg.__wbg_push_a5b05aedc7234f9f=((a,b)=>{const d=c(a).push(c(b));return d});b.wbg.__wbg_instanceof_Error_e20bb56fd5591a93=(a=>{let b;try{b=c(a) instanceof Z}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_new_28c511d9baebfa89=((a,b)=>{var c=F(a,b);const d=new Z(c);return k(d)});b.wbg.__wbg_message_5bf28016c2b49cfb=(a=>{const b=c(a).message;return k(b)});b.wbg.__wbg_name_e7429f0dda6079e2=(a=>{const b=c(a).name;return k(b)});b.wbg.__wbg_toString_ffe4c9ea3b3532e9=(a=>{const b=c(a).toString();return k(b)});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return G(((a,b,d)=>{const e=c(a).call(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_getTime_2bc4375165f02d15=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_getTimezoneOffset_38257122e236c190=(a=>{const b=c(a).getTimezoneOffset();return b});b.wbg.__wbg_new_cf3ec55744a78578=(a=>{const b=new ad(c(a));return k(b)});b.wbg.__wbg_new0_7d84e5b2cd9fdc73=(()=>{const a=new ad();return k(a)});b.wbg.__wbg_newwithyearmonthdayhrminsec_19ea6fc6146755a0=((a,b,c,d,e,f)=>{const g=new ad(a>>>_,b,c,d,e,f);return k(g)});b.wbg.__wbg_now_3014639a94423537=(()=>{const a=ad.now();return a});b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=ac.is(c(a),c(b));return d});b.wbg.__wbg_toString_c816a20ab859d0c1=(a=>{const b=c(a).toString();return k(b)});b.wbg.__wbg_exec_b9996525463e30df=((a,b,d)=>{var e=F(b,d);const f=c(a).exec(e);return p(f)?_:k(f)});b.wbg.__wbg_new_3c970fa9da0c5794=((a,b,c,d)=>{var e=F(a,b);var f=F(c,d);const g=new RegExp(e,f);return k(g)});b.wbg.__wbg_new_81740750da40724f=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=_;try{return H(d,c.b,a,b)}finally{c.a=d}};const e=new ae(d);return k(e)}finally{c.a=c.b=_}});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=ae.resolve(c(a));return k(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_then_a73caa9a87991566=((a,b,d)=>{const e=c(a).then(c(b),c(d));return k(e)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const e=new $(c(a),b>>>_,d>>>_);return k(e)});b.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new $(c(a));return k(b)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>_)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_buffer_dd7f74bc60f1faab=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_byteLength_58f7b4fab1919d44=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_byteOffset_81d60f7392524f62=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_has_0af94d20077affa2=function(){return G(((a,b)=>{const d=ab.has(c(a),c(b));return d}),arguments)};b.wbg.__wbg_set_1f9b04f170055d33=function(){return G(((a,b,d)=>{const e=ab.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbg_stringify_8887fe74e1c50d81=function(){return G((a=>{const b=a7(c(a));return k(b)}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/aa+ a0]=g;r()[b/aa+ _]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new Z(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper2977=((a,b,c)=>{const d=w(a,b,af,x);return k(d)});b.wbg.__wbindgen_closure_wrapper2979=((a,b,c)=>{const d=w(a,b,af,y);return k(d)});b.wbg.__wbindgen_closure_wrapper2981=((a,b,c)=>{const d=w(a,b,af,y);return k(d)});b.wbg.__wbindgen_closure_wrapper2983=((a,b,c)=>{const d=w(a,b,af,y);return k(d)});b.wbg.__wbindgen_closure_wrapper2985=((a,b,c)=>{const d=w(a,b,af,y);return k(d)});b.wbg.__wbindgen_closure_wrapper2987=((a,b,c)=>{const d=w(a,b,af,y);return k(d)});b.wbg.__wbindgen_closure_wrapper4159=((a,b,c)=>{const d=w(a,b,1925,z);return k(d)});b.wbg.__wbindgen_closure_wrapper4522=((a,b,c)=>{const d=w(a,b,ag,A);return k(d)});b.wbg.__wbindgen_closure_wrapper4524=((a,b,c)=>{const d=w(a,b,ag,B);return k(d)});b.wbg.__wbindgen_closure_wrapper6543=((a,b,c)=>{const d=w(a,b,3087,C);return k(d)});b.wbg.__wbindgen_closure_wrapper6865=((a,b,c)=>{const d=w(a,b,3197,D);return k(d)});b.wbg.__wbindgen_closure_wrapper8580=((a,b,c)=>{const d=w(a,b,3233,E);return k(d)});return b});var A=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h717fb03e426c1034(b,c)});var D=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h27d4c22c98b38a7e(b,c)});var o=((a,b,c)=>{if(c===V){const c=m.encode(a);const d=b(c.length,a0)>>>_;i().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,a0)>>>_;const f=i();let g=_;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==_){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,a0)>>>_;const b=i().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written;e=c(e,d,g,a0)>>>_};l=g;return e});var C=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h451023975497dfeb(b,c,k(d))});var c=(a=>b[a]);var e=(a=>{if(a<132)return;b[a]=d;d=a});var x=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf8d9aafd3ac98c36(b,c)});var f=(a=>{const b=c(a);e(a);return b});var j=((a,b)=>{a=a>>>_;return g.decode(i().subarray(a,a+ b))});var B=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcab8365b32ee74ef(b,c,k(d))});var z=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5ed37d4e122caa99(b,c,k(d))});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:a0,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=_;try{return e(c,f.b,...b)}finally{if(--f.cnt===_){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var Q=((a,b)=>{});var p=(a=>a===V||a===W);var t=(()=>{if(s===W||s.byteLength===_){s=new Float64Array(a.memory.buffer)};return s});var y=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1e9f5db365a99da4(b,c,k(d))});var S=(b=>{if(a!==V)return a;const c=P();Q(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return R(d,b)});var F=((a,b)=>{if(a===_){return c(b)}else{return j(a,b)}});var H=((b,c,d,e)=>{a.wasm_bindgen__convert__closures__invoke2_mut__h16c2ccaf94d27ca8(b,c,k(d),k(e))});let a;const b=new U(128).fill(V);b.push(V,W,!0,!1);let d=b.length;const g=typeof TextDecoder!==X?new TextDecoder(Y,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Z(`TextDecoder not available`)}};if(typeof TextDecoder!==X){g.decode()};let h=W;let l=_;const m=typeof TextEncoder!==X?new TextEncoder(Y):{encode:()=>{throw Z(`TextEncoder not available`)}};const n=typeof m.encodeInto===a1?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=W;let s=W;const v=typeof a8===X?{register:()=>{},unregister:()=>{}}:new a8(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});const I=typeof a8===X?{register:()=>{},unregister:()=>{}}:new a8(b=>a.__wbg_intounderlyingbytesource_free(b>>>_));class J{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=_;I.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b)}type(){try{const e=a.__wbindgen_add_to_stack_pointer(-a9);a.intounderlyingbytesource_type(e,this.__wbg_ptr);var b=r()[e/aa+ _];var c=r()[e/aa+ a0];var d=F(b,c);if(b!==_){a.__wbindgen_free(b,c,a0)};return d}finally{a.__wbindgen_add_to_stack_pointer(a9)}}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>_}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,k(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,k(b));return f(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const K=typeof a8===X?{register:()=>{},unregister:()=>{}}:new a8(b=>a.__wbg_intounderlyingsink_free(b>>>_));class L{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=_;K.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,k(b));return f(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return f(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,k(b));return f(d)}}const M=typeof a8===X?{register:()=>{},unregister:()=>{}}:new a8(b=>a.__wbg_intounderlyingsource_free(b>>>_));class N{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=_;M.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,k(b));return f(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default T;export{J as IntoUnderlyingByteSource,L as IntoUnderlyingSink,N as IntoUnderlyingSource,S as initSync}