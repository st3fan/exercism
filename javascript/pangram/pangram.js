
export const isPangram = (sentence) => {
  let characters = new Set();
  for (const c of sentence.toLowerCase()) {
    if (c >= 'a' && c <= 'z') {
      characters.add(c);
    }
  }
  return characters.size === 26;
};
