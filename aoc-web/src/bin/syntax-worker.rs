use yew_agent::Registrable;

use aoc_web::syntax::SyntaxHighlightTask;

fn main() {
    SyntaxHighlightTask::registrar().register();
}
