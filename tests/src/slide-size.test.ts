import { describe, it } from "mocha";
import { $, expect, browser } from "@wdio/globals";
import { slides } from "./slides/slides.list";

describe("Slide", () => {
  for (const slide of slides) {
    it(
      " " +
        slide +
        " should not be higher than 1333 pixels or wider than 750 pixels",
      async () => {
        await browser.url("/" + slide);
        const main_element = $("#content > main");
        const main_element_size = await main_element.getSize();
        expect(main_element_size.height < 1333).toBe(true);
        expect(main_element_size.width <= 750).toBe(true);
      },
    );
  }
});
