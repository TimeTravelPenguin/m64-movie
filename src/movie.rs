/// An enum representing the buttons on a Mupen64 controller.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ControllerButton {
    /// The right directional pad button.
    DPadRight,
    /// The left directional pad button.
    DPadLeft,
    /// The down directional pad button.
    DPadDown,
    /// The up directional pad button.
    DPadUp,
    /// The start button.
    Start,
    /// The Z button.
    Z,
    /// The B button.
    B,
    /// The A button.
    A,
    /// The C-right button.
    CRight,
    /// The C-left button.
    CLeft,
    /// The C-down button.
    CDown,
    /// The C-up button.
    CUp,
    /// The right trigger button.
    TriggerRight,
    /// The left trigger button.
    TriggerLeft,
    /// Reserved button 01.
    Reserved01,
    /// Reserved button 02.
    Reserved02,
}
