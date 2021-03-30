import { sync, unwrap } from "./plugin.ts";

 type Event = {
  event: string
 }

export class Wry {
  readonly id: bigint;

   constructor(url: string) {
      this.id = unwrap(sync("wry_new", { url }));
   }

  loop(): boolean {
    return unwrap(sync("wry_loop", { id: this.id })) === false;
  }

  step(): Event[] {
    return unwrap(sync("wry_step", { id: this.id }));
  }

  run(
    callback?: (events: Event) => void,
    delta = 1000 / 60,
  ): Promise<void> {
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
