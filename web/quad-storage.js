var ctx=null,memory;params_set_mem=function(e){memory=e,ctx={}},params_register_js_plugin=function(e){e.env.quad_storage_length=function(){return localStorage.length},e.env.quad_storage_has_key=function(e){return+(localStorage.key(e)!=null)},e.env.quad_storage_key=function(e){return js_object(localStorage.key(e))},e.env.quad_storage_has_value=function(e){return+(localStorage.getItem(get_js_object(e))!=null)},e.env.quad_storage_get=function(e){return js_object(localStorage.getItem(get_js_object(e)))},e.env.quad_storage_set=function(e,t){localStorage.setItem(get_js_object(e),get_js_object(t))},e.env.quad_storage_remove=function(e){localStorage.removeItem(get_js_object(e))},e.env.quad_storage_clear=function(){localStorage.clear()}},miniquad_add_plugin({register_plugin:params_register_js_plugin,on_init:params_set_mem,name:"quad_storage",version:"0.1.2"})