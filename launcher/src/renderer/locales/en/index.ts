import common from "./common.json";
import home from "./home.json";
import navbar from "./navbar.json";
import replays from "./replays.json";
import saves from "./saves.json";
import settings from "./settings.json";
import supervisor from "./supervisor.json";

export default {
  common,
  home,
  navbar,
  replays,
  saves,
  settings,
  supervisor,
} as { [namespace: string]: { [key: string]: string } };
