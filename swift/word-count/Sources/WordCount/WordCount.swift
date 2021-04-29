import NaturalLanguage

struct WordCount {
    let words: String
    
    func count() -> [String: Int] {
        let tokenizer = NLTokenizer(unit: .word)
        tokenizer.string = words
        var counts = [String: Int]()
        for wordRange in tokenizer.tokens(for: words.startIndex..<words.endIndex) {
            let word = String(words[wordRange]).lowercased()
            counts[word, default: 0] += 1
        }
        return counts
    }
}
