import { $, browser } from "@wdio/globals";

export default class Slide {
  /**
   * conviencene functions for interacting with a slide
   **/

  get main_content() {
    return $("#content > main");
  }

  /**
   *
   * @param element the element to be checked
   * @param height the maximum height
   * @param width the maximum width
   * @returns true if either height or width is higher than the provided numbers
   */
  async violates_max_size(
    element: ChainablePromiseElement,
    height: number,
    width: number,
  ): Promise<boolean> {
    const main_element_size = await element.getSize();
    return (
      main_element_size.height >= height || main_element_size.width > width
    );
  }

  async load(slide_path: string) {
    // ensure this is prefixed with /
    if (!slide_path.startsWith("/")) {
      slide_path = "/" + slide_path;
    }
    return await browser.url(slide_path);
  }
}
