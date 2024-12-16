
#[derive(Debug, PartialEq)]
enum Suit {
    Hearts,
    Diamonts,
    Clubs,
    Spades,
}

struct Card {
    rank: i32,
    suit: Suit,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_card_rank() {
        // Arrange
        let value = Card {
            rank: 10,
            suit: Suit::Hearts,
        };

        // Assert / Act
        assert_eq!(value.rank, 10);
        assert_eq!(value.suit, Suit::Hearts);
    }
}
