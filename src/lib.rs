pub mod article;
pub mod note;
pub mod error;

use error::KBGenError;

use handlebars::Handlebars;

const TITLE_TEMPLATE_NAME: &str = "title";
const TITLE_TEMPLATE: &str = "# {{title}}";

pub trait Generateable {
    fn setup(&self, hbs: &mut Handlebars)  -> Result<(), KBGenError>{
        hbs.register_template_string(TITLE_TEMPLATE_NAME, TITLE_TEMPLATE)?;
        Ok(())
    }
    fn generate(&self, destination: &str, hbs: &mut Handlebars)  -> Result<(), KBGenError>;
}
