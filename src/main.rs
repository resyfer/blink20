use std::time::Duration;

use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween, sound::static_sound::StaticSoundData,
};
use notify_rust::Notification;
use tokio::{sync::oneshot, time};

async fn periodic_notif() {
    let mut long = true;

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        .expect("Could not play audio");
    let sound_data =
        StaticSoundData::from_file("./assets/bell.wav").expect("Could not load audio.");

    loop {
        let interval = if long {
            20 * 60 /* 20 minutes */
        } else {
            20 /* 20 seconds */
        };

        let summary = if long {
            "LOOK AWAY!"
        } else {
            "BACK TO WORK!!!"
        };

        time::sleep(Duration::from_secs(interval)).await;

        let mut sound_handler = manager
            .play(sound_data.clone())
            .expect("Could not play audio.");
        sound_handler.set_loop_region(..);

        let (sendr, recvr) = oneshot::channel::<Result<(), &'static str>>();

        Notification::new()
            .summary(&summary)
            .appname("Blink 20")
            .timeout(0)
            .show()
            .unwrap()
            .wait_for_action(|action| match action {
                "__closed" => {
                    let _ = sendr.send(Ok(()));
                }
                x => {
                    dbg!(&x);
                    let _ = sendr.send(Err("Hi"));
                }
            });

        let recvr_ret = recvr.await;

        sound_handler.stop(Tween::default());

        if let Err(_) = recvr_ret.unwrap() {
            return;
        }

        long = !long;
    }
}

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(periodic_notif()).await;
}
