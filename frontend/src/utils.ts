export function findOrFail(selector: string, source: Element | Document): Element {
  let result = source.querySelector(selector);
  if (!result) {
    throw new Error("Could not find element for selector '" + selector + "' and source " + source);
  }
  return result;
}

export function positiveModule(value: number, modulus: number) {
  let result = value % modulus;
  return result < 0 ? result + modulus : result;
}
