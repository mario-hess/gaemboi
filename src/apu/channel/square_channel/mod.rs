pub mod channel1;
pub mod channel2;
mod core;

use crate::apu::channel::square_channel::{channel1::Channel1, channel2::Channel2, core::Core};

pub trait SquareChannelController {
    fn read_byte(&self, core: &Core, address: u16) -> u8;
    fn write_byte(&mut self, core: &mut Core, address: u16, value: u8);
}

pub enum ChannelType {
    Channel1,
    Channel2,
}

pub struct SquareChannel {
    core: Core,
    channel: Box<dyn SquareChannelController>,
}

impl SquareChannel {
    pub fn new(channel_type: ChannelType) -> Self {
        let channel: Box<dyn SquareChannelController> = match channel_type {
            ChannelType::Channel1 => Box::new(Channel1::new()),
            ChannelType::Channel2 => Box::new(Channel2::new()),
        };

        Self {
            core: Core::new(),
            channel,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.channel.write_byte(&mut self.core, address, value);
    }
}
