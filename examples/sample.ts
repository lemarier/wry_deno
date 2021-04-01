import { Wry } from "https://raw.githubusercontent.com/lemarier/wry_deno/main/mod.ts";
import { listenAndServe } from "https://deno.land/std/http/server.ts";

const wryApplication = new Wry("http://localhost:8000");

listenAndServe({ port: 8000 }, (req) => {
   req.respond({ body: `Hello from Deno ${Deno.version.deno}` });
});

wryApplication.run(({event}) => {
  switch (event) {
    case 'close':
      Deno.exit()
      break;
    case 'windowCreated':
      console.log("It works! Window created , if webview didn't show, try to resize window");
      break;
    case 'domContentLoaded':
      console.log("It works! domContentLoaded")
      break;
    }
});