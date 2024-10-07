use opencv::{
    prelude::*,
    videoio::{
        self,
        VideoCapture,
    },
};

use image::DynamicImage;

use std::{
    thread::{
        self,
        JoinHandle,
    },
    sync::mpsc::{
        self,
        Sender,
        Receiver,
    },
};

pub struct Camera{
    camera_thread: JoinHandle<()>,
    frame_receiver: Receiver<Result<DynamicImage, CameraError>>,
}

pub struct CameraError{
    error_message: String,
}

impl Camera{
    pub fn new() -> Result<Self, CameraError>{
        let camera_stream_result = VideoCapture::new(0, videoio::CAP_ANY);
        let mut camera_stream = match camera_stream_result{
            Ok(camera_stream) => {
                camera_stream
            },
            Err(_error) => {
                return Err(CameraError{error_message: String::from("Unable to open camera")});
            }
        };

        let (frame_sender, frame_receiver) = mpsc::channel::<Result<DynamicImage, CameraError>>();

        let camera_thread = thread::spawn(move || {
            let mut frame = Mat::default();
            loop{
                match camera_stream.read(&mut frame){
                    Ok(_res) => {
                        let dyn_img: DynamicImage = frame.();

                    },
                    Err(_error) => {
                        frame_sender.send(
                            Err(
                                CameraError{
                                    error_message: String::from("Unable to capture frame")
                                }
                            )
                        ).expect("Unable to send error");
                    }
                };
            }
        });

        return Ok(
            Self{
                camera_thread,
                frame_receiver,
        });
    }
}
