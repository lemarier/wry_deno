import { sync, unwrap } from "./plugin.ts";

 type Event = {
  event: string
 }

export type Size = { physical: PhysicalSize } | { logical: LogicalSize };
export type PhysicalSize = { width: number; height: number };
export type LogicalSize = { width: number; height: number };

export class Wry {
  readonly id: bigint;

   constructor(url: string) {
      this.id = unwrap(sync("wry_new", { url }));
   }

   set_minimized(minimized: boolean): boolean {
    return unwrap(sync("wry_set_minimized", { id: this.id, minimized }));
  }

  set_maximized(maximized: boolean): boolean {
    return unwrap(sync("wry_set_maximized", { id: this.id, maximized }));
  }

  set_visible(visible: boolean): boolean {
    return unwrap(sync("wry_set_visible", { id: this.id, visible }));
  }

  set_inner_size(size: Size): void {
    unwrap(sync("wry_set_inner_size", { id: this.id, size }));
  }

  loop(): boolean {
    return unwrap(sync("wry_loop", { id: this.id })) === false;
  }

  step(): Event[] {
    return unwrap(sync("wry_step", { id: this.id }));
  }

  run(
    callback?: (events: Event) => void,
    delta = 1,
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
