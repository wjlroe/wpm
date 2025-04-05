use winit::event::ModifiersState;

pub const NO_MODS: ModifiersState = ModifiersState {
    ctrl: false,
    alt: false,
    shift: false,
    logo: false,
};

pub const MOD_CMD: ModifiersState = ModifiersState {
    ctrl: false,
    alt: false,
    shift: false,
    logo: true,
};

pub const MOD_CTRL: ModifiersState = ModifiersState {
    ctrl: true,
    alt: false,
    shift: false,
    logo: false,
};
