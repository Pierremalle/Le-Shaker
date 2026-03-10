use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::borrow::Cow;
use std::collections::HashMap;

const QUESTIONS_JSON: &str = r#"
{
  "Chiche": [
    { "question": "Fais quelque chose", "answer": "Bois 3 shots" },
    { "question": "Fais quelque chose 2 fois", "answer": "Bois 6 shots" },
    { "question": "Fais quelque chose 3 fois", "answer": "Bois 9 shots" }
  ],
  "Vérité": [
    { "question": "On adore Fafa ?", "answer": "Bois 9999 shots" },
    { "question": "On adore Lucas ?", "answer": "Bois 1 shot" }
  ]
}
"#;

#[derive(Deserialize)]
pub struct Question {
    pub question: Cow<'static, str>,
    pub answer: Cow<'static, str>,
}

#[derive(Serialize, Clone)]
pub struct QuestionAndCategory {
    pub category: Cow<'static, str>,
    pub question: Cow<'static, str>,
    pub answer: Cow<'static, str>,
}

pub struct QuestionDeck {
    original: Vec<QuestionAndCategory>,
    deck: Vec<QuestionAndCategory>,
}

impl QuestionDeck {
    pub fn new() -> Self {
        let parsed: HashMap<String, Vec<Question>> = serde_json::from_str(QUESTIONS_JSON).unwrap();

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

        let mut deck = questions.clone();
        deck.shuffle(&mut rand::rng());

        Self {
            original: questions,
            deck,
        }
    }

    fn reshuffle(&mut self) {
        self.deck = self.original.clone();
        self.deck.shuffle(&mut rand::rng());
    }

    pub fn next(&mut self) -> QuestionAndCategory {
        if let Some(q) = self.deck.pop() {
            return q;
        }

        self.reshuffle();
        self.deck.pop().unwrap()
    }
}
