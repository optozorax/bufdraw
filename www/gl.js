const canvas=document.querySelector("#glcanvas"),gl=canvas.getContext("webgl");function assert(e,r){0==e&&alert(r)}function acquireVertexArrayObjectExtension(e){var r=e.getExtension("OES_vertex_array_object");r?(e.createVertexArray=function(){return r.createVertexArrayOES()},e.deleteVertexArray=function(e){r.deleteVertexArrayOES(e)},e.bindVertexArray=function(e){r.bindVertexArrayOES(e)},e.isVertexArray=function(e){return r.isVertexArrayOES(e)}):alert("Unable to get OES_vertex_array_object extension")}function acquireInstancedArraysExtension(e){var r=e.getExtension("ANGLE_instanced_arrays");r&&(e.vertexAttribDivisor=function(e,t){r.vertexAttribDivisorANGLE(e,t)},e.drawArraysInstanced=function(e,t,n,a){r.drawArraysInstancedANGLE(e,t,n,a)},e.drawElementsInstanced=function(e,t,n,a,o){r.drawElementsInstancedANGLE(e,t,n,a,o)})}function getArray(e,r,t){return new r(memory.buffer,e,t)}function UTF8ToString(e,r){let t=new Uint8Array(memory.buffer);if(string="",null==r)for(;;){let r=t[e];if(null==r)return void console.log("is it assert in js style?");if(0==r)break;string+=String.fromCharCode(r),e++}else for(let n=0;n<r;n++)string+=String.fromCharCode(t[e+n]);return string}null===gl&&alert("Unable to initialize WebGL. Your browser or machine may not support it."),acquireVertexArrayObjectExtension(gl),acquireInstancedArraysExtension(gl),null==gl.getExtension("WEBGL_depth_texture")&&alert("Cant initialize WEBGL_depth_texture extension");var Module,wasm_exports,GL={counter:1,buffers:[],mappedBuffers:{},programs:[],framebuffers:[],renderbuffers:[],textures:[],uniforms:[],shaders:[],vaos:[],contexts:{},programInfos:{},getNewId:function(e){for(var r=GL.counter++,t=e.length;t<r;t++)e[t]=null;return r},validateGLObjectID:function(e,r,t,n){0!=r&&(null===e[r]?console.error(t+" called with an already deleted "+n+" ID "+r+"!"):e[r]||console.error(t+" called with an invalid "+n+" ID "+r+"!"))},getSource:function(e,r,t,n){for(var a="",o=0;o<r;++o){var i=0==n?void 0:getArray(n+4*o,Uint32Array,1)[0];a+=UTF8ToString(getArray(t+4*o,Uint32Array,1)[0],i)}return a},populateUniformTable:function(e){GL.validateGLObjectID(GL.programs,e,"populateUniformTable","program");for(var r=GL.programs[e],t=GL.programInfos[e]={uniforms:{},maxUniformLength:0,maxAttributeLength:-1,maxUniformBlockNameLength:-1},n=t.uniforms,a=gl.getProgramParameter(r,35718),o=0;o<a;++o){var i=gl.getActiveUniform(r,o),l=i.name;t.maxUniformLength=Math.max(t.maxUniformLength,l.length+1),"]"==l.slice(-1)&&(l=l.slice(0,l.lastIndexOf("[")));var s=gl.getUniformLocation(r,l);if(s){var c=GL.getNewId(GL.uniforms);n[l]=[i.size,c],GL.uniforms[c]=s;for(var g=1;g<i.size;++g){var f=l+"["+g+"]";s=gl.getUniformLocation(r,f),c=GL.getNewId(GL.uniforms),GL.uniforms[c]=s}}}}};function resize(e,r){var t=e.clientWidth,n=e.clientHeight;e.width==t&&e.height==n||(e.width=t,e.height=n,null!=r&&r(Math.floor(t),Math.floor(n)))}_glGenObject=function(e,r,t,n,a){for(var o=0;o<e;o++){var i=gl[t](),l=i&&GL.getNewId(n);i?(i.name=l,n[l]=i):(console.error("GL_INVALID_OPERATION"),GL.recordError(1282),alert("GL_INVALID_OPERATION in "+a+": GLctx."+t+" returned null - most likely GL context is lost!")),getArray(r+4*o,Int32Array,1)[0]=l}},_webglGet=function(e,r,t){if(!r)return console.error("GL_INVALID_VALUE in glGet"+t+"v(name="+e+": Function called with null out pointer!"),void GL.recordError(1281);var n=void 0;switch(e){case 36346:n=1;break;case 36344:return void("EM_FUNC_SIG_PARAM_I"!=t&&"EM_FUNC_SIG_PARAM_I64"!=t&&(GL.recordError(1280),err("GL_INVALID_ENUM in glGet"+t+"v(GL_SHADER_BINARY_FORMATS): Invalid parameter type!")));case 34814:case 36345:n=0;break;case 34466:var a=gl.getParameter(34467);n=a?a.length:0;break;case 33309:assert(!1,"unimplemented");break;case 33307:case 33308:assert(!1,"unimplemented")}if(void 0===n){var o=gl.getParameter(e);switch(typeof o){case"number":n=o;break;case"boolean":n=o?1:0;break;case"string":return GL.recordError(1280),void console.error("GL_INVALID_ENUM in glGet"+t+"v("+e+") on a name which returns a string!");case"object":if(null===o)switch(e){case 34964:case 35725:case 34965:case 36006:case 36007:case 32873:case 34229:case 35097:case 36389:case 34068:n=0;break;default:return GL.recordError(1280),void console.error("GL_INVALID_ENUM in glGet"+t+"v("+e+") and it returns null!")}else{if(o instanceof Float32Array||o instanceof Uint32Array||o instanceof Int32Array||o instanceof Array){for(var i=0;i<o.length;++i)assert(!1,"unimplemented");return}try{n=0|o.name}catch(r){return GL.recordError(1280),void console.error("GL_INVALID_ENUM in glGet"+t+"v: Unknown object returned from WebGL getParameter("+e+")! (error: "+r+")")}}break;default:return GL.recordError(1280),void console.error("GL_INVALID_ENUM in glGet"+t+"v: Native code calling glGet"+t+"v("+e+") and it returns "+o+" of type "+typeof o+"!")}}switch(t){case"EM_FUNC_SIG_PARAM_I64":getArray(r,Int32Array,1)[0]=n;case"EM_FUNC_SIG_PARAM_I":getArray(r,Int32Array,1)[0]=n;break;case"EM_FUNC_SIG_PARAM_F":getArray(r,Float32Array,1)[0]=n;break;case"EM_FUNC_SIG_PARAM_B":getArray(r,Int8Array,1)[0]=n?1:0;break;default:throw"internal glGet error, bad type: "+t}},animation=function(){wasm_exports.frame(),window.requestAnimationFrame(animation)},into_sapp_keycode=function(e){switch(event.code){case"KeyA":return 65;case"KeyS":return 83;case"KeyD":return 68;case"KeyW":return 87;case"ArrowRight":return 262;case"ArrowLeft":return 263;case"ArrowDown":return 264;case"ArrowUp":return 265;case"Space":return 32}};var start,emscripten_shaders_hack=!1,importObject={env:{console_debug:function(e){console.debug(UTF8ToString(e))},console_log:function(e){console.log(UTF8ToString(e))},console_info:function(e){console.info(UTF8ToString(e))},console_warn:function(e){console.warn(UTF8ToString(e))},console_error:function(e){console.error(UTF8ToString(e))},set_emscripten_shader_hack:function(e){emscripten_shaders_hack=e},rand:function(){return Math.floor(2147483647*Math.random())},time:function(){return(Date.now()-start)/1e3},canvas_width:function(){return Math.floor(canvas.clientWidth)},canvas_height:function(){return Math.floor(canvas.clientHeight)},glClearDepthf:function(e){gl.clearDepth(e)},glClearColor:function(e,r,t,n){gl.clearColor(e,r,t,n)},glClearStencil:function(e){gl.clearColorStencil(e)},glScissor:function(e,r,t,n){gl.scissor(e,r,t,n)},glClear:function(e){gl.clear(e)},glGenTextures:function(e,r){_glGenObject(e,r,"createTexture",GL.textures,"glGenTextures")},glActiveTexture:function(e){gl.activeTexture(e)},glBindTexture:function(e,r){GL.validateGLObjectID(GL.textures,r,"glBindTexture","texture"),gl.bindTexture(e,GL.textures[r])},glTexImage2D:function(e,r,t,n,a,o,i,l,s){gl.texImage2D(e,r,t,n,a,o,i,l,s?getArray(s,Uint8Array,n*a*4):null)},glTexParameteri:function(e,r,t){gl.texParameteri(e,r,t)},glUniform1fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform1fv","location"),assert(0==(3&t),"Pointer to float data passed to glUniform1fv must be aligned to four bytes!");var n=getArray(t,Float32Array,1);gl.uniform1fv(GL.uniforms[e],n)},glUniform2fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform2fv","location"),assert(0==(3&t),"Pointer to float data passed to glUniform2fv must be aligned to four bytes!");var n=getArray(t,Float32Array,2);gl.uniform2fv(GL.uniforms[e],n)},glUniform3fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform3fv","location"),assert(0==(3&t),"Pointer to float data passed to glUniform3fv must be aligned to four bytes!");var n=getArray(t,Float32Array,3);gl.uniform3fv(GL.uniforms[e],n)},glUniform4fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniform4fv","location"),assert(0==(3&t),"Pointer to float data passed to glUniform4fv must be aligned to four bytes!");var n=getArray(t,Float32Array,4);gl.uniform4fv(GL.uniforms[e],n)},glBlendFunc:function(e,r){gl.blendFunc(e,r)},glBlendEquationSeparate:function(e,r){gl.blendEquationSeparate(e,r)},glDisable:function(e){gl.disable(e)},glDrawElements:function(e,r,t,n){gl.drawElements(e,r,t,n)},glGetIntegerv:function(e,r){_webglGet(e,r,"EM_FUNC_SIG_PARAM_I")},glUniform1f:function(e,r){GL.validateGLObjectID(GL.uniforms,e,"glUniform1f","location"),gl.uniform1f(GL.uniforms[e],r)},glUniform1i:function(e,r){GL.validateGLObjectID(GL.uniforms,e,"glUniform1i","location"),gl.uniform1i(GL.uniforms[e],r)},glGetAttribLocation:function(e,r){return gl.getAttribLocation(GL.programs[e],UTF8ToString(r))},glEnableVertexAttribArray:function(e){gl.enableVertexAttribArray(e)},glDisableVertexAttribArray:function(e){gl.disableVertexAttribArray(e)},glVertexAttribPointer:function(e,r,t,n,a,o){gl.vertexAttribPointer(e,r,t,!!n,a,o)},glGetUniformLocation:function(e,r){GL.validateGLObjectID(GL.programs,e,"glGetUniformLocation","program");var t=0;if("]"==(r=UTF8ToString(r))[r.length-1]){var n=r.lastIndexOf("[");t="]"!=r[n+1]?parseInt(r.slice(n+1)):0,r=r.slice(0,n)}var a=GL.programInfos[e]&&GL.programInfos[e].uniforms[r];return a&&t>=0&&t<a[0]?a[1]+t:-1},glUniformMatrix4fv:function(e,r,t,n){GL.validateGLObjectID(GL.uniforms,e,"glUniformMatrix4fv","location"),assert(0==(3&n),"Pointer to float data passed to glUniformMatrix4fv must be aligned to four bytes!");var a=getArray(n,Float32Array,16);gl.uniformMatrix4fv(GL.uniforms[e],!!t,a)},glUseProgram:function(e){GL.validateGLObjectID(GL.programs,e,"glUseProgram","program"),gl.useProgram(GL.programs[e])},glUniform4fv:function(e,r,t){GL.validateGLObjectID(GL.uniforms,e,"glUniformMatrix4fv","location"),assert(0==(3&t),"Pointer to float data passed to glUniformMatrix4fv must be aligned to four bytes!");var n=getArray(t,Float32Array,4);gl.uniform4fv(GL.uniforms[e],n)},glGenVertexArrays:function(e,r){_glGenObject(e,r,"createVertexArray",GL.vaos,"glGenVertexArrays")},glGenFramebuffers:function(e,r){_glGenObject(e,r,"createFramebuffer",GL.framebuffers,"glGenFramebuffers")},glBindVertexArray:function(e){gl.bindVertexArray(GL.vaos[e])},glBindFramebuffer:function(e,r){GL.validateGLObjectID(GL.framebuffers,r,"glBindFramebuffer","framebuffer"),gl.bindFramebuffer(e,GL.framebuffers[r])},glGenBuffers:function(e,r){_glGenObject(e,r,"createBuffer",GL.buffers,"glGenBuffers")},glBindBuffer:function(e,r){GL.validateGLObjectID(GL.buffers,r,"glBindBuffer","buffer"),gl.bindBuffer(e,GL.buffers[r])},glBufferData:function(e,r,t,n){gl.bufferData(e,t?getArray(t,Uint8Array,r):r,n)},glBufferSubData:function(e,r,t,n){gl.bufferSubData(e,r,n?getArray(n,Uint8Array,t):t)},glEnable:function(e){gl.enable(e)},glDepthFunc:function(e){gl.depthFunc(e)},glBlendFuncSeparate:function(e,r,t,n){gl.blendFuncSeparate(e,r,t,n)},glViewport:function(e,r,t,n){gl.viewport(e,r,t,n)},glDrawArrays:function(e,r,t){gl.drawArrays(e,r,t)},glCreateProgram:function(){var e=GL.getNewId(GL.programs),r=gl.createProgram();return r.name=e,GL.programs[e]=r,e},glAttachShader:function(e,r){GL.validateGLObjectID(GL.programs,e,"glAttachShader","program"),GL.validateGLObjectID(GL.shaders,r,"glAttachShader","shader"),gl.attachShader(GL.programs[e],GL.shaders[r])},glLinkProgram:function(e){GL.validateGLObjectID(GL.programs,e,"glLinkProgram","program"),gl.linkProgram(GL.programs[e]),GL.populateUniformTable(e)},glFramebufferTexture2D:function(e,r,t,n,a){GL.validateGLObjectID(GL.textures,n,"glFramebufferTexture2D","texture"),gl.framebufferTexture2D(e,r,t,GL.textures[n],a)},glGetProgramiv:function(e,r,t){(assert(t),GL.validateGLObjectID(GL.programs,e,"glGetProgramiv","program"),e>=GL.counter)?console.error("GL_INVALID_VALUE in glGetProgramiv"):GL.programInfos[e]?35716!=r&&35719!=r&&35722!=r&&35381!=r?getArray(t,Int32Array,1)[0]=gl.getProgramParameter(GL.programs[e],r):console.error("unsupported operation"):console.error("GL_INVALID_OPERATION in glGetProgramiv(program="+e+", pname="+r+", p=0x"+t.toString(16)+"): The specified GL object name does not refer to a program object!")},glCreateShader:function(e){var r=GL.getNewId(GL.shaders);return GL.shaders[r]=gl.createShader(e),r},glShaderSource:function(e,r,t,n){GL.validateGLObjectID(GL.shaders,e,"glShaderSource","shader");var a=GL.getSource(e,r,t,n);if(emscripten_shaders_hack){var o="";-1!=(a=(a=a.replace(/#extension GL_OES_standard_derivatives : enable/g,"")).replace(/#extension GL_EXT_shader_texture_lod : enable/g,"")).indexOf("gl_FragColor")&&(o+="out mediump vec4 GL_FragColor;\n",a=a.replace(/gl_FragColor/g,"GL_FragColor")),a=(a=(a=(a=(a=(a=(a=(a=(a=(a=(a=(a=-1!=a.indexOf("attribute")?(a=a.replace(/attribute/g,"in")).replace(/varying/g,"out"):a.replace(/varying/g,"in")).replace(/textureCubeLodEXT/g,"textureCubeLod")).replace(/texture2DLodEXT/g,"texture2DLod")).replace(/texture2DProjLodEXT/g,"texture2DProjLod")).replace(/texture2DGradEXT/g,"texture2DGrad")).replace(/texture2DProjGradEXT/g,"texture2DProjGrad")).replace(/textureCubeGradEXT/g,"textureCubeGrad")).replace(/textureCube/g,"texture")).replace(/texture1D/g,"texture")).replace(/texture2D/g,"texture")).replace(/texture3D/g,"texture")).replace(/#version 100/g,"#version 300 es\n"+o)}gl.shaderSource(GL.shaders[e],a)},glGetProgramInfoLog:function(e,r,t,n){GL.validateGLObjectID(GL.programs,e,"glGetProgramInfoLog","program");var a=gl.getProgramInfoLog(GL.programs[e]);assert(null!==a);let o=getArray(n,Uint8Array,r);for(var i=0;i<r;i++)o[i]=a.charCodeAt(i)},glCompileShader:function(e,r,t,n){GL.validateGLObjectID(GL.shaders,e,"glCompileShader","shader"),gl.compileShader(GL.shaders[e])},glGetShaderiv:function(e,r,t){if(assert(t),GL.validateGLObjectID(GL.shaders,e,"glGetShaderiv","shader"),35716==r){var n=gl.getShaderInfoLog(GL.shaders[e]);assert(null!==n),getArray(t,Int32Array,1)[0]=n.length+1}else if(35720==r){var a=gl.getShaderSource(GL.shaders[e]),o=null===a||0==a.length?0:a.length+1;getArray(t,Int32Array,1)[0]=o}else getArray(t,Int32Array,1)[0]=gl.getShaderParameter(GL.shaders[e],r)},glGetShaderInfoLog:function(e,r,t,n){GL.validateGLObjectID(GL.shaders,e,"glGetShaderInfoLog","shader");var a=gl.getShaderInfoLog(GL.shaders[e]);assert(null!==a);let o=getArray(n,Uint8Array,r);for(var i=0;i<r;i++)o[i]=a.charCodeAt(i)},glVertexAttribDivisor:function(e,r){gl.vertexAttribDivisor(e,r)},glDrawArraysInstanced:function(e,r,t,n){gl.drawArraysInstanced(e,r,t,n)},glDrawElementsInstanced:function(e,r,t,n,a){gl.drawElementsInstanced(e,r,t,n,a)},glDeleteShader:function(e){gl.deleteShader(e)},glDeleteBuffers:function(e,r){for(var t=0;t<e;t++){var n=getArray(r+4*t,Uint32Array,1)[0],a=GL.buffers[n];a&&(gl.deleteBuffer(a),a.name=0,GL.buffers[n]=null)}},glDeleteTextures:function(e,r){for(var t=0;t<e;t++){var n=getArray(r+4*t,Uint32Array,1)[0],a=GL.textures[n];a&&(gl.deleteTexture(a),a.name=0,GL.textures[n]=null)}},init_opengl:function(e){start=Date.now(),canvas.onmousemove=function(e){var r=e.clientX,t=e.clientY;wasm_exports.mouse_move(Math.floor(r),Math.floor(t))},canvas.onmousedown=function(e){var r=e.clientX,t=e.clientY,n=e.button;wasm_exports.mouse_down(r,t,n)},canvas.onmouseup=function(e){var r=e.clientX,t=e.clientY,n=e.button;wasm_exports.mouse_up(r,t,n)},canvas.onkeydown=function(e){var r=into_sapp_keycode();wasm_exports.key_down(r)},canvas.onkeyup=function(e){var r=into_sapp_keycode();wasm_exports.key_up(r)},window.onresize=function(){resize(canvas,wasm_exports.resize)},window.requestAnimationFrame(animation)}}};function load(e){var r=fetch(e);"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(r,importObject).then(e=>{memory=e.instance.exports.memory,wasm_exports=e.instance.exports,e.instance.exports.main()}):r.then(function(e){return e.arrayBuffer()}).then(function(e){return WebAssembly.instantiate(e,importObject)}).then(function(e){memory=e.instance.exports.memory,wasm_exports=e.instance.exports,e.instance.exports.main()})}resize(canvas);