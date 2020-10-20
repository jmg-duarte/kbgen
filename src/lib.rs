pub mod article;
pub mod error;
pub mod note;

use std::convert::TryFrom;

use error::KBGenError;

use handlebars::Handlebars;
use serde::Serialize;

const TITLE_TEMPLATE_NAME: &str = "title";
const TITLE_TEMPLATE: &str = "# {{title}}";

pub struct Generator<'a, T>
where
    T: Serialize,
{
    data: T,
    template_name: &'a str,
    template: &'a str,
    hbs: Handlebars<'a>,
}

impl<'a, T> Generator<'a, T>
where
    T: Serialize,
{
    pub fn new(data: T, template_name: &'a str, template: &'a str) -> Result<Self, KBGenError> {
        Self {
            data,
            template_name,
            template,
            hbs: Handlebars::new(),
        }
        .setup()
    }

    fn setup(mut self) -> Result<Self, KBGenError> {
        self.hbs
            .register_template_string(TITLE_TEMPLATE_NAME, TITLE_TEMPLATE)?;
        self.hbs
            .register_template_string(self.template_name, self.template)?;
        Ok(self)
    }

    pub fn generate(&self) -> Result<String, KBGenError> {
        // let path = path::Path::new(&destination);
        // if path.exists() {
        //     return Err(KBGenError::FileExists(destination.to_string()));
        // }

        let render_output = self.hbs.render(self.template_name, &self.data)?;
        Ok(render_output)
    }
}

impl TryFrom<article::Article> for Generator<'_, article::Article> {
    type Error = KBGenError;

    fn try_from(value: article::Article) -> Result<Self, Self::Error> {
        Self::new(value, article::TEMPLATE_NAME, article::TEMPLATE)
    }
}

impl TryFrom<note::Note> for Generator<'_, note::Note> {
    type Error = KBGenError;

    fn try_from(value: note::Note) -> Result<Self, Self::Error> {
        Self::new(value, note::TEMPLATE_NAME, note::TEMPLATE)
    }
}

#[cfg(test)]
mod tests {
    use crate::{article, note, Generator};
    use std::convert::TryFrom;
    #[test]
    fn test_article_gen() {
        let article = article::Article {
            title: String::from("A somewhat long title for the article"),
            authors: String::from("José Duarte"),
            doi: String::from("666-666"),
        };
        let generator = Generator::try_from(article).unwrap();
        let output = generator.generate().unwrap();
        assert_eq!(
            output,
            "# A somewhat long title for the article

---

**Authors** - José Duarte

**DOI** - [666-666](https://doi.org/666-666)

---

---

**Notes**

**References**

---"
        );
    }

    #[test]
    fn test_note_gen() {
        let note = note::Note {
            title: String::from("A somewhat long title for the note"),
        };
        let generator = Generator::try_from(note).unwrap();
        let output = generator.generate().unwrap();
        assert_eq!(
            output,
            "# A somewhat long title for the note"
        );
    }
}
