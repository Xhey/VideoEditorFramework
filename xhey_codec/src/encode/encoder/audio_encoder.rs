use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use crate::encode::common::packets::AudioPacket;
use std::thread;
use std::io::BufWriter;
use std::mem::transmute_copy;
use std::io::Write;

pub struct PCMWriter {
    file_path: PathBuf,
}

impl PCMWriter {
    pub fn new(file_path: String) -> PCMWriter {
        let file_path = PathBuf::from(file_path);
        PCMWriter {
            file_path
        }
    }

    pub fn start(&self, receiver: Receiver<AudioPacket>) {
        let path = self.file_path.to_str().unwrap();
        info!("file path {}", path);
        let file_path = self.file_path.clone();
        info!("call PCMWriter start method...will create a child thread...");
        thread::Builder::new().name("PCMWriter thread".to_string()).spawn(move || {
            let file = File::create(file_path).expect("open file failed");
            info!("in child thread open file success");
            let mut wtr = BufWriter::new(file);
            loop {
                match receiver.recv() {
                    Ok(packet) => {
                        let len = wtr.write(packet.samples()).expect("write packet.samples failed");
                        info!("write samples to file len = {}", len);
                    },
                    Err(e) => info!("receive packet error {:?}", e)
                }
            }
        }).unwrap();
    }

}