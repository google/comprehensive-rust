/**
 * This file contains tests that check the style of the slides.
 * It checks that the slides are not too high or wide and that
 * the code examples are not too wide or high based on the visibility of scrollbars.
 *
 * Slides that exist on the exemptions lists are tested for that violation
 * and if they are not violating the style, this alerts and the author should remove
 * the slide from the exemption list. This acts as a regression check.
 */
import { describe, it } from "mocha";
import { expect } from "@wdio/globals";
import { slides } from "./slides/slides.list.ts";
import {
  playground_size_exemptions,
  size_exemptions,
} from "./slides/slide-exemptions.list.ts";
import Slide from "./objects/slide.ts";

// these are empirically determined values in 16:9 ratio
const MAX_HEIGHT = 1333;
const MAX_WIDTH = 750;

const slide = new Slide();
slides.forEach((slide_path) => {
  describe("Slide " + slide_path, () => {
    before(async () => {
      slide.load(slide_path);
    });

    // slide size evaluation
    if (size_exemptions.includes(slide_path)) {
      // This slide is exempted and violated rules before.
      // It is expected to still do this and if not it should be removed from exemptions.
      // This acts as a regression check
      it("is on the exemption list but should be removed from size_exemptions in slide-exemptions.list.ts", async () => {
        const main_element = slide.main_content;
        console.info("slide " + slide_path + " is on the exemption list");
        expect(
          await slide.violates_max_size(main_element, MAX_HEIGHT, MAX_WIDTH),
        ).toBe(true);
      });
    } else {
      it(
        "should not be higher than " +
          MAX_HEIGHT +
          " pixels or wider than " +
          MAX_WIDTH +
          " pixels",
        async () => {
          const main_element = slide.main_content;
          expect(
            await slide.violates_max_size(main_element, MAX_HEIGHT, MAX_WIDTH),
          ).toBe(false);
        },
      );
    }

    // playground code examples are not too wide
    if (playground_size_exemptions.includes(slide_path)) {
      it("is on the exemption list but should be removed from playground_size_exemptions in slide-exemptions.list.ts", async () => {
        // This slide is exempted and violated rules before.
        // It is expected to still do this and if not it should be removed from exemptions.
        // This acts as a regression check
        await Promise.any([
          expect(slide.scrollbar_h).toBeDisplayed(),
          expect(slide.scrollbar_v).toBeDisplayed(),
        ]);
      });
    } else {
      it("should not show a scrollbar", async () => {
        if (await slide.has_code_example) {
          await Promise.all([
            expect(slide.scrollbar_h).not.toBeDisplayed(),
            expect(slide.scrollbar_v).not.toBeDisplayed(),
          ]);
        }
      });
    }
  });
});
