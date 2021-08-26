import("../pkg/index.js")
  .then((wasm) => {
    var global_buffer;

    document.getElementById("img_display").style.display = "none";
    document.getElementById("process").style.display = "none";

    document.getElementById("hash").addEventListener("click", () => {
      let text = document.getElementById("string").value;
      let a = wasm.digest(text);
      document.getElementById(
        "hash_string"
      ).innerHTML = `The hash of ${text} is ${a}`;
      document.getElementById("string").value = "";
    });

    //select image
    document.getElementById("img_file").addEventListener("change", () => {
      var selectedFile = document.getElementById("img_file").files[0];
      var reader = new FileReader();
      reader.readAsArrayBuffer(selectedFile);
      reader.onload = function () {
        var uint8Array = new Uint8Array(this.result);
        write_buffer_to_html(uint8Array);
      };
    });


    //apply filter
    document.getElementById("fileeffect").addEventListener("click", () => {
      if (global_buffer !== undefined) {
        let effect = document.getElementById("effects").value;
        let b = wasm.image_effect(global_buffer, effect);
        write_buffer_to_html(b);
      } else {
        alert("select an image");
      }
    });

    //resize image
    document.getElementById("resize_btn").addEventListener("click", () => {
      if (global_buffer !== undefined) {
        let height = parseInt(document.getElementById("img_height").value);
        let weight = parseInt(document.getElementById("img_width").value);
        let b = wasm.resize(global_buffer, height, weight);
        write_buffer_to_html(b);
      } else {
        alert("select an image");
      }
    });

    //blur image
    document.getElementById("blur_btn").addEventListener("click", () => {
      if (global_buffer !== undefined) {
        let sigma = parseInt(document.getElementById("blur_sigma").value);
        let b = wasm.blur(global_buffer, sigma);
        write_buffer_to_html(b);
      } else {
        alert("select an image");
      }
    });

    //write buffer to image tag and keep track of global buffer state
    function write_buffer_to_html(array) {
      document.getElementById("img_display").style.display = "block";

      document.getElementById("my_img").src = URL.createObjectURL(
        new Blob([array.buffer], { type: "image/png" })
      );
      global_buffer = array;
    }
  })
  .catch(console.error);
