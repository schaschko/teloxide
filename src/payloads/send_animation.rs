// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{ChatId, InputFile, Message, MessageEntity, ParseMode, ReplyMarkup};

impl_payload! {
    @[multipart]
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [`Message`] is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, Clone, Serialize)]
    pub SendAnimation (SendAnimationSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Animation to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub animation: InputFile,
        }
        optional {
            /// Duration of the animation in seconds
            pub duration: u32,
            /// Animation width
            pub width: u32,
            /// Animation height
            pub height: u32,
            /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub thumb: InputFile,
            /// Animation caption (may also be used when resending videos by _file\_id_), 0-1024 characters after entities parsing
            pub caption: String [into],
            /// Mode for parsing entities in the animation caption. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the photo caption, which can be specified instead of _parse\_mode_
            pub caption_entities: Vec<MessageEntity> [collect],
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
