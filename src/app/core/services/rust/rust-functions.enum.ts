export enum RustFunctionName {
// dataSource service
    connectMidi = "start_listening_midi",
    stopMidi = "disconnect_midi",
    listMidiDevices = "list_midi_devices"
}

export enum RustEventsName {
    midiNote = "MIDI_NOTE"
}