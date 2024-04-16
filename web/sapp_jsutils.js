"use strict";var ctx=null,unique_js_id,js_objects={};js_objects[-1]=null,js_objects[-2]=void 0,unique_js_id=0;function register_plugin(e){e.env.js_create_string=function(e,t){var n=UTF8ToString(e,t);return js_object(n)},e.env.js_create_buffer=function(e,t){var n=new Uint8Array(wasm_memory.buffer,e,t),s=new Uint8Array(new ArrayBuffer(n.byteLength));return s.set(new Uint8Array(n)),js_object(s)},e.env.js_create_object=function(){var e={};return js_object(e)},e.env.js_set_field_f32=function(e,t,n,s){var o=UTF8ToString(t,n);js_objects[e][o]=s},e.env.js_set_field_u32=function(e,t,n,s){var o=UTF8ToString(t,n);js_objects[e][o]=s},e.env.js_set_field_string=function(e,t,n,s,o){var i=UTF8ToString(t,n),a=UTF8ToString(s,o);js_objects[e][i]=a},e.env.js_unwrap_to_str=function(e,t,n){for(var i=js_objects[e],o=toUTF8Array(i),a=o.length,r=new Uint8Array(wasm_memory.buffer,t,n),s=0;s<a;s++)r[s]=o[s]},e.env.js_unwrap_to_buf=function(e,t,n){for(var o=js_objects[e],i=o.length,a=new Uint8Array(wasm_memory.buffer,t,n),s=0;s<i;s++)a[s]=o[s]},e.env.js_string_length=function(e){var t=js_objects[e];return toUTF8Array(t).length},e.env.js_buf_length=function(e){var t=js_objects[e];return t.length},e.env.js_free_object=function(e){delete js_objects[e]},e.env.js_have_field=function(e,t,n){var s=UTF8ToString(t,n);return js_objects[e][s]!==void 0},e.env.js_field_f32=function(e,t,n){var s=UTF8ToString(t,n);return js_objects[e][s]},e.env.js_field_u32=function(e,t,n){var s=UTF8ToString(t,n);return js_objects[e][s]},e.env.js_field=function(e,t,n){var s=UTF8ToString(t,n),o=js_objects[e][s];return js_object(o)},e.env.js_field_num=function(e,t,n){var s=UTF8ToString(t,n);return js_objects[e][s]}}miniquad_add_plugin({register_plugin,version:"0.1.5",name:"sapp_jsutils"});function toUTF8Array(e){for(var t,n=[],s=0;s<e.length;s++)t=e.charCodeAt(s),t<128?n.push(t):t<2048?n.push(192|t>>6,128|t&63):t<55296||t>=57344?n.push(224|t>>12,128|t>>6&63,128|t&63):(s++,t=65536+((t&1023)<<10|e.charCodeAt(s)&1023),n.push(240|t>>18,128|t>>12&63,128|t>>6&63,128|t&63));return n}function js_object(e){if(e==null)return-2;if(e===null)return-1;var t=unique_js_id;return js_objects[t]=e,unique_js_id+=1,t}function consume_js_object(e){var t=js_objects[e];return delete js_objects[e],t}function get_js_object(e){return js_objects[e]}