      const compiled_wasm_target = "./target/wasm32-unknown-unknown/release/wasm.wasm";
      const memory = new WebAssembly.Memory({initial: 256,
                                             maximum: 4096 });
      const importObj = {
         env: {
          abortStackOverflow: () => { throw new Error('overflow'); },
          table: new WebAssembly.Table({ initial: 0,
                                         maximum: 0,
                                         element: 'anyfunc' }),
          tableBase: 0,
          memory: memory,
          memoryBase: 1024,
          STACKTOP: 0,
          STACK_MAX: memory.buffer.byteLength
        }
      };
      function wasm_callback(compiled_wasm_object) {
          var checkerboard_lib = compiled_wasm_object.instance.exports;
//// inner js



      }
      if (WebAssembly.instantiateStreaming) {
       WebAssembly.instantiateStreaming(fetch(compiled_wasm_target),
          importObj).then(function (web_obj) {
              return wasm_callback(web_obj);
            });
      } else {
         var gmodule = null;
         fetch(compiled_wasm_target).then(response =>
           response.arrayBuffer()).then(bytes =>
             WebAssembly.compile(bytes)).then(function(module) {
                
                gmodule = module;
                return WebAssembly.instantiate(module, importObj);
             }).then(instance => wasm_callback({
                                                instance:instance,
                                                module:gmodule,
                                               }));
      }    
