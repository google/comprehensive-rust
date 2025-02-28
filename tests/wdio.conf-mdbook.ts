import { deepmerge } from "deepmerge-ts";
import { config as default_config } from "./wdio.conf.js";

// have main config file as default but overwrite how the code is served
export const config = deepmerge(
  default_config,
  {
    // use the mdbook served content
    baseUrl: "http://localhost:3000",
    // clean services
    services: [],
  },
  { clone: false },
);
