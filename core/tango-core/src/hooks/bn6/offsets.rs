#[derive(Clone, Copy)]
pub(super) struct EWRAMOffsets {
    // Outgoing packet.
    pub(super) tx_packet: u32,

    // Incoming packet.
    pub(super) rx_packet_arr: u32,

    /// Location of the battle state struct in memory.
    pub(super) battle_state: u32,

    /// Start screen jump table control.
    pub(super) start_screen_control: u32,

    /// Title menu jump table control.
    pub(super) title_menu_control: u32,

    /// START menu jump table control.
    pub(super) menu_control: u32,

    /// START menu submenu (e.g. comm menu) jump table control.
    pub(super) submenu_control: u32,

    /// Local RNG state. Doesn't need to be synced.
    pub(super) rng1_state: u32,

    /// Shared RNG state. Must be synced.
    pub(super) rng2_state: u32,
}

#[derive(Clone, Copy)]
pub(super) struct ROMOffsets {
    /// This is the entry point for the start screen, i.e. when the CAPCOM logo is displayed.
    ///
    /// It is expected that at this point, you may write to the start_screen_control EWRAM address to skip to the title screen.
    pub(super) start_screen_jump_table_entry: u32,

    /// This is immediately after SRAM is copied to EWRAM and unmasked.
    ///
    /// At this point, it is safe to do the equivalent of selecting the CONTINUE on the START menu.
    pub(super) start_screen_sram_unmask_ret: u32,

    /// This is immediately after game initialization is complete: that is, the internal state is set correctly.
    ///
    /// At this point, it is safe to jump into the link battle menu.
    pub(super) game_load_ret: u32,

    /// This is directly after where KEYINPUT is read into r4 and then processed.
    ///
    /// Input is injected here directly by Tango into r4 from client. We avoid doing it via the usual input interrupt handling mechanism because this is more precise.
    pub(super) main_read_joyflags: u32,

    /// This hooks the return from the function that is called to determine the current state of copying input data.
    ///
    /// Expected values are: 2 if input is ready, 4 if remote has disconnected.
    pub(super) get_copy_data_input_state_ret: u32,

    /// This hooks the entry into the function that will copy received input data from rx_packet_arr into game state, as well as copies the next game state into tx_packet.
    ///
    /// Received packets should be injected here into rx_packet_arr.
    pub(super) copy_input_data_entry: u32,

    /// This hooks the exit into the function that will copy received input data from rx_packet_arr into game state, as well as copies the next game state into tx_packet.
    ///
    /// Packets to transmit should be injected here into tx_packet.
    pub(super) copy_input_data_ret: u32,

    /// This hooks the point after the game determines who the winner is, returned in r0.
    ///
    /// If r0 = 1, the local player won the last round.
    /// If r0 = 2, the remote player won the last round.
    /// Otherwise, the battle hasn't ended.
    pub(super) round_run_unpaused_step_cmp_retval: u32,

    /// This hooks the point after the battle start routine is complete.
    ///
    /// Tango initializes its own battle tracking state at this point.
    pub(super) round_start_ret: u32,

    /// This hooks the point when the round is ending and the game will process no further input.
    ///
    /// At this point, Tango will clean up its round state and commit the replay.
    pub(super) round_ending_ret: u32,

    /// This hooks the point after the battle end routine is complete.
    pub(super) round_end_entry: u32,

    /// This hooks the point determining if the player is player 2 or not.
    ///
    /// r0 should be set to the local player index.
    pub(super) battle_is_p2_tst: u32,

    /// This hooks another point determining if the player is player 2 or not.
    ///
    /// r0 should be set to the local player index.
    pub(super) link_is_p2_ret: u32,

    /// This is the entry point to the comm menu.
    ///
    /// Here, Tango jumps directly into link battle.
    pub(super) comm_menu_init_ret: u32,

    /// This is the entry point to link battle in the comm menu: that is, the first match has started.
    ///
    /// We need to perform some initialization we skipped here, such as setting stage and background.
    pub(super) comm_menu_init_battle_entry: u32,

    /// This handles underlying link cable SIO in the comm menu.
    ///
    /// This should never be called.
    pub(super) comm_menu_handle_link_cable_input_entry: u32,

    /// This handles in-battle link cable SIO in the comm menu.
    ///
    /// This should be skipped.
    pub(super) comm_menu_in_battle_call_comm_menu_handle_link_cable_input: u32,

    /// This hooks the entrypoint to the function that is called when a match ends.
    ///
    /// Tango ends its match here.
    pub(super) comm_menu_end_battle_entry: u32,

    /// This is where the opponent's name (usually MegaMan) is stored in ROM.
    ///
    /// This is strictly a whimsical thing. Set it to 0 if you don't care for it.
    pub(super) opponent_name: u32,
}

static EWRAM_OFFSETS_US: EWRAMOffsets = EWRAMOffsets {
    tx_packet: 0x2036780,
    rx_packet_arr: 0x020399f0,
    battle_state: 0x02034880,
    start_screen_control: 0x02011800,
    title_menu_control: 0x0200ad10,
    menu_control: 0x0200df20,
    submenu_control: 0x02009a30,
    rng1_state: 0x02001120,
    rng2_state: 0x020013f0,
};

