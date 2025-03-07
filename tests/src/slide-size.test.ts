import { describe, it } from "mocha";
import { $, expect, browser } from "@wdio/globals";
import { slides } from "./slides/slides.list";
import { exemptions } from "./slides/slide-exemptions.list";

// these are empirically determined values in 16:9 ratio
const MAX_HEIGHT = 1333;
const MAX_WIDTH = 750;

describe("Slide", () => {
  for (const slide of slides) {
    if (exemptions.includes(slide)) {
      // This slide is exempted and violated rules before.
      // It is expected to still do this and if not it should be removed from exemptions.
      // This acts as a regression check
      it(
        " " +
          slide +
          " is on the exemption list but should be removed from slide-exemptions.list.ts",
        async () => {
          await browser.url("/" + slide);
          const main_element = $("#content > main");
          const main_element_size = await main_element.getSize();
          console.info("slide " + slide + " is on the exemption list");
          // one of them (height, width) should fail
          expect(
            main_element_size.height >= MAX_HEIGHT ||
              main_element_size.width > MAX_WIDTH,
          ).toBe(true);
        },
      );
    } else {
      it(
        " " +
          slide +
          " should not be higher than " +
          MAX_HEIGHT +
          " pixels or wider than " +
          MAX_WIDTH +
          " pixels",
        async () => {
          await browser.url("/" + slide);
          const main_element = $("#content > main");
          const main_element_size = await main_element.getSize();
          expect(
            main_element_size.height < MAX_HEIGHT &&
              main_element_size.width <= MAX_WIDTH,
          ).toBe(true);
        },
      );
    }
  }
});
