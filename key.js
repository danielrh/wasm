          
          var img_ptr = checkerboard_lib.wasm_alloc_u8(canvas_size);
          var img_buf = new Uint8Array(checkerboard_lib.memory.buffer, img_ptr, canvas_size);
          img_buf.set(canvasData.data)
          var ret = checkerboard_lib.wasm_checkerboard(img_ptr, canvas_size, 1, 56);
          var img_buf = new Uint8Array(checkerboard_lib.memory.buffer, img_ptr, canvas_size);
          canvasData.data.set(img_buf);
          checkerboard_lib.wasm_free_u8(img_ptr, canvas_size);
