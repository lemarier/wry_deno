import { Application as Oak } from "https://deno.land/x/oak/mod.ts";
import { listenAndServe } from "https://deno.land/std/http/server.ts";
import { Wry } from '../mod.ts'

const wryApplication = new Wry("http://localhost:8080");
const oakApplication = new Oak();

oakApplication.use((ctx) => {
  ctx.response.body = `Deno: ${Deno.version.deno} Typescript: ${Deno.version.typescript}`;
});

// using OAK without blocking the thread
listenAndServe({ port: 8080 }, async (request) => {
  const response = await oakApplication.handle(request);
  if (response) {
    request.respond(response);
  }
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
