use serenity::{
    prelude::Context,
    model::channel::Message,
    framework::standard::{ CommandResult, macros::command },
};

#[command]
#[aliases("version", "ver")]
fn versions(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let lang_manager = data.get::<crate::LangManager>().unwrap().lock().unwrap();
    let mut fields: Vec<(String, String, bool)> = Vec::new();
    for boxed_lang in lang_manager.get_languages().values() {
        if lang_manager.is_language_available(&(*boxed_lang)) {
            fields.push((
                boxed_lang.get_lang_name(),
                boxed_lang.check_compiler_or_interpreter().stdout_capture().read().unwrap(),
                true
            ));
        }
    }
    fields.sort();

    let _ = msg.channel_id.send_message(&ctx, |m| m
        .embed(|e| e
            .title("Versions")
            .description("A list of versions of languages available.")
            .fields(fields)
        )
    )?;

    Ok(())
}
