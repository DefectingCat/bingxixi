import { warn, debug, info, error } from "@tauri-apps/plugin-log";

function forwardConsole(
  fnName: "log" | "debug" | "info" | "warn" | "error",
  logger: (message: string) => Promise<void>,
) {
  const original = console[fnName];
  console[fnName] = (...message) => {
    original(...message);
    logger(message.toString());
  };
}

export function initConsole() {
  forwardConsole("log", info);
  forwardConsole("debug", debug);
  forwardConsole("info", info);
  forwardConsole("warn", warn);
  forwardConsole("error", error);
}
