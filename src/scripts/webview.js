class Webview {
   constructor(url) {
      this.id = Deno.core.jsonOpSync('wry_new', { url });
   }

   loop() {
      return Deno.core.jsonOpSync('wry_loop', { id: this.id }) === false;
   }

   step() {
      return Deno.core.jsonOpSync('wry_step', { id: this.id });
   }

   run(
      callback,
      delta = 1000/30,
    ) {
      return new Promise((resolve) => {
        const interval = setInterval(() => {
          const success = this.loop();
  
          if (callback !== undefined) {
            const events = this.step();
  
            for (const event of events) {
              callback(event);
            }
          }
  
          if (!success) {
            resolve();
            clearInterval(interval);
          }
        }, delta);
      });
    }
}