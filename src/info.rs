use rmk::event::ControllerEvent;
use rmk::keycode::ModifierCombination;

pub(crate) struct DispInfo<const PERIPHERAL_COUNT: usize> {
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
    /// temporary bongo up and down
    pub up: bool,
}

impl<const PERIPHERAL_COUNT: usize> Default for DispInfo<PERIPHERAL_COUNT> {
    fn default() -> Self {
        Self {
            battery: 0,
            charging: false,
            layer: 0,
            modifier: ModifierCombination::default(),
            wpm: 0,
            connection_type: 0,
            peripheral: [false; PERIPHERAL_COUNT],
            up: false,
        }
    }
}
impl<const PERIPHERAL_COUNT: usize> DispInfo<PERIPHERAL_COUNT> {
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
