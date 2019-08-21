use crate::lib::schedule::SchedulerKey;
use crate::try_send_message_content;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
    prelude::*,
};
use white_rabbit::{DateResult, Duration};

group!({
    name: "misc",
    options: {},
    commands: [set_reminder]
});

#[command]
#[aliases("remindme", "reminder")]
pub fn set_reminder(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    // It might be smart to set a moderately high minimum value for `time`
    // to avoid abuse like tasks that repeat every 100ms, especially since
    // channels have send-message rate limits.
    let time: u64 = args.single()?;
    let args = args.rest().to_string().clone();

    {
        let scheduler = {
            let mut context = ctx.data.write();
            context.get_mut::<SchedulerKey>().unwrap().clone()
        };

        let http = ctx.http.clone();
        let msg = msg.clone();

        let mut scheduler = scheduler.write();

        // Pretty much identical with the `true`-case except for the returned
        // variant.
        scheduler.add_task_duration(Duration::milliseconds(time as i64), move |_| {
            crate::tasks::misc::task_reminder(http.clone(), msg.channel_id, &args);
            DateResult::Done
        });
    }

    try_send_message_content!(ctx, msg, "Ok!")?;

    Ok(())
}
