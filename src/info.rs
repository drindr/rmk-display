use rmk::event::ControllerEvent;
use rmk::keycode::ModifierCombination;

pub(crate) struct Infomation<const PERIPHERAL_COUNT: usize> {
    /// Battery level
    pub battery: u8,
    /// Charging state changed
    pub charging: bool,
    /// Layer changed
    pub layer: u8,
    /// Modifier changed
    pub modifier: ModifierCombination,
    /// Typing speed
    pub wpm: u16,
    /// Usb or Ble connection
    pub connection_type: u8,
    /// Split peripheral connection
    pub peripheral: [bool; PERIPHERAL_COUNT],
}

impl<const PERIPHERAL_COUNT: usize> Default for Infomation<PERIPHERAL_COUNT> {
    fn default() -> Self {
        Self {
            battery: 0,
            charging: false,
            layer: 0,
            modifier: ModifierCombination::default(),
            wpm: 0,
            connection_type: 0,
            peripheral: [false; PERIPHERAL_COUNT],
        }
    }
}
impl<const PERIPHERAL_COUNT: usize> Infomation<PERIPHERAL_COUNT> {
    pub fn update_info(&mut self, event: &ControllerEvent) {
        match event {
            ControllerEvent::Battery(battery) => {
                self.battery = *battery;
            }
            ControllerEvent::ChargingState(charging) => {
                self.charging = *charging;
            }
            ControllerEvent::Layer(layer) => {
                self.layer = *layer;
            }
            ControllerEvent::Modifier(modifier) => {
                self.modifier = *modifier;
            }
            ControllerEvent::Wpm(wpm) => {
                self.wpm = *wpm;
            }
            ControllerEvent::ConnectionType(connection_type) => {
                self.connection_type = *connection_type;
            }
            ControllerEvent::SplitPeripheral(index, connected) => {
                if *index < PERIPHERAL_COUNT {
                    self.peripheral[*index] = *connected;
                }
            }
            _ => {}
        }
    }
}

/// for type erasing, reduce the noise of PERIPHERAL_COUNT
pub trait InfoProvider {
    fn update(&mut self, event: &ControllerEvent);
    fn battery(&self) -> u8;
    fn charging(&self) -> bool;
    fn layer(&self) -> u8;
    fn modifier(&self) -> ModifierCombination;
    fn wpm(&self) -> u16;
    fn connection_type(&self) -> u8;
    fn peripheral(&self, index: usize) -> Option<bool>;
}

impl<const PERIPHERAL_COUNT: usize> InfoProvider for Infomation<PERIPHERAL_COUNT> {
    fn update(&mut self, event: &ControllerEvent) {
        self.update_info(event);
    }
    fn battery(&self) -> u8 {
        self.battery
    }
    fn charging(&self) -> bool {
        self.charging
    }
    fn layer(&self) -> u8 {
        self.layer
    }
    fn modifier(&self) -> ModifierCombination {
        self.modifier
    }
    fn wpm(&self) -> u16 {
        self.wpm
    }
    fn connection_type(&self) -> u8 {
        self.connection_type
    }
    fn peripheral(&self, index: usize) -> Option<bool> {
        if index < PERIPHERAL_COUNT {
            Some(self.peripheral[index])
        } else {
            None
        }
    }
}
