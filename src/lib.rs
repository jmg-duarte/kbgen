pub mod article;
pub mod note;

use handlebars::Handlebars;

const TITLE_TEMPLATE_NAME: &str = "title";
const TITLE_TEMPLATE: &str = "# {{title}}";

pub trait Generateable {
    fn setup(&self, hbs: &mut Handlebars) {
        hbs.register_template_string(TITLE_TEMPLATE_NAME, TITLE_TEMPLATE).unwrap();
    }
    fn generate(&self, destination: &str, hbs: &mut Handlebars);
}
