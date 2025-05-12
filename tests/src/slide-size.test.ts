import { describe, it } from "mocha";
import { expect } from "@wdio/globals";
import { slides } from "./slides/slides.list.ts";
import { size_exemptions } from "./slides/slide-exemptions.list.ts";
import Slide from "./objects/slide.ts";

// these are empirically determined values in 16:9 ratio
const MAX_HEIGHT = 1333;
const MAX_WIDTH = 750;

describe("Slide", () => {
  const slide = new Slide();
  for (const slide_path of slides) {
    if (size_exemptions.includes(slide_path)) {
      // This slide is exempted and violated rules before.
      // It is expected to still do this and if not it should be removed from exemptions.
      // This acts as a regression check
      it(
        " " +
          slide_path +
          " is on the exemption list but should be removed from slide-exemptions.list.ts",
        async () => {
          await slide.load(slide_path);
          const main_element = slide.main_content;
          console.info("slide " + slide_path + " is on the exemption list");
          expect(
            await slide.violates_max_size(main_element, MAX_HEIGHT, MAX_WIDTH),
          ).toBe(true);
        },
      );
    } else {
      it(
        " " +
          slide_path +
          " should not be higher than " +
          MAX_HEIGHT +
          " pixels or wider than " +
          MAX_WIDTH +
          " pixels",
        async () => {
          await slide.load(slide_path);
          const main_element = slide.main_content;
          expect(
            await slide.violates_max_size(main_element, MAX_HEIGHT, MAX_WIDTH),
          ).toBe(false);
        },
      );
    }
  }
});
