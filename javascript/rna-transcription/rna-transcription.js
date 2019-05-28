import { ScriptTransformer } from "jest-runtime";

/**
 * Given a DNA strand, return its RNA complement (per RNA transcription).
 * @param {string} The DNA strand.
 * @returns {string} The transcribed RNA complement.
 * @throws An Error on invalid input.
 */

export const toRna = (strand) => {
    let rna = '';
    for (const nucleotide of strand) {
        switch (nucleotide) {
            case 'G':
                rna += 'C';
                break;
            case 'C':
                rna += 'G';
                break;
            case 'T':
                rna += 'A';
                break;
            case 'A':
                rna += 'U';
                break;
            default:
                throw new Error("Invalid nucleotide: " + nucleotide);
        }
    }
    return rna;
};

