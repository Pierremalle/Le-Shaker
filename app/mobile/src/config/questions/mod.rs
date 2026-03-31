use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;

use rand::rng;
use rand::seq::SliceRandom;

/// Struct for a question
/// The answer here can represent a pure answer or a return for an undone bet
#[derive(Deserialize, Clone)]
pub struct Question {
    pub question: Cow<'static, str>,
    pub alternative: Cow<'static, str>,
}

/// Struct for a question
/// The answer here can represent a pure answer or a return for an undone bet
/// Added a category for this one for futur usage
#[derive(Clone, Debug)]
pub struct QuestionAndCategory {
    pub category: Cow<'static, str>,
    pub question: Cow<'static, str>,
    pub alternative: Cow<'static, str>,
}

/// A deck of question used to get a new question
/// And ensure we don't repeat already answered one before everything is used.
#[derive(Debug, Clone)]
pub struct QuestionDeck {
    original: Vec<QuestionAndCategory>,
    deck: Vec<QuestionAndCategory>,
    cursor: usize,
}

impl QuestionDeck {
    /// Create a new question deck from file and suffle it
    pub fn new() -> Self {
        const QUESTIONS_JSON: &str = include_str!("questions.json");

        let parsed: HashMap<String, Vec<Question>> =
            serde_json::from_str(QUESTIONS_JSON).unwrap_or_else(|_| HashMap::new());

        let mut questions = Vec::new();

        for (category, qs) in parsed {
            for q in qs {
                questions.push(QuestionAndCategory {
                    category: Cow::Owned(category.clone()),
                    question: q.question,
                    alternative: q.alternative,
                });
            }
        }

        if questions.is_empty() {
            questions.push(QuestionAndCategory {
                category: Cow::Borrowed("Default"),
                question: Cow::Borrowed("No question"),
                alternative: Cow::Borrowed("No alternative"),
            });
        }

        questions.shuffle(&mut rng());

        Self {
            original: questions.clone(),
            deck: questions,
            cursor: 0,
        }
    }

    // Reset the deck and reshuffle it
    fn reset(&mut self) {
        self.deck = self.original.clone();

        self.deck.shuffle(&mut rng());

        self.cursor = 0;
    }
}

/// Iterator for a QuestionDeck
pub struct QuestionDeckIterator {
    deck: Vec<QuestionAndCategory>,
}

impl Iterator for QuestionDeckIterator {
    type Item = QuestionAndCategory;

    /// Simple implementation of an iterator for QuestionDeck
    fn next(&mut self) -> Option<Self::Item> {
        self.deck.pop()
    }
}
impl IntoIterator for QuestionDeck {
    type Item = QuestionAndCategory;
    type IntoIter = QuestionDeckIterator;

    /// Convert a QuestionDeck into an Iterator of cards
    fn into_iter(self) -> Self::IntoIter {
        let mut deck = self.original;
        deck.shuffle(&mut rand::rng());

        QuestionDeckIterator { deck }
    }
}

impl QuestionDeck {
    /// Give a new card and reste the deck if empty
    pub fn next_card(&mut self) -> QuestionAndCategory {
        if self.deck.is_empty() {
            self.reset();
        }

        self.deck.pop().expect("Deck should not be empty")
    }
}
