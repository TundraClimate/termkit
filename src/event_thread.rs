use crossterm::event::{self, Event};
use std::error::Error;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct EventThread {
    rv: Receiver<Event>,
    sd: Sender<Option<()>>,
}

impl EventThread {
    pub async fn shatdown(&mut self) -> Result<(), Box<dyn Error>> {
        self.sd.send(Some(())).await?;
        Ok(())
    }

    pub async fn respond(&mut self) -> Result<(), Box<dyn Error>> {
        self.sd.send(None).await?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<Event, mpsc::error::TryRecvError> {
        self.rv.try_recv()
    }

    pub fn spawn() -> EventThread {
        let (handler, rv) = mpsc::channel::<Event>(100);
        let (sd, mut wait) = mpsc::channel::<Option<()>>(100);

        tokio::spawn(async move {
            loop {
                if let Ok(event) = event::read() {
                    handler.send(event).await.expect("found send error");
                    if let Some(Some(_)) = wait.recv().await {
                        break;
                    }
                }
            }
        });

        EventThread { rv, sd }
    }
}