static EWRAM_OFFSETS_JP: EWRAMOffsets = EWRAMOffsets {
    start_screen_control: 0x02011c00,
    ..EWRAM_OFFSETS_US
};

#[derive(Clone, Copy)]
pub struct Offsets {
    pub(super) rom: ROMOffsets,
    pub(super) ewram: EWRAMOffsets,
}

pub static MEGAMAN6_FXX: Offsets = Offsets {
    ewram: EWRAM_OFFSETS_US,
    rom: ROMOffsets {
        start_screen_jump_table_entry: 0x0803d1ca,
        start_screen_sram_unmask_ret: 0x0802f5ea,
        game_load_ret: 0x08004dde,
        main_read_joyflags: 0x080003fa,
        get_copy_data_input_state_ret: 0x0801feec,
        copy_input_data_entry: 0x0801ff18,
        copy_input_data_ret: 0x0801ffd4,
        round_run_unpaused_step_cmp_retval: 0x08008102,
        round_start_ret: 0x08007304,
        round_end_entry: 0x08007ca0,
        round_ending_ret: 0x0800951a,
        battle_is_p2_tst: 0x0803dd52,
        link_is_p2_ret: 0x0803dd86,
        comm_menu_init_ret: 0x08129298,
        comm_menu_init_battle_entry: 0x0812b608,
        comm_menu_handle_link_cable_input_entry: 0x0803eae4,
        comm_menu_in_battle_call_comm_menu_handle_link_cable_input: 0x0812b5ca,
        comm_menu_end_battle_entry: 0x0812b708,
        opponent_name: 0x086f12cb,
    },
};

pub static MEGAMAN6_GXX: Offsets = Offsets {
    ewram: EWRAM_OFFSETS_US,
    rom: ROMOffsets {
        start_screen_jump_table_entry: 0x0803d19e,
        start_screen_sram_unmask_ret: 0x0802f5ea,
        game_load_ret: 0x08004dde,
        main_read_joyflags: 0x080003fa,
        get_copy_data_input_state_ret: 0x0801feec,
        copy_input_data_entry: 0x0801ff18,
        copy_input_data_ret: 0x0801ffd4,
        round_run_unpaused_step_cmp_retval: 0x08008102,
        round_start_ret: 0x08007304,
        round_end_entry: 0x08007ca0,
        round_ending_ret: 0x0800951a,
        battle_is_p2_tst: 0x0803dd26,
        link_is_p2_ret: 0x0803dd5a,
        comm_menu_init_ret: 0x0812b074,
        comm_menu_init_battle_entry: 0x0812d3e4,
        comm_menu_handle_link_cable_input_entry: 0x0803eab8,
        comm_menu_in_battle_call_comm_menu_handle_link_cable_input: 0x0812d3a6,
        comm_menu_end_battle_entry: 0x0812d4e4,
        opponent_name: 0x086ef24f,
    },
};

pub static ROCKEXE6_RXX: Offsets = Offsets {
    ewram: EWRAM_OFFSETS_JP,
    rom: ROMOffsets {
        start_screen_jump_table_entry: 0x0803e23a,
        start_screen_sram_unmask_ret: 0x0803059a,
        game_load_ret: 0x08004dc2,
        main_read_joyflags: 0x080003fa,
        get_copy_data_input_state_ret: 0x08020300,
        copy_input_data_entry: 0x080203ea,
        copy_input_data_ret: 0x080204b6,
        round_run_unpaused_step_cmp_retval: 0x0800811a,
        round_start_ret: 0x080072f8,
        round_end_entry: 0x08007c9c,
        round_ending_ret: 0x080096ea,
        battle_is_p2_tst: 0x0803ed96,
        link_is_p2_ret: 0x0803edca,
        comm_menu_init_ret: 0x08131cbc,
        comm_menu_init_battle_entry: 0x08134008,
        comm_menu_handle_link_cable_input_entry: 0x0803fb28,
        comm_menu_in_battle_call_comm_menu_handle_link_cable_input: 0x08133fca,
        comm_menu_end_battle_entry: 0x08134108,
        opponent_name: 0,
    },
};

pub static ROCKEXE6_GXX: Offsets = Offsets {
    ewram: EWRAM_OFFSETS_JP,
    rom: ROMOffsets {
        start_screen_jump_table_entry: 0x0803e20e,
        start_screen_sram_unmask_ret: 0x0803059a,
        game_load_ret: 0x08004dc2,
        main_read_joyflags: 0x080003fa,
        get_copy_data_input_state_ret: 0x08020300,
        copy_input_data_entry: 0x080203ea,
        copy_input_data_ret: 0x080204b6,
        round_run_unpaused_step_cmp_retval: 0x0800811a,
        round_start_ret: 0x080072f8,
        round_end_entry: 0x08007c9c,
        round_ending_ret: 0x080096ea,
        battle_is_p2_tst: 0x0803ed6a,
        link_is_p2_ret: 0x0803ed9e,
        comm_menu_init_ret: 0x08133a84,
        comm_menu_init_battle_entry: 0x08135dd0,
        comm_menu_handle_link_cable_input_entry: 0x0803fafc,
        comm_menu_in_battle_call_comm_menu_handle_link_cable_input: 0x08135d92,
        comm_menu_end_battle_entry: 0x08135ed0,
        opponent_name: 0,
    },
};
