import { describe, it } from "mocha";
import { $, expect, browser } from "@wdio/globals";
import { slides } from "./slides/slides.list.ts";
import { exemptions } from "./slides/slide-exemptions.list.ts";

// these are empirically determined values in 16:9 ratio
const MAX_HEIGHT = 1333;
const MAX_WIDTH = 750;

async function main_too_big(
  main_element: ChainablePromiseElement,
): Promise<boolean> {
  const main_element_size = await main_element.getSize();
  return (
    main_element_size.height >= MAX_HEIGHT ||
    main_element_size.width > MAX_WIDTH
  );
}

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
          console.info("slide " + slide + " is on the exemption list");
          // one of them (height, width) should fail
          expect(await main_too_big(main_element)).toBe(true);
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
          expect(!(await main_too_big(main_element))).toBe(true);
        },
      );
    }
  }
});
