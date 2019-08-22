use crate::lib::internal::ScheduleId;
use crate::lib::schedule::SchedulerKey;
use crate::lib::settings::reminders::RemindersSettingsHandler;
use crate::lib::settings::SettingsHandler;
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

        let reminders = {
            let mut context = ctx.data.write();
            context
                .get_mut::<RemindersSettingsHandler>()
                .unwrap()
                .clone()
        };

        let http = ctx.http.clone();
        let msg = msg.clone();
        let user_id = msg.author.id.0 as i64;
        let cloned_args = args.clone();

        let mut scheduler = scheduler.write();

        // Pretty much identical with the `true`-case except for the returned
        // variant.
        scheduler.add_task_duration(Duration::milliseconds(time as i64), move |_| {
            crate::tasks::misc::task_reminder(http.clone(), msg.channel_id, &args);
            DateResult::Done
        });

        // TODO(kyranet): This entry should get removed from the db once executed
        if let Err(why) = reminders.insert(
            &ScheduleId(time as i64),
            &["user_id", "content"],
            &[&user_id, &cloned_args],
        ) {
            return Err(serenity::framework::standard::CommandError(why.to_string()));
        }
    }

    try_send_message_content!(ctx, msg, "Ok!")?;

    Ok(())
}
