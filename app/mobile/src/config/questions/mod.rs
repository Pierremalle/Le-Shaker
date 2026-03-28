use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;

mod list;
pub use list::QUESTIONS_JSON;

/// Struct for a question
/// The answer here can represent a pure answer or a return for an undone bet
#[derive(Deserialize, Clone)]
pub struct Question {
    pub question: Cow<'static, str>,
    pub answer: Cow<'static, str>,
}

/// Struct for a question
/// The answer here can represent a pure answer or a return for an undone bet
/// Added a category for this one for futur usage
#[derive(Clone, Debug)]
pub struct QuestionAndCategory {
    pub category: Cow<'static, str>,
    pub question: Cow<'static, str>,
    pub answer: Cow<'static, str>,
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
    /// Create a new question decj by parsing the JSON
    pub fn new() -> Self {
        let parsed: HashMap<String, Vec<Question>> =
            serde_json::from_str(QUESTIONS_JSON).unwrap_or_else(|_| HashMap::new());

        let mut questions = Vec::new();

        for (category, qs) in parsed {
            for q in qs {
                questions.push(QuestionAndCategory {
                    category: Cow::Owned(category.clone()),
                    question: q.question,
                    answer: q.answer,
                });
            }
        }

        // if JSON empty
        if questions.is_empty() {
            questions.push(QuestionAndCategory {
                category: Cow::Borrowed("Default"),
                question: Cow::Borrowed("No question"),
                answer: Cow::Borrowed("No answer"),
            });
        }

        Self {
            original: questions.clone(),
            deck: questions,
            cursor: 0,
        }
    }

    /// Give a new question with an iterator
    pub fn next(&mut self) -> QuestionAndCategory {
        if self.deck.is_empty() {
            self.reset();
        }

        let item = self.deck[self.cursor % self.deck.len()].clone();
        self.cursor = self.cursor.wrapping_add(1);

        item
    }

    /// Reset the deck
    fn reset(&mut self) {
        self.deck = self.original.clone();
        self.cursor = 0;
    }
}
