import { Application as Oak } from "https://deno.land/x/oak/mod.ts";
import { listenAndServe } from "https://deno.land/std/http/server.ts";
import { Wry } from '../mod.ts'

const wryApplication = new Wry("http://localhost:8080");
const oakApplication = new Oak();

oakApplication.use((ctx) => {
  ctx.response.body = "Hello World!";
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
  }
}, 1)
