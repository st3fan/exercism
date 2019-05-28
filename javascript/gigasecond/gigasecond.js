
const GIGASECOND_MILLIS = 1000000000000;

export const gigasecond = (date) => {
    return new Date(date.getTime() + GIGASECOND_MILLIS);
};
