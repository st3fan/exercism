
export const COLORS = ["black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"];

// I decided to do something different than most other submissions: instead of letting indexOf return -1 when you
// pass it an invalid color name, I throw an error. JavaScript is notorious for being very forgiving and I think
// returning -1 will make errors in code using this function hard to find.

export const colorCode = function (color) {
    const value = COLORS.indexOf(color);
    if (value === -1) {
        throw new Error(`Invalid color name: ${color}`)
    }
    return value;
};
